use nu_plugin::Plugin;

use crate::commands::{FirstWhere, Handle, Is, Main, Other, Pure, Then};

mod commands;

pub struct FpPlugin;

impl Plugin for FpPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(FirstWhere),
            Box::new(Handle),
            Box::new(Is),
            Box::new(Main),
            Box::new(Other),
            Box::new(Pure),
            Box::new(Then),
        ]
    }
}
