use serde::Deserialize;
use crate::substring::Substring;
use chrono::Datelike;
use crate::args::ProgramArgs;

#[derive(Deserialize)]
struct StatEntry {
    header:     String,
    title:      String,
    time:       String,
    subtitles:  Vec<StatSubtitleEntry>
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

#[derive(Deserialize, Clone)]
struct ItemSnippet {
    title:      String,

    #[serde(rename(deserialize = "channelName"))]
    channel_name: String
}

pub async fn parse(input: &str, conf: &ProgramArgs) -> (Vec<String>, Vec<String>){
    let entries: Vec<StatEntry> = serde_json::from_str(input).unwrap();

    let mut artists = Vec::default();
    let mut songs = Vec::default();
    let mut ids_to_process = Vec::default();

    let year = chrono::Utc::now().year();
    for entry in entries {
        if !entry.header.eq("YouTube Music") {
            continue;
        }

        let title = entry.title.clone().substring(8).to_string();
        let time = chrono::DateTime::parse_from_rfc3339(&entry.time).unwrap();
        if time.year() < year {
            continue;
        }

        if title.starts_with("https://") {
            let id = title.substring(32);
            ids_to_process.push(id.to_string());
        } else {
            let artist = {
                let mut artist= String::default();
                for subtitle in entry.subtitles {
                    artist = subtitle.name;
                }

                fix_artist_name(artist)
            };

            artists.push(artist);
            songs.push(title);
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
            .query(&[("key", conf.api_key.as_ref().unwrap().as_str()), ("id", &id), ("part", "snippet")])
            .send()
            .await;

        let r = match r {
            Ok(r) => r,
            Err(e) => panic!("{:#?}", e)
        };

        let json: ApiResponse = r.json().await.unwrap();
        for entry in json.items {
            let snippet = entry.snippet.clone();
            let artist = fix_artist_name(snippet.channel_name);

            artists.push(artist);
            songs.push(snippet.title);
        }
    }

    (artists, songs)
}

fn fix_artist_name(i: String) -> String {
    let i = i.split("-").collect::<Vec<&str>>().get(0).unwrap();
    let i = i.split(" ").collect::<Vec<&str>>();

    let mut artist = Vec::new();
    for e in i {
        if e.is_empty() { continue; }
        artist.push(e.clone());
    }

    let artist: String = artist.join(" ");

    let artist = {
        let a = artist.replace("VEVO", "");
        let a = a.to_uppercase();

        let regex = regex::Regex::new(r#"(?<!_)(?=[A-Z])"#).unwrap();

        let a = a.chars().nth(1).unwrap().to_string() + &regex.replace_all(&a, "").to_string();
        a
    };

    artist

}