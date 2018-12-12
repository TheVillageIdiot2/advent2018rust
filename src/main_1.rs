use std::fs::File;
use std::ops::{AddAssign, MulAssign};
use std::io::Read;

#[macro_use]
extern crate nom;

// Make a function to convert arbitrary bufs to int
// Stolen from stackoverflow
fn buf_to_int<T>(s: &[u8]) -> T
where T: AddAssign + MulAssign + From<u8>,
{
    let mut sum = T::from(0);
    for digit in s {
        sum *= T::from(10);
        sum += T::from(*digit - b'0');
    }
    sum
}

// Make a parser to express sign as a factor 1 or -1
named!(sign <&[u8], i32>, alt!(
    value!(1,  tag!("+")) |
    value!(-1, tag!("-"))
)
);

// Make a parser to read until newline and then return that as an int
named!(int_to_end <&[u8], i32>,
    map!(take_until_and_consume!("\n"), buf_to_int)
);


// Make a parser to handle line by line input
named!(reading <&[u8], i32>,
    map!(pair!(sign, int_to_end), {|(s, v): (i32, i32)|  s * v})
);



fn main() {
    println!("Hello, world!");
    // Get the file
    let mut file : File = File::open("./input_1.txt").expect("File not found");
    let mut contents: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut contents).unwrap();

    // Let er rip
    let mut buf = &contents[..];
    let mut sum : i32 = 0;

    loop {
        let result = reading(buf);
        match result {
            Ok((remainder, amount)) => {
                sum += amount;
                println!("Got {}. Total now {}", amount, sum);
                buf = remainder;
            },

            Err(e) => {break}
        }
    }



}
