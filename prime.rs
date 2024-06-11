//Implement a function that checks whether a given number is prime or not.


fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; 
    }
    if n <= 3 {
        return true; 
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false; 
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
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
