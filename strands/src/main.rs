use scraper::{Html, Selector};
use std::{error::Error, vec};

const STRANDS_URL: &str = "https://www.nytimes.com/games/strands";
const STRANDS_LETTER_SELECTOR: &str = "[id^=button]";
const STRANDS_BOARD_WIDTH: usize = 6;
const STRANDS_BOARD_HEIGHT: usize = 8;

fn main() -> Result<(), Box<dyn Error>> {
    let strands_response = reqwest::blocking::get(STRANDS_URL)?;
    let strands_html = Html::parse_document(&strands_response.text()?);

    let board: Vec<Vec<char>> = strands_html
        .select(&Selector::parse(STRANDS_LETTER_SELECTOR)?)
        .map(|element| element.text().next().expect("Board has missing letter"))
        .map(|string| string.chars().next().expect("Unable to get character"))
        .collect::<Vec<_>>()
        .chunks_exact(STRANDS_BOARD_WIDTH)
        .map(|row| row.to_vec())
        .collect();

    // assert!(board.)

    Ok(())
}
