use nu_plugin::SimplePluginCommand;
use nu_protocol::{Category, Example, Signature, SyntaxShape, Type, Value};

use crate::FpPlugin;

#[derive(Clone)]
pub struct Is;

impl SimplePluginCommand for Is {
    type Plugin = FpPlugin;

    fn name(&self) -> &str {
        "fp is"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(SimplePluginCommand::name(self))
            .input_output_type(Type::Any, Type::Bool)
            .required("type", SyntaxShape::String, "The expected type")
            .category(Category::Formats)
    }

    fn description(&self) -> &str {
        "Check if the input data is a spcified type or not."
    }

    fn extra_description(&self) -> &str {
        r#"For any input data, check the data is a type or not, returns true if is."#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["type", "type-check"]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        let target_type = call.req::<String>(0)?;
        let input_type = input.get_type();
        let ty = input_type.to_string();

        // TODO: Check on generic types with semantic type meaning.
        //
        // e.g.
        //
        // `{name: "Alice", rank: 10} | fp is 'record<name: string, rank: int>' shall be true
        // `{name: "Alice", rank: 10} | fp is 'record<name:string,rank:int>' shall be true, despite the spaces.
        // `{name: "Alice", rank: 10} | fp is 'record<name, rank>' shall be true, without specifying field types.
        // `{name: "Alice", rank: 10} | fp is 'record<name: string, rank>' shall be true, with partial specified field types.
        // `{name: "Alice", rank: 10} | fp is 'record<rank, name>' shall be true, with unordered field names.

        let table_check = if target_type.as_str() == "table" {
            input_type.is_subtype_of(&Type::table())
        } else {
            false
        };
        let record_check = if target_type.as_str() == "record" {
            input_type.is_subtype_of(&Type::record())
        } else {
            false
        };
        let list_check = if target_type.as_str() == "list" {
            input_type.is_subtype_of(&Type::list(Type::Any))
        } else {
            false
        };

        Ok(Value::bool(
            ty == target_type || table_check || record_check || list_check,
            call.head,
        ))
    }

    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            Example {
                description: "Check input is int type or not",
                example: "1 | fp is int",
                result: Some(Value::test_bool(true)),
            },
            Example {
                description: "Check input is string type or not",
                example: "1 | fp is string",
                result: Some(Value::test_bool(false)),
            },
            Example {
                description: "Check input is list type or not",
                example: "[1, 2] | fp is list",
                result: Some(Value::test_bool(true)),
            },
            Example {
                description: "Check input is list of int or not",
                example: "[1, 2] | fp is list<int>",
                result: Some(Value::test_bool(true)),
            },
        ]
    }
}
