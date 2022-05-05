use crate::types::ArgOption;
use crate::types::Problem;
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq)]
pub struct ArgParser {
    pub problem: Problem,
    pub option: HashSet<ArgOption>,
    pub source_path: String,
}

impl ArgParser {
    pub fn build(args: Vec<String>, working_dir: String) -> Result<Self> {
        lazy_static! {
            static ref PROBLEM_ID: Regex =  Regex::new(r"^(abc|arc|agc)(\d+)([A-Ha-h])$").unwrap();
            // todo => generalization
        }
        // args example: [$working_dir,"abc234", "a.rs","-t"]
        let mut args = VecDeque::from(args);
        args.pop_front().with_context(|| "No argument")?;

        let problem_id = args.pop_front().with_context(|| "No argument")?;
        if !PROBLEM_ID.is_match(&problem_id) {
            return Err(anyhow!("No contest"));
        }
        let parsed_problem_id = PROBLEM_ID.captures(&problem_id).unwrap();

        let contest_type = parsed_problem_id[1].to_string();
        let contest_number = parsed_problem_id[2].to_string().parse::<u32>().unwrap();
        let contest_category = parsed_problem_id[3].to_string().to_ascii_lowercase();

        let source_name = args.pop_front().with_context(|| "No argument")?;
        let source_path = format!("{}/{}", working_dir, source_name);

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
            source_path: source_path,
        };

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {

    use crate::types;

    use super::*;
    #[test]
    fn test_parse_args_ok() {
        let arg_parser = ArgParser::build(
            vec![
                "rAstCoder/target/debug/rAstCoder".to_string(),
                "abc249a".to_string(),
                "a.rs".to_string(),
                "-t".to_string(),
            ],
            "working_dir".to_string(),
        )
        .unwrap();
        let expected = ArgParser {
            problem: Problem::new("abc".to_string(), 249, "a".to_string()),
            option: HashSet::from([types::ArgOption::Test('t')]),
            source_path: "working_dir/a.rs".to_string(),
        };
        assert_eq!(expected, arg_parser);
    }

    #[test]
    fn test_parse_args_ok_with_no_option() {
        let arg_parser = ArgParser::build(
            vec![
                "rAstCoder/target/debug/rAstCoder".to_string(),
                "abc249a".to_string(),
                "a.rs".to_string(),
            ],
            "working_dir".to_string(),
        )
        .unwrap();
        let expected = ArgParser {
            problem: Problem::new("abc".to_string(), 249, "a".to_string()),
            option: HashSet::<ArgOption>::new(),
            source_path: "working_dir/a.rs".to_string(),
        };
        assert_eq!(expected, arg_parser);
    }

    #[test]
    fn test_parse_args_ng_wrong_testcase() {
        let arg_parser = ArgParser::build(
            vec![
                "rAstCoder/target/debug/rAstCoder".to_string(),
                "abc249ab".to_string(),
                "a.rs".to_string(),
                "-t".to_string(),
            ],
            "working_dir".to_string(),
        );
        assert!(arg_parser.is_err());
    }
    #[test]
    fn test_parse_args_ng_wrong_option() {
        let arg_parser = ArgParser::build(
            vec![
                "rAstCoder/target/debug/rAstCoder".to_string(),
                "abc249a".to_string(),
                "a.rs".to_string(),
                "-a".to_string(),
            ],
            "working_dir".to_string(),
        );
        assert!(arg_parser.is_err());
    }
}
