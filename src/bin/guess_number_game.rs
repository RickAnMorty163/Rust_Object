use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess a number");

    //生成随机数secret_number
    let secret_number = rand::thread_rng().gen_range(0, 101);

    //println!("Thr secret number is: {}", secret_number);
    loop {
        println!("Please guess a number:");

        //读取用户输入
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //变量隐藏：Rust中允许用一个新的值隐藏guess之前的值
        //trim()方法去掉字符串前后的空格和\n换行符，parse()方法将字符串解析为u64类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     //成功解析，返回对应的u64类型
            Err(_) => continue, //失败，返回continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            //比较guess和secret_number
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Your are winner! ");
                break;
            }
        }
    }
}
