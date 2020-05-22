extern crate cfg_if;
extern crate wasm_bindgen;

// mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use chrono::{TimeZone, Utc};
use rss::{ChannelBuilder, ImageBuilder, ItemBuilder};
use serde_json::Value;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn parser(json: &str) -> String {
    let mut items: Vec<rss::Item> = vec![];

    let json: Value = serde_json::from_str(json).unwrap();

    let contents = json["graphql"]["user"]["edge_owner_to_timeline_media"]["edges"]
        .as_array()
        .unwrap();
    for content in contents {
        let content = &content["node"];

        let title = match content["accessibility_caption"].as_str() {
            Some(caption) => caption.to_string(),
            None => format!(
                "Photo by {}{}",
                json["graphql"]["user"]["full_name"].as_str().unwrap(),
                match content["location"]["name"].as_str() {
                    Some(location) => format!(" at {}", location),
                    None => "".to_string(),
                }
            ),
        };

        let photos = match content["edge_sidecar_to_children"]["edges"].as_array() {
            Some(medias) => medias
                .iter()
                .map(|media| {
                    format!(
                        "<img src=\"{}\" />",
                        media["node"]["display_url"].as_str().unwrap()
                    )
                })
                .collect(),
            None => vec![format!(
                "<img src=\"{}\" />",
                content["display_url"].as_str().unwrap()
            )],
        };
        let desc = format!(
            "<p>{}</p>{}",
            content["edge_media_to_caption"]["edges"][0]["node"]["text"]
                .as_str()
                .unwrap()
                .to_string(),
            photos.concat()
        );
        let link = format!(
            "https://www.instagram.com/p/{}/",
            content["shortcode"].as_str().unwrap()
        );
        let date = content["taken_at_timestamp"].as_i64().unwrap();
        let date = Utc.timestamp(date, 0);

        let item = ItemBuilder::default()
            .title(title)
            .description(desc)
            .link(link)
            .pub_date(date.to_rfc2822())
            .build()
            .unwrap();
        items.push(item);
    }

    let title = format!(
        "{} (@{}) from instagram",
        json["graphql"]["user"]["full_name"].as_str().unwrap(),
        json["graphql"]["user"]["username"].as_str().unwrap()
    );
    let link = format!(
        "https://www.instagram.com/{}",
        json["graphql"]["user"]["username"].as_str().unwrap()
    );
    let description = json["graphql"]["user"]["biography"].as_str().unwrap();
    let image = format!(
        "{}",
        json["graphql"]["user"]["profile_pic_url_hd"]
            .as_str()
            .unwrap()
    );

    let image = ImageBuilder::default()
        .title(&title)
        .link(&link)
        .url(image)
        .build()
        .unwrap();

    ChannelBuilder::default()
        .title(&title)
        .description(description)
        .link(&link)
        .image(image)
        .items(items)
        .build()
        .unwrap()
        .to_string()
}
