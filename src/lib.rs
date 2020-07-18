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
    let json: Value = serde_json::from_str(json).unwrap();
    let json = &json["graphql"]["user"];

    let username = json["username"].as_str().unwrap_or("unknown");
    let contents = json["edge_owner_to_timeline_media"]["edges"]
        .as_array()
        .unwrap();

    let mut items: Vec<rss::Item> = vec![];
    for content in contents {
        let content = &content["node"];

        let description = content["edge_media_to_caption"]["edges"][0]["node"]["text"].as_str();
        let alt = content["accessibility_caption"].as_str();
        let location = content["location"]["name"].as_str();

        let title = alt.map(|alt| alt.to_string()).unwrap_or(format!(
            "Photo shared by @{}{}",
            username,
            location
                .map(|location| format!(" at {}", location))
                .unwrap_or(String::new())
        ));
        let photos = match content["edge_sidecar_to_children"]["edges"].as_array() {
            Some(medias) => medias
                .iter()
                .map(|media| {
                    format!(
                        "<img src=\"{}\"{}>",
                        media["node"]["display_url"].as_str().unwrap(),
                        media["node"]["accessibility_caption"]
                            .as_str()
                            .map(|alt| format!(" alt=\"{}\"", alt))
                            .unwrap_or(String::new())
                    )
                })
                .collect(),
            None => vec![format!(
                "<img src=\"{}\"{}>",
                content["display_url"].as_str().unwrap(),
                alt.map(|alt| format!(" alt=\"{}\"", alt))
                    .unwrap_or(String::new())
            )],
        };
        let description = description
            .map(|description| format!("<p>{}</p>{}", description, photos.concat()))
            .unwrap_or(photos.concat());
        let link = format!(
            "https://www.instagram.com/p/{}/",
            content["shortcode"].as_str().unwrap()
        );
        let date = content["taken_at_timestamp"].as_i64().unwrap_or(0);
        let date = Utc.timestamp(date, 0);

        let item = ItemBuilder::default()
            .title(title)
            .description(description)
            .link(link)
            .pub_date(date.to_rfc2822())
            .build()
            .unwrap();
        items.push(item);
    }

    let title = match json["full_name"].as_str() {
        Some(fullname) => format!("{} (@{}) from instagram", fullname, username),
        None => format!("@{} from instagram", username),
    };
    let link = format!("https://www.instagram.com/{}", username);
    let description = json["biography"].as_str().unwrap_or("");
    let icon = json["profile_pic_url_hd"].as_str().unwrap_or("");

    let image = ImageBuilder::default()
        .title(&title)
        .link(&link)
        .url(icon)
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
