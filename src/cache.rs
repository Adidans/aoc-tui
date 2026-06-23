use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn cache_path(year: u32, day: u32) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("aoc-tui");
    path.push(year.to_string());
    path.push(format!("{:02}", day.to_string()));
    path.push("input.txt");
    path
}

pub fn read_cache(year: u32, day: u32) -> Option<String> {
    let path = cache_path(year, day);

    fs::read_to_string(path).ok()
}

pub fn fetch_input(year: u32, day: u32) -> Result<String> {
    let session = std::env::var("AOC_SESSION")?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::blocking::Client::new();

    let res = client
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()?
        .text()?;

    Ok(res)
}

pub fn save_cache(year: u32, day: u32, input: &str) -> Result<()> {
    let path = cache_path(year, day);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, input)?;
    Ok(())
}

pub fn get_input(year: u32, day: u32) -> Result<String> {
    if let Some(input) = read_cache(year, day) {
        println!("using cache");
        return Ok(input);
    }

    println!("fetching input...");
    
    let input = fetch_input(year, day)?;
    save_cache(year, day, &input)?;
    
    Ok(input)
}
