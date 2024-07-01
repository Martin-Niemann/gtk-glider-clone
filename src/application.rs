use gtk::{Box, GestureClick, GestureDrag, Orientation, ScrolledWindow};
use adw::{
    prelude::*, Application, ApplicationWindow, Banner, Bin, HeaderBar, NavigationPage,
    NavigationView, ToolbarView,
};
use gtk::glib::{self, clone};

use reqwest::Client;
use tokio::runtime::Runtime;

use std::sync::OnceLock;
use async_channel::Sender;

use crate::{
    card::build_card, network::{fetch_stories, Item}, transform::{stories_to_card_data_transform, CardData}
};

pub struct App {}

impl App {
    pub fn new() -> Application {
        let application = Application::builder()
            .application_id("com.example.FirstAdwaitaApp")
            .build();

        application.connect_activate(move |app| {
            let (sender, receiver) = async_channel::bounded(1);

            let client = Client::builder().use_rustls_tls().build().unwrap();

            // start fetching stories from the Hacker News API in parallel with the user interface being constructed 
            // speed is key for a mobile app, and this way the user has to wait less time before the content appears
            spawn_cards_fetch_and_send(&sender, &client);

            let reload_banner: Banner = Banner::builder()
                .button_label("")
                .revealed(false)
                .title("Reloading")
                .build();

            let reload_gesture: GestureDrag = GestureDrag::builder().button(0).n_points(1).build();
            
            reload_gesture.connect_drag_end(clone!(@weak reload_banner, @strong sender, @strong client => move |gesture, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                if gesture.offset().is_some() {
                    println!("{}", gesture.offset().unwrap().1);
                    if gesture.offset().unwrap().1 > 70.0 {
                        println!("we dragged down!!");
                        spawn_cards_fetch_and_send(&sender, &client);
                        reload_banner.set_title(format!("You pulled {}, and triggered a refresh!", gesture.offset().unwrap().1).as_str());
                        reload_banner.set_revealed(true);
                    }
                }
            }));

            let header_bar: HeaderBar = HeaderBar::builder().decoration_layout("").build();

            let news_feed: ScrolledWindow = ScrolledWindow::builder()
                .margin_bottom(0)
                .margin_top(0)
                .has_frame(false)
                .propagate_natural_height(true)
                //.child(&container)
                .build();
            news_feed.add_controller(reload_gesture);

            let content_container: Box = Box::new(Orientation::Vertical, 0);
            content_container.append(&reload_banner);
            content_container.append(&news_feed);

            let toolbar_view: ToolbarView = ToolbarView::builder().build();
            toolbar_view.add_top_bar(&header_bar);
            toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);
            toolbar_view.set_content(Some(&content_container));

            let story_page: NavigationPage = NavigationPage::builder()
                .title("Top Stories".to_string())
                .child(&toolbar_view)
                .build();

            let nav_view: NavigationView = NavigationView::builder().build();
            nav_view.add(&story_page);

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
                while let Ok(card_data_vec) = receiver.recv().await {
                    let container: Box = Box::builder()
                        .orientation(Orientation::Vertical)
                        .margin_top(7)
                        .margin_start(12)
                        .margin_end(12)
                        .build();
                    
                    card_data_vec.into_iter().for_each(|card_data| {
                        let gesture_click = GestureClick::new();
                        gesture_click.connect_pressed(|gesture_click, _, _, _| {
                            gesture_click.set_state(gtk::EventSequenceState::None);
                            println!("Clicked a story!");
                        });
                        
                        let item: Bin = Bin::builder()
                            .margin_bottom(18)
                            .child(&build_card(card_data))
                            .build();
                        item.add_controller(gesture_click);

                        container.append(&item);
                    });
                    news_feed.set_child(Some(&container));
                    println!("all done! hiding banner.");
                    reload_banner.set_revealed(false);
                }
            };

            // spawns a infinitely running glib thread that handles all events received from signals
            glib::spawn_future_local(event_handler);
        });
        application
    }
}

// spawns a Tokio runtime that uses reqwest to fetch stories,
// maps them to the Item model and returns these in a vector,
// then tranforms these into a vector of CardData, which is Item data that has been processed for putting into Card widgets,
// and finally sends them in a message on the Vec<CardData> async channel to be received by the watcher at an indeterminate point
fn spawn_cards_fetch_and_send(sender: &Sender<Vec<CardData>>, client: &Client) {
    runtime().spawn(clone!(@strong sender, @strong client => async move {
            
        let stories_result = fetch_stories(&client).await;
        let mut story_items: Vec<Item> = vec![];

        match stories_result {
            Ok(items) => story_items = items,
            Err(e) => println!("{}", e),
        }

        let card_data_vec: Vec<CardData> = stories_to_card_data_transform(story_items);

        if card_data_vec.is_empty() {
            panic!("Failed to load any stories");
        }

        sender.send(card_data_vec).await.expect("The channel needs to be open.");
    }));      
}

// https://gtk-rs.org/gtk4-rs/stable/latest/book/main_event_loop.html#tokio
// reqwest requires the Tokio runtime, this initializes a Tokio runtime that is not blocked by the glib main loop(?)
fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}