use nu_plugin::SimplePluginCommand;
use nu_protocol::{
    Category, Example, Signature, Spanned, SyntaxShape, Type, Value,
    engine::{self, Closure},
};

use crate::FpPlugin;

#[derive(Clone)]
pub struct Handle;

impl SimplePluginCommand for Handle {
    type Plugin = FpPlugin;

    fn name(&self) -> &str {
        "fp handle"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .input_output_type(Type::Any, Type::Any)
            .required(
                "on_good",
                SyntaxShape::OneOf(vec![
                    SyntaxShape::Any,
                    SyntaxShape::Closure(Some(vec![SyntaxShape::Any])),
                    SyntaxShape::Closure(None),
                ]),
                "The value (or how to produce the value) to use when input is positive.",
            )
            .required(
                "on_bad",
                SyntaxShape::OneOf(vec![
                    SyntaxShape::Any,
                    SyntaxShape::Closure(Some(vec![SyntaxShape::Any])),
                    SyntaxShape::Closure(None),
                ]),
                "The value (or how to produce the value) to use when input is negative.",
            )
            .category(Category::Conversions)
    }

    fn description(&self) -> &str {
        "Do something when the input is good or do some other thing when input is bad. A combination of `then` and `other`."
    }

    fn extra_description(&self) -> &str {
        r#"Do something (e.g. eval a closure) when input is not `null` and do some other thing if input is `null`.
        A combination of `then` and `other`."#
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
            // The thing we do in `other` command.
            if let Ok(c) = call.req::<Spanned<engine::Closure>>(1) {
                Ok(engine.eval_closure(&c, vec![], Some(input.clone()))?)
            } else {
                Ok(call.req(1)?)
            }
        } else {
            // The thing we do in `then` command.
            if let Ok(c) = call.req::<Spanned<Closure>>(0) {
                Ok(engine.eval_closure(&c, vec![input.clone()], Some(input.clone()))?)
            } else {
                Ok(call.req(0)?)
            }
        }
    }

    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            Example {
                description: "Increase the input by 200 if input is not null, or use 200",
                example: "100 | fp handle {$in + 200} {200}",
                result: Some(Value::test_int(300)),
            },
            Example {
                description: "Increase the input by 200 if input is not null, or use 200, input is null",
                example: "null | fp handle {$in + 200} {200}",
                result: Some(Value::test_int(200)),
            },
            Example {
                description: "Map the elements in input list with +10 if input is not null, or use [200,300,400]",
                example: "[1,2,3] | fp pure | fp handle {|x| $x | each {$in + 10} } {[200,300,400]}",
                result: Some(Value::test_list(vec![
                    Value::test_int(11),
                    Value::test_int(12),
                    Value::test_int(13),
                ])),
            },
            Example {
                description: "Map the elements in input list with +10 if input is not null, or use [200,300,400], input is null",
                example: "[] | fp pure | fp handle {|x| $x | each {$in + 10} } {[200,300,400]}",
                result: Some(Value::test_list(vec![
                    Value::test_int(200),
                    Value::test_int(300),
                    Value::test_int(400),
                ])),
            },
        ]
    }
}
