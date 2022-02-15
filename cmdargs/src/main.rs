use std::str::FromStr;
use std::env;

fn gcd(m: u64, n: u64) -> u64 {
    let mut m = core::cmp::max(m, n);
    let mut d = core::cmp::min(n, n);
    loop {
        let rem = m % d;
        if rem == 0 { break; }
        m = d;
        d = rem;
    }
    d 
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);

        assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                       3 * 7 * 11 * 13 * 19),
                   3 * 11); 
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
    Ok(())
}
