extern crate azul;
use azul::prelude::*;
use azul::widgets::text_input::*;


// extern crate rusty_yaml;
// use rusty_yaml::*;

// mod buildbot;
// use buildbot::{MasterYaml};




/// This will be used to run Rusty-CI commands like `Setup`,
/// `Build`, `Start`, `Kill`, and `Install`
struct RustyCIGui {
    text_input: TextInputState,
    // master_yaml: MasterYaml
}


impl Default for RustyCIGui {
    fn default() -> Self {
        Self {
            text_input: TextInputState::new("Hover mouse over rectangle and press keys"),
        }
    }
}

impl Layout for RustyCIGui {
    fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
        TextInput::new()
            .bind(info.window, &self.text_input, &self)
            .dom(&self.text_input)
            .with_id("text_input_1")
    }
}



const CSS: &str = "";

fn main() {
    let mut app = App::new(
        RustyCIGui::default(),
        AppConfig::default()
    ).unwrap();

    let css = css::override_native(CSS).unwrap();
    
    let window = app.create_window(
        WindowCreateOptions::default(), css
    ).unwrap();

    app.run(window).unwrap();
}
