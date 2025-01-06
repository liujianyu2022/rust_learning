
// 切片：一种不支持所有权的数据类型
// 字符串切片：指向字符串一部分内容的引用

fn main(){
    let str: String = String::from("hello world");

    let sub_str1: &str = &str[0..5];
    let sub_str2: &str = &str[6..11];
    let sub_str3: &str = &str[..5];
    let sub_str4: &str = &str[6..];
    let sub_str5: &str = &str[..];

    let sub_str6: &str = test1(&str);
    println!("{}", sub_str6);

    test2(sub_str1);
    test2(&str[..]);
}

// 遍历一个字符串，截取 ' ' 之前的字符串
fn test1(str: &String) -> &str {

    for (index, char) in str.chars().enumerate() {
        if char == ' ' {
            return &str[..index];
        }
    }

    return &str[..];
}

// 将函数参数定义为 &str ，这样的话在传参的时候，可以接收 字符串字面值 和 &String
fn test2(str: &str){}