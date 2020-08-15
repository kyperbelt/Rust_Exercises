use std::env;


/*
 Things I learned here:
    - recursion sometimes is not the best approach ha!
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let input : usize = {
        match &args[1].parse::<usize>() {
            Ok(x) => *x,
            Err(_) => panic!("invalid input"),
        }
    };
    
    println!("nth fibonacci number for {0} is: \n {1}",input,better_fibo(input));
  
}

//This is the solution that I came up with - too slow with bigger numbers
#[allow(dead_code)]
fn fibo(number:i128)->i128{
    if number <= 1 {number} else{fibo(number-1)+fibo(number-2)}
}

//this solution is much faster
fn better_fibo(number:usize)->i128{
    let mut x : Vec<i128> = vec![0,1];
    for _ in 2..(number+1){
        x.push(x[x.len()-1] + x[x.len()-2]);
    }
    x[number]
}


