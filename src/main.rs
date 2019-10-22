use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use rand::prelude::*;
use std::time::Duration;

mod conc;

fn main() -> () {
    let (tx, rx): (Sender<Person>, Receiver<Person>) =
        mpsc::channel();

    let sender = tx.clone();

    thread::spawn(move || {
        loop {
            sender.send(Person { name: "Vasya".to_owned(), age: random::<usize>() % 40 + 10 });
            thread::sleep(Duration::from_millis(random::<u64>() % 800));
        }
    });

    while let Ok(val) = rx.recv_timeout(Duration::from_secs(1)) {
        println!("{}", "x".repeat(val.age ));
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Person {
    name: String,
    age: usize,
}