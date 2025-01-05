fn main(){
    let arr1: [u8; 5] = [1, 2, 3, 4, 5];

    for (index, &item) in arr1.iter().enumerate() {
        println!("Index: {}, Value: {}", index, item);
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("counter = {}, result = {}", counter, result);
}