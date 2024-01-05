use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;

fn blocking_receiver() {
    let (tx, rx) = mpsc::channel();

   
    let first_tx = tx.clone();
    let transmitter_join = thread::spawn(move || {
        let values = vec!["bob", "john", "ken", "paul", "stop", "already stop"];
        for value in values {
            sleep(Duration::from_millis(500));
            println!("A: Send {value}");
            first_tx.send(String::from(value)).unwrap_or_else(|_| {
                println!("`{value}` was not sent. Receiver is probably down.");
            });
        }
        
    });


    let transmitter_join = thread::spawn(move || {
        let values = vec!["jane", "anna", "lola", "mary", "susan"];
        for value in values {
            sleep(Duration::from_millis(700));
            println!("B: Send {value}");
            tx.send(String::from(value)).unwrap_or_else(|_| {
                println!("`{value}` was not sent. Receiver is probably down.");
            });
        }
        
    });
    let receiver_join = thread::spawn(move || {
        let mut exit = false;
        let mut iteration = 0;
        while (!exit) {
            iteration += 1;
            println!("Do some stuff ({iteration})");
            sleep(Duration::from_millis(100));

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

fn non_blocking_receiver() {
    let (tx, rx) = mpsc::channel();


    let first_tx = tx.clone();
    let transmitter_join = thread::spawn(move || {
        let values = vec!["bob", "john", "ken", "paul", "stop", "already stop"];
        for value in values {
            sleep(Duration::from_millis(500));
            println!("A: Send {value}");
            first_tx.send(String::from(value)).unwrap_or_else(|_| {
                println!("`{value}` was not sent. Receiver is probably down.");
            });
        }
        
    });


    let transmitter_join = thread::spawn(move || {
        let values = vec!["jane", "anna", "lola", "mary", "susan"];
        for value in values {
            sleep(Duration::from_millis(700));
            println!("B: Send {value}");
            tx.send(String::from(value)).unwrap_or_else(|_| {
                println!("`{value}` was not sent. Receiver is probably down.");
            });
        }
        
    });
    let receiver_join = thread::spawn(move || {
        let mut exit = false;
        let mut iteration = 0;
        while (!exit) {
            iteration += 1;
            println!("Do some stuff ({iteration})");

            sleep(Duration::from_millis(100));
            let received = rx.try_recv();
            
            if let Ok(name) = received {
                if name == "stop" {
                    exit = true;
                } else {
                    println!("Hi {}", name);
                }
            };
          
        }
        println!("Goodbye everyone...");

    });

    receiver_join.join().unwrap();
}

fn main() {

    println!("Run blocking receiver");
    blocking_receiver();
    
    println!("Run non blocking receiver");
    non_blocking_receiver();
}