use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::process::exit;

#[derive(Clone)]
struct ColumnData {
    a: i64,
    b: String,
    c: String,

}


fn main() {
    let file = File::open("test_data.txt").expect("Failed to open file");

    let mut columns: Vec<ColumnData> = vec![];

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split('\t').collect();

        let a = fields[0].parse().unwrap();
        let b = String::from(fields[1]);
        let c = String::from(fields[2]);

        let column = ColumnData {
            a,
            b,
            c,
        };

        columns.push(column);
    }

    //sort_select(&mut columns);
    sort_bubbles(&mut columns);

    for column in columns{
        println!("data:{}/{}/{}",column.a,column.b,column.c);
    }

}

//選択ソート
fn sort_select( columns:&mut Vec<ColumnData> ) {
    
    let len = columns.len();

    for i in 0..len - 1 {
        let mut min_idx = i;
        for j in i + 1..len {
            if columns[j].a < columns[min_idx].a {
                min_idx = j;
            }
        }

        // 最小値を入れ替え
        if min_idx != i {
            columns.swap(i, min_idx);
        }
    }
}

//バブルソート
fn sort_bubbles(columns:&mut Vec<ColumnData>){
    let len = columns.len();
    for i in 0..len - 1{
        for j in i + 1..len{
            if columns[j].a < columns[i].a{
                columns.swap(i, j);
            }
        }
    }
}