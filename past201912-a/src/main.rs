// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        num: String,
    }

    let num: i32 = match num.parse() {
        Ok(num) => num,
        Err(_e) => {
            println!("error");
            return;
        }
    };

    println!("{}", num*2);
}
