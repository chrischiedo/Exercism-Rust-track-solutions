pub fn factors(n: u64) -> Vec<u64> {
    let mut current = n;
    let mut prime_factors: Vec<u64> = vec![];

    let max_factor = (n as f64).sqrt() as u64 + 1;

    for i in 2..max_factor + 1 {
        if current % i == 0 && is_prime(i) {
            while current% i == 0 {
                prime_factors.push(i);  
                current /= i;
            }
        }
    }
    if current > 1 && is_prime(current) {
        prime_factors.push(current);
    }
    prime_factors
}

// Utility function to determine if a given number is prime
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    true
}
