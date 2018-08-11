use std::io;

fn main() {
    println!("Please Input Count > ");

    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Failed to read line");;

    let count: u64 = count.trim().parse().expect("Need u64");
    println!("PI = {}",wallis_fomula(count));
}

fn wallis_fomula(count : u64) -> f64 {
    let mut pi : f64 = 1.0;

    for i in 1..count {
        let n = (4 * i * i) as f64;
        pi *= n / (n - 1.0);
    }
    let ans = pi * 2.0;
    return ans;
}
