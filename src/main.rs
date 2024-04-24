// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好";
//     let english = "World, hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter() {
//         println!("{}", &region);
//     }
// }

// fn main() {
//     // greet_world()
//     // println!("Hello, world!");
//     let penguin_data= "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";

//     let records = penguin_data.lines();
//     for (i, record) in records.enumerate() {
//         if i==0 || record.trim().len()==0 {
//             continue;
//         }
//         // 声明一个 fields 变量，类型是 Vec
//     // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//     // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫

//     let fields: Vec<_>=record
//         .split(',')
//         .map(|filed| filed.trim())
//         .collect();
//     if cfg!(debug_assertions) {
//          // 输出到标准错误输出
//        eprintln!("debug: {:?} -> {:?}",
//        record, fields);
//     }

//     let name = fields[0];
//     // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
//     //
//     // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//     //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//     //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
//     //
//     // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//     if let Ok(length) = fields[1].parse::<f32>() {
//         // 输出到标准输出
//         println!("{}, {}cm", name, length);
//     }
//     }
// }

// Rust 程序入口函数，跟其它语言一样，都是 main，该函数目前无返回值
// fn main() {
//     // 使用let来声明变量，进行绑定，a是不可变的
//     // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
//     // 语句的末尾必须以分号结尾
//     let a = 10;
//     // 主动指定b的类型为i32
//     let b: i32 = 20;
//     // 这里有两点值得注意：
//     // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
//     // 2. c是可变的，mut是mutable的缩写
//     let mut c = 30i32;
//     // 还能在数值和类型中间添加一个下划线，让可读性更好
//     let d = 30_i32;
//     // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
//     let e = add(add(a, b), add(c, d));

//     // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
//     // 该函数将指定的格式化字符串输出到标准输出中(控制台)
//     // {}是占位符，在具体执行过程中，会把e的值代入进来

//     println!("( a + b ) + ( c + d ) = {}", e);
// }

// // 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
// fn add(i: i32, j: i32) -> i32 {
//     // 返回相加值，这里可以省略return
//     i + j
// }

// 字符串使用双引号 "" 而不是单引号 ''，Rust 中单引号是留给单个字符类型（char）使用的
// Rust 使用 {} 来作为格式化输出占位符，其它语言可能使用的是 %s，%d，%p 等，由于 println! 会自动推导出具体的类型，因此无需手动指定

// fn main() {
//     let mut x =5;
//     println!("The value of x is: {}",x);
//     x = 6;
//     println!("The value of x is: {}",x);
// }

// 使用下划线开头忽略未使用的变量
// fn main() {
//     let _x = 5;
//     let y = 10;
//     let _y = 10;
// }

//变量解构
// fn main() {
//     let (a, mut b): (bool, bool) = (true, false);
//     // a = true,不可变; b = false，可变
//     println!("a={:?}, b={:?}",a,b);

//     b= true;
//     assert_eq!(a,b)
// }

// //解构体赋值
// struct Struct {
//     e: i32,
// }

// fn main() {
//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };

//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
// }

// 常量
// const MAX_POINTS: u32 = 100_00000;

// // 变量遮蔽
// fn main() {
//     let x = 5;
//     // 在main函数的作用域内对之前的x进行遮蔽
//     let x = x + 1;
//     {
//         // 在当前的花括号作用域内，对之前的x进行遮蔽
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }
//     println!("The value of x is: {}", x);

//     // 字符串类型
//     let spaces = " ";
//     // usize数值类型
//     let spaces = spaces.len();

//     let mut spaces = "   ";
//     spaces = spaces.len();
// }

// practice
// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let x: i32 = 2; // 未初始化，但被使用
//     let _y: i32; // 未初始化，也未被使用
//     println!("x is equal to {}", x);
// }

// 完形填空，让代码编译
// fn main() {
//     let mut x = 1;
//     x += 2;

//     println!("x = {}", x);
// }

// 修复下面代码的错误并使用尽可能少的改变
// fn main() {
//     let x: i32 = 10;
// let y: i32 = 20;
//     {
//         let y: i32 = 5;
//         println!("x 的值是 {}, y 的值是 {}", x, y);
//     }
//     println!("x 的值是 {}, y 的值是 {}", x, y);
// }

// 修复错误
// fn main() {
//     let x = define_x();
//     println!("{}, world", x);
// }

// fn define_x() -> String {
//     // let x = "hello";
//     // let x = "hello";
//     let x = "hello".to_string();
//     x
// }

// // 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
// fn main() {
//     let x: i32 = 5;
//     {
//         let x = 12;
//         assert_eq!(x, 12);
//     }

//     assert_eq!(x, 5);

//     let x = 42;
//     println!("{}", x); // 输出 "42".
// }

// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // 遮蔽且再次绑定
//     let x = x;

//     let y = 4;
//     // 遮蔽
//     let y = "I can also be bound to text!";
// }

// fn main() {
//     let _x = 1;
// }

// // compiler warning: unused variable: `x`

// 修复下面代码的错误并尽可能少的修改
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// fn main() {
//     let (x, y) = (1, 2);
//     let x = 3;

//     assert_eq!(x, 3);
//     assert_eq!(y, 2);
// }

// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     // 填空，让代码工作
//     assert_eq!([x,y], [3,2]);
// }


// 类型推导与标注
fn main() {
    // let guess = "42".parse().expect("Not a number!");
    let guess: i32 = "42".parse().expect("Not a number!");

}
