fn main() {
    // 演示 Vec 的 into_iter() 如何消费所有权
    let vec = vec![1, 2, 3];

    // 第一次使用 vec
    println!("Length: {}", vec.len());

    // 第二次使用 vec - 这里会出错！
    // for item in vec.into_iter() {
    //     println!("Item: {}", item);
    // }

    // 正确的做法：只使用一次
    let vec = vec![1, 2, 3];
    for item in vec.into_iter() {
        println!("Item: {}", item);
    }

    // 或者使用引用
    let vec = vec![1, 2, 3];
    println!("Length: {}", vec.len());
    for item in &vec {
        println!("Item: {}", item);
    }
}
