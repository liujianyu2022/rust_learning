
fn main(){

    let vector1: Vec<i32> = vec![0, 1, 2];

    let item: &i32= &vector1[2];                    // 索引访问，如果索引超出范围，程序会报错

    match vector1.get(2) {                          // get方法访问，如果索引超出范围，程序会执行None分支
        Some(item) => println!("{}", item),
        None => println!("none")
    }

    for item in &vector1 {
        println!("{}", item);
    }

    println!("---------------------------------------");

    let mut vector2: Vec<i32> = Vec::new();
    vector2.push(0);
    vector2.push(1);
    vector2.push(2);

    // 修改可变的vector
    for item in &mut vector2 {
        *item += 10;
    }

    for (index, item) in vector2.iter().enumerate() {
        println!("index: {}, item: {}", index, item);
    }



}