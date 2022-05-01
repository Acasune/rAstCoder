use regex::Regex;
use reqwest::{redirect, Client, StatusCode};
use scraper::{node::Element, ElementRef, Html, Selector};
use std::{
    fmt::Write,
    fs::{self, File},
    io::{self, BufReader, Read, Write as OtherWrite},
};

const base_url: &str = "https://atcoder.jp/";
const LOGIN_URL: &str = "https://atcoder.jp/login";
const sample_url: &str = "https://atcoder.jp/contests/abc249/tasks/abc249_a";

#[tokio::main]
async fn main() {
    let a = get_samples().await;
    for aa in a.unwrap() {
        let mut dir = String::new();
        write!(&mut dir, "./testcase/{}", aa.type_.as_str()).unwrap();
        let res = fs::create_dir_all(&dir);
        println!("{:?}", res);
        write!(&mut dir, "/{}.txt", aa.id).unwrap();
        println!("{}", dir);
        let mut file = fs::File::create(dir).unwrap();
        {
            use std::io::Write;
            file.write_all(&aa.val.as_bytes()).unwrap();
        }
    }
}

async fn get_samples() -> Result<Vec<testcase>, Box<std::error::Error>> {
    // GET
    let client = Client::builder().build()?;
    let sec_select = Selector::parse("section").unwrap();
    let h3_select = Selector::parse("h3").unwrap();
    let mut ret = vec![];

    let res = client.get(sample_url).send().await?.text().await?;
    let html = Html::parse_document(&res);
    let secs = html.select(&sec_select);
    for sec in secs {
        let mut h3el = sec.select(&h3_select);
        while let Some(cand) = h3el.next() {
            let tst = create_testcase(cand);
            if let Some(t) = tst {
                ret.push(t);
            }
        }
    }
    Ok(ret)
}

fn create_testcase(node: ElementRef) -> Option<testcase> {
    let re = Regex::new(r"Sample ((In|Out)put) (\d+)").unwrap();
    let txt = &node.inner_html().to_string();

    if let Some(val) = re.captures(txt) {
        let type_ = val.get(1).map_or("", |m| m.as_str());
        let id = val
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let val = ElementRef::wrap(node.next_sibling().unwrap())
            .unwrap()
            .inner_html()
            .to_string();
        Some(testcase {
            id: id,
            val: val,
            type_: type_.to_string().to_ascii_lowercase(),
        })
    } else {
        None
    }
}

#[derive(Debug)]
struct testcase {
    id: i32,
    val: String,
    type_: String,
}
