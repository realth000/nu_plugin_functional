use nu_plugin::SimplePluginCommand;
use nu_protocol::{
    Category, Example, Signature, Spanned, SyntaxShape, Type, Value, engine::Closure,
};

use crate::FpPlugin;

#[derive(Clone)]
pub struct Then;

impl SimplePluginCommand for Then {
    type Plugin = FpPlugin;

    fn name(&self) -> &str {
        "fp then"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .input_output_type(Type::Any, Type::Any)
            .required(
                "value",
                SyntaxShape::OneOf(vec![
                    SyntaxShape::Any,
                    SyntaxShape::Closure(Some(vec![SyntaxShape::Any])),
                    SyntaxShape::Closure(None),
                ]),
                "The value (or how to produce the value) to use when input is not `null`.",
            )
            .category(Category::Conversions)
    }

    fn description(&self) -> &str {
        "Do something with the input when input is not `null`."
    }

    fn extra_description(&self) -> &str {
        r#"Do something (e.g. eval a closure) when input is not `null`, return `null` if input is `null`."#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["transform", "conversion"]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_protocol::LabeledError> {
        if input.is_nothing() {
            Ok(input.clone())
        } else {
            if let Ok(c) = call.req::<Spanned<Closure>>(0) {
                // It is safe to always pass the input value to closure, because both:
                //
                // 1 | fp then { || $in + 2 }
                // and
                // 1 | fp then { |x| $x + 2 }
                // works.
                Ok(engine.eval_closure(&c, vec![input.clone()], Some(input.clone()))?)
            } else {
                Ok(call.req(0)?)
            }
        }
    }

    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            Example {
                description: "Use 100 if input is not null",
                example: "1 | fp then 100",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "Increase the value by 2 if input is not null",
                example: "1 | fp then { $in + 2 }",
                result: Some(Value::test_int(3)),
            },
            Example {
                description: "Increase the value by 2 if input is not null, input is null",
                example: "null | fp then { $in + 2 }",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "Use $foo + 2 if input is not null",
                example: "let foo = 2; 1 | fp then { $foo + 2 }",
                result: Some(Value::test_int(4)),
            },
        ]
    }
}
