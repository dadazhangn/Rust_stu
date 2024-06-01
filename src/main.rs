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
// fn main() {
//     // let guess = "42".parse().expect("Not a number!");
//     let guess: i32 = "42".parse().expect("Not a number!");

// }

// 整型溢出
// fn main() {
//     let a: u8 = 255;
//     let b = a.wrapping_add(20);
//     println!("{}", b);
// }

// 浮点类型
// fn main() {
//     // let x = 2.0;
//     // let y: f32 = 3.0;

//     // assert!(0.1+0.2==0.3);
//     // assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.0001);

//     // let abc: (f32,f32,f32) = (0.1,0.2,0.3);
//     // let xyz: (f64,f64,f64) = (0.1,0.2,0.3);

//     // println!("abc (f32)");
//     // println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     // println!("         0.3: {:x}", (abc.2).to_bits());
//     // println!();

//     // println!("xyz (f64)");
//     // println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     // println!("         0.3: {:x}", (xyz.2).to_bits());
//     // println!();

//     // assert!(abc.0 + abc.1 == abc.2);
//     // assert!(xyz.0 + xyz.1 == xyz.2);

//     // NAN
//     // let x = (-42.0_f32).sqrt();
//     // // assert_eq!(x,x);

//     // if x.is_nan() {
//     //     println!("未定义的数学行为")
//     // }

//     // 数字运算
//         // 加法
//         let sum = 5 + 10;

//         // 减法
//         let difference = 95.5 - 4.3;

//         // 乘法
//         let product = 4 * 30;

//         // 除法
//         let quotient = 56.7 / 32.2;

//         // 求余
//         let remainder = 43 % 5;

// }

// fn main() {
//     // 编译器会进行自动推导，给予twenty i32的类型
//     let twenty = 20;
//     // 类型标注
//     let twenty_one: i32 = 21;
//     // 通过类型后缀的方式进行类型标注：22是i32类型
//     let twenty_two = 22i32;

//     // 只有同样类型，才能运算
//     let addition = twenty + twenty_one + twenty_two;
//     println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

//     // 对于较长的数字，可以用_进行分割，提升可读性
//     let one_million: i64 = 1_000_000;
//     println!("{}", one_million.pow(2));

//     // 定义一个f32数组，其中42.0会自动被推导为f32类型
//     let forty_twos = [
//       42.0,
//       42f32,
//       42.0_f32,
//     ];

//     // 打印数组中第一个值，并控制小数位为2位
//     println!("{:.2}", forty_twos[0]);
//   }

// 位运算
// fn main() {
//     // 二进制为00000010
//     let a:i32 = 2;
//     // 二进制为00000011
//     let b:i32 = 3;

//     println!("(a & b) value is {}", a & b);

//     println!("(a | b) value is {}", a | b);

//     println!("(a ^ b) value is {}", a ^ b);

//     println!("(!b) value is {} ", !b);

//     println!("(a << b) value is {}", a << b);

//     println!("(a >> b) value is {}", a >> b);

//     let mut a = a;
//     // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
//     a <<= b;
//     println!("(a << b) value is {}", a);
// }

// 序列
// fn main() {
//     for i in 1..=5 {
//         // println!("{}",i);
//     }

//     for i in 'a'..='z' {
//         println!("{}",i);
//     }

// }

// 有理数和复数
// use num::complex::Complex;

// fn main() {
//     let a = Complex {re: 2.1, im: -1.2};
//     let b = Complex::new(11.1,22.2);
//     let result = a+b;
//     println!("{}+{}", result.re, result.im);

// }

// practice

// 移除某个部分让代码工作
// fn main() {
//     let x: i32 = 5;
//     let mut y = 5;

//     y = x;

//     let z = 10; // 这里 z 的类型是?
// }

// // 填空
// fn main() {
//     let v: u16 = 38_u8 as u16;
// }

// 修改 `assert_eq!` 让代码工作
// fn main() {
//     let x = 5;
//     assert_eq!("i32".to_string(), type_of(&x));
// }

// // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// 填空，让代码工作
// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 255);
// }

// 解决代码中的错误和 `panic`
// fn main() {
//     let v1 = 247_u8 + 8;
//     let v2 = i8::checked_add(119, 8).unwrap();
//     println!("{},{}",v1,v2);
//  }

// // 修改 `assert!` 让代码工作
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
//     assert!(v == 1597);
// }

// 将 ? 替换成你的答案
// fn main() {
//     let x = 1_000.000_1; // f64
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64
// }

// use num::complex::ComplexFloat;

// use num::Float;

// fn main() {
//     // assert!(0.1+0.2==0.3);
//     assert!((0.1+0.2-0.3).abs()< 0.0001);
//     assert!(0.1_f32+0.2_f32==0.3_f32);
// }

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z' {
//         println!("{}",c as u8);
//     }
// }

// 填空
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));
// }

// // // 填空，并解决错误
// fn main() {
//     // 整数加法
//     assert!(1u32 + 2 == 3);

//     // 整数减法
//     assert!(1i32 - 2 == -1);
//     assert!(1i8 - 2 == -1);

//     assert!(3 * 50 == 150);

//     assert!(9 / 3 == 3); // error ! 修改它让代码工作

//     assert!(24 % 5 == 4);

//     // 逻辑与或非操作
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);

//     // 位操作
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

// 字符
// fn main() {
//     // let c = 'z';
//     // let z = 'ℤ';
//     // let g = '国';
//     // let heart_eyed_cat = '😻';

//     let x = '中';
//     println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
// }

// 布尔
// fn main() {
//     let t = true;

//     let f: bool = false; // 使用类型标注,显式指定f的类型

//     if f {
//         println!("这是段毫无意义的代码");
//     }
// }

// practice
// 修改2处 `assert_eq!` 让代码工作

// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),4);

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),4);

//     println!("Success!")
// }

// // 修改一行让代码正常打印
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// }

// fn print_char(c : char) {
//     println!("{}", c);
// }

// 使成功打印
// fn main() {
//     let _f: bool = false;

//     let t = true;
//     if t {
//         println!("Success!")
//     }
// }

// fn main() {
//     let f = true;
//     let t = true && false;
//     assert_eq!(!t, f);

//     println!("Success!")
// }

// 让代码工作，但不要修改 `implicitly_ret_unit` !
// fn main() {
//     let v0: () = ();

//     let v = (2, 3);
//     assert_eq!(v0, implicitly_ret_unit());

//     println!("Success!")
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()")
// }

// // 不要使用下面的函数，它只用于演示！
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }

// 让代码工作：修改 `assert!` 中的 `4`
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);

//     println!("Success!")
// }

// 语句和表达式
// fn add_with_extra(x: i32, y: i32) -> i32 {
//     let x = x + 1; //语句
//     let y = y + 5; //语句
//     x + y //表达式
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }

// fn main() {
//     assert_eq!(ret_unit_type(), ())
// }

// fn ret_unit_type() {
//     let x = 1;
//     // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
//     // 类似三元运算符，在Rust里我们可以这样写
//     let y = if x % 2 == 1 {
//         "odd"
//     } else {
//         "even"
//     };
//     // 或者写成一行
//     let z = if x % 2 == 1 { "odd" } else { "even" };
// }

// practice
// fn main() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // 下面表达式的值将被赋给 `y`
//         x_cube + x_squared + x
//     };

//     let z = {
//         // 分号让表达式变成了语句，因此返回的不再是表达式 `2 * x` 的值，而是语句的值 `()`
//         2 * x;
//     };

//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

// 使用两种方法让代码工作起来
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2;
//         x
//     };

//     assert_eq!(v, 3);
//  }
//  fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };

//     assert_eq!(v, ());
//
//  fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };

//     assert_eq!(v, ());
//  }

// fn main() {
//     let v = {
//         let x = 3;
//         x
//     };

//     assert!(v == 3);
// }

// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// 函数参数
// fn main() {
//     another_function(5, 6.1);
// }

// fn another_function(x: i32, y: f32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// 函数返回
// fn main() {
//     let x = plus_five(5);

//     println!("The value of x is: {}", x);
// }

// fn plus_five(x: i32) -> i32 {
//     x + 5
// }

// fn plus_or_mins(x:i32)-> i32 {
//   if x >5 {
//     return x-5
//   }
//   x+5
// }

// fn main() {
//   let x = plus_or_mins(5);
//   println!("The value of x is: {}",x);
// }

// Rust 中的特殊返回类型

// 无返回值
// use std::fmt::Debug;

// fn report<T: Debug>(item: T) {
//   println!("{:?}", item);

// }

// fn add(x:u32,y:u32) -> u32 {
//   x + y;
// }

// 当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )
// fn dead_end() -> ! {
//   panic!("你已经到了穷途末路，崩溃吧！");
// }

// practice

// fn main() {
//     // 不要修改下面两行代码!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn main() {
//   print();
// }

// // 使用另一个类型来替代 i32
// fn print() -> () {
//   println!("hello,world");
// }

// 用两种方法求解
// fn main() {
//   never_return();
// }

// fn never_return() -> !{
//   // 实现这个函数，不要修改函数签名!
//   panic!("never to return")
// }

// 用两种方法求解
// fn main() {
//   never_return();
// }

// use std::thread;
// use std::time;

// fn never_return() -> ! {
//     // implement this function, don't modify fn signatures
//     loop {
//         println!("I return nothing");
//         // sleeping for 1 second to avoid exhausting the cpu resource
//         thread::sleep(time::Duration::from_secs(1))
//     }

// use std::panic::panic_any;

// //   }
// fn main() {
//   println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//   match tp {
//       1 => {
//           // TODO
//       }
//       _ => {
//           // TODO
//       }
//   };

//   // 这里与其返回一个 None，不如使用发散函数替代
//   never_return_fn()
// }

// // 使用三种方法实现以下发散函数
// fn never_return_fn() -> ! {
//     panic!()
// }

// fn main() {
//   // 填空
//   let b = false;

//   let _v = match b {
//       true => 1,
//       // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
//       false => {
//           println!("Success!");
//           panic!("we have no value for `false`, but we can panic")
//       }
//   };

//   println!("Exercise Failed if printing out this line!");
// }

// String
// fn main() {

//   let mut s = String::from("hello");

//   s.push_str(", world");  // push_str() 在字符串后追加字面值

//   // let s1 =s;

//   // println!("{}", s)

//   let s2 = "hello world";
//   let s3 = s2;
//   println!("{}",s3);
//   // 深拷贝
//   let s4 = s.clone();

// }

// fn main() {
//   let s = String::from("hello world");

//   takes_ownership(s);

//   // println!("{}",s); // s 的值移动到函数里 ...
//   // ... 所以到这里不再有效

//   let x = 5;

//   makes_copy(x);
//     // x 应该移动函数里，
//     // 但 i32 是 Copy 的，所以在后面可继续使用 x

// }// 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 所以不会有特殊操作

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//   println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//   println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

// fn main() {
//     let s1 = gives_owernship();
//     // println!("{}",s1);

//     let s2 = String::from("hello world");

//     let s3 = takes_and_gives_back(s2);
//     // println!("{}",s3);
//   }

// fn gives_owernship() -> String {
//   let owership = String::from("hello");
//   owership
// }

// fn takes_and_gives_back(s :String) -> String{
//   s
// }

// practice

// 1
// fn main() {
//   // 使用尽可能多的方法来通过编译
//   let x = String::from("hello, world");
//   let y = x.clone();
//   println!("{},{}",x,y);
// }

// fn main() {
//   // 使用尽可能多的方法来通过编译
//   let x = "hello, world";
//   let y = x;
//   println!("{},{}",x,y);
// }

// 2
// 不要修改 main 中的代码
// fn main() {
//   let s1 = String::from("hello, world");
//   let s2 = take_ownership(s1);

//   println!("{}", s2);
// }

// // 只能修改下面的代码!
// fn take_ownership(s: String) ->String{
//   // println!("{}", s);
//   s
// }

// 3

// fn main() {
//   let s = give_ownership();
//   println!("{}", s);
// }

// // // 只能修改下面的代码!
// fn give_ownership() -> String {
//   let s = String::from("hello, world");
//   // convert String to Vec
//   // 将 String 转换成 Vec 类型
//   // let s1 = s.clone();
//   // let _s = s.into_bytes();
//   s
// }

// 4
// 修复错误，不要删除任何代码行
// fn main() {
//   let s = String::from("hello, world");

//   let s1 = s.clone();
//   print_str(s);

//   println!("{}", s1);
// }

// fn print_str(s: String)  {
//   println!("{}",s)
// }

// 5

// 不要使用 clone，使用 copy 的方式替代
// fn main() {
//   let x = (1, 2, (), "hello");
//   // let y = x.clone();
//   // let y = x.3;
//   let y = x;
//   println!("{:?}, {:?}", x, y);
// }

// 6

// fn main() {
//   let s = String::from("hello, ");

//   // 只修改下面这行代码 !
//   let mut s1 = s;

//   s1.push_str("world")
// }

// 7

// fn main() {
//   let x = Box::new(5);

//   let let mut y = Box::new(3);     // 完成该行代码，不要修改其它行！

//   *y = 4;

//   assert_eq!(*x, 5);
// }

// 8

// fn main() {
//   let t = (String::from("hello"), String::from("world"));

//   let _s = t.0;

//   // 仅修改下面这行代码，且不要使用 `_s`
//   println!("{:?}", t.1);
// }

// 9

// fn main() {
//   let t = (String::from("hello"), String::from("world"));

//   // 填空，不要修改其它代码
//   // let (t, s1, s2) = (t.clone(), t.0, t.1);
//   let(s1, s2) = t.clone();

//   println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }

//引用与解引用
// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

// 不可变引用
// fn main() {
//   let x = String::from("hello world");

//   let y = calculate_length(&x);

//   println!("{}",y);
// }

// fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
//     s.len()
// }// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生

// fn main() {
//   let s1 = String::from("hello");
//   let s1 = String::from("hello");

//   change_str(&s1);
// }

// fn change_str(s: &String) {
//   s.push_str(".world");
// }

// fn main() {
//   let mut s1 = String::from("hello");
//   // let s1 = String::from("hello");

//   change_str(&mut s1);
// }

// fn change_str(s: &mut String) {
//   s.push_str(".world");
// }
// 可变引用同时只能存在一个
// fn main() {
//     let mut s = String::from("hello");

//     // let r1 = &mut s;
//     let r2 = &mut s;

//     {
//       let r1 = &mut s;

//   } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

//     // println!("{}, {}", r1, r2);
// }

// fn main() {
//   // 可变引用与不可变引用不能同时存在
//   let mut s = String::from("hello");

//   let r1 = &mut s;
//   let r2 = &s;
//   let r3 = &s;

//   println!("{}, {}, and {}", r1, r2, r3);
// }

// fn main() {
//   let mut s = String::from("hello");

//    let r1 = &s;
//    let r2 = &s;
//    println!("{} and {}", r1, r2);
//    // 新编译器中，r1,r2作用域在这里结束

//    let r3 = &mut s;
//    println!("{}", r3);
// } // 老编译器中，r1、r2、r3作用域在这里结束
//  // 新编译器中，r3作用域在这里结束

// 悬垂引用
// fn main() {
//   // let reference_to_nothing = dangle1();
//   let reference_to_nothing = dangle2();
// }

// // fn dangle1() -> &String {
// //   let s = String::from("hello");

// //   &s
// // } // 这里 s 离开作用域并被丢弃。其内存被释放。
// // // 危险！

// fn dangle2() -> String {
//   let s = String::from("hello");
//   s
// }

// practice
// 1

// fn main() {
//   let x = 5;
//   // 填写空白处
//   let p = &x;

//   println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
// }

// 2

// fn main() {
//   let x = 5;
//   let y = &x;

//   // 只能修改以下行
//   assert_eq!(5, *y);
// }

// 3

// 修复错误
// fn main() {
//   let mut s = String::from("hello, ");

//   borrow_object(&s)
// }

// fn borrow_object(s: &String) {}

// 4

// 修复错误
// fn main() {
//   let mut s = String::from("hello, ");

//   push_str(& mut s)
// }

// fn push_str(s: &mut String) {
//   s.push_str("world")
// }

// 5

// fn main() {
//   let mut s = String::from("hello, ");

//   // 填写空白处，让代码工作
//   let p = & mut s;

//   p.push_str("world");
// }

// 6

// fn main() {
//   let c = '中';

//   let r1 = &c;
//   // 填写空白处，但是不要修改其它行的代码
//   let ref r2 = c;

//   assert_eq!(*r1, *r2);

//   // 判断两个内存地址的字符串是否相等
//   assert_eq!(get_addr(r1),get_addr(r2));
// }

// // 获取传入引用的内存地址的字符串形式
// fn get_addr(r: &char) -> String {
//   format!("{:p}", r)
// }

// 7

// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
// fn main() {
//   let mut s = String::from("hello");

//   let r1 = &s;
//   let r2 = &s;

//   println!("{}, {}", r1, r2);
// }

// 8

// fn main() {
//   // 通过修改下面一行代码来修复错误
//   let mut s = String::from("hello, ");

//   borrow_object(&mut s)
// }

// fn borrow_object(s: &mut String) {}

// 9

// 从可变对象借用不可变

// 下面的代码没有任何错误
// fn main() {
//   let mut s = String::from("hello, ");

//   borrow_object(&s);

//   s.push_str("world");
// }

// fn borrow_object(s: &String) {}

// 10

// 注释掉一行代码让它工作
// fn main() {
//   let mut s = String::from("hello, ");

//   let r1 = &mut s;
//   r1.push_str("world");
//   let r2 = &mut s;
//   r2.push_str("!");

//   // println!("{}",r1);
// }

// 11

// fn main() {
//   let mut s = String::from("hello, ");

//   let r1 = &mut s;
//   let r2 = &mut s;

//   // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
//   // 你不能同时使用 r1 和 r2
//   println!("{},{}",r1,r2);
// }

// 复合类型
// #![allow(unused_variables)]
// type File = String;

// fn open(f: &mut File) -> bool {
//     true
// }
// fn close(f: &mut File) -> bool {
//     true
// }

// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

// fn main() {
//     let mut f1 = File::from("f1.txt");
//     open(&mut f1);
//     //read(&mut f1, &mut vec![]);
//     close(&mut f1);
// }

// fn main() {
//   let name = "pascal";
//   greey(name);
// }

// fn greey(name: String) {
//   println!("hello, {}", name);
// }

// 切片 按字节切
// fn main() {
//   let s = String::from("hello world");
//   let hello = &s[0..5];
//   let world = &s[6..11];

//   println!("{},{}",hello,world);

//   // 从索引0开始
//   let slice1 = &s[0..2];
//   let slice2 = &s[..2];

//   // 如果你的切片想要包含 String 的最后一个字节
//   let len = s.len();
//   let slice3 = &s[10..len];
//   // println!("{}",slice3)

//   // 截取完整的 String 切片
//   let slice4 = &s[0..len];
//   let slice5 = &s[..];

//   // 中文切片
//   let chinese = String::from("中国人");
//   let a = &chinese[0..3];
//   println!("{}",a)

// }

// fn main() {
//   let mut s = String::from("hello");
//   let word = first_word(&s);
//   // s.clear();
//   println!("{}",word);

// }

// fn first_word(s: &String) -> &str {
//     &s[..1]
// }

// // 其他切片
// fn main() {
//   let a = [1,2,3,4,5,6];
//   let slice = &a[1..3];
//   assert_eq!(slice,[2,3]);
// }

// String 与 &str 的转换
// String::from("hello,world")
// "hello,world".to_string()

// fn main() {
//   let s = String::from("hello,world");
//   say_hello(&s);
//   say_hello(&s[..10]);
//   say_hello(s.as_str());
// }
// fn say_hello(s: &str) {
//     println!("{}",s);
// }

// 字符串索引 Rust 不允许去索引字符串

// 操作字符串
// 追加
// fn main() {
//   let mut s = "hello".to_string();
//   s.push_str(" rust");

//   println!("追加字符串{}",s);

//   s.push('!');
//   println!("追加字符{}",s);
// }

// 插入
// fn main() {
//   let mut s = String::from("hello world");
//   // 插入字符
//   s.insert(5, ',');
//   println!("插入字符：{}", s);
//   // 插入字符串
//   s.insert_str(6, "i like");
//    println!("插入字符串：{}", s);
// }

// 替换
// fn main() {
//   // replace 返回一个新串
//   let string_replace = String::from("I like rust. Learning rust is my favorite!");
//   let new_string_replace = string_replace.replace("rust", "Rust");
//   dbg!(new_string_replace);

//   // replacen 返回一个新串
//   let new_string_replacen = string_replace.replacen("rust", "RUST", 2);
//   dbg!(new_string_replacen);

//   // replace_range 仅适用于 String 类型 直接操作原来的字符串
//   let mut string_replace_range = String::from("I like rust!");
//   string_replace_range.replace_range(7..8, "R");
//   dbg!(string_replace_range);

// }

// 删除

// pop —— 删除并返回字符串的最后一个字符 直接操纵原来字符串
// fn main() {
//   let mut s = String::from("rust pop 中文");
//   let p1 = s.pop();
//   let p2 = s.pop();

//   dbg!(p1);
//   dbg!(p2);
//   dbg!(s);
// }

// remove —— 删除并返回字符串中指定位置的字符 直接操纵原来字符串
// fn main() {
//     let mut s = String::from("测试remove方法");

//     println!(
//         "string_remove 占 {} 个字节",
//         std::mem::size_of_val(s.as_str())
//     );

//     // 删除第一个汉字
//     let p1 = s.remove(0);
//     // 按照字节来处理字符串
//     // let p2 = s.remove(1);
//     // let p3 = s.remove(2);
//     // let p3 = s.remove(3);

//     dbg!(p1);
//     dbg!(s);

// }

// truncate —— 删除字符串中从指定位置开始到结尾的全部字符 直接操纵原来字符串
// fn main() {
//   let mut s = String::from("测试truncate");
//   s.truncate(3);

//   dbg!(s);

// }

// clear —— 清空字符串 直接操纵原来字符串
// fn main() {
//   let mut s = String::from("string clear");
//   s.clear();

//   dbg!(s);
// }

// Concatenate
// fn main() {
//   let string_append = String::from("hello ");
//   let string_rust = String::from("rust");

//   let result = string_append + &string_rust;

//   let mut result = result+ "!";

//   result+="!!!";
//   println!("连接字符串 + -> {}", result);
// }

// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
//     let s3 = s1 + &s2;
//     assert_eq!(s3, "hello,world!");
//     // 下面的语句如果去掉注释，就会报错
//     // println!("{}",s1);

//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     // String = String + &str + &str + &str + &str
//     let s = s1 + "-" + &s2 + "-" + &s3;
// }

// fn main() {
//   let s1 = "hello";
//   let s2 = String::from("rust");

//   let s = format!("{},{}",s1,s2);

//   dbg!(s);
// }

// // 字符串转义
// fn main() {
//   // 通过 \ + 字符的十六进制表示，转义输出一个字符
//   let byte_escape = "I'm writing \x52\x75\x73\x74!";
//   println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//   // \u 可以输出一个 unicode 字符
//   let unicode_codepoint = "\u{211D}";
//   let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//   println!(
//       "Unicode character {} (U+211D) is called {}",
//       unicode_codepoint, character_name
//   );

//   // 换行了也会保持之前的字符串格式
//   // 使用\忽略换行符
//   let long_string = "String literals
//                       can span multiple lines.
//                       The linebreak and indentation here ->\
//                       <- can be escaped too!";
//   println!("{}", long_string);
// }

// fn main() {
//   println!("{}", "hello \\x52\\x75\\x73\\x74");
//   let raw_str = r"Escapes don't work here: \x3F \u{211D}";
//   println!("{}", raw_str);

//   // 如果字符串包含双引号，可以在开头和结尾加 #
//   let quotes = r#"And then I said: "There is no escape!""#;
//   println!("{}", quotes);

//   // 如果还是有歧义，可以继续增加，没有限制
//   let longer_delimiter = r###"A string with "# in it. And even "##!"###;
//   println!("{}", longer_delimiter);
// }

//操作字符串
// fn main() {
//   for c in "中国人" .chars() {
//     // println!("{}",c);
//   }

//   // 返回字节数组
//   for b in "中国人".bytes() {
//     println!("{}",b);
//   }
//  }

// practice

// 修复错误，不要新增代码行
// 1
// fn main() {
//   let s: &str = "hello, world";
// }

// 2

// 使用至少两种方法来修复错误
// fn main() {
//   let s: Box<str> = "hello, world".into();
//   greetings(&s);
// }

// fn greetings(s: &str) {
//   println!("{}",s)
// }

// fn main() {
//   let s: Box<&str> = "hello, world".into();
//   // let s: &str = "hello, world";
//   greetings(*s);
// }

// fn greetings(s: &str) {
//   println!("{}",s)
// }

// 3

// 填空
// fn main() {
//   let mut s = "".to_string();
//   s.push_str("hello, world");
//   s.push('!');

//   assert_eq!(s, "hello, world!");
// }

// 4

// 修复所有错误，并且不要新增代码行
// fn main() {
//   let mut s = String::from("hello");
//   s.push(',');
//   s.push_str(" world");
//   // s += &"!".to_string();
//   s += "!";

//   println!("{}", s)
// }

// 5

// 填空
// fn main() {
//   let s = String::from("I like dogs");
//   // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
//   let s1 = s.replace("dogs", "cats");

//   assert_eq!(s1, "I like cats")
// }

// 6

// 修复所有错误，不要删除任何一行代码
// fn main() {
//   let s1 = String::from("hello,");
//   let s2 = String::from("world!");
//   let s3 = s1.clone() + &s2;
//   assert_eq!(s3,"hello,world!");
//   println!("{}",s1);
// }

// 7

// 使用至少两种方法来修复错误
// fn main() {
//   let s = "hello, world".to_string();
//   greetings(s)
// }

// fn greetings(s: String) {
//   println!("{}",s)
// }

// fn main() {
//   let s = String::from("hello, world");
//   greetings(s)
// }

// fn greetings(s: String) {
//   println!("{}",s)
// }

// 8

// 使用两种方法来解决错误，不要新增代码行
// fn main() {
//   let s = "hello, world".to_string();
//   let s1: &str = &s;
// }

// fn main() {
//   let s = "hello, world";
//   let s1: &str = s;
// }

// fn main() {
//   let s = "hello, world".to_string();
//   let s1: &String = s;
// }

// 9
// fn main() {
//   // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
//   // 填空以输出 "I'm writing Rust"
//   let byte_escape = "I'm writing Ru\x73__!";
//   println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//   // 也可以使用 Unicode 形式的转义字符
//   let unicode_codepoint = "\u{211D}";
//   let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//   println!("Unicode character {} (U+211D) is called {}",
//               unicode_codepoint, character_name );

//   // 还能使用 \ 来连接多行字符串
//   let long_string = "String literals
//                       can span multiple lines.
//                       The linebreak and indentation here \
//                        can be escaped too!";
//   println!("{}", long_string);
// }

// 10
/* 填空并修复所有错误 */
// fn main() {
//   let raw_str = r"Escapes don't work here: \x3F \u{211D}";
//   // 修改上面的行让代码工作
//   assert_eq!(raw_str, r"Escapes don't work here: \x3F \u{211D}");

//   // 如果你希望在字符串中使用双引号，可以使用以下形式
//   let quotes = r#"And then I said: "There is no escape!""#;
//   println!("{}", quotes);

//   // 如果希望在字符串中使用 # 号，可以如下使用：
//   let  delimiter = r###"A string with "# in it. And even "##!"###;
//   println!("{}", delimiter);

//   // 填空
//   // let long_delimiter = r####"hello, ##""####;
//   let long_delimiter = "Hello, \"##\"".to_string();
//   assert_eq!(long_delimiter, "Hello, \"##\"")
// }

// 11

// fn main() {
//   let s1 = String::from("hi,中国");
//   let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
//   assert_eq!(h, "h");

//   let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
//   assert_eq!(h1, "中");
// }

// 12
//
// fn main() {
//   // 填空，打印出 "你好，世界" 中的每一个字符
//   for c in "你好，世界".chars() {
//       println!("{}", c)
//   }
// }

// 13
// use utf8_slice;
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";

//     let rocket = utf8_slice::slice(s, 4, 5);
//     // 结果是 "🚀"
//     dbg!(rocket);
// }

// Slice practice

// 1
// 修复代码中的错误，不要新增代码行!
// fn main() {
//   let arr = [1, 2, 3];
//   let s1: &[i32] = &arr[0..2];

//   let s2: &str = "hello, world" as &str;
// }

// 2

// fn main() {
//   let arr: [char; 3] = ['中', '国', '人'];

//   let slice = &arr[..2];

//   // 修改数字 `8` 让代码工作
//   // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
//   assert!(std::mem::size_of_val(&slice) == 16);
//   dbg!(slice);
// }

// 3

// fn main() {
//   let arr: [i32; 5] = [1, 2, 3, 4, 5];
//  // 填空让代码工作起来
//  let slice:&[i32] = &arr[1..4];
//  assert_eq!(slice, &[2, 3, 4]);
// }

// 4

// fn main() {
//   let s = String::from("hello");

//   let slice1 = &s[0..2];
//   // 填空，不要再使用 0..2
//   let slice2 = &s[..2];

//   assert_eq!(slice1, slice2);
// }

// 5

// fn main() {
//   let s = "你好，世界";
//   // 修改以下代码行，让代码工作起来
//   let slice = &s[0..3];

//   assert!(slice == "你");
// }

// 6

// 修复所有错误
// fn main() {
//   let mut s = String::from("hello world");

//   // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
//   // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
//   let ch = first_character(&s);

//   println!("the first character is: {}", ch);
//   s.clear(); // error!

// }
// fn first_character(s: &str) -> &str {
//   &s[..1]
// }
// 元组
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (x,y,z) = tup;
//     println!("the value of y is {}",y);

//     let five_hundred = tup.0;

//     let six_point_four = tup.1;

//     let one  =tup.2;

// }

// use std::usize;

// fn main() {
//   let s1 = String::from("hello");
//   // let tup: (String, usize) =  calculate_length(s1);
//   // println!("{},{}",tup.0,tup.1);

//   let (s2, len) = calculate_length(s1);

//   println!("The length of '{}' is {}.", s2, len);

// }

// fn calculate_length(s:String) -> (String, usize) {
//   let len = s.len();
//   (s, len)
// }

// tuple practice
// 1

// fn main() {
//   let _t0: (u8,i16) = (0, -1);
//   // 元组的成员还可以是一个元组
//   let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//   // 填空让代码工作
//   let t: (u8, u16, i64, &str,String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
// }

// 2

// 修改合适的地方，让代码工作
// fn main() {
//   let t = ("i", "am", "sunface");
//   assert_eq!(t.2, "sunface");
// }

// 3

// 修复代码错误
// fn main() {
//   let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
//   println!("too long tuple: {:?}", too_long_tuple);
// }

// 4

// fn main() {
//   let tup = (1, 6.4, "hello");

//   // 填空
//   let (x,z,y) = tup;

//   assert_eq!(x, 1);
//   assert_eq!(y, "hello");
//   assert_eq!(z, 6.4);
// }

// 5
// fn main() {
//   let (x, y, z);

//   // 填空
//   (y, z, x) = (1, 2, 3);

//   assert_eq!(x, 3);
//   assert_eq!(y, 1);
//   assert_eq!(z, 2);
// }

// 6
// fn main() {
//   // 填空，需要稍微计算下
//   let (x, y) = sum_multiply((2,3));

//   assert_eq!(x, 5);
//   assert_eq!(y, 6);
// }

// fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
//   (nums.0 + nums.1, nums.0 * nums.1)
// }

// // 结构体
// struct User {
//   activate: bool,
//   username: String,
//   email: String,
//   sign_in_count: u64,
// }
// fn main() {

// // 创建结构体实例
// // let user1 = User{
// //   email: String::from("someone@example.com"),
// //   username: String::from("someusername123"),
// //   activate: true,
// //   sign_in_count: 1,
// // };

// let mut user1 = User{
//   email: String::from("someone@example.com"),
//   username: String::from("someusername123"),
//   activate: true,
//   sign_in_count: 1,
// };

// user1.email=String::from("anotheremail@example.com");

// let user1 = bulid_user("email".to_string(), "username".to_string());
// }

// // 简化结构体创建
// fn bulid_user(email: String, username: String) -> User {
//     // User {
//     //     email: email,
//     //     username: username,
//     //     activate: true,
//     //     sign_in_count: 1,
//     // }

//     User {
//         email,
//         username,
//         activate: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     struct User {
//         activate: bool,
//         username: String,
//         email: String,
//         sign_in_count: u64,
//     }

//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         activate: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         activate: user1.activate,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };

//     // username 所有权被转移给了 user2，导致了 user1 无法再被使用，
//     // 但是并不代表 user1 内部的其它字段不能被继续使用
//     println!("{}",user1.activate);

//     println!("{:?}",user1);

//     // let user2 = User {
//     //   email: String::from("another@example.com"),
//     // //  结构体更新语法 ..必须在结构体尾部使用
//     //   ..user1
//     // };
// }

// use std::fs::File;

// 结构体内存排列
// #[derive(Debug)]
// struct File {
//   name: String,
//   data: Vec<u8>,
// }

// fn main() {
//   let f1 = File {
//     name: String::from("f1.txt"),
//     data:Vec::new(),
//   };

//   let f1_name = &f1.name;
//   let f1_length = &f1.data.len();

//   println!("{:?}", f1);
//   println!("{} is {} bytes long", f1_name, f1_length);

// }

// 元组结构体
// struct  Color (i32,i32,i32);
// struct Point (i32,i32,i32);

// let black = Color(0,0,0);
// let orange = Point(0,0,0);

// // 单元结构体
// struct AlwayEqual;
// fn main() {
//   let subject = AlwayEqual;

//   impl SomeTrait for AlwayEqual {

//   }
// }

// 结构体数据的所有权
// struct User {
//   username: &str,
//   email: &str,
//   sign_in_count: u64,
//   active: bool,
// }

// fn main() {
//   let user1 = User {
//       email: "someone@example.com",
//       username: "someusername123",
//       active: true,
//       sign_in_count: 1,
//   };
// }

// #[derive(Debug)]
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// prcatice

// 1
// fix the error
// struct Person {
//   name: String,
//   age: u8,
//   hobby: String
// }
// fn main() {
//   let age = 30;
//   let p = Person {
//       name: String::from("sunface"),
//       age:11,
//       hobby:String::from("lanqiu")
//   };
// }

// 2

// struct Unit;
// trait SomeTrait {
//     // ...定义一些行为
// }

// // 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// // 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
// impl SomeTrait for Unit {  }
// fn main() {
//     let u = Unit;
//     do_something_with_unit(u);
// }

// // 填空，让代码工作
// fn do_something_with_unit(u: Unit) {   }

// 3

// // // 填空并修复错误
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(0, 127, 255);
//     check_color(v);
// }

// fn check_color(p: Point) {
//     let Point(x, _, _) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(p.2, 255);
//  }

// 4

// 填空并修复错误，不要增加或移除代码行
// struct Person {
//   name: String,
//   age: u8,
// }
// fn main() {
//   let age = 18;
//   let mut p = Person {
//       name: String::from("sunface"),
//       age,
//   };

//   // how can you believe sunface is only 18?
//   p.age = 30;

//   // 填空
//   p.name = String::from("sunfei");
// }

// 5

// 填空
// struct Person {
//   name: String,
//   age: u8,
// }
// fn main() {}

// fn build_person(name: String, age: u8) -> Person {
//   Person {
//       age,
//       name,
//   }
// }

// 6

// 填空，让代码工作
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let u1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("sunface"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let u2 = set_email(u1);
// }

// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }

// 7

// 填空，让代码工作
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
//         height: 50,
//     };

//     dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr

//     println!("{:?}", rect1); // 打印 debug 信息到标准输出 stdout
// }

// 8

// fn main() {
//   #[derive(Debug)]
//   struct Person {
//       name: String,
//       age: Box<u8>,
//   }

//   let person = Person {
//       name: String::from("Alice"),
//       age: Box::new(20),
//   };

//   // 通过这种解构式模式匹配，person.name 的所有权被转移给新的变量 `name`
//   // 但是，这里 `age` 变量却是对 person.age 的引用, 这里 ref 的使用相当于: let age = &person.age
//   let Person { name, ref age } = person;

//   println!("The person's age is {}", age);

//   println!("The person's name is {}", name);

//   // Error! 原因是 person 的一部分已经被转移了所有权，因此我们无法再使用它
//   //println!("The person struct is {:?}", person);

//   // 虽然 `person` 作为一个整体无法再被使用，但是 `person.age` 依然可以使用
//   println!("The person's age from person struct is {}", person.age);
// }

// 9

// 修复错误
// #[derive(Debug)]
// struct File {
//     name: String,
//     data: String,
// }
// fn main() {
//     let f = File {
//         name: String::from("readme.md"),
//         data: "Rust By Practice".to_string()
//     };

//     let _name = f.name;

//     // 只能修改这一行
//     println!("{}", f.data);
//     // println!("{}, {}, {:?}",f.name, f.data, f);
// }

// 枚举
// #[derive(Debug)]
// enum PokerSuit {
//   Clubs,
//   Spades,
//   Diamonds,
//   Hearts,
// }

// struct PockerCard {
//   suit: PokerSuit,
//   value:u8
// }

// fn main() {
//   //枚举值
//   // let heart = PokerSuit::Hearts;
//   // let diamond = PokerSuit::Diamonds;

//   // print_suit(heart);
//   // print_suit(diamond);

//   let c1 = PockerCard {
//     suit: PokerSuit::Clubs,
//     value: 1,
//   };

//   let c2 = PockerCard {
//     suit: PokerSuit::Diamonds,
//     value:12,
//   };

// }

// fn print_suit(card: PokerSuit) {

//   println!("{:?}",card);
// }

// enum PockerCard {
//   Clubs(u8),
//   Spades(u8),
//   // Diamonds(u8),
//   // Hearts(u8),
//   Diamonds(char),
//   Hearts(char),
// }

// fn main() {
//   let c1 = PockerCard::Clubs(1);
//   let c2 = PockerCard::Diamonds('A');
// }

// enum Message {
//   Quit,
//   Move {x:i32, y: i32}, //匿名结构体
//   Writer(String),
//   ChangeColor(i32,i32,i32),
// }

// fn main() {
//   let m1 = Message::Quit;
//   let m2 = Message::Move { x: 1, y: 2 };
//   let m3 = Message::ChangeColor(255, 255, 0);
// }

// Option 枚举用于处理空值
// fn main() {
//     // let some_number = Some(5);
//     // let some_string = Some("a string");

//     // let absent_number: Option<i32> = None;

//     // let x: i8 = 5;
//     // let y: Option<i8> = Some(5);

//     // let sum = x + y;

//     fn plus_one(x: Option<i32>) -> Option<i32> {
//       match x {
//           None => None,
//           Some(i) => Some(i + 1),
//       }
//   }

//   let five = Some(5);
//   let six = plus_one(five);
//   let none = plus_one(None);

//   dbg!(six);
//   dbg!(none);

// }

// enum practice

// 1
// // 修复错误
// enum Number {
//   Zero,
//   One,
//   Two,
// }

// enum Number1 {
//   Zero = 0,
//   One,
//   Two,
// }

// // C语言风格的枚举定义
// enum Number2 {
//   Zero = 0,
//   One = 1,
//   Two = 2,
// }

// fn main() {
//   // 通过 `as` 可以将枚举值强转为整数类型

//   assert_eq!(Number::One as u8, Number1::One as u8);
//   assert_eq!(Number1::One as u8 , Number2::One as u8);
// }

// 2

// // 填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg1 = Message::Move { x: 1, y: 2 }; // 使用x = 1, y = 2 来初始化
//     let msg2 = Message::Write("hello, world!".to_string()); // 使用 "hello, world!" 来初始化
// }


// 3

// 仅填空并修复错误
// enum Message {
//   Quit,
//   Move { x: i32, y: i32 },
//   Write(String),
//   ChangeColor(i32, i32, i32),
// }

// fn main() {
//   let msg = Message::Move{x: 2, y: 2};

//   if let Message::Move{x: a, y: b} = msg {
//       assert_eq!(a, b);
//   } else {
//       panic!("不要让这行代码运行！");
//   }
// } 

// 4

// 填空，并修复错误
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs:[Message; 3]  = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// } 

// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }


// 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     if let Some(n) = six {
//         println!("{}", n);
//         return
//     } 
        
//     panic!("不要让这行代码运行！");
// } 

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// 数组
// fn main() {
//     // let a = [1,2,3,45,];
//     // let b: [i32;5] = [1,2,3,4,5];
//     // // 某个值重复出现 N 次的数组
//     // let c = [3;5];

//     let a = [9,8,7,6,5];
//     let first = a[0];
//     let second = a[1];


// }

// 数组越界访问
// use std::io;
// fn main() {
//   let a = [1,2,3,4,5];
//   println!("Please enter an array index");
//   let mut index = String::new();
//   // 读取控制台
//   io::stdin()
//     .read_line(&mut index)
//     .expect("Fail to read line");

//   let index: usize = index
//       .trim()
//       .parse()
//       .expect("Index entered was not a number");

//   let element =a[index];
//   println!(
//     "The value of the element at index {} is {}",
//     index, element
//   );
// }

// fn main() {
//   let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good"));
//   println!("{:#?}", array);
// }

// 数组切片
// fn main() {
//   let a: [i32; 5] = [1,2,3,4,5];
//   let slice: &[i32] = &a[1..3];
//   assert_eq!(slice, &[2,3]);
// }

// fn main() {
//   let one = [1,2,3];

//   let two: [u8; 3] = [1,2,3];
//   let blank1 = [0;3];
//   let blank2: [u8; 3] = [0;3];

//   let arrays: [[u8; 3]; 4] = [one, two, blank1,blank2];
//   for a in &arrays {
//     println!("{:?}", a);

//     for n in a.iter() {
//       print!("\t{} + 10 = {}", n, n+10);
//     }
//     let mut sum = 0;
//     for i in 0..a.len() {
//       sum+=a[i];
//     }
//     println!("\t({:?} = {})", a, sum);
//   }

// }

// practice

// 1
// fn main() {
//   // 使用合适的类型填空
//   let arr: [i32; 5] = [1, 2, 3, 4, 5];

//   // 修改以下代码，让它顺利运行
//   assert!(arr.len() == 5);
// }

// 2

// fn main() {
//   // 很多时候，我们可以忽略数组的部分类型，也可以忽略全部类型，让编译器帮助我们推导
//   let arr0 = [1, 2, 3];
//   let arr: [char; 3] = ['a', 'b', 'c'];
  
//   // 填空
//   // 数组分配在栈上， `std::mem::size_of_val` 函数会返回整个数组占用的内存空间
//   // 数组中的每个 char 元素占用 4 字节的内存空间，因为在 Rust 中， char 是 Unicode 字符
//   assert!(std::mem::size_of_val(&arr) == 12);
// }

//3
 
// fn main() {
//   // 填空
//   let list: [i32; 100] = [1;100] ;

//   assert!(list[0] == 1);
//   assert!(list.len() == 100);
// }

// 4

// fn main() {
//   // 修复错误
//   let _arr = [1, 2, 3];
// }

// 5

// fn main() {
//   let arr = ['a', 'b', 'c'];
  
//   let ele = arr[0]; // 只修改此行来让代码工作

//   assert!(ele == 'a');
// }

// 6

// 修复代码中的错误
// fn main() {
//   let names = [String::from("Sunfei"), "Sunface".to_string()];
  
//   // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
//   let name0 = names.get(0).unwrap();
//   println!("{}",name0);

//   // 但是下标索引就存在越界的风险了
//   let _name1 = &names[0];
// }

// //流程控制
// fn main() {
//   let condition = true;
//   let number = if condition {
//     5
//   } else {
//     6
//   };
//   println!("The value of number is: {}", number);
// }

// fn main() {
//   let n = 6;
//   if n%4==0 {
//     println!("number is divisible by 4");
//   } else if n%3==0 {
//       println!("number is divisible by 3");
//   } else if n%2==0 {
//       println!("number is divisible by 2");
//   } else {
//       println!("number is divisible by 4,3,or 2");
//   }
//  }

// fn main() {
//   // for i in 1..5 {
//   //   println!("{}",i);
//   // }
//   // let a = [4,3,2,1];
//   // // `.iter()` 方法把 `a` 数组变成一个迭代器
//   // for (i,v) in a.iter().enumerate() {
//   //   println!("第{}个元素是{}",i+1,v);
//   // }
//   for _ in 0..10 {

//   } 
// }

// fn main() {
//   let collection = [1,2,3,4,5];
//   // for i in 0..collection.len() {
//   //   let item = collection[i];
//   // }

//     for i in collection {
      
//     }
// }

// fn main() {
//   let mut n = 0;

//   while n<=5 {
//       println!("{}",n);
//       n=n+1;
//   }
//   println!("我出来了");
// }

// fn main() {
//   let mut counter = 0;
//   let result = loop {
//     counter +=1;

//     if counter==10 {
//       break counter*2;
//     }
//   };
//   println!("The result is {}", result);
// }

// practice 
//1

// 填空
// fn main() {
//   let n = 5;

//   if n < 0 {
//       println!("{} is negative", n);
//   } else if n > 0 {
//       println!("{} is positive", n);
//   } else {
//       println!("{} is zero", n);
//   }
// } 

// 2

// 修复错误
// fn main() {
//   let n = 5;

//   let big_n =
//       if n < 10 && n > -10 {
//           println!(" 数字太小，先增加 10 倍再说");

//           10 * n
//       } else {
//           println!("数字太大，我们得让它减半");

//           n / 2
//       };

//   println!("{} -> {}", n, big_n);
// } 

// 3

// fn main() {
//   for n in 1..100 { // 修改此行，让代码工作
//       if n == 100 {
//           panic!("NEVER LET THIS RUN")
//       }
//   }
// } 

// 4

// 修复错误，不要新增或删除代码行
// fn main() {
//   let names = [String::from("liming"),String::from("hanmeimei")];
//   for name in &names {
//       // do something with name...
//     }
//   println!("{:?}", names);


//   let numbers = [1, 2, 3];
//   // numbers中的元素实现了 Copy，因此无需转移所有权
//   for n in numbers {
//       // do something with name...
//     }
    
//     println!("{:?}", numbers);
// } 

// 5
// fn main() {
//   let a = [4,3,2,1];

//   // 通过索引和值的方式迭代数组 `a` 
//   for (i,v) in a.iter().enumerate() {
//       println!("第{}个元素是{}",i+1,v);
//   }
// }

// 6

// 填空，让最后一行的  println! 工作 !
// fn main() {
//   // 一个计数值
//   let mut n = 1;

//   // 当条件为真时，不停的循环
//   while n < 10 {
//       if n % 15 == 0 {
//           println!("fizzbuzz");
//       } else if n % 3 == 0 {
//           println!("fizz");
//       } else if n % 5 == 0 {
//           println!("buzz");
//       } else {
//           println!("{}", n);
//       }


//       n+=1;
//   }

//   println!("n 的值是 {}, 循环结束",n);
// }

// 7

// 填空，不要修改其它代码
// fn main() {
//   let mut n = 0;
//   for i in 0..=100 {
//      if n == 66 {
//          break;
//      }
//      n += 1;
//   }

//   assert_eq!(n, 66);
// }

// 8

// 填空，不要修改其它代码
// fn main() {
//   let mut n = 0;
//   for i in 0..=100 {
//      if n != 66 {
//          n+=1;
//          continue;     
//      }
     
    
//     break;
//   }
  
//   assert_eq!(n, 66);
// }

// 9

// 填空，不要修改其它代码
// fn main() {
//   let mut count = 0u32;

//   println!("Let's count until infinity!");

//   // 无限循环
//   loop {
//       count += 1;

//       if count == 3 {
//           println!("three");

//           // 跳过当此循环的剩余代码
//           continue;
//       }

//       println!("{}", count);

//       if count == 5 {
//           println!("OK, that's enough");

//           break;
//       }
//   }

//   assert_eq!(count, 5);
// }

// 10

// 填空
// fn main() {
//   let mut counter = 0;

//   let result = loop {
//       counter += 1;

//       if counter == 10 {
//           break counter*2;
//       }
//   };

//   assert_eq!(result, 20);
// }

// 11

// 填空
// fn main() {
//   let mut count = 0;
//   'outer: loop {
//       'inner1: loop {
//           if count >= 20 {
//               // 这只会跳出 inner1 循环
//               break 'inner1; // 这里使用 `break` 也是一样的
//           }
//           count += 2;
//       }

//       count += 5;

//       'inner2: loop {
//           if count >= 30 {
//               break 'outer;
//           }

//           continue 'outer;
//       }
//   }

//   assert!(count == 30)
// }

// 模式匹配
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//   let dire = Direction::South;
//   match dire {
//       Direction::East => println!("East!"),
//       Direction::North | Direction::South => {
//         println!("South or North");
//       },
//       _=> println!("West"),
//   };
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//   match coin {
//       Coin::Penny=> {
//         println!("Lucky penny!");
//         1
//       },
//       Coin::Nickel=>5,
//       Coin::Dime=>10,
//       Coin::Quarter=>25,
//   }
// }

// enum IpAddr {
//     Ipv4,
//     Ipv6,
// }
// fn main() {
//   let ip1 = IpAddr::Ipv6;
//   let ip_str = match ip1 {
//       IpAddr::Ipv4=>"127.0.0.1",
//       _=>"::1",
//   };
//   println!("{}",ip_str);
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny =>1,
//         Coin::Nickel =>5,
//         Coin::Dime =>10,
//         Coin::Quarter(state) => {
//           println!("State quarter from {:?}", state);
//           25
//         },
//     }
// }

// fn main() {
//   let coin = Coin::Quarter(UsState::Alaska);
//   let result = value_in_cents(coin);
//   println!("{}",result);
// }

// enum Action {
//     Say(String),
//     MoveTo(i32,i32),
//     ChangeColorRGB(u16,u16,u16),
// }

// fn main() {
//   let actions = [
//     Action::Say("hello rust".to_string()),
//     Action::MoveTo(1, 2),
//     Action::ChangeColorRGB(255, 255, 0),
//   ];
//   for action in actions {
//     match action {
//         Action::Say(s) => {
//           println!("{}",s);
//         },
//         Action::MoveTo(a, b) => {
//           println!("point from (0,0) move to ({},{})",a,b);
//         },
//         Action::ChangeColorRGB(r, g, _) => {
//           println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
//                 r,g);
//         }
//     }
//   }
// }

// 穷尽匹配
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//   let dire = Direction::South;
//   match dire {
//       Direction::East => println!("East"),
//       Direction::North | Direction::West => {
//         println!("North or West");
//       },
//   };
// }

// 通配符
// fn main() {
//   let some_u8_value = 0u8;
//   match some_u8_value {
//       1 => println!("one"),
//       3 => println!("three"),
//       5 => println!("five"),
//       7 => println!("seven"),
//       _ => (),
//   }
// }
// 用一个变量来承载其他情况
#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}
fn main() {
  let dire = Direction::South;
  match dire {
      Direction::East => println!("East"),
      other => println!("{:?}", other),
  };
}
