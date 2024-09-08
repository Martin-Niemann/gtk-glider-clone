use futures::{stream::iter, StreamExt};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Item {
    pub id: u32,
    pub deleted: Option<bool>,
    pub r#type: Option<Type>,
    pub by: Option<String>,
    pub time: Option<i64>,
    pub text: Option<String>,
    pub dead: Option<bool>,
    pub parent: Option<u32>,
    pub poll: Option<u32>,
    pub kids: Option<Vec<u32>>,
    pub url: Option<String>,
    pub score: Option<u32>,
    pub title: Option<String>,
    pub parts: Option<Vec<u32>>,
    pub descendants: Option<u32>,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Type {
    job,
    story,
    comment,
    poll,
    pollopt,
}

const ITEM_URL: &str = "https://hacker-news.firebaseio.com/v0/item/";
const ITEM_URL_TRAIL: &str = ".json";
const TOP_STORIES_URL: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

pub async fn fetch_stories(client: &Client) -> Result<Vec<Item>, String> {
    let mut stories: Vec<Item> = vec![];

    let story_ids: Vec<u32> = match fetch_ids(client).await {
        Ok(s) => s[..20].to_vec(),
        Err(e) => return Err(e.to_string()),
    };

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
            Ok(item) => {
                stories.push(item);
            }
            Err(_e_) => (),
        }
    }

    Ok(stories)
}

pub async fn fetch_ids(client: &Client) -> Result<Vec<u32>, reqwest::Error> {
    let body = client
        .get(TOP_STORIES_URL)
        .send()
        .await?
        .json::<Vec<u32>>()
        .await?;

    Ok(body)
}