use std::{fs::File, io::Read};

use base64::{engine::general_purpose, Engine};

pub fn b64_encode(input: &str) -> anyhow::Result<()> {
    let mut reader = get_read(input)?;
    let mut res = String::new();
    let _ = reader.read_to_string(&mut res)?;
    println!("encode read resource string:{}", res);
    let res = general_purpose::STANDARD.encode(res.trim());
    println!("encode string:{}", res);
    anyhow::Ok(())
}

pub fn b64_decode(input: &str) -> anyhow::Result<()> {
    let mut reader = get_read(input)?;
    let mut res = String::new();
    let _ = reader.read_to_string(&mut res)?;
    println!("decode read resource string:{}", res);
    let res = general_purpose::STANDARD.decode(res.trim())?;
    let res = String::from_utf8(res)?;
    println!("decode string:{}", res);
    anyhow::Ok(())
}

fn get_read(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let read: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    anyhow::Ok(read)
}
