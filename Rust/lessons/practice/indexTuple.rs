fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;
    // let second = &numbers.1;

    println!("second = {}", second);
    println!("second from tuple = {}", numbers.1);
}

fn main() {

    indexing_tuple();
}