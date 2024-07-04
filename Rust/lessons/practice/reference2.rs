// fn main() {
//     let mut a = 10;
//     let b = &a;

//     // println!("B = {}", b);

//     a = 25;
//     println!("B = {}", b);
// }

fn main() {
    let mut a = 10;
    
    {
        let b = &a;
        println!("B = {}", b); // Use `b` within its own scope
    } // `b` goes out of scope here

    a = 25; // Now it is safe to modify `a`
    println!("A = {}", a); // Use `a` after modification
}
