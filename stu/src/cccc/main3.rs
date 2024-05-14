fn main() {
    let s1 = String::from("123");
    let s2 = &s1;
    print(&s1);

}

fn print(m: &str){
    println!("print info: {m}")
}