use std::thread;
use std::time::Duration;

// fn start_thread(name: String){
//     thread::spawn(||{
//         for i in 1..10{
//             println!("number {} from {}", i, name);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
// }

fn main() {
    // start_thread(String::from("t1"));
    // start_thread(String::from("t2"));
    thread::spawn(||{
        for i in 1..10{
            println!("number {} from {}", i, "t1");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("{} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
