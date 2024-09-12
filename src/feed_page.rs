use adw::subclass::prelude::NavigationPageImpl;
use glib::subclass::{
    types::{ObjectSubclass, ObjectSubclassExt},
    InitializingObject,
};
use gtk::glib::Object;
use gtk::prelude::{Cast, CastNone};
use gtk::subclass::widget::WidgetClassExt;
use gtk::subclass::{
    prelude::{ObjectImpl, ObjectImplExt},
    widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
};
use gtk::{ListItem, SignalListItemFactory};
use gtk::CompositeTemplate;
use gtk::{glib, NoSelection};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::TemplateChild;
use std::cell::RefCell;


use gtk::{gio::ListStore, ListView};
use gtk::prelude::ListItemExt;

use crate::story_card::StoryCard;
use crate::story_object::{StoryData, StoryObject};

glib::wrapper! {
    pub struct FeedPage(ObjectSubclass<imp::FeedPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget;
}

impl FeedPage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    fn cards(&self) -> ListStore {
        self.imp()
            .cards
            .borrow()
            .clone()
            .expect("Could not get current cards.")
    }

    fn setup_model_and_view(&self) {
        // Create new model
        let model = ListStore::new::<StoryObject>();

        // Get state and set model
        self.imp().cards.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.cards()));
        self.imp().cards_list.set_model(Some(&selection_model));
    }

    pub fn setup_cards(&self, story_data_vec: Vec<StoryData>) {
        // this may be a candidate for using rayon?
        // https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html
        for story_data in story_data_vec {
            let story_object = StoryObject::new(story_data);
            self.cards().append(&story_object);
        }
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `StoryCard` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `StoryCard`
            let story_card = StoryCard::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&story_card));
        });

        // Tell factory how to bind `StoryCard` to a `StoryObject`
        factory.connect_bind(move |_, list_item| {
            // Get `StoryObject` from `ListItem`
            let story_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<StoryObject>()
                .expect("The item has to be an `StoryObject`.");

            // Get `StoryCard` from `ListItem`
            let story_card = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<StoryCard>()
                .expect("The child has to be a `StoryCard`.");

            story_card.bind(&story_object);
        });

        // Tell factory how to unbind `StoryCard` from `StoryObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `TaskRow` from `ListItem`
            let story_card = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<StoryCard>()
                .expect("The child has to be a `StoryCard`.");

            story_card.unbind();
        });

        // Set the factory of the list view
        self.imp().cards_list.set_factory(Some(&factory));
    }
}

mod imp {
    use super::*;

    // ANCHOR: struct_and_subclass
    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(file = "src/ui/feed_page.blp")]
    pub struct FeedPage {
        #[template_child]
        pub cards_list: TemplateChild<ListView>,
        pub cards: RefCell<Option<ListStore>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for FeedPage {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "FeedPage";
        type Type = super::FeedPage;
        type ParentType = adw::NavigationPage;

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
    impl ObjectImpl for FeedPage {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Setup
            let obj = self.obj();
            obj.setup_model_and_view();
            obj.setup_factory();
        }

        //fn signals() -> &'static [glib::subclass::Signal] {
        //    static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        //    SIGNALS.get_or_init(|| vec![Signal::builder("fetch-cards").build()])
        //}
    }
    // ANCHOR_END: constructed

    // Trait shared by all widgets
    impl WidgetImpl for FeedPage {}

    // Trait shared by all NavigationPages
    impl NavigationPageImpl for FeedPage {}
}
