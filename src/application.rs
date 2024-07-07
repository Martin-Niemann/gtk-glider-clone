use gtk::{glib::{self, clone}, Box, GestureDrag, Orientation};
use adw::{
    prelude::*, Application, ApplicationWindow, Bin, NavigationView,
};
use reqwest::Client;
use tokio::runtime::Runtime;
use std::sync::OnceLock;

use crate::{
    card::build_card, details::Details, feed::{spawn_cards_fetch_and_send, Feed}, transform::CardData
};

pub enum Event {
    SentCardData(Vec<CardData>),
    ClickedStory(),
}

pub struct App {}

impl App {
    pub fn new() -> Application {
        let application = Application::builder()
            .application_id("com.example.FirstAdwaitaApp")
            .build();

        application.connect_activate(move |app| {
            let (sender, receiver) = async_channel::bounded::<Event>(1);

            let client = Client::builder().use_rustls_tls().build().unwrap();

            // start fetching stories from the Hacker News API in parallel with the user interface being constructed 
            // speed is key for a mobile app, and this way the user has to wait less time before the content appears
            spawn_cards_fetch_and_send(&sender, &client);

            // initialize the screens
            let feed: Feed = Feed::new(&sender, &client);
            let details: Details = Details::new();

            let nav_view: NavigationView = NavigationView::builder().build();
            nav_view.add(&feed.story_page);

            let window: ApplicationWindow = ApplicationWindow::builder()
                .application(app)
                .title("First App")
                .content(&nav_view)
                .default_height(654)
                .default_width(328)
                .build();
            window.present();

            let event_handler = async move {
                // the user interface has now been initialized.
                // we now wait to recieve a Vec<CardData> on the async channel, 
                // then construct the card widgets and add them to the view to be displayed
                while let Ok(event) = receiver.recv().await {
                    match event {
                        Event::SentCardData(card_data_vec) => {
                            let container: Box = Box::builder()
                                .orientation(Orientation::Vertical)
                                .margin_top(7)
                                .margin_start(12)
                                .margin_end(12)
                                .build();
                    
                            card_data_vec.into_iter().for_each(|card_data| {
                                let sender_clone = sender.clone();
                                let is_click_gesture: GestureDrag = GestureDrag::builder().button(0).n_points(1).build();
                                is_click_gesture.connect_drag_end(move |gesture, _, _| {
                                    // even by just tapping the screen, a finished "drag" is registered.
                                    // we use this fact to set the treshold for what we consider a "click"
                                    // to be a drag of 2 pixels or less in either vertical direction.
                                    // to trigger a reload, the drag strength has to be above 70 pixels.
                                    if gesture.offset().is_some() && gesture.offset().unwrap().1 <= 2.0 && gesture.offset().unwrap().1 >= -2.0 {
                                        gesture.set_state(gtk::EventSequenceState::Claimed);
                                        println!("Clicked a story!");
                                        let send_clicked = clone!(@strong sender_clone => async move {
                                            sender_clone.send(Event::ClickedStory()).await.expect("The channel needs to be open.");
                                        });
                                        glib::spawn_future_local(send_clicked);
                                    } else {
                                        gesture.set_state(gtk::EventSequenceState::Denied);
                                    }
                                });
                        
                                let item: Bin = Bin::builder()
                                    .margin_bottom(18)
                                    .child(&build_card(card_data))
                                    .build();
                                item.add_controller(is_click_gesture);

                                container.append(&item);
                            });
                            feed.news_feed.set_child(Some(&container));
                            println!("all done! hiding banner.");
                            //feed.reload_banner.set_revealed(false);
                            //feed.spinner_revealer.set_reveal_child(false);
                            feed.spinner.set_opacity(0.0);
                            feed.spinner.set_margin_top(0);
                        },
                        Event::ClickedStory() => {
                            nav_view.push(&details.details_page);
                        },
                    }
                }
            };

            // spawns a infinitely running glib thread that handles all events received from signals
            glib::spawn_future_local(event_handler);
        });
        application
    }
}

// https://gtk-rs.org/gtk4-rs/stable/latest/book/main_event_loop.html#tokio
// reqwest requires the Tokio runtime, this initializes a Tokio runtime that is not blocked by the glib main loop(?)
pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}