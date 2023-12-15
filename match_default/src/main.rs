fn main() {
    // Option
    let x = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");
    // Result
    let _path = std::env::var("IMPORTANT_PATH")
        .expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");

    // Option
    let x = Some("air");
    assert_eq!(x.unwrap(), "air");
    // Result
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 21);

    // Option
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    assert_eq!(Some("air").unwrap_or("bi"),"car");
    assert_eq!(None.unwrap_or("bike"),"bike");
    // Result
    let  default :u32 = 2;
    let x:Result<u32,&str> = Ok(9);
    assert_eq!(x.unwrap_or(default),9);
    let x:Result<u32,&str>= Err("error");
    assert_eq!(x.unwrap_or(default),default);

    // Option
    let x: Option<u32> = None;
    let y: Option<u32> = Some(12);

    assert_eq!(x.unwrap_or_default(), 0);
    assert_eq!(y.unwrap_or_default(), 12);
    // Result
    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year = good_year_from_input.parse().unwrap_or_default();
    let bad_year = bad_year_from_input.parse().unwrap_or_default();
    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);
}

// expect(): panic并抛出指定异常信息
// unwrap(): panic不带信息
// unwrap_or(): 不panic,返回设置的默认值
// unwrap_or_default(): 不panic,返回正确值类型的默认值
