use super::*;
use std::io::{BufRead, BufReader, Cursor, Read};

#[test]
fn test_name() {
    let cur = Cursor::new(b"GET /index.html HTTP/1.1\r\nfoo");
    let mut reader = BufReader::new(cur);

    let mut buff = String::with_capacity(1024);
    reader.read_line(&mut buff).unwrap();
    println!("Result: {buff:}")
}
