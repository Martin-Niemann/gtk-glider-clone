use gtk::{Box, GestureDrag, Image, Label, Orientation, ScrolledWindow, Widget};
use adw::{
    prelude::*, Application, ApplicationWindow, Banner, Bin, HeaderBar, NavigationPage,
    NavigationView, ToolbarView,
};
use gtk::glib::{self, clone};

use reqwest::Client;
use tokio::runtime::Runtime;

use std::sync::OnceLock;
use async_channel::{Receiver, Sender};

use crate::{
    transform::{stories_to_card_data_transform, CardData},
    network::{fetch_stories, Item},
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

            let container: Box = Box::builder()
                .orientation(Orientation::Vertical)
                .margin_top(7)
                .margin_start(12)
                .margin_end(12)
                .build();

            let news_feed: ScrolledWindow = ScrolledWindow::builder()
                .margin_bottom(0)
                .margin_top(0)
                .has_frame(false)
                .propagate_natural_height(true)
                .child(&container)
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

            // the user interface has now been initialized.
            // we now wait to recieve a Vec<CardData> on the async channel, 
            // then construct the card widgets and add them to the view to be displayed
            spawn_cards_reciever_watcher(receiver, window, container, reload_banner);

        });
        application
    }
}

// https://gtk-rs.org/gtk4-rs/stable/latest/book/main_event_loop.html#tokio
// reqwest requires the Tokio runtime, this initializes a Tokio runtime that is not blocked by the glib main loop(?)
fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
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

// spawns a infinitely running glib thread that watches for new messages on the Vec<CardData> async channel,
// builds wrapped cards from the CardData structs, then adds them to the stories container widget to be displayed
fn spawn_cards_reciever_watcher(receiver: Receiver<Vec<CardData>>, window: ApplicationWindow, container: Box, reload_banner: Banner) {
    glib::spawn_future_local(async move {
        loop {
            if let Ok(card_data_vec) = receiver.recv().await {
                card_data_vec.into_iter().for_each(|card_data| {
                    let item: Bin = Bin::builder()
                        .margin_bottom(18)
                        .child(&build_card(card_data))
                        .build();
                    container.append(&item);
                });
                println!("all done! hiding banner.");
                reload_banner.set_revealed(false);
            }
            // kill this thread when the user closes the application
            if window.in_destruction() {
                break;
            }
        }
    });
}

fn build_card(card_data: CardData) -> Widget {
    let top_widget = build_card_top_wiget(card_data.clone());
    let bottom_widget = build_card_bottom_widget(card_data.clone());
    top_widget.set_margin_bottom(10);

    let card_box: Box = Box::builder().orientation(Orientation::Vertical).build();
    card_box.append(&top_widget);
    card_box.append(&bottom_widget);
    card_box.into()
}

fn build_card_top_wiget(card_data: CardData) -> Widget {
    let top_label: Label = Label::builder()
        .xalign(0.0)
        .wrap(true)
        .natural_wrap_mode(gtk::NaturalWrapMode::None)
        .wrap_mode(gtk::pango::WrapMode::Word)
        .lines(2)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .build();

    top_label.set_markup(
        format!(
            "<span size=\"115%\">{}</span> <span foreground=\"grey\">({})</span>",
            card_data.title.clone(),
            &card_data.url.clone()
        )
        .as_str(),
    );
    let top_label_widget: Widget = top_label.into();

    let top_box: Box = Box::builder().orientation(Orientation::Horizontal).build();
    top_box.append(&top_label_widget);

    top_box.into()
}

fn build_card_bottom_widget(card_data: CardData) -> Widget {
    let arrow_icon_image: Image = Image::from_resource(
        "/org/gtk/gtk-glider-clone/icons/scalable/actions/arrow2-up-symbolic.svg",
    );
    arrow_icon_image.set_pixel_size(12);
    let arrow_icon_widget: Widget = arrow_icon_image.into();
    arrow_icon_widget.set_margin_end(4);

    let score_text: Label = Label::new(Some(&card_data.score.to_string()));
    score_text.set_size_request(12, 12);
    let score_text_widget: Widget = score_text.into();

    let chat_icon_image: Image = Image::from_resource(
        "/org/gtk/gtk-glider-clone/icons/scalable/actions/chat-bubble-emtpy-symbolic.svg",
    );
    chat_icon_image.set_pixel_size(12);
    let chat_icon_widget: Widget = chat_icon_image.into();
    chat_icon_widget.set_margin_start(10);
    chat_icon_widget.set_margin_end(4);

    let comments_text: Label = Label::new(Some(&card_data.descendants.to_string()));
    comments_text.set_size_request(12, 12);
    let comments_text_widget: Widget = comments_text.into();

    let by_text: Label = Label::new(Some(&card_data.by));
    by_text.set_size_request(12, 12);
    let by_text_widget: Widget = by_text.into();
    by_text_widget.set_margin_start(10);

    let time_text: Label = Label::builder().build();
    time_text.set_markup(
        format!(
            "<span foreground=\"grey\">{}</span>",
            card_data.time.clone(),
        )
        .as_str(),
    );
    time_text.set_size_request(12, 12);
    let time_text_widget: Widget = time_text.into();
    time_text_widget.set_halign(gtk::Align::End);
    time_text_widget.set_hexpand(true);

    let bottom_box: Box = Box::builder().orientation(Orientation::Horizontal).build();
    bottom_box.append(&arrow_icon_widget);
    bottom_box.append(&score_text_widget);
    bottom_box.append(&chat_icon_widget);
    bottom_box.append(&comments_text_widget);
    bottom_box.append(&by_text_widget);
    bottom_box.append(&time_text_widget);
    bottom_box.into()
}
