use glib::subclass::{types::ObjectSubclass, InitializingObject};
use gtk::glib::Object;
use gtk::subclass::box_::BoxImpl;
use gtk::subclass::widget::WidgetClassExt;
use gtk::subclass::{
    prelude::{ObjectImpl, ObjectImplExt},
    widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
};
use gtk::CompositeTemplate;
use gtk::{
    glib::{self},
    prelude::ObjectExt,
    subclass::prelude::ObjectSubclassIsExt,
};
use gtk::{Label, TemplateChild};
use std::cell::RefCell;

use crate::story_object::StoryObject;

glib::wrapper! {
    pub struct StoryCard(ObjectSubclass<imp::StoryCard>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for StoryCard {
    fn default() -> Self {
        Self::new()
    }
}

impl StoryCard {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, story_object: &StoryObject) {
        let title_and_url_label = self.imp().title_and_url_label.get();
        let score_count_label = self.imp().score_count_label.get();
        let comments_count_label = self.imp().comments_count_label.get();
        let author_label = self.imp().author_label.get();
        let time_formatted_label = self.imp().time_formatted_label.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        bindings.push(
            story_object
                .bind_property("title-and-url", &title_and_url_label, "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            story_object
                .bind_property("score-count", &score_count_label, "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            story_object
                .bind_property("comments-count", &comments_count_label, "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            story_object
                .bind_property("author", &author_label, "label")
                .sync_create()
                .build(),
        );

        bindings.push(
            story_object
                .bind_property("time-formatted", &time_formatted_label, "label")
                .sync_create()
                .build(),
        );
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}

mod imp {
    use glib::Binding;

    use super::*;

    // ANCHOR: struct_and_subclass
    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(file = "src/ui/story_card.blp")]
    pub struct StoryCard {
        #[template_child]
        pub title_and_url_label: TemplateChild<Label>,
        #[template_child]
        pub score_count_label: TemplateChild<Label>,
        #[template_child]
        pub comments_count_label: TemplateChild<Label>,
        #[template_child]
        pub author_label: TemplateChild<Label>,
        #[template_child]
        pub time_formatted_label: TemplateChild<Label>,
        pub bindings: RefCell<Vec<Binding>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for StoryCard {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "StoryCard";
        type Type = super::StoryCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }
    // ANCHOR_END: struct_and_subclass

    // ANCHOR: constructed
    // Trait shared by all GObjects
    impl ObjectImpl for StoryCard {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Setup
            //let obj = self.obj();
        }
    }
    // ANCHOR_END: constructed

    // Trait shared by all widgets
    impl WidgetImpl for StoryCard {}

    // Trait shared by all NavigationPages
    impl BoxImpl for StoryCard {}
}
