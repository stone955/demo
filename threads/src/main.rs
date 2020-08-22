use std::thread;
use std::time::Duration;

fn main() {
    {
        let t1 = thread::spawn(|| {
            for i in 0..10 {
                println!("Thread 1 number {}", i);
                thread::sleep(Duration::from_millis(5));
            }
        });

        let t2 = thread::spawn(|| {
            for i in 0..10 {
                println!("Thread 2 number {}", i);
                thread::sleep(Duration::from_millis(5));
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();
    }
}


