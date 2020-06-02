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
}
