use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn start_thread(name: String) -> thread::JoinHandle<()>{
    let handle =  thread::spawn(move ||{
        for i in 1..10{
            println!("number {} from {}", i, name);
            thread::sleep(Duration::from_millis(1));
        }
    });
    /// below will error, we already moved name in closure above.
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
}
