use std::borrow::Borrow;

use adw::{prelude::*, Application};
use gtk::glib::{self, closure_local, Bytes};
use reqwest::Client;

use crate::{
    feed_page::FeedPage, story_object::StoryData, story_page::StoryPage,
    transform::spawn_cards_fetch_and_send, window::GliderCloneWindow,
};

pub enum Event {
    SentStoryData(Vec<StoryData>),
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

            // initialize the application screens
            let feed_page: FeedPage = FeedPage::new();

            //setup listener to react when the feed page wants to refresh the feed
            feed_page.connect_closure(
                "fetch-cards",
                false,
                closure_local!(
                    #[strong]
                    sender,
                    move |_: FeedPage| {
                        spawn_cards_fetch_and_send(&sender, &client);
                    }
                ),
            );

            let story_page: StoryPage = StoryPage::new();

            let window = GliderCloneWindow::new(app);

            let provider = gtk::CssProvider::new();
            provider.load_from_bytes(&Bytes::from_static(include_bytes!("./ui/style.css")));

            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not connect to a display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            window.add_nav_page(&feed_page.borrow());
            window.present();

            let event_handler = async move {
                // the user interface has now been initialized.
                // we now wait to recieve a Vec<StoryData> on the async channel,
                // then construct the card widgets and add them to the view to be displayed
                while let Ok(event) = receiver.recv().await {
                    match event {
                        Event::SentStoryData(story_data_vec) => {
                            feed_page.setup_cards(story_data_vec);
                        }
                        Event::ClickedStory() => {
                            window.push_nav_page(&story_page.borrow());
                        }
                    }
                }
            };

            // spawns a future on the glib thread for handling all events received from the async channel
            glib::spawn_future_local(event_handler);
        });
        application
    }
}
