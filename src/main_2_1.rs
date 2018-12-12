use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

//#[macro_use]
//extern crate nom;

const LETTERS: usize = 26;

struct Abacus {
    char_counts : [usize; LETTERS],
    double_counts : usize,
    triple_counts : usize
}

impl Abacus {
    fn new() -> Abacus {
        Abacus {
            char_counts : [0; LETTERS],
            double_counts : 0,
            triple_counts : 0
        }
    }
}

fn main() {
    println!("Hello, world!");
    // Get the file
    let file : File = File::open("./input_2.txt").expect("File not found");
    let mut reader  = BufReader::new(file);

    // Parse
    let mut box_id : String = String::new();
    let mut count_with_double = 0;
    let mut count_with_triple = 0;
    while let Ok(_) = reader.read_line(&mut box_id) {
        // Break if empty
        if box_id.is_empty() {
            break;
        }

        // Create our abacus
        let mut abacus = Abacus::new();

        // Iterate the string
        for c in box_id.as_bytes() {
            // Skip newlines
            if *c == b'\n' { break; }

            // Get the index in our abacus
            let index : usize = (c - b'a') as usize;

            // Increment the appropriate index by 1
            abacus.char_counts[index] += 1;
            let tot = abacus.char_counts[index];

            // Update our total doubles/triples
            match tot {
                2 => {
                    abacus.double_counts += 1;
                },
                3 => {
                    abacus.double_counts -= 1;
                    abacus.triple_counts += 1;
                },
                4 => {
                    abacus.triple_counts -= 1;
                },
                _ => ()
            }
        }

        // Add counts as necessary
        if abacus.double_counts != 0 {
            count_with_double += 1;
        }
        if abacus.triple_counts != 0 {
            count_with_triple += 1;
        }

        // Empty our string
        box_id = String::new();
    }

    // Print our final results
    println!("Final checksum {}", count_with_double * count_with_triple);
}
