use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_functional::FpPlugin;

fn main() {
    serve_plugin(&FpPlugin, MsgPackSerializer {});
}
