// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

enum Query {
    Follow(i32, i32),
    FollowBack(i32),
    FollowFollow(i32),
}

#[derive(Clone)]
struct User {
    id: i32,
    follows: HashSet<i32>,
}

impl User {
    fn new(id: i32) -> User {
        User {
            id,
            follows: HashSet::new()
        }
    }

    fn follow(&mut self, user_id: i32) {
        if self.id == user_id {
            return
        }

        self.follows.insert(user_id);
    }
}

fn main() {
    let (mut users, queries) = read_input_and_initialize();

    for query in queries {
        users = match query {
            Query::Follow(a, b) => {
                let user = users.iter_mut().find(|u| u.id == a).unwrap();
                user.follow(b);
                users
            },

            Query::FollowBack(a) => {
                let followers: HashSet<i32> = users.iter()
                    .filter(|u| u.follows.contains(&a))
                    .map(|f| f.id)
                    .collect();

                let user = users.iter_mut().find(|u| u.id == a).unwrap();
                for f in followers {
                    user.follow(f);
                }

                users
            },

            Query::FollowFollow(a) => {
                let user = users.iter().find(|u| u.id == a).unwrap();
                let target_users: Vec<i32> = users.iter()
                    .filter(|u| user.follows.contains(&u.id))
                    .map(|u| u.follows.clone())
                    .flatten()
                    .collect();

                let user = users.iter_mut().find(|u| u.id == a).unwrap();
                for u in target_users {
                    user.follow(u);
                }

                users
            },

            _ => { panic!("invalid query") }
        };
    }

    print_users(users);
}

fn print_users(users: Vec<User>) {
    let len = users.len();
    for i in 0..len {
        for j in 0..len {
            if users[i].follows.contains(&((j+1) as i32)) {
                print!("Y");
            }
            else {
                print!("N");
            }
        }

        if i != len-1 {
            println!();
        }
    }
}

fn read_input_and_initialize() -> (Vec<User>, Vec<Query>) {
    input! {
        n: usize,
        q: usize,
    };

    // create users
    let mut users = Vec::with_capacity(n);
    for i in 0..n {
        users.push(User::new((i+1) as i32));
    }

    // create queries
    let mut queries = Vec::with_capacity(q);

    for _i in 0..q {
        input! {
            query: i32,
        };

        match query {
            1 => {
                input! {
                    a: i32,
                    b: i32,
                };
                queries.push(Query::Follow(a, b));
            }
            2 => {
                input! {
                    a: i32,
                };
                queries.push(Query::FollowBack(a));
            }
            3 => {
                input! {
                    a: i32,
                };
                queries.push(Query::FollowFollow(a));
            }
            _ => { panic!("invalid query") }
        }
    }

    (users, queries)
}

