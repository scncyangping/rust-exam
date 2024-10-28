use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::Write;
struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

// 实现Debug trait 打印字符串
impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn main() {
    let mut f = File::create("rust-exam/trait_exam/ttt").unwrap();
    let w: &mut dyn Write = &mut f;

    w.write_all(b"heelo").unwrap();

    // error the `by_ref` method cannot be invoked on a trait object
    // trait object 返回值不能是Self或者携带范型参数
    // 上述 let w: &mut dyn Write = &mut f;修改为
    // let w = &mut f;即可解决
    //let w1 = w.by_ref();
    //w1.write_all(b"word").unwrap();

    let mut buf = BufBuilder::new();

    buf.write_all(b"Hello world").unwrap();

    println!("{:?}", buf);

    println!("result u8: {}", u8::parse("255 hello world").unwrap());

    println!("result f64: {}", f64::parse("255.234 hello world").unwrap());

    // add test
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);

    println!("{:?}", c1 + c2);

    // c1, c2 已经被移动，下面这句编译失败
    // 若想操作过后依然能使用，为Complex引用类型也实现trait
    //println!("{:?}", &c1 + &c2);
    //println!("{:?}", c1 + c2);
}

use std::str::FromStr;

pub trait Parse {
    // 定义带有关联类型的trait
    // 定义一种Error, 这个Error在具体实现trait时定义,不用预先定义
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl<R> Parse for R
where
    R: FromStr + Default,
{
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();

        if let Some(cap) = re.captures(s) {
            cap.get(0)
                .map_or(Err("failed to captrue".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_err| "failed to parse".to_string())
                })
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

// pub trait Parse {
//     fn parse(s: &str) -> Self;
// }

// impl Parse for u8 {
//     fn parse(s: &str) -> Self {
//         let re: Regex = Regex::new(r"^[0-9]+").unwrap();
//         if let Some(cp) = re.captures(s) {
//             cp.get(0).map_or(0, |s| s.as_str().parse().unwrap_or(0))
//         } else {
//             0
//         }
//     }
// }

#[test]
fn parse_should_work() {
    assert_eq!(u8::parse("123abcd"), Ok(123));
    assert_eq!(u8::parse("abcd"), Err("failed to parse string".into()));
}

// add trait
use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imag = self.imagine + rhs.imagine;

        Self::new(real, imag)
    }
}

impl Add for &Complex {
    // 此时，返回值就不是Self了，因为此时Self是&Complex
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imag = self.imagine + rhs.imagine;

        Complex::new(real, imag)
    }
}
