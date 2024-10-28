use std::{fmt, ops::Deref, str};

// MyString里, String 有3个Word,供24字节，所以它以8字节对齐
// 所以enum的tag + padding最少 8 字节，整个结构占 32字节
// 所以MiniString可以有 30 字节(再加上1字节长度和1字节tag)，就是32字节
const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();

        let length = bytes.len();

        let mut data = [0u8; MINI_STRING_MAX_LEN];

        data[..length].copy_from_slice(bytes);

        Self {
            len: length as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
        // 也可以直接用unsafe版本
        // unsafe {
        //     str::from_utf8_unchecked(&self.data[..self.len as usize])
        // }
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 这里由于实现了Deref trait, 可以直接得到一个&str输出
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.to_owned()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("len1:{}, len2:{}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "hello worldhello worldhello中国 worldhello worldhello world".into();
    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    // display输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );
    // Mystring 可以使用一切&str接口
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("hello"));

    let x = String::from("world world");
    let xx = vec!['a', 'b', 'c', 'd', 'e'];
    let y = &x[1..2];
    let yy = &xx[1..2];
}
