
fn connect1(){
    let str1: String = String::from("hello ");
    let str2: String = String::from("world");

    // 使用了类似于 fn add(self, s: &str) -> String {} 这样的方法
    // 第二项使用了 解引用强制转换，将 String 的引用改为了 字符串字面值 
    let str3: String = str1 + &str2;                 // 使用 + 拼接字符串，前面会交出所有权，后面只是引用

    // println!("{}", str1);                         // 由于上面拼接操作的时候，str1已经把所有权移交了，因此 str1 不能再用了
    println!("{}", str2);
    println!("{}", str3);
}

fn connect2() {
    let str1: String = String::from("hello ");
    let str2: String = String::from("world");

    let str3: String = format!("{}-{}", str1, str2);    // 不会获得参数的所有权

    println!("{}", str1);                         
    println!("{}", str2);
    println!("{}", str3);

}

// 在 rust 中，有三种看待字符串的方式
// 字节         str1.bytes()
// 标量值       str1.chars()
// 字形簇

fn concept() {
    let str1: &str = "hello!";
    let str2: &str = "你好！";

    // 一个英文字母占1个字节
    for (index, byte) in str1.bytes().enumerate() {
        println!("The {} byte is {}", index + 1, byte);
    }

    println!("---------------------------");
    
    for (index, character) in str1.chars().enumerate() {
        println!("The {} byte is {}", index + 1, character);
    }

    println!("---------------------------");

    // 一个汉字占3个字节
    for (index, byte) in str2.bytes().enumerate() {
        println!("The {} byte is {}", index + 1, byte);
    }

    println!("---------------------------");
    
    for (index, character) in str2.chars().enumerate() {
        println!("The {} byte is {}", index + 1, character);
    }
}


fn main() {
    let mut str1: String = String::new();

    let mut str2: String = "666".to_string();           // 字符串字面值转为String
    let str3: String = String::from("777");             // 

    str2.push_str("六");
    str2.push_str(&str3);                               // 追加字符串

    concept();
}