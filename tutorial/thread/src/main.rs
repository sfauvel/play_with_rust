use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let transmitter_join = thread::spawn(move || {
        let values = vec!["bob", "john", "ken", "paul", "stop", "already stop"];
        for value in values {
            sleep(Duration::from_millis(500));
            tx.send(String::from(value)).unwrap();
        }
        
    });

    let receiver_join = thread::spawn(move || {
        let mut exit = false;
        while (!exit) {
            let received = rx.recv().unwrap();
            
            if received == "stop" {
                exit = true;
            } else {
                println!("Hi {}", received);
            }
        }
        println!("Goodbye everyone...");

    });

    receiver_join.join().unwrap();
}