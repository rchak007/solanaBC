fn main() {
    let a = 10;
    

    let b = &a;
    println!("B = {}", b); 
    let c = &b;

    println!("A = {}", a); // Use `a` after modification
    println!("C = {}", c); // Use `a` after modification

    let mut d = a;
    d = d + 5;

    println!("A = {}", a); // Use `a` after modification
    println!("D = {}", d); // Use `a` after modification

}