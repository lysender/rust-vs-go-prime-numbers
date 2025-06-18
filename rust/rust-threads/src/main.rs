use std::io::{BufWriter, Write, stdout};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::spawn;

// Not sure what's going on, this causes infinite loop :)
fn generate(tx: Sender<i32>) {
    for i in 2.. {
        if tx.send(i).is_err() {
            break;
        }
    }
}

fn filter(rx: Receiver<i32>, tx: Sender<i32>, prime: i32) {
    for i in rx.iter().filter(|&x| x % prime != 0) {
        if tx.send(i).is_err() {
            break;
        }
    }
}

fn main() {
    let n = 10;
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let (tx, rx) = channel();
    spawn(move || generate(tx));
    let mut rx = rx;
    for _ in 0..n {
        let prime = rx.recv().unwrap();
        writeln!(out, "{}", prime).unwrap();
        let (next_tx, next_rx) = channel();
        spawn(move || filter(rx, next_tx, prime));
        rx = next_rx;
    }
}
