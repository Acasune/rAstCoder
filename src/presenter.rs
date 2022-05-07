use crate::types::ResultCase;
use colour;
use std::collections::BTreeMap;

pub struct Presenter {}

impl Presenter {
    pub fn validate(&mut self, results_set: &BTreeMap<u32, ResultCase>) {
        for result in results_set.iter() {
            let expected = &result.1.expected;
            let actual = &result.1.actual;
            println!("--*--*--*--");
            print!("Case {}: ", result.0);
            if result.1.valid() {
                colour::green_ln!("AC ✅");
                println!("Expected");
                println!("{}", expected.trim());
                println!("Output");
                println!("{}", actual.trim());
            } else {
                colour::red_ln!("Wrong ❌");
                println!("Expected");
                println!("{}", expected.trim());
                println!("Output");
                println!("{}", actual.trim());
            }
        }
    }
}
