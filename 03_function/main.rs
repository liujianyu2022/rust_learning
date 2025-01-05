// 函数体有一系列语句组成，可以选择由一个表达式结束
// 语句：不会返回值，只是执行一些动作
// 表达式：会返回一个值

fn main(){
    let x = 5;              // 可以理解为表达式，把 5 返回给了y

    let y = {
        x + 3               // 可以理解为表达式，把 x+3 返回给了 y
    };

    let z = {
        x + 3;              // x+3; 这是一个语句，相当于返回了一个空的元组()，就类似于js中默认返回undefined
    };

    let flag: bool = if x > 0 { true } else { false };         // 这也是一个表达式，类似于三元运算，注意：每个分支的结果类型必须是一样的

    println!("{}, {}, {:#?}", x, y, z);

    println!("{}", flag);

    println!("test1(x) = {}", test1(x));
    println!("test2(x) = {}", test2(x));
}

// 在 Rust 中，返回值就是函数体里面最后一个表达式的值
// 如果想要提前返回，那么就使用return
fn test1(x: i32) -> i32 {
    x + 3                   // 这是表达式
}

fn test2(x: i32) -> i32 {
    return x + 3;
}