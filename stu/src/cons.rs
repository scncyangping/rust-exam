//! 常量定义
//
//! # Author
//!
//! - Yapi
//!
//! # Date
//!
//! 2024/06/07

use std::borrow::Cow;

pub trait ConstTrait {
    const ACCOUNT_FIELD: &'static str = "account";
    const PASSWORD_FIELD: &'static str = "password";
    fn do1(&self);
}

pub struct A {
    name: String,
}

impl ConstTrait for A {
    const ACCOUNT_FIELD: &'static str = "account_A";
    const PASSWORD_FIELD: &'static str = "password_A";
    fn do1(&self) {
        println!(
            "A account_filed: {}, account_password: {}",
            Self::ACCOUNT_FIELD,
            Self::PASSWORD_FIELD
        )
    }
}

pub struct B {
    name: String,
}

impl ConstTrait for B {
    fn do1(&self) {
        println!(
            "B account_filed: {}, account_password: {}",
            Self::ACCOUNT_FIELD,
            Self::PASSWORD_FIELD
        )
    }
}

#[test]
fn test_print() {
    let a = A {
        name: "123".to_string(),
    };
    let b = B {
        name: "123".to_string(),
    };

    a.do1();
    b.do1();
}

#[test]
fn test_cow() {
    fn do_update_str(st: &str) -> String {
        if st.contains("A") {
            let mut x = st.to_owned();
            x.push_str("1111");
            x
        } else {
            return st.to_owned();
        }
    }

    fn do_update_str_with_cow(st: &str) -> Cow<str> {
        if st.contains("A") {
            let mut x = st.to_owned();
            x.push_str("1111");
            Cow::Owned(x)
        } else {
            Cow::Borrowed(st)
        }
    }

    fn print(st: &String) {
        println!("st: {}", st)
    }
    let x = "vvv";

    let result = do_update_str_with_cow(x);
}
