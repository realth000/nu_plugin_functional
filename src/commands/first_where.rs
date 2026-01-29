use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{
    Category, Example, LabeledError, PipelineData, Signature, SyntaxShape, Type, Value, record,
};

use crate::FpPlugin;

#[derive(Clone)]
pub struct FirstWhere;

// ref: https://github.com/nushell/nushell/blob/main/crates/nu-command/src/filters/where_.rs
impl PluginCommand for FirstWhere {
    type Plugin = FpPlugin;

    fn name(&self) -> &str {
        "fp first-where"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .input_output_types(vec![
                (
                    Type::List(Box::new(Type::Any)),
                    Type::OneOf(Box::new([Type::Any, Type::Nothing])),
                ),
                (
                    Type::table(),
                    Type::OneOf(Box::new([Type::record(), Type::Nothing])),
                ),
                (
                    Type::Range,
                    Type::OneOf(Box::new([Type::Any, Type::Nothing])),
                ),
            ])
            .required(
                "condition",
                SyntaxShape::RowCondition, // RowCondition covers Clousure type.
                "Row condition or closure to filter the first element satify",
            )
            .category(Category::Filters)
    }

    fn description(&self) -> &str {
        "Find the first element which meets a condition."
    }

    fn extra_description(&self) -> &str {
        r#"
Find the first element which meets a condition, returns `null` if no element meets the condition.

Supported input types:

* List
* Table
* Range
"#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["filter", "find", "search", "condition"]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        // ref: https://github.com/nushell/plugin-examples/blob/main/rust/nu_plugin_example/src/commands/for_each.rs
        let closure = call.req(0)?;

        for value in input {
            let result = engine.eval_closure(&closure, vec![value.clone()], Some(value.clone()))?;
            if result.is_true() {
                return Ok(PipelineData::Value(value, None));
            }
        }

        Ok(PipelineData::Value(Value::nothing(call.head), None))
    }

    fn examples(&self) -> Vec<nu_protocol::Example<'_>> {
        vec![
            Example {
                description: "Filter the first element in a list that larger than 5",
                example: "[1, 2, 4, 8] | fp first-where $it > 5",
                result: Some(Value::test_int(8)),
            },
            Example {
                description: "Filter the first element in a list that larger than 5, using closure as condition",
                example: "[1, 2, 4, 8] | fp first-where {|x| $x > 5}",
                result: Some(Value::test_int(8)),
            },
            Example {
                description: "Filter the first element in a list that larger than 5, null result",
                example: "[1, 2, 4] | fp first-where $it > 5",
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "Filter the first row in a table that name start with 's'",
                example: r#"[{name: "Alice", rank: 10}, {name: "Bob", rank: 7}] | fp first-where $it.name =~ "A.*""#,
                result: Some(Value::test_record(record! {
                    "name" => Value::test_string("Alice"),
                    "rank" => Value::test_int(10),
                })),
            },
            Example {
                description: "Filter the first row in a table that name start with 's', using closure as condition",
                example: r#"[{name: "Alice", rank: 10}, {name: "Bob", rank: 7}] | fp first-where {|x| $x.name | str starts-with A}"#,
                result: Some(Value::test_record(record! {
                    "name" => Value::test_string("Alice"),
                    "rank" => Value::test_int(10),
                })),
            },
            Example {
                description: "Filter the first row in a table that name start with 's', null result",
                example: r#"[{name: "Bob", rank: 7}] | fp first-where {|x| $x.name | str starts-with A}"#,
                result: Some(Value::test_nothing()),
            },
            Example {
                description: "Filter the first element in a range that name larger than 5",
                example: "1..10 | fp first-where $it > 5",
                result: Some(Value::test_int(6)),
            },
        ]
    }
}
