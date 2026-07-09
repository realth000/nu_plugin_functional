use nu_plugin::{EvaluatedCall, PluginCommand};
use nu_protocol::{Category, IntoSpanned, LabeledError, Signature, SyntaxShape, Value};

use crate::FpPlugin;

#[derive(Clone)]
pub struct Invoke;

impl PluginCommand for Invoke {
    type Plugin = FpPlugin;

    fn name(&self) -> &str {
        "fp invoke"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(self.name())
            .required(
                "cmdline",
                SyntaxShape::List(Box::new(SyntaxShape::String)),
                "Command and arguments to invoke",
            )
            .category(Category::Shells)
    }

    fn description(&self) -> &str {
        "Invoke nushell native commands with arguments, all passed in a list."
    }

    fn extra_description(&self) -> &str {
        r#"Invoke nushell native commands with arguments.

The command should be the first element in the option and all later elements
are the arguments passed to the command.

This plugin command is intended to use with nushell native commands.

For calling external commands with list of arguments, use `...$arg_list` syntax.
    e.g. `^ls ...[-a, -l]` == `^ls -a -l`

This plugin command is here because nushell native commands can not be used with
spreading args syntax.
"#
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: nu_protocol::PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
        let contents = call.req::<Vec<Value>>(0)?;
        if contents.is_empty() {
            return Err(
                LabeledError::new(format!("Empty command")).with_label("empty cmdline", call.head)
            );
        }

        let command = match contents[0].clone().into_string() {
            Ok(v) => v,
            Err(e) => {
                return Err(
                    LabeledError::new(format!("Invalid command name")).with_label(
                        format!("command can not be convert into string: {e}"),
                        contents[0].span(),
                    ),
                );
            }
        };

        let command = engine.find_decl(&command).unwrap().unwrap();
        let mut evaled_call = EvaluatedCall::new(call.head);

        for raw_arg in contents.into_iter().skip(1) {
            let parsed_arg = raw_arg.as_str().unwrap().trim_start_matches(|ch| ch == '-');
            if parsed_arg.len() == 1 {
                return Err(
                    LabeledError::new(format!("Short name flags are not allowed")).with_label(
                        format!("Short name flag \"{parsed_arg}\" not works at runtime, use long flag name instead"),
                        raw_arg.span(),
                    ).with_help("For example, use `--long` instead `-l` for ls command"),
                );
            }
            evaled_call.add_flag(parsed_arg.into_spanned(call.head));
        }

        let result = engine.call_decl(
            command,
            evaled_call,
            nu_protocol::PipelineData::Empty,
            true,
            false,
        )?;

        Ok(result)
    }
}
