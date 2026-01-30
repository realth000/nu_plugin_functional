use nu_plugin::SimplePluginCommand;
use nu_protocol::{Category, Signature, Value};

use crate::FpPlugin;

mod else_;
mod first_where;
mod is;
mod pure;
mod then;

pub use else_::Else;
pub use first_where::FirstWhere;
pub use is::Is;
pub use pure::Pure;
pub use then::Then;

pub struct Main;

impl SimplePluginCommand for Main {
    type Plugin = FpPlugin;

    fn name(&self) -> &str {
        "fp"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name()).category(Category::Custom("functional".into()))
    }

    fn description(&self) -> &str {
        "Several functional programming style commands for pipelines."
    }

    fn extra_description(&self) -> &str {
        r#"The `fp` plugin provides several functional programming style commands to help piping commands.

Provided commands:

* else
* first-where
* is
* then"#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["fp", "functional"]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_protocol::LabeledError> {
        Ok(Value::string(engine.get_help()?, call.head))
    }
}
