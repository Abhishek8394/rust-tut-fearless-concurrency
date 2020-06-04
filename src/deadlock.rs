use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

pub fn deadlock_demo(){
    let r1 = Arc::new(Mutex::new(5));
    let r2 = Arc::new(Mutex::new(10));
    println!("r1 = {}", r1.lock().unwrap());
    println!("r2 = {}", r2.lock().unwrap());
    println!("Starting deadlock");

    let h1r1 = Arc::clone(&r1);
    let h2r1 = Arc::clone(&r1);
    let h1r2 = Arc::clone(&r2);
    let h2r2 = Arc::clone(&r2);

    let h1 = thread::spawn(move ||{
        println!("h1 trying to get r1");
        let res1 = h1r1.lock().unwrap();
        println!("h1 acquired r1: {:?}", res1);
        println!("h1 going to sleep");
        thread::sleep(Duration::from_millis(50));
        println!("h1 trying to get r2");
        let res2 = h1r2.lock().unwrap();
        println!("h1 acquired r2: {:?}", res2);
        
    });

    let h2 = thread::spawn(move ||{
        println!("h2 trying to get r2");
        let res2 = h2r2.lock().unwrap();
        println!("h2 acquried r2: {:?}", res2);
        println!("h2 going to sleep");
        thread::sleep(Duration::from_millis(50));
        println!("h2 trying to get r1");
        let res1 = h2r1.lock().unwrap();
        println!("h2 acquried r1: {:?}", res1);
        
    });
    
    h2.join().unwrap();
    h1.join().unwrap();


}
