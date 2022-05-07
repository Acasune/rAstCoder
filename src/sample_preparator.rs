use crate::types::{Problem, Testcase};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use scraper::{ElementRef, Html, Selector};
use std::path::Path;

pub struct SamplePreparator {
    pub problem: Problem,
}

impl SamplePreparator {
    pub fn new(problem: Problem) -> Self {
        SamplePreparator { problem: problem }
    }
    pub fn prepare(self) -> Result<(String, String)> {
        let dir = format!("./testcase/{}", self.problem.problem_id);
        // dir: "./testcase/abc249_a"
        let input_dir = format!("{}{}", dir, "/input");
        let output_dir = format!("{}{}", dir, "/output");
        if !Path::new(&dir).exists() {
            let body = reqwest::blocking::get(self.problem.url)?
                .text()?
                .to_string();
            let html = Html::parse_document(&body);
            let testcases = get_samples(html).unwrap();
            for case in testcases {
                let dir_internal = format!("{}/{}", dir, case.case_type);
                let path = format!("{}/{}.txt", dir_internal, case.id);
                std::fs::create_dir_all(&dir_internal)?;
                let mut file = std::fs::File::create(path).unwrap();
                {
                    use std::io::Write;
                    file.write_all(case.content.as_bytes()).unwrap();
                }
            }
        }
        Ok((input_dir, output_dir))
    }
}

fn get_samples(html: Html) -> Result<Vec<Testcase>, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref SECTION_SELECTOR: Selector = Selector::parse("section").unwrap();
        static ref H3_SELECTOR: Selector = Selector::parse("h3").unwrap();
    }

    let mut ret = vec![];
    let sections = html.select(&SECTION_SELECTOR);
    for sec in sections {
        let mut h3el = sec.select(&H3_SELECTOR);
        while let Some(sample_node) = h3el.next() {
            let tst = create_testcase(sample_node);
            if let Some(t) = tst {
                ret.push(t);
            }
        }
    }
    Ok(ret)
}
fn create_testcase(sample_node: ElementRef) -> Option<Testcase> {
    lazy_static! {
        static ref SAMPLE_SECTION: Regex = Regex::new(r"Sample ((In|Out)put) (\d+)").unwrap();
    }

    let txt = &sample_node.inner_html().to_string();

    if let Some(val) = SAMPLE_SECTION.captures(txt) {
        let case_type = val.get(1).map_or("", |m| m.as_str());
        let id = val
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<u32>()
            .unwrap();
        let content = ElementRef::wrap(sample_node.next_sibling().unwrap())
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
    fn test_html_to_testcases() {
        let body = include_str!("../test_resources/abc249_a");
        let html = Html::parse_document(&body);
        let testcases = get_samples(html).unwrap();
        assert_eq!(
            testcases[0],
            Testcase {
                id: 1,
                content: "4 3 3 6 2 5 10\n".to_string(),
                case_type: "input".to_string(),
                problem_category: "A".to_string(),
            },
        );
        assert_eq!(
            testcases[1],
            Testcase {
                id: 1,
                content: "Takahashi\n".to_string(),
                case_type: "output".to_string(),
                problem_category: "A".to_string()
            }
        );
    }
    #[test]
    fn test_htmlnode_to_testcase_ok() {
        lazy_static! {
            static ref H3_SELECTOR: Selector = Selector::parse("h3").unwrap();
        }
        let source = r#"<div class="part">
        <section>
        <h3>Sample Input 1</h3><pre>4 3 3 6 2 5 10
        </pre>
        </section>
        </div>"#;
        let html = Html::parse_document(&source);
        let target = html.select(&H3_SELECTOR).next().unwrap();
        let testcase = create_testcase(target);
        assert_eq!(
            testcase.unwrap(),
            Testcase {
                id: 1,
                content: "4 3 3 6 2 5 10\n        ".to_string(),
                case_type: "input".to_string(),
                problem_category: "A".to_string()
            }
        );
    }
    #[test]
    fn test_htmlnode_to_testcase_ng() {
        lazy_static! {
            static ref H3_SELECTOR: Selector = Selector::parse("h3").unwrap();
        }
        let source = r#"<div class="part">
        <section>
        <h3>Problem Statement</h3><p>Takahashi and Aoki decided to jog.<br />
        Takahashi repeats the following: "walk at <var>B</var> meters a second for <var>A</var> seconds and take a rest for <var>C</var> seconds."<br />
        Aoki repeats the following: "walk at <var>E</var> meters a second for <var>D</var> seconds and take a rest for <var>F</var> seconds."<br />
        When <var>X</var> seconds have passed since they simultaneously started to jog, which of Takahashi and Aoki goes ahead?</p>
        </section>
        </div>"#;
        let html = Html::parse_document(&source);
        let target = html.select(&H3_SELECTOR).next().unwrap();
        let testcase = create_testcase(target);
        assert_eq!(testcase, None);
    }
}
