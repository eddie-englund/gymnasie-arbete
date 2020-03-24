use std::collections::HashMap;

fn fib(n: u128, map: &mut HashMap<u128, u128>) -> u128 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = fib(n - 1, map) + fib(n-2, map);
                map.insert(n, val);
                return val
            }
        } 
    }
    
}

fn main() {
    let mut map = HashMap::new();



        for i in 1..186 {
            println!("{}: {}", i, fib(i, &mut map))
        }
    
}


