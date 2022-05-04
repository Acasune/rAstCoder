use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File};
use std::io::Read;

use crate::executor::{self, Executor};
use crate::types::ResultCase;

pub struct Presenter {}

impl Presenter {
    pub fn validate(&mut self, results_set: &BTreeMap<u32, ResultCase>) {
        for result in results_set.iter() {
            let expected = &result.1.expected;
            let actual = &result.1.actual;
            let flg = result.1.valid();
            println!("--*--*--*--");
            println!("Case {}:", result.0);
            if result.1.valid() {
                println!("Passed :");
                println!("Expected");
                println!("{}", *expected);
                println!("Actual");
                println!("{}", *actual);
            } else {
                println!("Wrong");
                println!("Expected");
                println!("{}", expected);
                println!("Actual");
                println!("{}", actual);
            }
        }
    }
}
