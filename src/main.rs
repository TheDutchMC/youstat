use std::fs;
use std::collections::HashMap;
use std::hash::Hash;

mod args;
mod parser;
mod substring;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    let args = args::ProgramArgs::new();
    let validity = args.is_valid();
    if !validity.0 {
        panic!("{}", validity.1);
    }

    let stat_file_path = std::path::PathBuf::from(args.input_file.as_ref().unwrap());
    if !stat_file_path.exists() {
        panic!("Provided input file '{}' does not exist.", args.input_file.as_ref().unwrap());
    }

    let stat_file = match fs::read_to_string(stat_file_path.as_path()) {
        Ok(f) => f,
        Err(e) => panic!("{:#?}", e)
    };

    let (artists, songs) = parser::parse(&stat_file, &args).await;
    let artists = get_sorted_list(artists);
    let songs = get_sorted_list(songs);

    println!("YouTube Music Statistics for {}", &args.year.unwrap());

    println!(">------- Top Artists -------<");
    for i in 0..artists.len() {
        let artist = artists.get(i).unwrap();
        println!("{}. {} - {} times", i, artist.0, artist.1);

        if i as u64 >= args.max.unwrap() {
            break;
        }
    }

    print!("\n");
    println!(">------- Top Songs -------<");
    for i in 0..artists.len() {
        let song = songs.get(i).unwrap();
        println!("{}. {} - {} times", i, song.0, song.1);

        if i as u64 >= args.max.unwrap() {
            break;
        }
    }

    Ok(())
}

fn get_sorted_list<T: Clone + Hash + Eq>(input: Vec<T>) -> Vec<(T, usize)> {
    let mut map = HashMap::new();
    for i in input {
        let val = match map.get(&i) {
            Some(val) => val + 1,
            None => 1
        };

        map.insert(i.clone(), val);
    }

    let hash_vec: Vec<(&T, &usize)> = map.iter().collect();
    let mut result = Vec::new();
    for i in hash_vec {
        result.push((i.0.clone(), i.1.clone()));
    }
    result.sort_by(|a, b| b.1.cmp(&a.1));

    result
}