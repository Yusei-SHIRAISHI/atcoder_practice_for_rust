// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i32,
        mut v: [i32; n],
    };

    let a: HashSet<i32> = v.iter().cloned().collect();
    let b: HashSet<i32> = (1..n + 1).collect();

    let diffs = b.difference(&a).collect::<Vec<&i32>>();
    let diff = match diffs.first() {
        Some(x) => **x,
        None => {
            print!("Correct");
            return;
        }
    };

    v.push(diff);
    v.sort();

    let mut dup = 0;

    for (i, e) in v.iter().enumerate() {
        if *e != i as i32 + 1 {
            dup = *e;
            break;
        }
    }

    print!("{} {}", dup, diff);
}
