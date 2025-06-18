fn is_prime(n: i32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    let mut count = 0;
    let mut start = 2;
    while count < 1000 {
        if is_prime(start) {
            count += 1;
            println!("{}", start);
        }
        start += 1;
    }
}
