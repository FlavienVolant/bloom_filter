# Bloom Filter


My implementaton of a Bloom Filter.

## API

Bloom filter for a String
```rs
const M: usize;
let hashs: Vec<fn(&String) -> usize>;
let mut filter = bloom_filter::BloomFilter::<String, M>::new(hashs);
```

```rs
let a = String::from("Hello");
let b = String::from("Hi");
let c = String::from("World !");

filter.add(&a);
filter.add(&b);
filter.add(&c);

filter.check(&a); // true
filter.check(&c); // true
filter.check(&String::from("World")); // should be false 
filter.check(&String::from("World !")); // true
```
