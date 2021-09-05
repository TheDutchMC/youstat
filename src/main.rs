use std::fs;
use std::collections::HashMap;
use std::hash::Hash;
use clap::{App, Arg};
use chrono::Datelike;

mod parser;
mod substring;

pub struct Args<'a> {
    pub api_key:        &'a str,
    pub input_file:     &'a str,
    pub year:           u64,
    pub max:            u64
}

/// We define the main function manually, rather than using `async fn main()` and `#[tokio::main]` so we don't have to enable Tokio's `macro` feature
fn main() -> std::io::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .expect("Failed to create Tokio runtime");
    let _guard = rt.enter();
    rt.block_on(program())
}

async fn program() -> std::io::Result<()> {
    let matches = App::new("youstat")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("api_key")
            .short("k")
            .long("api-key")
            .value_name("api key")
            .help("Google API key")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("input_file")
            .short("i")
            .long("input-file")
            .value_name("input file")
            .help("Google Takeout export file")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("year")
            .short("y")
            .long("year")
            .value_name("year")
            .help("The year Youstat should parse statistics for")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("max")
            .short("m")
            .long("max")
            .value_name("max")
            .help("The maximum amount of stats to produce per category")
            .takes_value(true)
            .required(false))
        .get_matches();

    let args = Args {
        api_key: matches.value_of("api_key").unwrap(),
        input_file: matches.value_of("input_file").unwrap(),
        year: if let Some(y) = matches.value_of("max") { y.parse().expect("Program argument 'year' is not a u64") } else { chrono::Utc::now().year() as u64 },
        max: if let Some(y) = matches.value_of("year") { y.parse().expect("Program argument 'max' is not a u64") } else { 10u64 },
    };

    let stat_file_path = std::path::PathBuf::from(args.input_file);
    if !stat_file_path.exists() {
        panic!("Provided input file '{}' does not exist.", args.input_file);
    }

    let stat_file = match fs::read_to_string(&stat_file_path) {
        Ok(f) => f,
        Err(e) => panic!("{:#?}", e)
    };

    let (mut artists, mut songs) = parser::parse(&stat_file, &args).await;
    let artists = artists.occurence_sort();
    let songs = songs.occurence_sort();

    println!("YouTube Music Statistics for {}", &args.year);

    println!(">------- Top Artists -------<");
    for i in 0..artists.len() {
        let artist = artists.get(i).unwrap();
        println!("{}. {} - {} times", i, *artist.0, artist.1);

        if i as u64 >= args.max {
            break;
        }
    }

    print!("\n");
    println!(">------- Top Songs -------<");
    for i in 0..artists.len() {
        let song = songs.get(i).unwrap();
        println!("{}. {} - {} times", i, *song.0, song.1);

        if i as u64 >= args.max {
            break;
        }
    }

    Ok(())
}

trait OccurenceSort<T> {
    fn occurence_sort(&mut self) -> Vec<(T, usize)> where T: Hash + Eq;
}

impl<T> OccurenceSort<T> for Vec<T> {
    fn occurence_sort(&mut self) -> Vec<(T, usize)> where T: Hash + Eq {
        let mut map = HashMap::new();
        for i in self.drain(..) {
            let v = match map.get(&i) {
                Some(v) => v + 1,
                None => 1
            };

            map.insert(i, v);
        }

        let mut hash_vec: Vec<(T, usize)> = map.into_iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(&a.1));
        hash_vec
    }
}