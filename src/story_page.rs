use adw::subclass::prelude::NavigationPageImpl;
use glib::subclass::{
    types::{ObjectSubclass, ObjectSubclassExt},
    InitializingObject,
};
use gtk::glib;
use gtk::glib::Object;
use gtk::subclass::{
    prelude::{ObjectImpl, ObjectImplExt},
    widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
};
use gtk::CompositeTemplate;

glib::wrapper! {
    pub struct StoryPage(ObjectSubclass<imp::StoryPage>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget;
}

impl StoryPage {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use super::*;

    // ANCHOR: struct_and_subclass
    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(file = "src/ui/story_page.blp")]
    pub struct StoryPage { }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for StoryPage {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "StoryPage";
        type Type = super::StoryPage;
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
    impl ObjectImpl for StoryPage {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Setup
            let _obj = self.obj();
        }
    }
    // ANCHOR_END: constructed

    // Trait shared by all widgets
    impl WidgetImpl for StoryPage {}

    // Trait shared by all NavigationPages
    impl NavigationPageImpl for StoryPage {}
}
