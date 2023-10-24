use rand::Rng; // trait
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜测数字!");
    // 默认使用i32
    // 经过guess比较之后变为u32
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("神秘数字是{}", secret_number);

    // 循环体
    loop {
        println!("猜测一个数字");
        // 在rust中，默认所有的变量都是不可变的
        // let mut foo = 1;
        // let bar = foo; // immutable

        // 可变参数 let mut guess
        let mut guess = String::new();
        // io::Result Ok,Err
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
