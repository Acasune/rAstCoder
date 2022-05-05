use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Testcase {
    pub id: u32,
    pub content: String,
    pub case_type: String,
    pub problem_category: String,
}

pub struct ResultCase {
    pub id: u32,
    pub expected: String,
    pub actual: String,
}

impl ResultCase {
    pub fn valid(&self) -> bool {
        self.expected == self.actual
    }
}

pub struct Problem {
    pub contest_type: String,
    pub contest_number: u32,
    pub contest_id: String,
    pub problem_category: String,
    pub problem_id: String,
    pub url: String,
}

impl Problem {
    pub fn new(contest_type: String, contest_number: u32, problem_category: String) -> Self {
        let contest_id = format!("{}{}", contest_type, contest_number);
        let problem_id = format!("{}_{}", contest_id, problem_category);
        let url = format!(
            "https://atcoder.jp/contests/{}/tasks/{}",
            //example: "https://atcoder.jp/contests/abc249/tasks/abc249_a"
            contest_id,
            problem_id
        );
        Problem {
            contest_type: contest_type,
            contest_number: contest_number,
            contest_id: contest_id,
            problem_category: problem_category,
            problem_id: problem_id,
            url: url,
        }
    }
}

pub struct TestResult {
    pub test_id: String,
    pub input: u32,
    pub output: String,
    pub expected: String,
    pub has_passed: bool,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ArgOption {
    Test(char), // todo
                // Add other enum such as Submit
}

enum ContestType {
    ABC,
    ARC,
    AGC,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_problem_structure() {
        let contest_type = "abc".to_string();
        let contest_id = 249;
        let problem_category = "a".to_string();
        let problem = Problem::new(contest_type, contest_id, problem_category);
        assert_eq!(
            "https://atcoder.jp/contests/abc249/tasks/abc249_a",
            problem.url
        );
    }
}
