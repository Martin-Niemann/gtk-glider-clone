use adw::subclass::prelude::AdwApplicationWindowImpl;
use adw::NavigationPage;
use adw::NavigationView;
use adw::Application;
use glib::subclass::InitializingObject;
use glib::Object;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib, subclass::prelude::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct GliderCloneWindow(ObjectSubclass<imp::GliderCloneWindow>)
        @extends adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl GliderCloneWindow {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    pub fn add_nav_page(&self, nav_page: &NavigationPage) {
        self.imp().nav_view.add(nav_page);
    }

    pub fn push_nav_page(&self, nav_page: &NavigationPage) {
        self.imp().nav_view.push(nav_page);
    }
}

mod imp {
    use super::*;

    // ANCHOR: struct_and_subclass
    // Object holding the state
    #[derive(CompositeTemplate, Default)]
    #[template(file = "src/ui/window.blp")]
    pub struct GliderCloneWindow {
        #[template_child]
        pub nav_view: TemplateChild<NavigationView>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for GliderCloneWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "GliderCloneWindow";
        type Type = super::GliderCloneWindow;
        type ParentType = adw::ApplicationWindow;

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
    impl ObjectImpl for GliderCloneWindow {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Setup
            let _obj = self.obj();
        }
    }
    // ANCHOR_END: constructed

    // Trait shared by all widgets
    impl WidgetImpl for GliderCloneWindow {}

    // Trait shared by all windows
    impl WindowImpl for GliderCloneWindow {}

    // Trait shared by all application windows
    impl ApplicationWindowImpl for GliderCloneWindow {}

    // Trait shared by all application windows
    impl AdwApplicationWindowImpl for GliderCloneWindow {}
}
