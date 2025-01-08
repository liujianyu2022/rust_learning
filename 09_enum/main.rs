// 定义一个枚举
enum IpKind1 {
    V4,
    V6
}

// 每个变体可以指定不同的数据类型和关联的数据量
enum IpKind2 {
    V4(String),
    V6(String)
}
enum IpKind3 {
    V4(u8, u8, u8, u8),
    V6(String)
}

// 定义一个结构体
struct IpAddr {
    IpKind1: IpKind1,
    address: String,
}


// rust 中类似于 Null 概念的枚举  Option<T>
// Option<T>, Some<T>, None 都是包含在 prelude 预导入模块中，可以直接使用
// 如果想要使用 Option<T> 中的 T，必须将其转为 T
// 可以避免如下的错误：let a = null; let b = a + "123"
// 标准库中的定义：
// enum Option<T> {
//     Some<T>,
//     None
// }






fn main(){
    let v4: IpKind1 = IpKind1::V4;
    let v6: IpKind1 = IpKind1::V6;

    let ipAddr1: IpAddr = IpAddr { 
        IpKind1: IpKind1::V4, 
        address: String::from("127.0.0.1")
    };

    let num: Option<u8> = Some(5);
    let str: Option<&str> = Some("5");
    let none: Option<u8> = None;
}