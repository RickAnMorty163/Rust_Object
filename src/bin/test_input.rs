fn main() {
    println!("Hello World");
    for val in 1..10 {
        println!("the {} th fibonacci number is {} ", val, fibs(val));
    }
}

fn fibs(n: u32) -> u32 {
    let mut prev_1 = 0;
    let mut prev_2 = 1;

    for _ in 2..n {
        let mut temp=prev_1+prev_2;
        prev_1=prev_2;
        prev_2=temp;
    }
    prev_1
}
