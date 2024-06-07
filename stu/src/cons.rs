//! 常量定义
//
//! # Author
//!
//! - Yapi
//!
//! # Date
//!
//! 2024/06/07

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
