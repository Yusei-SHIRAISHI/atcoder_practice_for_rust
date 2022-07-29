// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [i32; n],
    };

    let mut iter = v.iter().peekable();

    while let Some(i) = iter.next() {
        let next = match iter.peek() {
            Some(i) => i,
            None => break
        };

        let val = *i - **next;

        if val < 0 {
            println!("up {}", val.abs());
        }
        else if val > 0 {
            println!("down {}", val);
        }
        else {
            println!("stay");
        }
    }
}
