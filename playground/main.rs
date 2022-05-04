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
        mut S:Chars
    }
    S.sort();
    let mut flg = false;
    let mut flg2 = false;
    for i in 0..S.len() - 1 {
        if S[i] == S[i + 1] {
            println!("{}", "No");
            return;
        }
    }
    for i in 0..S.len() {
        if 'a' as u8 <= S[i] as u8 && S[i] as u8 <= 'z' as u8 {
            flg = true;
        }
        if 'A' as u8 <= S[i] as u8 && S[i] as u8 <= 'Z' as u8 {
            flg2 = true;
        }
    }
    if flg && flg2 {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
