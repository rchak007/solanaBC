fn main() {

    // Reverse an Array
    // Here's a simple puzzle to get you more familair with Rust and it's
    // methods.

    let mut array = [1, 2, 3, 4,];
    // add your code here
    let array1 = array;
    let mut arrayLen = array.len();
    println!("Array length {}", arrayLen);

    println!("before reversing {:?}", array);

    for i in 0..array.len() {
        array[(arrayLen-1)] = array1[i];
        arrayLen = arrayLen - 1;
    }

    println!("{:?}", array);
    println!("{:?}", array1);


    }