use std::fmt::Write;

#[derive(Debug)]
pub struct Testcase {
    pub id: i32,
    pub val: String,
    pub case_type: String,
    pub problem_category: String,
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
        let mut contest_id = String::new();
        let mut problem_id = String::new();
        let mut url = String::new();
        write!(&mut contest_id, "{}{}", contest_type, contest_number).unwrap();
        write!(&mut problem_id, "{}_{}", contest_id, problem_category).unwrap();
        write!(
            &mut url,
            "https://atcoder.jp/contests/{}/tasks/{}",
            //example: "https://atcoder.jp/contests/abc249/tasks/abc249_a"
            contest_id,
            problem_id
        )
        .unwrap();
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
