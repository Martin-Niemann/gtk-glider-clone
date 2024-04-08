use adw::{
    prelude::*, Application, ApplicationWindow, Bin, HeaderBar, NavigationPage, NavigationView, ToolbarView
};
use chrono::{DateTime, Utc};
use futures::{stream::iter, StreamExt};
use gtk::{gio, Box, Image, Label, Orientation, ScrolledWindow, Widget};
use reqwest::Client;
use serde::Deserialize;
use url::Url;

#[derive(Clone)]
struct CardData {
    title: String,
    url: String,
    score: u32,
    descendants: u32,
    by: String,
    time: String,
}

#[derive(Deserialize, Debug, Clone, Copy)]
enum Type {
    job,
    story,
    comment,
    poll,
    pollopt,
}

#[derive(Deserialize, Debug, Clone)]
struct Item {
    id: u32,
    deleted: Option<bool>,
    r#type: Option<Type>,
    by: Option<String>,
    time: Option<i64>,
    text: Option<String>,
    dead: Option<bool>,
    parent: Option<u32>,
    poll: Option<u32>,
    kids: Option<Vec<u32>>,
    url: Option<String>,
    score: Option<u32>,
    title: Option<String>,
    parts: Option<Vec<u32>>,
    descendants: Option<u32>,
}

const ITEM_URL: &str = "https://hacker-news.firebaseio.com/v0/item/";
const ITEM_URL_TRAIL: &str = ".json";
const TOP_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

async fn fetch_stories(client: &Client) -> Result<Vec<Item>, String> {
    let story_ids: Vec<u32>;
    let mut stories: Vec<Item> = vec![];

    match fetch_ids(client).await {
        Ok(s) => story_ids = s[..20].to_vec(),
        Err(e) => return Err(e.to_string()),
    }

    let requests = iter(story_ids.clone())
        .map(|id| async move {
            let request = client
                .get(format!("{}{}{}", ITEM_URL, id, ITEM_URL_TRAIL))
                .send()
                .await?;
            request.json::<Item>().await
        })
        .buffered(story_ids.len());

    let responses: Vec<Result<Item, reqwest::Error>> = requests.collect().await;

    for response in responses {
        match response {
            Ok(item) => stories.push(item),
            Err(_e_) => (),
        }
    }

    Ok(stories)
}

async fn fetch_ids(client: &Client) -> Result<Vec<u32>, reqwest::Error> {
    let body = client
        .get(TOP_STORIES_URL)
        .send()
        .await?
        .json::<Vec<u32>>()
        .await?;

    Ok(body)
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

fn build_card(card_data: CardData) -> Widget {
    let top_widget = build_card_top_wiget(card_data.clone());
    let bottom_widget = build_card_bottom_widget(card_data.clone());
    top_widget.set_margin_bottom(10);

    let card_box: Box = Box::builder().orientation(Orientation::Vertical).build();
    card_box.append(&top_widget);
    card_box.append(&bottom_widget);
    card_box.into()
}

fn stories_to_card_data_transform(story_items: Vec<Item>) -> Vec<CardData> {
    let mut cards: Vec<CardData> = vec![];
    story_items.into_iter().for_each(|story_item| {
        let mut url: String = "".to_string();

        if story_item.url.is_some() {
            let parsed_url = Url::parse(story_item.url.clone().unwrap().as_str());

            if parsed_url.is_ok() && parsed_url.unwrap().host_str().is_some() {
                url = Url::parse(story_item.url.unwrap().as_str())
                    .unwrap()
                    .host_str()
                    .unwrap()
                    .to_string();
            }
        }

        let mut time_string: String = "".to_string();
        let date_time = DateTime::from_timestamp(story_item.time.unwrap_or(0), 0);
        let time_difference = Utc::now() - date_time.unwrap();

        if time_difference.num_days() == 0 {
            time_string = format!("{} hours ago", time_difference.num_hours());
        } else if time_difference.num_days() > 0 && time_difference.num_weeks() == 0 {
            time_string = format!("{} days ago", time_difference.num_days());
        } else if time_difference.num_weeks() > 0 && time_difference.num_weeks() < 4 {
            time_string = format!("{} weeks ago", time_difference.num_weeks());
        } else if time_difference.num_weeks() >= 4 && time_difference.num_weeks() < 52 {
            let num_months = ((time_difference.num_weeks() / 4) as f32).round() as u32;
            time_string = format!("{} months ago", num_months);
        } else if time_difference.num_weeks() >= 52 {
            let num_years: u32 = ((time_difference.num_weeks() / 52) as f32).round() as u32;
            time_string = format!("{} years ago", num_years);
        }

        cards.push(CardData {
            title: story_item.title.unwrap_or("".to_string()),
            url,
            score: story_item.score.unwrap_or(0),
            descendants: story_item.descendants.unwrap_or(0),
            by: story_item.by.unwrap_or("".to_string()),
            time: time_string,
        });
    });
    cards
}

#[tokio::main]
async fn main() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    let client = Client::new();

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

    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_activate(move |app| {
        let header_bar: HeaderBar = HeaderBar::builder()
            .decoration_layout("")
            .build();

        let container: Box = Box::builder()
            .orientation(Orientation::Vertical)
            .margin_start(12)
            .margin_end(12)
            .build();

        let news_feed: ScrolledWindow = ScrolledWindow::builder()
            .margin_bottom(0)
            .margin_top(0)
            .has_frame(false)
            .propagate_natural_height(true)
            .max_content_height(300)
            .child(&container)
            .build();

        <Vec<CardData> as Clone>::clone(&card_data_vec)
            .into_iter()
            .for_each(|card_data| {
                let item: Bin = Bin::builder()
                    .margin_bottom(18)
                    .child(&build_card(card_data))
                    .build();
                container.append(&item);
            });

        let toolbar_view: ToolbarView = ToolbarView::builder().build();
        toolbar_view.add_top_bar(&header_bar);
        toolbar_view.set_top_bar_style(adw::ToolbarStyle::Flat);
        toolbar_view.set_content(Some(&news_feed));

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
            .build();
        window.present();
    });

    application.run();
}
