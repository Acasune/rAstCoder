use crate::types::{Problem, Testcase};

use futures::executor::block_on;
use regex::Regex;
use reqwest::{redirect, Client, StatusCode};
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
    pub fn prepare(self) -> String {
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
                    file.write_all(case.val.as_bytes()).unwrap();
                }
            }
        }

        dir
    }
}
fn get_samples(problem: Problem) -> Result<Vec<Testcase>, Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;
    let sec_select = Selector::parse("section").unwrap();
    let h3_select = Selector::parse("h3").unwrap();
    let mut ret = vec![];

    let res = block_on(block_on(client.get(problem.url).send()).unwrap().text()).unwrap();
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
fn create_testcase(node: ElementRef) -> Option<Testcase> {
    let re = Regex::new(r"Sample ((In|Out)put) (\d+)").unwrap();
    let txt = &node.inner_html().to_string();

    if let Some(val) = re.captures(txt) {
        let case_type = val.get(1).map_or("", |m| m.as_str());
        let id = val
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let val = ElementRef::wrap(node.next_sibling().unwrap())
            .unwrap()
            .inner_html()
            .to_string();
        Some(Testcase {
            id: id,
            val: val,
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
