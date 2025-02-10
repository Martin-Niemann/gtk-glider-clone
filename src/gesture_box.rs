use glib::subclass::types::{ObjectSubclass, ObjectSubclassExt};
use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::WidgetExt;
use gtk::subclass::box_::BoxImpl;
use gtk::subclass::{
    prelude::{ObjectImpl, ObjectImplExt},
    widget::WidgetImpl,
};

glib::wrapper! {
    pub struct GestureBox(ObjectSubclass<imp::GestureBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, adw::Swipeable;
}

impl GestureBox {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use std::cell::Cell;

    use adw::subclass::prelude::SwipeableImpl;

    use super::*;

    // ANCHOR: struct_and_subclass
    // Object holding the state
    #[derive(Default)]
    pub struct GestureBox {
        pub swipe_progress: Cell<f64>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for GestureBox {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "GestureBox";
        type Type = super::GestureBox;
        type ParentType = gtk::Box;
        type Interfaces = (adw::Swipeable,);
    }
    // ANCHOR_END: struct_and_subclass

    // ANCHOR: constructed
    // Trait shared by all GObjects
    impl ObjectImpl for GestureBox {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();

            // Setup
            let obj = self.obj();
        }

        //fn signals() -> &'static [glib::subclass::Signal] {
        //    static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        //    SIGNALS.get_or_init(|| vec![Signal::builder("fetch-cards").build()])
        //}
    }
    // ANCHOR_END: constructed

    // Trait shared by all widgets
    impl WidgetImpl for GestureBox {}

    // Trait shared by all Boxes
    impl BoxImpl for GestureBox {}

    impl SwipeableImpl for GestureBox {
        fn cancel_progress(&self) -> f64 {
            0.0
        }

        fn distance(&self) -> f64 {
            self.obj().height() as f64
        }

        fn progress(&self) -> f64 {
            self.swipe_progress.get()
        }

        fn snap_points(&self) -> Vec<f64> {
            vec![-0.0, 0.0, 1.0]
        }

        fn swipe_area(
            &self, navigation_direction: adw::NavigationDirection, is_drag: bool) -> gtk::gdk::Rectangle {
            gtk::gdk::Rectangle::new(0, 0, self.obj().width(), self.obj().height())
        }
    }
}
