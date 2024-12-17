use rand::seq::SliceRandom;

const RAND_STR: &[u8] = b"123456789QWERTYU!@#$%^&Mqwertyuiopasdhgjk";

pub fn gen_pass_with_length(len: u8) -> anyhow::Result<()> {
    // step1. generate password
    let mut rand_gen = rand::thread_rng();
    let mut password = Vec::new();
    for _ in 0..len {
        password.push(*RAND_STR.choose(&mut rand_gen).expect("error"));
    }
    let res = String::from_utf8(password)?;
    eprintln!("generate passwrod : {}", res);
    // step2. check password
    let estimate = zxcvbn::zxcvbn(&res, &[]);
    println!("{}", estimate.score()); // 3

    anyhow::Ok(())
}
