use std::env;


/*
    Things learned here:
        - Environment Variables 
        - Assignment using match
        - Error Handling
        - Using Function pointers
        - String to Slice (String => &str using &s[..])
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let command  : fn(f32)->f32 = {
        match  &args[1][..]{
            "f2c"=> f_to_c,
            "c2f"=> c_to_f,
            _=> panic!("{0} not a valid command.",args[0])
        }
    };
    let input : f32 = {
        match &args[2].parse::<f32>() {
            Ok(x) => *x,
            Err(_) => panic!("invalid input"),
        }
    };
    let result = command(input);
    println!("when {0} is converted using [{1}] the result is :{2}",input,args[1],result);

}

pub fn f_to_c(fahrenheit : f32) -> f32 {
    (fahrenheit - 32.0) * 0.55555555555
}

pub fn c_to_f(celcius : f32) -> f32{
    celcius * 1.8 + 32.0
}
