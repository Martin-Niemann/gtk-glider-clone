use gtk::{Box, GestureSwipe, Orientation, Overlay, ScrolledWindow, Spinner};
use adw::{
    prelude::*, Banner, HeaderBar, NavigationPage, ToolbarView,
};
use gtk::glib::{self, clone};

use reqwest::Client;
use async_channel::Sender;

use crate::{application::{runtime, Event}, network::{fetch_stories, Item}, transform::{stories_to_card_data_transform, CardData}};

pub struct Feed {
    pub story_page: NavigationPage,
    pub news_feed: ScrolledWindow,
    pub reload_banner: Banner,
    pub spinner: Spinner
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
                .build();

            let spinner: Spinner = Spinner::new();
            spinner.set_spinning(true);
            spinner.set_opacity(0.0);
            spinner.set_sensitive(false);
            spinner.set_height_request(38);
            spinner.set_width_request(38);
            spinner.set_valign(gtk::Align::Start);
            spinner.set_halign(gtk::Align::Center);

            let reload_gesture: GestureSwipe = GestureSwipe::builder().button(0).n_points(1).build();

            unsafe { 
                reload_gesture.set_data("previous_swipe_point", 0.0);
                reload_gesture.set_data("previous_opacity", 0.0); 
            };

            reload_gesture.connect_begin(|gesture, sequence| unsafe {
                gesture.set_data("previous_swipe_point", gesture.point(sequence).unwrap().1);
                gesture.set_data("previous_opacity", gesture.point(sequence).unwrap().1);
            });

            reload_gesture.connect_update(clone!(@weak spinner, @weak news_feed, @strong sender, @strong client => move |gesture, sequence| unsafe  {
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
                        spawn_cards_fetch_and_send(&sender, &client);
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

            news_feed.add_controller(reload_gesture);

            let header_bar: HeaderBar = HeaderBar::builder().decoration_layout("").build();

            let content_container: Box = Box::new(Orientation::Vertical, 0);
            content_container.append(&reload_banner);
            content_container.append(&news_feed);

            let spinner_overlay: Overlay = Overlay::new();
            spinner_overlay.set_child(Some(&content_container));
            spinner_overlay.add_overlay(&spinner);

            let toolbar_view: ToolbarView = ToolbarView::builder().build();
            toolbar_view.add_top_bar(&header_bar);
            toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);
            toolbar_view.set_content(Some(&spinner_overlay));

            let story_page: NavigationPage = NavigationPage::builder()
                .title("Top Stories".to_string())
                .child(&toolbar_view)
                .build();

            let feed: Feed = Feed { story_page, news_feed, reload_banner, spinner };
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