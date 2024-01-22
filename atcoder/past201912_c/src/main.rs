// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut v: [i32; 6],
    };

    v.sort();

    print!("{}", v[3]);
}
