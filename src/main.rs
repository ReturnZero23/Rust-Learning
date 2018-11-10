fn main() {
    //打印字符串
    println!("Hello, world!");
    println!("Hello, {} {}!", "rust","world!");

    //元组
    let num : (i8,i8) = (1,2);

    println!("{}", num.0);

    //数组类型，注意数组长度的标识
    let array: [i32;4]= [1,2,3,4];

    println!("{}", array[2]);

    println!("{}", "print array");

    //这里会发生数越界的情况 使用export RUST_BACKTRACE=1，能够查看详细的调用堆栈的信息
    for _i in 0..3 {
        println!("{}", array[_i]);
    }

}
