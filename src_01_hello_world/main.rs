use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // 在 rust 中，所有的变量默认是 immutable 的，如果需要变量可变，那么需要添加 mut
    // 生成一个随机数字 1..=101 表示 [1, 101)
    let secret_number = rand::thread_rng().gen_range(1..=101);

    loop{
        println!("input a number !");

        // 在 rust 中，所有的变量默认是 immutable 的，如果需要变量可变，那么需要添加 mut
        let mut guess_number = String::new();
    
        io::stdin().read_line(&mut guess_number).expect("can not read line");
    
        // 在 rust 中，允许变量重名。
        let guess_number: u32 = match guess_number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("input number is {}", guess_number);
    
        match guess_number.cmp(&secret_number){
            Ordering::Less => println!("The guess number is less than the secret number! "),
            Ordering::Greater => println!("The guess number is greater than the secret number! "),
            Ordering::Equal => {
                println!("Win!");
                break;
            } 
        }
    }
}
