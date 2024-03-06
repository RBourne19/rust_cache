# Rust Cache

## Overview

Very simple generic Cache implementation where entries expire after a set time.

## Usage



````
let c :Cache<i32, String> = Cache::new(Duration::from_secs(10));
    print!("{:#?}", c);
    let chars = "abcdefghijklmnopqrstuvwxyz";
    c.insert(0, generate(4, chars));
    c.insert(1, generate(4, chars));
    c.insert(2, generate(4, chars));
    loop{
        thread::sleep(Duration::from_secs(2));
        c.insert(0, generate(4, chars));
        println!("{:#?}", c);
    }
````
