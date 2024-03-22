fn main() {
    println!("Hello world!");
    let res: i32 = select_number(42);
    println!("{}", res);
}

fn select_number(x: i32) -> i32 {
    if x > 0 {
        return x;
    } else {
        return -x;
    }
}
