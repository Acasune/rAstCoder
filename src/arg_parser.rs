use crate::types::ArgOption;
use crate::types::Problem;
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

pub struct ArgParser {
    pub problem: Problem,
    pub option: HashSet<ArgOption>,
}

impl ArgParser {
    pub fn build(args: Vec<String>) -> Result<Self> {
        lazy_static! {
            static ref PROBLEM_ID: Regex =  Regex::new(r"^(abc|arc|agc)(\d+)([A-Ha-h])$").unwrap();
            // todo => generalization
        }
        let mut args = VecDeque::from(args);
        args.pop_front().with_context(|| "No argument")?;
        let problem_id = args.pop_front().with_context(|| "No argument")?;
        if !PROBLEM_ID.is_match(&problem_id) {
            return Err(anyhow!("No contest"));
        }
        let matches = PROBLEM_ID.captures(&problem_id).unwrap();

        let contest_type = matches[1].to_string();
        let contest_number = matches[2].to_string().parse::<u32>().unwrap();
        let contest_category = matches[3].to_string().to_ascii_lowercase();

        let mut options = HashSet::new();
        while let Some(arg) = args.pop_front() {
            let mut arg_iter = arg.chars();
            if '-' != arg_iter.next().unwrap() {
                return Err(anyhow!("Wrong option format. Please begin with -"));
            }
            for ops in arg_iter {
                match ops {
                    't' => {
                        let _ = options.insert(ArgOption::Test('t'));
                    }
                    _ => return Err(anyhow!("Unexpected option format.")),
                }
            }
        }

        let ret = ArgParser {
            problem: Problem::new(contest_type, contest_number, contest_category),
            option: options,
        };

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_args_ok() {
        let arg_parser = ArgParser::build(vec![
            "rAstCoder/target/debug/rAstCoder".to_string(),
            "abc249a".to_string(),
            "-t".to_string(),
        ])
        .unwrap();
        assert_eq!("abc249_a", arg_parser.problem.problem_id);
    }

    #[test]
    fn test_parse_args_ok_with_no_option() {
        let arg_parser = ArgParser::build(vec![
            "rAstCoder/target/debug/rAstCoder".to_string(),
            "abc249a".to_string(),
        ])
        .unwrap();
        assert_eq!("abc249_a", arg_parser.problem.problem_id);
    }

    #[test]
    fn test_parse_args_ng_wrong_testcase() {
        let arg_parser = ArgParser::build(vec![
            "rAstCoder/target/debug/rAstCoder".to_string(),
            "abc249ab".to_string(),
            "-t".to_string(),
        ]);
        assert!(arg_parser.is_err());
    }
    #[test]
    fn test_parse_args_ng_wrong_option() {
        let arg_parser = ArgParser::build(vec![
            "rAstCoder/target/debug/rAstCoder".to_string(),
            "abc249a".to_string(),
            "-a".to_string(),
        ]);
        assert!(arg_parser.is_err());
    }
}
