
// 定义一个全局的 常量
const MAX_POINT: u32 = 100_000;

fn main(){
    // shadow 可以改变变量的数据类型
    let shadow_test: &str = "  ";                       // 字面量类型的字符串
    let shadow_test: usize = shadow_test.len();         // shadow 为 usize 数据类型

    // 整数类型
    // 8 bit        i8       [-2^7, 2^7 - 1]       u8        [0, 2^8 - 1]
    // 16 bit       i16                            u16
    // 32 bit       i32                            u32
    // 64 bit       i64                            u64
    // 128 bit      i128                           u128
    // arch         iszie                          usize
    let num: u8 = 100;

    // 浮点类型
    // f32
    // f64
    let num: f32 = 1.0;
    let num: f64 = 2.0;

    // 字符串
    // 字面量字符串（&str）     字面量字符串是不可变的切片，存储在程序的只读内存区域（通常是静态存储区域）。长度固定，高效、无需动态分配内存，适用于静态内容或无需修改的字符串
    // 堆分配字符串（String）   String 是一个动态可变的字符串类型，存储在堆上。可变（如果使用 mut 修饰）。可以动态增长或修改内容。通常用于需要构造或操作字符串的场景
    let str1: &str = "hello";

    let mut str2: String = String::from("world");
    str2.push_str(" ! ");

    let str1: String = str1.to_string();            // 字面量字符串 --> 堆分配字符串
    let str2: &str = &str2;                         // 堆分配字符串 --> 字面量字符串

    println!("{}", str1);
    println!("{}", str2);

    // 元组 Tuple
    // Tuple 中可以放置多个类型
    // Tuple 长度固定，一旦声明无法更改
    let tuple: (u8, f64, bool, &str) = (100, 2.9, true, "str");
    let (a, b, c, d) = tuple;                           // 解构tuple
    println!("{}, {}, {}, {}", a, b, c, d);
    println!("{}, {}, {}, {}", tuple.0, tuple.1, tuple.2, tuple.3);

    // 数组
    // 数组中元素必须相同
    // 数组长度固定
    let arr: [u8; 3] = [0, 1, 2];
    let arr: [u8; 3] = [5; 3];                  // 相当于 [5, 5, 5]



}