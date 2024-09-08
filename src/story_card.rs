use glib::subclass::{
    types::{ObjectSubclass, ObjectSubclassExt},
    InitializingObject,
};
use gtk::{glib, prelude::ObjectExt, subclass::prelude::ObjectSubclassIsExt};
use gtk::glib::Object;
use gtk::subclass::widget::WidgetClassExt;
use gtk::subclass::{
    prelude::{ObjectImpl, ObjectImplExt},
    widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
};
use gtk::CompositeTemplate;
use gtk::{Label, TemplateChild};
use gtk::subclass::prelude::DerivedObjectProperties;
use std::cell::RefCell;
use glib::Properties;
use gtk::subclass::box_::BoxImpl;

use crate::transform::CardData;

glib::wrapper! {
    pub struct StoryCard(ObjectSubclass<imp::StoryCard>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Orientable;
}

impl StoryCard {
    pub fn new(card_data: CardData) -> Self {
        Object::builder()
        .property("card_data", card_data)
        .build()
    }

    fn setup_card(&self) {
        self.format_title_and_url();
        self.imp().score_count_label.set_label(&self.score_count().to_string());
        self.imp().comments_count_label.set_label(&self.comments_count().to_string());
        self.imp().author_label.set_label(&self.author());
        self.format_time_formatted();
    }

    fn format_title_and_url(&self) {
        self.imp().title_and_url_label.set_markup(
            format!(
                "<span size=\"115%\">{}</span> <span foreground=\"grey\">({})</span>",
                self.title(),
                self.url()
            )
            .as_str(),
        );
    }

    fn format_time_formatted(&self) {
        self.imp().time_formatted_label.set_markup(
            format!(
                "<span foreground=\"grey\">{}</span>",
                self.time_formatted()
            )
            .as_str(),
        );
    }
}

mod imp {
    use super::*;

    // ANCHOR: struct_and_subclass
    // Object holding the state
    #[derive(CompositeTemplate, Properties, Default)]
    #[properties(wrapper_type = super::StoryCard)]
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
        #[property(get, set, construct_only)]
        #[property(name = "title", get, set, type = String, member = title)]
        #[property(name = "url", get, set, type = String, member = url)]
        #[property(name = "score-count", get, set, type = u32, member = score)]
        #[property(name = "comments-count", get, set, type = u32, member = descendants)]
        #[property(name = "author", get, set, type = String, member = by)]
        #[property(name = "time-formatted", get, set, type = String, member = time)]
        card_data: RefCell<CardData>
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
    #[glib::derived_properties]
    impl ObjectImpl for StoryCard {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Setup
            let obj = self.obj();
            obj.setup_card();
        }
    }
    // ANCHOR_END: constructed

    // Trait shared by all widgets
    impl WidgetImpl for StoryCard {}

    // Trait shared by all NavigationPages
    impl BoxImpl for StoryCard {}
}
