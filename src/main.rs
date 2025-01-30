use std::io::Lines;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

struct Records {
    iter: Lines<BufReader<File>>
}

impl Iterator for Records {
    type Item = HashMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = HashMap::new();
        let next = self.iter.next();

        if next.is_none() {
            return None
        }

        while next_line(&mut self.iter) != "M  END" { }

        loop {
            let line = next_line(&mut self.iter);

            if line == "$$$$" {
                break Some(result);
            }

            let name = read_name(&line);
            let data = read_data(&mut self.iter);

            result.insert(name, data);
        }
    }
}

fn next_line(iter: &mut Lines<BufReader<File>>) -> String {
    match iter.next() {
        Some(line) => {
            line.expect("reading line")
        },
        None => panic!("unexpected EOF")
    }
}

fn read_name(line: &String) -> String {
    let mut characters = line.chars();

    if let Some(first) = characters.next() {
        if first != '>' {
            panic!("no leading > on header line");
        }
    } else {
        panic!("unexpected blank line");
    }

    let mut result = String::new();
    let mut capture = false;

    for character in characters {
        if capture {
            if character == '>' {
                return result;
            } else {
                result.push(character);
            }
        } else {
            if character == '<' {
                capture = true;
            }
        }
    }

    if capture == true {
        panic!("no closing >");
    } else {
        panic!("no opening <");
    }
}

fn read_data(iter: &mut Lines<BufReader<File>>) -> String {
    let mut result = next_line(iter);

    loop {
        let line = next_line(iter);

        if line.is_empty() {
            break result;
        }

        result.push_str(&line);
    }
}

fn main() -> std::io::Result<()> {
    let name = "/Users/rich/Downloads/Compound_000000001_000500000.sdf";
    let file = File::open(name)?;
    let reader = BufReader::new(file);
    let records = Records { iter: reader.lines() };
    let stop = "5-(5-diazoimidazol-4-yl)-1H-1,2,4-triazole";

    for record in records {
        if let Some(name) = record.get("PUBCHEM_IUPAC_NAME") {
            if name == stop {
                let cid = record.get("PUBCHEM_COMPOUND_CID").expect("CID");

                println!("Found name at CID {}", cid);

                return Ok(());
            }
        }
    }

    println!("name not found");

    Ok(())
}
