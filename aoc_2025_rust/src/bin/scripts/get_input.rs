use dotenvy::dotenv;
use reqwest::{blocking::Client, header::COOKIE};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let day = get_day()?;
    let session = get_session()?;
    let input_data = fetch_input(day, &session)?;
    let file_path = prepare_file_path(day)?;
    save_input(&file_path, &input_data)?;
    println!("Saved input to {:?}", file_path);
    Ok(())
}

fn get_day() -> Result<u32, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: get_input <day>".into());
    }
    let day: u32 = args[1].parse()?;
    Ok(day)
}

fn get_session() -> Result<String, Box<dyn std::error::Error>> {
    let session = env::var("SESSION")?;
    Ok(session)
}

fn fetch_input(day: u32, session: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://adventofcode.com/2025/day/{day}/input");
    println!("Fetching input from `{}`", url);
    let client = Client::new();
    let response = client
        .get(url)
        .header(COOKIE, format!("session={session}"))
        .send()?
        .text()?;
    Ok(response)
}

fn prepare_file_path(day: u32) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let script_path = Path::new(file!());
    let project_root = script_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .ok_or("Failed to determine project root")?;
    let dir_path = project_root.join("bin").join(format!("{:02}", day));
    create_dir_all(&dir_path)?;
    Ok(dir_path.join("input.txt"))
}

fn save_input(file_path: &Path, input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
