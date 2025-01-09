
use std::fs::File;
use std::result::Result;
use std::io::Error;
use std::io::ErrorKind;


fn main(){
    let file1: Result<File, Error> = File::open("./src/hello.txt");
    let file2: Result<File, Error> = File::open("./src/hello.txt");

    let file1 = match file1 {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file {:?}", error)
        }
    };

    let file2 = match file2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./src/hello.txt") {
                Ok(file_created) => file_created,
                Err(file_created_error) =>  panic!("file created error opening file {:?}", file_created_error)
            },
            other => panic!("Other error opening file {:?}", other)
        }
    };

    // 下面的 unwrap 和 expect 都是上面的简写方法
    // 如果打开成功，那么就是返回file
    // 如果打开失败，那么unwrap就是返回内置的错误，该错误信息如法修改；expect就是输出自定义的错误，相比之下更加灵活
    let file3 = File::open("./src/unwrap.txt").unwrap();
    let file4 = File::open("./src/expect.txt").expect("无法打开文件 expect.txt");


}