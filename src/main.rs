pub mod application;
pub mod transform;
pub mod network;
pub mod card;
pub mod feed;
pub mod details;

use adw::{prelude::*, Application};
use application::App;
use gtk::gio;

fn main() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    let application: Application = App::new();

    application.run();
}