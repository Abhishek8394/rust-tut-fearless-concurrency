mod deadlock;

use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::time::Duration;
use crate::deadlock::{solved_deadlock_demo, deadlock_demo};

fn start_thread(name: String) -> thread::JoinHandle<()>{
    let handle =  thread::spawn(move ||{
        for i in 1..10{
            println!("number {} from {}", i, name);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // below will error, we already moved name in closure above.
    // drop(name);
    return handle;
}

fn main() {
    let t1 = start_thread(String::from("t1"));
    let t2 = start_thread(String::from("t2"));
    
    for i in 1..5{
        println!("{} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    t2.join().unwrap();
    t1.join().unwrap();

    {
        println!("Message passing demo");
        let (tx, rx) = mpsc::channel();
        thread::spawn(move ||{
            let val = String::from("duck");
            tx.send(val).unwrap();
            // below will error because send owns the value now.
            // println!("sent {}", val);
            let msgs = vec!["This", " is", " a", " long", " message"];
            for msg in msgs{
                tx.send(String::from(msg)).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        });
        // Use `try_recv` for non-blocking receive of messages.
        let received = rx.recv().unwrap();
        println!("Got: {}", received);

        for received in rx{
            println!("Got: {}", received);
        }
    }

    {
        println!("mpsc demo");
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move ||{
            let msgs = vec!["This", " is", " a", " message", " from", " thread-1"];
            for msg in msgs{
                tx1.send(String::from(msg)).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        });

        // as long as tx is in scope, rx will keep listening for more messages.
        // Two solutions:
        // - let tx2 = mpsc::Sender::clone(&tx); drop(tx);
        // - send tx in thread.
        // Point is to get tx removed.
        thread::spawn(move ||{
            let msgs = vec!["thread-2", " says", " hi"];
            for msg in msgs{
                // tx2.send(String::from(msg)).unwrap();
                tx.send(String::from(msg)).unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        });
        for received in rx {
            println!("Got: {}", received);
        }
    }

    {
        println!("mutex demo");
        let m = Mutex::new(5);
        {
            // mutex.lock returns LockResult<MutexGuard> and calling unwrap returns
            // MutexGuard a smart pointer.
            // The lock is auto released when MutexGuard goes out of scope.
            let mut num = m.lock().unwrap();
            *num = 10;
        }
        println!("m = {:?}", m);

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for i in 0..10{
            let c = Arc::clone(&counter);
            let h = thread::spawn(move ||{
                let mut n = c.lock().unwrap();
                *n += 1;
            });
            handles.push(h);
        }
        for h in handles{
            h.join().unwrap();
        }
        println!("counter: {:?}", counter.lock().unwrap());
    }

    {
        println!("\nsolved deadlock demo");
        solved_deadlock_demo();
        // uncomment below lines to view deadlock happen.
        // println!("\nunsolved deadlock demo");
        // deadlock_demo();
    }

    {
        println!("\nSend and Sync Trait demo");
        let mut i: i32 = 5;
        // using *j below will not work because &mut i32 does not not implement
        // Sync and Send traits.
        // let j = &mut i;
        let h1 = thread::spawn(move ||{
            println!("h1 {:?}", i);
            // thread::sleep(Duration::from_millis(30));
            i += 1;
            println!("h1 {:?}", i);
        });

        let h2 = thread::spawn(move ||{
            thread::sleep(Duration::from_millis(29));
            println!("h2 {:?}", i);
            i += 1;
            println!("h2 {:?}", i);
        });

        h1.join().unwrap();
        h2.join().unwrap();
    }
}
