use nth_prime::nth;

fn main(){
    println!("{}th prime is {}", 0, nth(0));
    println!("{}th prime is {}", 1, nth(1));
    println!("{}th prime is {}", 2, nth(2));
    println!("{}th prime is {}", 3, nth(3));
    println!("{}th prime is {}", 4, nth(4));
    println!("{}th prime is {}", 5, nth(5));
    println!("{}th prime is {}", 10000, nth(10000));
}