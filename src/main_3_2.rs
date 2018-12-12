use std::fs::File;
use std::ops::{AddAssign, MulAssign};
use std::io::Read;

#[macro_use]
extern crate nom;

// Make a function to convert arbitrary bufs to int
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

// Just get a number
named!(take_i32 <&[u8], i32>,
    map!(take_while!(nom::is_digit), buf_to_int)
);

// Make a parser to express the elf id
named!(elf_id <&[u8], i32>,
    preceded!(tag!("#"), take_i32)
);

// For the upper left corner
named!(origin <&[u8], (i32, i32)>,
    pair!(take_i32, preceded!(tag!(","), take_i32))
);

// For the dimensions
named!(dimension <&[u8], (i32, i32)>,
    pair!(take_i32, preceded!(tag!("x"), take_i32))
);

type Id = usize;

#[derive(Debug)]
struct Rectangle {
    elf_id: Id,
    top: usize,
    left: usize,
    width: usize,
    height: usize
}

named!(get_rectangle <&[u8], Rectangle>,
    do_parse!(
        id: elf_id >>
        tag!(" @ ") >>
        lt: origin >>
        tag!(": ") >>
        wh: dimension >>
        opt!(tag!("\n")) >>
        (Rectangle{elf_id: id as Id,
                   top: lt.1 as usize,
                   left: lt.0 as usize,
                   width: wh.0 as usize,
                   height: wh.1 as usize})
    )
);

const DIM: usize = 1000;
const UNCLAIMED: Id = 0;
const VALID : bool = true;
const INVALID : bool = false;


fn main() {
    println!("Hello, world!");
    // Get the file
    let mut file : File = File::open("./input_3.txt").expect("File not found");
    let mut contents: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut contents).unwrap();

    // Let er rip
    let mut buf = &contents[..];

    // Make the board
    let mut board = vec![vec![UNCLAIMED; DIM]; DIM]; // id of claims. 0 = uncliamed

    // Make a vec of "ok-ness", denoting if the specified ID has been crossed
    // True = ok
    let mut validity_table : Vec<bool> = Vec::new();

    // Push an arbirtrary value so all future accesses can just use id directly
    validity_table.push(INVALID);

    // Process file
    loop {
        let rect = get_rectangle(buf);
        match rect {
            Ok((rem, rect)) => {
                // Find the bounds of the rect
                let bot = rect.top + rect.height;
                let right = rect.left + rect.width;

                // Make a flag to track whether or not this rect was encroached on.
                let mut ok_flag = VALID;

                // Fill in areas that are designated
                for row in rect.top..bot {
                    for col in rect.left..right {
                        // Mark as invalid whatever was there before, since it intersects with us
                        // Unclaimed will randomly cycle the first index but who cars
                        let existing_claim = board[row][col];
                        validity_table[existing_claim] = INVALID;

                        // Now put ourselves on this tile of the board
                        board[row][col] = rect.elf_id;

                        // Also track whether WE are valid
                        if existing_claim != UNCLAIMED {
                            ok_flag = INVALID;
                        }
                    }
                }

                // Now that we have toggled that area, add this elf to the validity buffer
                validity_table.push(ok_flag);

                // Move the buf
                buf = rem;
            },

            Err(_e) => {
                break
            }
        }
    }

    // At this point there should be only one lucky elf in the validity table who is true
    for (id, is_valid) in validity_table.iter().enumerate() {
        if *is_valid {
            println!("Good elf!: {}", id);
        }
    }
}
