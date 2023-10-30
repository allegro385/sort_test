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
    //sort_bubbles(&mut columns);
    //sort_insert(&mut columns);
    //sort_insert_sub(&mut columns,1);
    quick_sort(&mut columns);
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

//挿入ソート
fn sort_insert(columns:&mut Vec<ColumnData>){
    //sort_insert_sub(columns,1)
    let len = columns.len();
    for i in 1..len{
        for j in (0..i-1).rev() {
            if columns[j].a >= columns[i].a {
                columns.swap(i, j);
            }
        }
    }
}

//挿入ソートサブ
// fn sort_insert_sub(columns:&mut Vec<ColumnData>,gap:usize){
//     let len = columns.len();
//     for i in gap..len{
//         for j in (0..i - gap).rev() {
//             if columns[j].a >= columns[i].a {
//                 columns.swap(i, j);
//             }
//         }
//     }
// }

// //シェルソート
// fn sort_shell(columns:&mut Vec<ColumnData>){
//     let n=3;
//     let len = columns.len();
//     for i in 0..len{

//     }
// }

//クイックソート
fn quick_sort(arr: &mut [ColumnData]) {
    let len = arr.len();

    if len < 2 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);

    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition(arr: &mut [ColumnData]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;

    arr.swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j].a <= arr[len - 1].a {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}
