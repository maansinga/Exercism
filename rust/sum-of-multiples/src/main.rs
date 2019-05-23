use sum_of_multiples::sum_of_multiples;

fn main(){
    println!("sum_of_multiples({}, &[{}, {}]) = {} should be {}",
             100, 3, 5, 2318, sum_of_multiples(100, &[3, 5]))
}