use std::io;
use std::collections::HashMap;


fn main() {
    let mut input = String::new();
    println!("Ingrese el numero N de la secuencia de fibonacci");
    io::stdin().read_line(&mut input).unwrap();
    let number: usize = input.trim().parse().unwrap();
    for i in 0..number {
        let mut memo = HashMap::new();
        println!("{}", fib_memorization(i, &mut memo));
    }

}

pub fn fib_memorization(n: usize, memoria: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = memoria.get(&n) {
        return *v;
    }
    let v = match n {
        0 | 1 => 1,
        _ => fib_memorization(n-2, memoria) + fib_memorization(n-1, memoria),
    };
    memoria.insert(n, v);
    v
}
