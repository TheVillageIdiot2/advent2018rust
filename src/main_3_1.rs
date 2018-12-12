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

#[derive(Debug)]
struct Rectangle {
    id: usize,
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
        (Rectangle{id: id as usize,
                   top: lt.1 as usize,
                   left: lt.0 as usize,
                   width: wh.0 as usize,
                   height: wh.1 as usize})
    )
);

const DIM: usize = 1000;

fn main() {
    println!("Hello, world!");
    // Get the file
    let mut file : File = File::open("./input_3.txt").expect("File not found");
    let mut contents: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut contents).unwrap();

    // Let er rip
    let mut buf = &contents[..];
    let mut board : [[u8; DIM]; DIM] = [[0; DIM]; DIM]; // Number of claims

    // Process file
    loop {
        let rect = get_rectangle(buf);
        match rect {
            Ok((rem, rect)) => {
                println!("Rect {:?}", rect);

                // Fill in areas that are designated
                let bot = rect.top + rect.height;
                let right = rect.left + rect.width;
                for row in rect.top..bot {
                    for col in rect.left..right {
                        board[row][col] += 1
                    }
                }

                // Move the buf
                buf = rem;
            },

            Err(_e) => {
                break
            }
        }
    }

    // Now count all of the trues
    let mut count = 0;
    for row in 0..DIM {
        for col in 0..DIM {
            if board[row][col] >= 2 {
                count += 1;
            }
        }
    }

    println!("Stacked claims: {}", count);
}
