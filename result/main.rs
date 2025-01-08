// 错误的传递：把错误返回给调用者

// ? 运算符：传播错误的一种快捷方式
// 如果Result是OK，Ok中的值就是表达式的结果，然后继续执行程序
// 如果Result是Err，Err就是整个函数的返回值，就像是return了

use std::io;
use std::io::Read;
use std::fs::File;

fn read_name_1() -> Result<String, io::Error>{
    let file = File::open("./src/hello.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut str = String::new();

    let result = match file.read_to_string(&mut str) {
        Ok(_) => Ok(str),
        Err(e) => Err(e)
    };

    return result;
}

fn read_name_2() -> Result<String, io::Error>{
    let mut file = File::open("./src/hello.txt")?;
    // 上面 ? 的含义就是下面这段代码
    // 如果成功了，那么就返回file对象
    // 如果失败了，那么就 return Err，整个函数执行到此结束
    // let mut file = match file {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };

    let mut str = String::new();

    file.read_to_string(&mut str)?;
    // let result = match file.read_to_string(&mut str) {
    //     Ok(_) => Ok(str),
    //     Err(e) => Err(e)
    // };
    // return result;

    Ok(str)
}

fn main (){
    let res1 = read_name_1();
    let res2 = read_name_2();

    println!("{:#?}", res1);
    println!("{:#?}", res2);
}