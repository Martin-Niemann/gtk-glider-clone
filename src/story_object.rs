use glib::Object;
use gtk::glib;

use std::cell::RefCell;

use glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct StoryObject(ObjectSubclass<imp::StoryObject>);
}

#[derive(Clone, Debug, Default, glib::Boxed)]
#[boxed_type(name = "CardData")]
pub struct StoryData {
    pub title_and_url: String,
    pub score_count: u32,
    pub comments_count: u32,
    pub author: String,
    pub time_formatted: String,
}

impl StoryObject {
    pub fn new(story_data: StoryData) -> Self {
        Object::builder()
            .property("data", story_data)
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::StoryObject)]
    pub struct StoryObject {
        #[property(get, set, construct_only)]
        #[property(name = "title-and-url", get, set, type = String, member = title_and_url)]
        #[property(name = "score-count", get, set, type = u32, member = score_count)]
        #[property(name = "comments-count", get, set, type = u32, member = comments_count)]
        #[property(name = "author", get, set, type = String, member = author)]
        #[property(name = "time-formatted", get, set, type = String, member = time_formatted)]
        pub data: RefCell<StoryData>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for StoryObject {
        const NAME: &'static str = "StoryObject";
        type Type = super::StoryObject;
    }

    // Trait shared by all GObjects
    #[glib::derived_properties]
    impl ObjectImpl for StoryObject {}
}
