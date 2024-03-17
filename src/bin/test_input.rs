use std::io
fn main() {
    println!("Hello World");
    let mut n:u32;
    io::stdin().read_line(&mut n).expect("Error");
    println!("fibs={:?}",fibs(n));
}

fn fibs(n:u32) ->u32{
    let mut prev_1=0;
    let mut prev_2=1;

    for _ in 2..=n {
        let next = prev_1+prev_2;
        prev_1=prev_2;
        prev_2=next;
    } 
    prev_1
}
