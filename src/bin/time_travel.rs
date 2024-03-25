fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("hello {}", hello);
    let world = &s[6..11];
    println!("world {}", world);
    let len = s.len();
    let slice = &s[..len];
    println!("slice {}", slice);

    let nums = [1, 2, 4, 5];
    let cut = &nums[1..4]; //数组切片
    println!("{:?}", cut);

    let res = first_world(&s);
    println!("{}", res);
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes(); //字符串转换为字节数组
    for (i, &item) in bytes.iter().enumerate() {
        //迭代字节数组
        if item == b' ' {
            //遇到空格
            return &s[0..i]; //返回字符串
        }
    }
    &s[..] //返回整个字符串
}
