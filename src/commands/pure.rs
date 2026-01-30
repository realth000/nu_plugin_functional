use nu_plugin::SimplePluginCommand;
use nu_protocol::{Category, Example, Filesize, Signature, Type, Value};

use crate::FpPlugin;
#[derive(Clone)]
pub struct Pure;

impl SimplePluginCommand for Pure {
    type Plugin = FpPlugin;

    fn name(&self) -> &str {
        "fp pure"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .input_output_type(Type::Any, Type::OneOf(Box::new([Type::Any, Type::Nothing])))
            .category(Category::Conversions)
    }

    fn description(&self) -> &str {
        "Convert a value to `null` if it is the default value of its type"
    }

    fn extra_description(&self) -> &str {
        r#"For the following types:

* int: `0`
* float: `0.0`, `Nan`, `Inf`, `-Inf`
* string: (empty string) `''`, `""`
* boolean: `false`
* duration `0day`, `0hr`, `0min`, `0sec`
* fize-size: `0b`, `0kb`, `0mb`, `0gb`, `0tb`, `0pb`, `0eb`
* list: `[]`
* record: `{}`
* table: `[{}]`
* nothing: `null`

These values will be converted to `null`.

Types not supported:

* date
* range
* binary
* closure
* block
* any
* glob
* error
"#
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_protocol::LabeledError> {
        match input.get_type() {
            Type::Int => {
                if input.as_int().unwrap() == 0 {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::Float => {
                if is_null_float(input.as_float().unwrap()) {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::String => {
                if input.as_str().unwrap() == "" {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::Bool => {
                if !input.as_bool().unwrap() {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::Duration => {
                if input.as_duration().unwrap() == 0i64 {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::Filesize => {
                if input.as_filesize().unwrap() == Filesize::ZERO {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::List(_) => {
                if input.as_list().unwrap().is_empty() {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::Record(..) => {
                if input.as_record().unwrap().is_empty() {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::Table(data) => {
                if data.is_empty() {
                    Ok(Value::nothing(call.head))
                } else {
                    Ok(input.clone())
                }
            }
            Type::Nothing => Ok(input.clone()),
            Type::Number => {
                if let Ok(i) = input.as_int() {
                    if i == 0 {
                        Ok(Value::nothing(call.head))
                    } else {
                        Ok(input.clone())
                    }
                } else {
                    if is_null_float(input.as_float().unwrap()) {
                        Ok(Value::nothing(call.head))
                    } else {
                        Ok(input.clone())
                    }
                }
            }

            Type::Any
            | Type::Binary
            | Type::Block
            | Type::CellPath
            | Type::Closure
            | Type::Custom(_)
            | Type::Date
            | Type::Error
            | Type::OneOf(_)
            | Type::Range
            | Type::Glob => Ok(input.clone()),
        }
    }

    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            Example {
                description: "zero value int after pure",
                example: "0 | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "zero value float after pure",
                example: "0. | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "inf float after pure",
                example: "Inf | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "empty string after pure",
                example: "'' | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "false value after pure",
                example: "false | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "zero value duration after pure",
                example: "0day | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "zero value file size after pure",
                example: "0kb | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "empty list after pure",
                example: "[] | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "empty record after pure",
                example: "{} | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "empty table after pure",
                example: "[{}] | fp pure",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "nothing after pure",
                example: "null | fp pure",
                result: Some(Value::test_nothing()),
            },
        ]
    }
}

fn is_null_float(value: f64) -> bool {
    value.is_infinite() || value == 0f64
}
