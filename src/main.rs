use rAstCoder::sample_preparator::{self, SamplePreparator};
use rAstCoder::types::Problem;
use regex::Regex;
use reqwest::{redirect, Client, StatusCode};
use scraper::{node::Element, ElementRef, Html, Selector};
use std::{
    fmt::Write,
    fs::{self, File},
    io::{self, BufReader, Read, Write as OtherWrite},
};

#[tokio::main]
async fn main() {
    let problem = Problem::new("abc".to_string(), 249, "a".to_string());
    let preparator = SamplePreparator { problem: problem };
    let dir = preparator.prepare();
    println!("{}", dir);
}
