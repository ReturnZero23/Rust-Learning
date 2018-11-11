mod lib; // 导入其他模块的代码

use lib::*; //使用 lib 模块中的模块  不需要带 lib 名

fn print_sum(a:i32,b:i32) -> i32 {
    println!("{}", a);
    println!("{}", b);
    println!("{}", a + b);
    a + b // 返回值这行不加分号
}

#[derive(Debug)]
struct People {
    name: String,
    id: i32,
    address: String,
}

impl People {
    // 关联函数 实现了一个类似于 c++ 的静态函数的功能，不依赖于某个实例，通常可以用来创建这个类型的对象
    fn make_people(name: String,id: i32) -> People {
        People {
            name,
            id,
            address: String::from("beijing"),
        }
    }

    fn rename(&mut self, name: String){
        self.name = name;
    }
}

// 这是一个全局的函数，不绑定特定的域
fn make_people(name: String, id: i32) -> People {
    People {
        name,
        id,
        address:String::from("tianjing"),
    }
}

#[derive(Debug)]
enum IPversion {
    v4,
    v6,
}

#[derive(Debug)]
enum Color {
    Red(String),
    Green(String),
    Blue(String),
}

#[derive(Debug)]
struct IPv4 {
    address:(i32,i32,i32,i32),
}

#[derive(Debug)]
struct IPv6 {
    address: String,
}

//枚举变量可以绑定不同类型的值
#[derive(Debug)]
enum IPVersion {
    IPv4(IPv4),
    IPv6(IPv6),
}


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

    let sum = print_sum(2, 10);

    println!("sum = {}", sum);

    let condition = true;

    // if 语句可以带返回值，不同分支的返回值的类型必须是一样的
    let res = if condition{
        println!("{}", "baidu");
    }else{
        println!("{}", "google");
    };

    println!("{:?}", res);

    // 1..4 只会打印到 3 结束，不会打印 4
    for pat in (1..4).rev() {
        println!("rev = {}", pat);
    }
    // for 循环迭代一个数组 需要使用。iter() 进行
    for pat in ["baidu","google","bing"].iter() {
        println!("{}", pat);
    }
    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i = i + 1;
    }


    let mut j = 0;
    loop {
        if j > 5 {
            break;
        }
        println!("loop = {}", j);
        j = j + 1;
    }

    let mut str1 = "baidu";
    str1 = "bing";

    let mut str2 = String::from("google");
    str2.push_str("baidu");

    //Copy trait 和 Drop trait 没有办法同事使用
    //clone 在对于只在栈上的数据类型没有什么意义，
    // 对于那些在栈上的数据类型，大小已知，所以默认是 clone 的
    // string 类型赋值给其他的 变量的时候，之前的变量会变成无效的
    let str3 = str2.clone();
    println!("str2 = {}", str2);
    println!("str3 = {}", str3);

    let s = String::from("baidu");
    takes_ownership(s.clone());
    println!("{}", s);

    let n : i32 = 0;
    makes_copy(n);
    println!("{}", n);

    //这里发生了所有权转移，s 在函数调用时候没有办法继续使用
    //let (s2,len) = calculate_length(s);

    //这里使用了引用，函数结束之后还能继续使用s
    let size = calculate_length_reference(&s);

    println!("{}", s);

    //允许同时存在多个不可变引用且不存在可变引用
    //只允许在一个作用域中存在一个可变引用，且不允许存在不可变引用，类似 c++ 中 unique_lock 和 shared_lock 的作用。

    let mut str3 = String::from("bing");
    // let str4 = &str3;
    // let str5 = &str3;
    // println!("{}", str4);
    // println!("{}", str5);
    //let str6 = &mut str3;
    let str6 = "hello world!";

    // first_word(str6);
    second_word(str6);

    let man : People = People {
        name: String::from("xiaoming"),
        id: 123,
        address: String::from("beijing"),
    };


    println!("{:#?}", man);

    let mut women : People = People::make_people(String::from("xiaohong"), 32);
    println!("{:#?}", women);
    women.rename(String::from("xiaoli"));
    println!("{:#?}", women);

    let mut child : People = make_people(String::from("xiaohai"), 6);
    println!("{:#?}", child);

    let version : IPversion = IPversion::v6;
    MatchIPVersion(version);

    let Redcolor : Color = Color::Red(String::from("红色"));

    let color = match Redcolor {
        Color::Red(str) => println!("{}", str),
        Color::Blue(str) => (),
        Color::Green(str) => (),
    };

    println!("{:?}", color);

    //同一个变量可以重新绑定
    let IP: IPVersion = IPVersion::IPv4(IPv4 {address:(127,0,0,1)});
    let IP: IPVersion = IPVersion::IPv6(IPv6 {address:String::from("::127:0:0:1")});

    // 解绑变量，可以直接使用
    // 匹配是穷举的，没有穷举的变量，使用_ 统配符 匹配
    match IP {
        IPVersion::IPv4(ip) => println!("{}.{}.{}.{}", ip.address.1,ip.address.2,ip.address.3,ip.address.0),
        _ => println!("通配符"),
    }

    let num: Option<i32> = Option::Some(5);

    match num {
        Some(expr) => println!("{}", expr),
        None => (),
    }

    let res = match num {
        Some(expr) => Some(expr + 1),
        None => None,
    };

    match res {
        Some(expr) => println!("{}", expr),
        None => (),
    }


    // if let 语法糖
    if let Some(6) = res {
        println!("{:?}", res)
    } else {
        println!("else  branch")
    }

    Network::PrintNetVersion();
}


// 存在堆上的数据 会被占用所有权，
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

//存在栈上的数据不会被占用所有权
fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String,usize) {
    let length = s.len();
    (s,length)
}
// 使用引用的语意 不会占用所有权，
fn calculate_length_reference(s:&String) -> usize {
    s.len()
}

//只能使用 &String 类型
fn first_word(s:&String) -> &str {
    &s[..]
}

// 使用&str 更加的通用，可以传入&String 或者是字面值
fn second_word(s:&str) -> &str{
    &s[..]
}


//返回一个借用，借用的变量的声明周期比所有者的声明周期更长，所以会发生变异错误
// fn dangle() -> &String {
//     let s = String::from("google");
//     &s
// }

fn MatchIPVersion(Version: IPversion){
    match Version {
        IPversion::v4 => println!("IPversion v4"),
        IPversion::v6 => println!("IPversion v6"),
    }
}
