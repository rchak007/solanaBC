

fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..5];
    // Iterate over the elements of the slice
    for element in nice_slice.iter() {
        println!("{}", element);
    }
}


fn main() {

    slice_out_of_array();
}