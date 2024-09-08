use adw::subclass::prelude::NavigationPageImpl;
use glib::subclass::{
    types::{ObjectSubclass, ObjectSubclassExt},
    InitializingObject,
};
use gtk::{glib::clone, prelude::ObjectExt, subclass::prelude::ObjectSubclassIsExt};
use adw::prelude::GestureExt;
use gtk::glib;
use gtk::glib::Object;
use gtk::subclass::{
    prelude::{ObjectImpl, ObjectImplExt},
    widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
};
use gtk::CompositeTemplate;
use gtk::{ScrolledWindow, TemplateChild};
use adw::Banner;
use gtk::Spinner;
use gtk::Box;
use gtk::GestureSwipe;
use gtk::prelude::AdjustmentExt;
use gtk::prelude::WidgetExt;
use gtk::prelude::EventControllerExt;
use gtk::subclass::widget::WidgetClassExt;
use std::sync::OnceLock;
use glib::subclass::Signal;

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

    pub fn set_cards(&self, container: &Box) {
        self.imp().news_feed.set_child(Some(container));
    }

    pub fn reset_spinner(&self) {
        self.imp().spinner.set_opacity(0.0);
        self.imp().spinner.set_margin_top(0);
    }

    fn setup_gesture(&self) {
        let reload_gesture = self.imp().reload_gesture.get();
        let news_feed = self.imp().news_feed.get();
        let spinner = self.imp().spinner.get();
        
        reload_gesture.connect_begin(|gesture, sequence| unsafe {
            gesture.set_data("previous_swipe_point", gesture.point(sequence).unwrap().1);
            gesture.set_data("previous_opacity", gesture.point(sequence).unwrap().1);
        });

        reload_gesture.connect_update(clone!(@weak self as obj_self, @weak spinner, @weak news_feed => move |gesture, sequence| unsafe  {
            //are we scrolled all the way to the top of the feed?
            if news_feed.vadjustment().value() == 0.0 {
                let previous_swipe_point = gesture.steal_data::<f64>("previous_swipe_point").unwrap();
                let swipe_point_difference: f64 = 
                    (gesture.point(sequence).unwrap().1 - previous_swipe_point).clamp(-1.0, 1.0) 
                    + ((gesture.point(sequence).unwrap().1 - previous_swipe_point) / 2.0);
                gesture.set_data("previous_swipe_point", gesture.point(sequence).unwrap().1);
                spinner.set_margin_top(spinner.margin_top() + swipe_point_difference as i32);
                println!("spinner margin: {}, previous margin: {}, difference: {}", spinner.margin_top(), previous_swipe_point, swipe_point_difference);

                let opacity_difference: f64 = (gesture.point(sequence).unwrap().1 - gesture.steal_data::<f64>("previous_opacity").unwrap()) / 100.0;
                gesture.set_data("previous_opacity", gesture.point(sequence).unwrap().1);
                spinner.set_opacity(spinner.opacity() + opacity_difference);

                // did we drag more than 70 pixels downwards?
                if spinner.margin_top() > 70 {
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                    gesture.reset();
                    println!("we dragged down!!");
                    spinner.set_margin_top(36);
                    gesture.set_data("previous_swipe_point", 0.0);
                    obj_self.emit_by_name::<()>("fetch-cards", &[]);
                    //spawn_cards_fetch_and_send(&sender, &client);
                }
            }
        }));

        reload_gesture.connect_end(clone!(@weak spinner => move |gesture, _| unsafe  {
            if !gesture.is_active() {
                spinner.set_opacity(0.0);
                spinner.set_margin_top(0);
                gesture.set_data("previous_swipe_point", 0.0);
            }
        }));
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
        pub news_feed: TemplateChild<ScrolledWindow>,
        #[template_child]
        pub reload_banner: TemplateChild<Banner>,
        #[template_child]
        pub spinner: TemplateChild<Spinner>,
        #[template_child]
        pub reload_gesture: TemplateChild<GestureSwipe>,
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
            obj.setup_gesture();
        }

        fn signals() -> &'static [glib::subclass::Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![Signal::builder("fetch-cards").build()]
            })
        }
    }
    // ANCHOR_END: constructed

    // Trait shared by all widgets
    impl WidgetImpl for FeedPage {}

    // Trait shared by all NavigationPages
    impl NavigationPageImpl for FeedPage {}
}
