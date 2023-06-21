use std::io;
use std::collections::HashMap;


fn main() {
    let mut input = String::new();
    println!("Ingrese el numero N de la secuencia de fibonacci");
    io::stdin().read_line(&mut input).unwrap();
    let number: usize = input.trim().parse().unwrap();
    for i in 0..number {
        let mut memo = HashMap::new();
        println!("{}", fib_memoization(i, &mut memo));
    }

}

pub fn fib_memoization(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = memo.get(&n) {
        return *v;
    }

    let v = match n {
        0 | 1 => 1,
        _ => fib_memoization(n-2, memo) + fib_memoization(n-1, memo),
    };

    memo.insert(n, v);
    v
}
