#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        x:[i64;7]
    }

    let mut taka =
        x[1] * x[0] * (x[6] / (x[0] + x[2])) + x[1] * i64::min(x[0], (x[6] % (x[0] + x[2])));
    let mut ao =
        x[4] * x[3] * (x[6] / (x[3] + x[5])) + x[4] * i64::min(x[3], (x[6] % (x[3] + x[5])));
    // println!("{}", taka);
    // println!("{}", ao);
    if taka == ao {
        println!("{}", "Draw");
    } else if taka > ao {
        println!("{}", "Takahashi");
    } else {
        println!("{}", "Aoki");
    }
}
