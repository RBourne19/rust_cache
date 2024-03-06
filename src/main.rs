use std::time::{Duration};
use std::thread;
use cache::Cache as Cache;
use random_string::generate;
mod cache;

fn main() {
    println!("Hello, world!");
    let c :Cache<i32, String> = Cache::new(Duration::from_secs(10));
    print!("{:#?}", c);
    let chars = "abcdefghijklmnopqrstuvwxyz";
    c.insert(0, generate(4, chars));
    c.insert(1, generate(4, chars));
    c.insert(2, generate(4, chars));
    loop{
        thread::sleep(Duration::from_secs(10));
        c.insert(0, generate(4, chars));
        print!("{:#?}", c);
        println!("HELLO");
    }
}   


