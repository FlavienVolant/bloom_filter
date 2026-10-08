use std::{println, hash::{Hash, Hasher, DefaultHasher}};

mod bloom_filter;

fn main() {
    
    const M: usize = 12;

    let hashs: Vec<fn(&String) -> usize> = vec![
        | s | { 
            let mut hasher = DefaultHasher::new();
            s.hash(&mut hasher);
            hasher.finish() as usize
        },
        | s | { 
            let mut hasher = DefaultHasher::new();
            s.hash(&mut hasher);
            (hasher.finish() as usize) + 1
        },
        | s | { 
            let mut hasher = DefaultHasher::new();
            s.hash(&mut hasher);
            (hasher.finish() as usize) + 2
        }
    ];


    let mut filter = bloom_filter::BloomFilter::<String, M>::new(hashs);

    println!("{}", filter);

    let a = String::from("Hello");
    let b = String::from("Hi");
    let c = String::from("World !");

    filter.add(&a);
    filter.add(&b);
    filter.add(&c);

    println!("a: {}", filter.check(&a));
    println!("c: {}", filter.check(&c));
    println!("World: {}", filter.check(&String::from("World")));
    println!("World !: {}", filter.check(&String::from("World !")));
}
