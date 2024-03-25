fn main(){
    println!("Hello, world!");
    let user1=User{
        email:String::from("rickanmorty@163.com"),
        username:String::from("RSK"),
        sign_n_count:1,
        active:true,
    };
    println!("user1 email {}",user1.email);
    println!("user1 username {}",user1.username);
    println!("user1 sign_n_count {}",user1.sign_n_count);
    println!("user1 active {}",user1.active);
}

struct User{
    username:String,
    email:String,
    sign_n_count:u64,
    active:bool,
}