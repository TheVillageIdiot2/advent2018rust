use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::Ordering;

// NOTE: This solution sucks ass

const LETTERS: usize = 26;

// Make a class to represent box ids, and their char sum
struct BoxRegistry {
    char_simple_hash : u64,
    char_counts : [u32; LETTERS],
    id : String,
}

// Make a new function that computes the sum
impl BoxRegistry {
    fn new(id : String) -> BoxRegistry {
        // Find the char counts / hash / whatever
        let mut char_counts = [0; LETTERS];
        let mut char_simple_hash : u64 = 1;
        id.trim_end().bytes().for_each(|c| {
            let c = c - 'a' as u8;
            let i = c as u64;
            char_counts[i as usize] += 1;
            char_simple_hash += i;
        });

        // Return the value
        BoxRegistry {
            id,
            char_counts,
            char_simple_hash
        }
    }
}

impl PartialEq for BoxRegistry {
    fn eq(&self, other: &BoxRegistry) -> bool {
        self.char_simple_hash == other.char_simple_hash
    }
}

impl PartialOrd for BoxRegistry {
    fn partial_cmp(&self, other: &BoxRegistry) -> Option<Ordering> {
        self.char_simple_hash.partial_cmp(&other.char_simple_hash)
    }
}

impl Eq for BoxRegistry {
//    fn eq(&self, other: &BoxRegistry) -> bool {
//        self.char_simple_hash == other.char_simple_hash
//    }
}

impl Ord for BoxRegistry {
    fn cmp(&self, other: &BoxRegistry) -> Ordering {
        self.char_simple_hash.cmp(&other.char_simple_hash)
    }
}

//impl Ord for BoxRegistry {

//}

fn main() {
    // Get the file
    let file : File = File::open("./input_2.txt").expect("File not found");
    let mut reader  = BufReader::new(file);

    // Create vars
    let mut box_id : String = String::new();
    let mut records : Vec<BoxRegistry> = Vec::new();

    // Parse into registry entries
    while let Ok(_) = reader.read_line(&mut box_id) {
        // Break if empty
        if box_id.is_empty() {
            break;
        }

        // Create our registry entry
        let record = BoxRegistry::new(box_id);

        // Put it in the tree
        records.push(record);

        // Empty our string
        box_id = String::new();
    }

    // Sort the vecs
    records.sort();

    // Create a buffer of "int range" values
    let mut lower_bound : usize = 0;
    for candidate in 1..records.len() {
        println!("Processing candidate");
        // Iterate over all values that could potentially match
        for sub_candidate in lower_bound..candidate {
            // Get the entries
            let candidate_val = &records[candidate];
            let sub_candidate_val = &records[sub_candidate];

            // Compute the delta
            let delta = candidate_val.char_simple_hash - sub_candidate_val.char_simple_hash;
            // Depending on delta, try a more refined comparison
            println!("Delta: {}", delta);
            match delta {
                0..=26 => {
                    // Make sure they differ by exactly one
                    let mut diff_count = 0;
                    for (c1, c2) in     candidate_val.id.as_bytes().iter()
                        .zip(               sub_candidate_val.id.as_bytes().iter()) {
                        if c1 != c2 {
                            diff_count += 1;
                        }
                        if diff_count > 1 {
                            // break;
                        }
                    }
                    println!("Scanning diffs; score = {}", diff_count);

                    // Now diff count is an int >= 1. If its 1, then we've solved!
                    if diff_count == 1 {
                        // Find the letters in common
                        // Should in theory just be all letters except the one that didn't work out
                        for (c1, c2) in candidate_val.id.chars().zip(sub_candidate_val.id.chars()) {
                            if c1 == c1 {
                                print!("{}", c1 as char );
                            }
                        }

                        println!("\n{}:{}\n{}:{}\nDone", candidate, candidate_val.id, sub_candidate, sub_candidate_val.id);
                        return;
                    }
                },
                _ => {
                    // Delta too big! Move range
                    println!("Bumped range");

                    if lower_bound < (candidate - 1) {
                        lower_bound += 1;
                    }
                }
            }
        }
    }

    // Print our final results
    println!("Failed");
}
