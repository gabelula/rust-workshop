// Writing a wordcount utility
// https://github.com/gabelula/rust-workshop/blob/master/curriculum/05-wordcount.md

use std::fs::File;
use std::io::Read;
use std::io;
use std::env;

// kittens -c myfile.txt
// kittens -l myfile.txt
// kittens -w myfile.txt
fn main() {

    let option = env::args().nth(1).expect("No argument");
    let file_path = env::args().nth(2).expect("No argument");

    if option == "c" {
        println!("{} has {} characters.", file_path, count_characters(&file_path).expect("Failing counting words"));
    }

    if option == "w" {
        println!("{} has {} words.", file_path, count_words(&file_path).expect("Failing counting words"));
    }

    if option == "l" {
        println!("{} has {} lines.", file_path, count_lines(&file_path).expect("Failing counting words"));
    }


}

// fn print_usage(program: &str, opts: Options) {
//     let brief = format!("Usage: {} FILE [options]", program);
//     print!("{}", opts.usage(&brief));
// }
//
fn count_words(file_path: &str) -> io::Result<i32> {

    let mut f = try!(File::open(file_path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    Ok(s.split_whitespace().count() as i32)

}

fn count_lines(file_path: &str) -> io::Result<i32> {
    let mut f = try!(File::open(file_path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    Ok(s.lines().count() as i32)
}

fn count_characters(file_path: &str) -> io::Result<i32> {
    let mut f = try!(File::open(file_path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    Ok(s.len() as i32)
}

// Test
// wc -w myfile.txt = 6
#[test]
fn it_works(){
    let file_path = "myfile.txt";
    assert_eq!(count_words(file_path).expect("Not counting words."), 6);

    assert_eq!(count_lines(file_path).expect("Not counting lines."), 1);

    assert_eq!(count_characters(file_path).expect("Not counting characters"), 35)
}
