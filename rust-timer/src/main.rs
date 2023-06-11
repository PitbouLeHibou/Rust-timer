use std::io;
use std::thread;
use std::time::Duration;
fn main() {
    //get input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("msg");
    let int_input:u64 = input.trim().parse().expect("failed to read input");

    let mut i = 0;
    while i != int_input  {
        println!("{i}");
        //sleep
        thread::sleep(Duration::from_millis(1000));    
        i+=1
    }
   print!("aaa");
   thread::sleep(Duration::from_millis(100000));  
}