use chrono::{DateTime, Utc};
use url::Url;

use crate::network::Item;

#[derive(Clone)]
pub struct CardData {
    pub title: String,
    pub url: String,
    pub score: u32,
    pub descendants: u32,
    pub by: String,
    pub time: String,
}

// process JSON data from the Hacker News API into presentable strings tailored for Card widgets
pub fn stories_to_card_data_transform(story_items: Vec<Item>) -> Vec<CardData> {
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
