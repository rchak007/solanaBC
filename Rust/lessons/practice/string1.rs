fn main() {
    let answer = current_favorite_course();
    println!("My course is {}", answer);

    let answer2 = current_favorite_course2();
    println!("My earlier course is {}", answer2);
}

fn current_favorite_course() -> String {
    "Solana".to_string()
}


fn current_favorite_course2() -> &'static str {
    "Cardano"
}