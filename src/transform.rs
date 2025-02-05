use std::sync::OnceLock;

use async_channel::Sender;
use chrono::{DateTime, Utc};
use gtk::glib::{clone, markup_escape_text};
use reqwest::Client;
use tokio::runtime::Runtime;
use url::Url;

use crate::{
    application::Event,
    network::{fetch_stories, Item},
    story_object::StoryData,
};

// https://gtk-rs.org/gtk4-rs/stable/latest/book/main_event_loop.html#tokio
// reqwest requires the Tokio runtime, this initializes a Tokio runtime that is not blocked by the glib main loop(?)
pub fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

// spawns a Tokio runtime that uses reqwest to fetch stories,
// maps them to the Item model and returns these in a vector,
// then tranforms these into a vector of CardData, which is Item data that has been processed for putting into Card widgets,
// and finally sends them in a message on the Vec<CardData> async channel to be received by the watcher at an indeterminate point
pub fn spawn_cards_fetch_and_send(sender: &Sender<Event>, client: &Client) {
    runtime().spawn(clone!(
        #[strong]
        sender,
        #[strong]
        client,
        async move {
            let stories_result = fetch_stories(&client).await;
            let mut story_items: Vec<Item> = vec![];

            match stories_result {
                Ok(items) => story_items = items,
                Err(e) => println!("{}", e),
            }

            let card_data_vec: Vec<StoryData> = stories_to_card_data_transform(story_items);

            if card_data_vec.is_empty() {
                panic!("Failed to load any stories");
            }

            sender
                .send(Event::SentStoryData(card_data_vec))
                .await
                .expect("The channel needs to be open.");
        }
    ));
}

// process JSON data from the Hacker News API into presentable strings tailored for Card widgets
pub fn stories_to_card_data_transform(story_items: Vec<Item>) -> Vec<StoryData> {
    let mut story_data: Vec<StoryData> = vec![];
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

        let title_and_url: String = format!(
            "<span size=\"115%\">{}</span> <span foreground=\"grey\">({})</span>",
            markup_escape_text(story_item.title.unwrap_or("".to_string()).as_str()),
            url
        );

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

        let time_formatted: String = format!(
            "<span foreground=\"grey\">{}</span>",
            markup_escape_text(time_string.as_str())
        );

        story_data.push(StoryData {
            title_and_url,
            score_count: story_item.score.unwrap_or(0),
            comments_count: story_item.descendants.unwrap_or(0),
            author: story_item.by.unwrap_or("".to_string()),
            time_formatted,
        });
    });
    story_data
}
