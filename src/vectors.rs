fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("{}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("{} there's a value", third),
        _ => println!(""),
    }

    //loop through vectors
    for i in &mut v {
        println!("{}", i);
        *i += 50;
        println!("increased with ref : {}", i);
    }
}
