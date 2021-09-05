use serde::Deserialize;
use crate::substring::Substring;
use chrono::Datelike;
use crate::Args;
use std::ops::Deref;

#[derive(Deserialize)]
struct StatEntry {
    header:     String,
    title:      String,
    time:       String,
    subtitles:  Option<Vec<StatSubtitleEntry>>
}

#[derive(Deserialize)]
struct StatSubtitleEntry {
    name:   String
}

#[derive(Deserialize)]
struct ApiResponse {
    items: Vec<ItemEntry>
}

#[derive(Deserialize)]
struct ItemEntry {
    snippet: ItemSnippet
}

#[derive(Deserialize)]
struct ItemSnippet {
    title:          String,

    #[serde(rename(deserialize = "channelTitle"))]
    channel_name:   String
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Artist(pub String);
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Track(pub String);

impl Deref for Artist {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Track {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn parse(input: &str, conf: &Args<'_>) -> (Vec<Artist>, Vec<Track>) {
    let mut entries: Vec<StatEntry> = serde_json::from_str(input).unwrap();

    let mut artists = Vec::default();
    let mut tracks = Vec::default();
    let mut ids_to_process = Vec::default();

    let year = conf.year;
    for entry in entries.drain(..) {
        if !entry.header.eq("YouTube Music") {
            continue;
        }

        let time = chrono::DateTime::parse_from_rfc3339(&entry.time).unwrap();
        if time.year() as u64 != year {
            continue;
        }

        let title = entry.title.clone().substring(8).to_string();

        if title.starts_with("https://") {
            let id = title.substring(32);
            ids_to_process.push(id);
        } else {
            let artist = {
                let mut artist= String::default();
                for subtitle in entry.subtitles.unwrap() {
                    artist = subtitle.name;
                }

                fix_artist_name(artist)
            };

            artists.push(Artist(artist));
            tracks.push(Track(title));
        }
    }

    let mut id_strings = Vec::default();
    let mut tmp = String::default();
    for i in 0..ids_to_process.len() {
        let s = ids_to_process.get(i).unwrap();
        tmp += s;

        if i % 50 == 0 {
            id_strings.push(tmp.clone());
            tmp.clear();
        } else {
            tmp += ",";
        }
    }

    for id in id_strings {
        let client = reqwest::Client::new();
        let r = client.get("https://youtube.googleapis.com/youtube/v3/videos")
            .query(&[("key", conf.api_key), ("id", id.as_str()), ("part", "snippet")])
            .send()
            .await;

        let r = match r {
            Ok(r) => r,
            Err(e) => panic!("{:#?}", e)
        };

        let txt = r.text().await.unwrap();
        let mut json: ApiResponse = serde_json::from_str(&txt).unwrap();

        for entry in json.items.drain(..) {
            let snippet = entry.snippet;
            let artist = fix_artist_name(snippet.channel_name);

            artists.push(Artist(artist));
            tracks.push(Track(snippet.title));
        }
    }

    (artists, tracks)
}

fn fix_artist_name(i: String) -> String {
    let i = i.split("-").collect::<Vec<&str>>();
    let i = i.get(0).unwrap();
    let mut i = i.split(" ").collect::<Vec<&str>>();

    let mut artist = Vec::new();
    for e in i.drain(..) {
        if e.is_empty() { continue; }
        artist.push(e);
    }

    let artist: String = artist.join(" ");

    let artist = {
        let a = artist.replace("VEVO", "");
        let a = a.to_uppercase();

        let chars: Vec<char> = a.chars().collect();
        let mut str = String::new();
        for i in 0..chars.len() {
            let curr_char = chars.get(i).unwrap();

            if i == 0 {
                str.push(curr_char.to_ascii_uppercase());
                continue;
            }

            let prev_char = chars.get(i-1).unwrap();
            if *prev_char == ' ' {
                str.push(curr_char.to_ascii_uppercase());
                continue;
            }

            str.push(curr_char.to_ascii_lowercase());
        }

        str
    };

    artist

}