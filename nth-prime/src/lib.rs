pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut num = 2;

    while primes.len() <= n as usize {
        if is_prime(num) {
            primes.push(num);
        }
        num += 1;
    }

    primes[n as usize]
}

fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}
