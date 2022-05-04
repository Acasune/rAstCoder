use crate::types::{Problem, Testcase};
use futures::executor::block_on;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::{blocking, Client};
use scraper::{node::Element, ElementRef, Html, Selector};
use std::{
    fmt::Write,
    fs::{self, File},
    io::{self, BufReader, Read},
    path::Path,
};

pub struct SamplePreparator {
    pub problem: Problem,
}

impl SamplePreparator {
    pub fn prepare(self) -> (String, String) {
        let dir = format!("./testcase/{}", self.problem.problem_id);
        // dir: "./testcase/abc249_a"
        if !Path::new(&dir).exists() {
            let testcases = get_samples(self.problem).unwrap();
            for case in testcases {
                let dir_internal = format!("{}/{}", dir, case.case_type);
                let path = format!("{}/{}.txt", dir_internal, case.id);
                fs::create_dir_all(&dir_internal);
                let mut file = fs::File::create(path).unwrap();
                {
                    use std::io::Write;
                    file.write_all(case.content.as_bytes()).unwrap();
                }
            }
        }
        let input_dir = format!("{}{}", dir, "/input");
        let output_dir = format!("{}{}", dir, "/output");

        (input_dir, output_dir)
    }
}
fn get_samples(problem: Problem) -> Result<Vec<Testcase>, Box<dyn std::error::Error>> {
    let section_selector = Selector::parse("section").unwrap();
    let h3_selector = Selector::parse("h3").unwrap();
    let mut ret = vec![];

    let body = reqwest::blocking::get(&problem.url)?.text()?.to_string();
    let html = Html::parse_document(&body);
    let sections = html.select(&section_selector);
    for sec in sections {
        let mut h3el = sec.select(&h3_selector);
        while let Some(cand) = h3el.next() {
            let tst = create_testcase(cand);
            if let Some(t) = tst {
                ret.push(t);
            }
        }
    }
    Ok(ret)
}
fn create_testcase(node: ElementRef) -> Option<Testcase> {
    lazy_static! {
        static ref SAMPLE_SECTION: Regex = Regex::new(r"Sample ((In|Out)put) (\d+)").unwrap();
    }

    let txt = &node.inner_html().to_string();

    if let Some(val) = SAMPLE_SECTION.captures(txt) {
        let case_type = val.get(1).map_or("", |m| m.as_str());
        let id = val
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<u32>()
            .unwrap();
        let content = ElementRef::wrap(node.next_sibling().unwrap())
            .unwrap()
            .inner_html()
            .to_string();
        Some(Testcase {
            id: id,
            content: content,
            case_type: case_type.to_string().to_ascii_lowercase(),
            problem_category: "A".to_string(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_problem_structure() {
        let problem = Problem::new("abc".to_string(), 249, "a".to_string());
        let a = get_samples(problem);
        dbg!(a);
    }
}
