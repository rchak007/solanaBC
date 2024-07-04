fn main() {


        let x = 10; // x is an i32 value
        let y = &x; // y is an immutable reference to x
    
        println!("Value of x: {}", x);
        println!("Value of y: {}", y); // y dereferences automatically in println!
    
        let mut z = 20; // z is a mutable i32 value
        let w = &mut z; // w is a mutable reference to z
    
        *w += 15; // Modify the value through the mutable reference
        // println!("Value of z after modification: {}", z);
        let ab = &y;
        *w += y;
        println!("Value of z now : {}", z);
        // println!("Value of w now : {}", w);

    


}