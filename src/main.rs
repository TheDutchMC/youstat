use std::fs;

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

    let stat_file_path = std::path::PathBuf::from(&args.input_file.unwrap());
    if !stat_file_path.exists() {
        panic!("Provided input file '{}' does not exist.", &args.input_file.unwrap());
    }

    let stat_file = match fs::read_to_string(stat_file_path.as_path()) {
        Ok(f) => f,
        Err(e) => panic!("{:#?}", e)
    };

    let (artist, songs) = parser::parse(&stat_file, &args);

}