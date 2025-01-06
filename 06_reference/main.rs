

fn main(){
    let str1: String = String::from("hello");

    let len1: usize = test1(&str1);             // 这里只是将str1的引用给了test1函数，当test1执行完毕之后，str1的数据并不会被销毁
    println!("{}", str1);

    let len2: usize = test2(str1);              // 这里将str1的所有权给了test2函数，当test1执行完毕之后，str1的数据并会被销毁，因此后面str无法被访问了
    // println!("{}", str1);                        // borrow of moved value: `str1`

    println!("----------------------------------------------------------------------");

    let mut str2: String = String::from("hello");   // 可变变量
    let len3: usize = test3(&mut str2);         // 可变引用，可以通过可变引用修改原来的值
    println!("{}, {}", str2, len3);

    // 可变引用：在特定作用域内，对于某个数据只能拥有一个可变引用
    let reference1: &mut String = &mut str2;
    println!("{}", reference1);

    // let reference2: &mut String = &mut str2;        // cannot borrow `str2` as mutable more than once at a time
    // println!("{}", reference2);                     // 由于对 str2 添加了两个可变引用，于是报错了，可以防止数据竞争

    println!("----------------------------------------------------------------------");

    // 可以创建新的作用域，来创建非同时的多个可变引用
    let mut str3: String = String::from("hello");   // 可变变量

    {
        let reference3: &mut String = &mut str3;
        println!("{}", reference3);
    }

    let reference4: &mut String = &mut str3;
    println!("{}", reference4);
    


}

// 让函数使用某个值，但是不获得其使用权
// & 引用 reference，允许引用某些值而不获得其所有权
// 借用：把引用作为函数参数的行为
fn test1(str: &String) -> usize {
    return str.len();
}

// 直接把值给函数，那么该函数将会获得其所有权
fn test2(str: String) -> usize {
    return str.len();
}

// 可变引用
fn test3(str: &mut String) -> usize {
    str.push_str("!");
    return str.len();
}