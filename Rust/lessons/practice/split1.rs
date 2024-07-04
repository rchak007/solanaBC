// use solana_program::{msg, program_error::ProgramError};
use std::str::from_utf8;
use std::io;


pub fn unpack(input: &[u8]) -> Result<(), io::Error> {
    let split = input.split_first();
    

    let (function_flag, _rest) = split.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid parameters passed"))?;

    println!("Function flag is {:?}", function_flag);
    println!("Rest is {:?}", _rest);
    let get5 = _rest.get(..5);
    let wrapRest5 = get5.unwrap();
    let unwrapRestStr = from_utf8(_rest).unwrap();
    println!("unwrapRestStr is {}", unwrapRestStr);
    println!("wrapRest5 is {:?}", wrapRest5);
    let wrapRest5Str = from_utf8(wrapRest5).unwrap();
    println!("wrapRest5Str ={} ", wrapRest5Str);
    


    match function_flag {
        0 => {
            // Handle case 0
            println!("Function flag is 0");
            Ok(())
        },
        48 => {
            // Handle case 0
            println!("Function flag is 48 from match");
            Ok(())
        },
        1 => {
            // Handle case 1
            println!("Function flag is 1");
            Ok(())
        },
        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid function flag")),
    }
}



fn main() {
    // Convert the string to bytes
    // let input_string = "";
    let input_string = "0Hello, Solana!";
    let input_bytes = input_string.as_bytes();
    // let input_bytes: [u8; 15] = [0, 72, 101, 108, 108, 111, 44, 32, 83, 111, 108, 97, 110, 97, 33]; // 0Hello, Solana!
    // println!("input_string = {}", input_string);
    println!("input_bytes = {:?}", input_bytes);

    // println!("input_bytes = {}", input_bytes);
    // Call the unpack function
    match unpack(&input_bytes) {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }

    let ex1Str = "Rafa the Goat";
    let ex1Str5 = ex1Str.get(..4);
    println!("ex1Str5 = {:?}", ex1Str5);
    let unwrapEx1Str5 = ex1Str5.unwrap();
    println!("Unwrapped str5 = {}", unwrapEx1Str5);


}