use std::fs::File;
use std::io::Write;
use std::{num, result};

#[derive(Debug)]
struct D {
    r: i32,
}

#[derive(Debug)]
struct Record<'a> {
    data: &'a str,
    len: usize,
}

impl Record<'_> {
    fn have_prefix(&self, prefix: &str) -> Option<&str> {
        if self.data.starts_with(prefix) {
            Some(self.data)
        } else {
            None
        }
    }
}

fn main() {
    let s = String::from("hello Sami");

    fn parse_record<'a>(data: &'a str) -> Record<'a> {
        Record {
            data,
            len: data.len(),
        }
    }
    let d2 = parse_record(&s);
    let result = d2.have_prefix("hello");
    match result {
        Some(data) => println!("Data:{}", data),
        None => println!("No data"),
    }
    let mut local_file = File::create("hello.txt").expect("Issue with create");
    say_hello(&mut local_file, &"Izancu so Hello, World! Iancu").expect("Issue with write");
}

fn say_hello<S>(out: &mut dyn Write, msg: S) -> std::io::Result<()>
where
    S: AsRef<str>,
{
    out.write_all(msg.as_ref().as_bytes())
        .expect("was some issue");
    out.flush()
}
