use nu_plugin::SimplePluginCommand;
use nu_protocol::{Category, Example, Signature, Spanned, SyntaxShape, Type, Value, engine};

use crate::FpPlugin;

#[derive(Clone)]
pub struct Else;

impl SimplePluginCommand for Else {
    type Plugin = FpPlugin;

    fn name(&self) -> &str {
        "fp else"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .input_output_type(Type::Any, Type::Any)
            .required(
                "value",
                SyntaxShape::Any,
                "The value (or how to produce the value) to use when input is `null`",
            )
            .category(Category::Conversions)
    }

    fn description(&self) -> &str {
        "Use another value when input is `null`."
    }

    fn extra_description(&self) -> &str {
        r#"Use another value to continue the pipeline if input is `null`.

The value can be a direct value, or a closure-like statement that produces that value."#
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
            if let Ok(c) = call.req::<Spanned<engine::Closure>>(0) {
                Ok(engine.eval_closure(&c, vec![], Some(input.clone()))?)
            } else {
                Ok(call.req(0)?)
            }
        } else {
            Ok(input.clone())
        }
    }

    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            Example {
                description: "Use \"foo\" if input is null",
                example: "null | fp else foo",
                result: Some(Value::test_string("foo")),
            },
            Example {
                description: "Use \"foo\" if input is null, input is not null",
                example: "1 | fp else foo",
                result: Some(Value::test_int(1)),
            },
            Example {
                description: "Use 200 if no element in a list is larger than 5",
                example: "[1, 2, 4] | fp first-where $it > 5 | fp else 100",
                result: Some(Value::test_int(100)),
            },
            Example {
                description: "Use 200 if no element in a list is larger than 5, input is not null",
                example: "[1, 2, 4, 8] | fp first-where $it > 5 | fp else 100",
                result: Some(Value::test_int(8)),
            },
        ]
    }
}
