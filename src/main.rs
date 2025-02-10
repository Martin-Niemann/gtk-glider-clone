pub mod application;
pub mod transform;
pub mod network;
pub mod window;
pub mod feed_page;
pub mod story_page;
pub mod story_card;
pub mod story_object;
pub mod gesture_box;

use adw::{prelude::*, Application};
use application::App;
use gtk::gio;

fn main() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    let application: Application = App::new();

    application.run();
}