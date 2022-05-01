use std::collections::HashMap;

use regex::Regex;
use reqwest::{redirect, Client, StatusCode};
use scraper::{ElementRef, Html, Selector};

const base_url: &str = "https://atcoder.jp/";
const LOGIN_URL: &str = "https://atcoder.jp/login";
const sample_url: &str = "https://atcoder.jp/contests/abc249/tasks/abc249_a";

#[tokio::main]
async fn main() -> Result<(), Box<std::error::Error>> {
    // GET
    let client = Client::builder().build()?;
    let sec_select = Selector::parse("section").unwrap();
    let h3_select = Selector::parse("h3").unwrap();
    let re_output = Regex::new(r"Sample Output (\d+)").unwrap();
    let re_input = Regex::new(r"Sample Input (\d+)").unwrap();

    let res = client.get(sample_url).send().await?.text().await?;
    let html = Html::parse_document(&res);
    let secs = html.select(&sec_select);

    for sec in secs {
        let mut h3el = sec.select(&h3_select);
        while let Some(cand) = h3el.next() {
            let h3_val = &cand.inner_html().to_string();
            if let Some(val) = re_output.captures(h3_val) {
                println!(
                    "Test case input: {:?}",
                    val.get(1).map_or("", |m| m.as_str())
                );
                let sib = ElementRef::wrap(cand.next_sibling().unwrap()).unwrap();
                println!("{:?}", sib.inner_html().to_string());
            } else if let Some(val) = re_input.captures(h3_val) {
                println!(
                    "Test case output: {:?}",
                    val.get(1).map_or("", |m| m.as_str())
                );
                let sib = ElementRef::wrap(cand.next_sibling().unwrap()).unwrap();
                println!("{:?}", sib.inner_html().to_string());
            }
        }
    }

    Ok(())
}
