


fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false; 
    }
    if num <= 3 {
        return true; 
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false; 
        }
        i += 6; 
    }
    true
}

fn main() {
    let num = 17;
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}
