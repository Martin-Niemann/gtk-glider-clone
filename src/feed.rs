use std::fmt::Debug;

use gtk::{Box, GestureDrag, GestureSwipe, Orientation, Overlay, Revealer, ScrolledWindow, Spinner};
use adw::{
    prelude::*, Banner, HeaderBar, NavigationPage, ToolbarView,
};
use gtk::glib::{self, clone};

use reqwest::Client;
use async_channel::Sender;

use crate::{application::{runtime, Event}, network::{fetch_stories, Item}, transform::{stories_to_card_data_transform, CardData}};

// make a struct where I can expose both story_page as well as reload_banner and whatever else becomes neccessary
// very nice as it it very easily be possible to see which widgets are modified throghout runtime

pub struct Feed {
    pub story_page: NavigationPage,
    pub news_feed: ScrolledWindow,
    pub reload_banner: Banner,
    pub spinner_revealer: Revealer
}

impl Feed {
    pub fn new(sender: &Sender<Event>, client: &Client) -> Feed {
        let reload_banner: Banner = Banner::builder()
                .button_label("")
                .revealed(false)
                .title("Reloading")
                .build();

            let news_feed: ScrolledWindow = ScrolledWindow::builder()
                .margin_bottom(0)
                .margin_top(0)
                .has_frame(false)
                .propagate_natural_height(true)
                //.child(&container)
                .build();

            let spinner: Spinner = Spinner::new();
            spinner.set_spinning(true);
            spinner.set_sensitive(false);
            spinner.set_height_request(38);
            spinner.set_width_request(38);
            //spinner.set_vexpand(false);
            //spinner.set_hexpand(false);
            spinner.set_valign(gtk::Align::Start);
            spinner.set_halign(gtk::Align::Center);
            spinner.set_margin_top(32);

            let spinner_revealer: Revealer = Revealer::new();
            spinner_revealer.set_child(Some(&spinner));
            spinner_revealer.set_transition_type(gtk::RevealerTransitionType::SlideDown);
            spinner_revealer.set_transition_duration(300);
            spinner_revealer.set_sensitive(false);

            let reload_gesture: GestureDrag = GestureDrag::builder().button(0).n_points(1).build();
            
            reload_gesture.connect_drag_end(clone!(@weak reload_banner, @weak spinner_revealer, @weak news_feed, @strong sender, @strong client => move |gesture, _, _| {
                // are we scrolled all the way to the top of the feed?
                if news_feed.vadjustment().value() == 0.0 {
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                    if gesture.offset().is_some() {
                        println!("{}", gesture.offset().unwrap().1);
                        // did we drag more than 70 pixels downwards?
                        if gesture.offset().unwrap().1 > 70.0 {
                            println!("we dragged down!!");
                            spinner_revealer.set_reveal_child(true);
                            spawn_cards_fetch_and_send(&sender, &client);
                            //reload_banner.set_title(format!("You pulled {}, and triggered a refresh!", gesture.offset().unwrap().1).as_str());
                            //reload_banner.set_revealed(true);
                        }
                    }
                };
            }));

            reload_gesture.connect_drag_update(|_, x, y| {
                println!("x: {}, y: {}", x, y);
            });

            // opdater spinnerens placering efterhånden som brugeren dragger ned, 
            // check om det er muligt at ændre den til at spinne langsomt/spinne når brugeren dragger via CSS.
            //reload_gesture.connect_drag_update()

            // når reload er ovre, lav en glib thread med et loop der reducerer opacity indtil spinneren er usynlig.

            news_feed.add_controller(reload_gesture);

            let header_bar: HeaderBar = HeaderBar::builder().decoration_layout("").build();

            let content_container: Box = Box::new(Orientation::Vertical, 0);
            content_container.append(&reload_banner);
            content_container.append(&news_feed);

            let spinner_overlay: Overlay = Overlay::new();
            spinner_overlay.set_child(Some(&content_container));
            spinner_overlay.add_overlay(&spinner_revealer);

            let toolbar_view: ToolbarView = ToolbarView::builder().build();
            toolbar_view.add_top_bar(&header_bar);
            toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);
            toolbar_view.set_content(Some(&spinner_overlay));

            let story_page: NavigationPage = NavigationPage::builder()
                .title("Top Stories".to_string())
                .child(&toolbar_view)
                .build();

            let feed: Feed = Feed { story_page, news_feed, reload_banner, spinner_revealer };
            feed
    }
}

// spawns a Tokio runtime that uses reqwest to fetch stories,
// maps them to the Item model and returns these in a vector,
// then tranforms these into a vector of CardData, which is Item data that has been processed for putting into Card widgets,
// and finally sends them in a message on the Vec<CardData> async channel to be received by the watcher at an indeterminate point
pub fn spawn_cards_fetch_and_send(sender: &Sender<Event>, client: &Client) {
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

        sender.send(Event::SentCardData(card_data_vec)).await.expect("The channel needs to be open.");
    }));
}