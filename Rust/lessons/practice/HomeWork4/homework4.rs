
fn fizz_buzz() {
    let mut counter = 0;
    for i in 1..300 {
        // println!("Iteration number: {}", i);
        if i % 3 == 0 {
            println!("fizz {}", i);
            if i % 5 == 0 {
                println!("fizz buzz {}", i);
                counter += 1;
            }
        } 
        if i % 5 == 0 {
            println!("buzz {}", i);
        }

    }
    println!("Number of fizzBuzz = {}", counter);
}


fn main() {

    println!("Welcome to homework 4 ");
    fizz_buzz();
}