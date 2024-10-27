use std::env;

fn fibo(max: usize) -> usize {
    if max == 0 {
        return 0;
    } else if max == 1 {
        return 1;
    } else {
        return fibo(max - 1) + fibo(max - 2);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = &args[1];
    let max = arg.parse::<usize>().unwrap();

    for i in 0..max + 1 {
        println!("{}: {}", i, fibo(i));
    }
}
