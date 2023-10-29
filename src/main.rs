use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::process::exit;

#[derive(Clone)]
struct ColumnData {
    a: String,
    b: String,
    c: String,

}


fn main() {
    //let file = File::open("characters.txt")?;
    let file = File::open("test_data.txt").expect("Failed to open file");

    let mut columns: Vec<ColumnData> = vec![];

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split('\t').collect();

        let a = String::from(fields[0]);
        let b = String::from(fields[1]);
        let c = String::from(fields[2]);

        let column = ColumnData {
            a,
            b,
            c,
        };
        columns.push(column);
    }

}
