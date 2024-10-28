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
// #[derive(Debug)]
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
//       other => println!("{:?}", other),
//   };
// }
// if let 匹配
// fn main() {
//     let v = Some(3u8);
//     // match v {
//     //     Some(3) => println!("three"),
//     //     _ => (),
//     // }

//     if let Some(3) = v {
//         println!("three");
//     }
// }

// matches!宏
// enum MyEnum {
//     Foo,
//     Bar,
// }

// fn main() {
//     // let v = vec![MyEnum::Foo, MyEnum::Bar,MyEnum::Foo];
//     // v.iter().filter(|x| x == MyEnum::Foo);
//     // v.iter().filter(|x| matches!(x, MyEnum::Foo));

//     let foo = 'f';
//     assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

//     let bar = Some(4);
//     assert!(matches!(bar, Some(x) if x>2));
// }

// 变量遮蔽
// fn main() {
//     let age = Some(30);
//     println!("在匹配之前,age是{:?}",age);
//     // if let Some(age) = age {
//     //     println!("匹配出来的age是{}", age);
//     // }
//     // println!("在匹配之后,age是{:?}",age);

//     // match age {
//     //     Some(age) => println!("匹配出来的age是{}",age),
//     //     _ => (),
//     // }
//     // println!("在匹配之后,age是{:?}",age);

//     match age {
//         Some(x) => println!("匹配出来的age是{}",x),
//         _ => (),
//     }
//     println!("在匹配之后,age是{:?}",age);
// }

// practice
// 1

// 填空
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South  => { // 在这里匹配 South 或 North
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }

// 2

// fn main() {
//     let boolean = true;

//     // 使用 match 表达式填空，并满足以下条件
//     //
//     // boolean = true => binary = 1
//     // boolean = false => binary = 0
//     let binary = match boolean {
//         true => 1,
//         false => 0,
//     };

//     assert_eq!(binary, 1);
// }

// 3

// 填空
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }
// }

// fn show_message(msg: Message) {
//     match msg {
//         Message::Move { x: a, y: b } => { // 这里匹配 Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(_, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         __ => println!("no data in these variants")
//     }
// }

// 4

// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // 使用 `matches` 填空
//     for ab in alphabets {
//         assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
//     }
// }

// 5

// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         // if  e == MyEnum::Foo { // 修复错误，只能修改本行代码
//         //     count += 1;
//         // }
//         if matches!(e, MyEnum::Foo) {
//             count+=1;
//         }
//     }

//     assert_eq!(count, 2);
// }

// 6

// fn main() {
//     let o = Some(7);

//     // 移除整个 `match` 语句块，使用 `if let` 替代
//     if let Some(i)= o {
//         println!("This is a really long string and `{:?}`", i);
//     }
// }

// 7

// 填空
// enum Foo {
//     Bar(u8)
// }

// fn main() {
//     let a = Foo::Bar(1);

//     if let Foo::Bar(i) = a {
//         println!("foobar 持有的值是: {}", i);
//     }
// }

// 8

// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }

// fn main() {
//     let a = Foo::Qux(10);

//     // 移除以下代码，使用 `match` 代替
//     // if let Foo::Bar = a {
//     //     println!("match foo::bar")
//     // } else if let Foo::Baz = a {
//     //     println!("match foo::baz")
//     // } else {
//     //     println!("match others")
//     // }

//     match a {
//         Foo::Bar => {
//             println!("match foo::bar")
//         },
//         Foo::Baz => {
//             println!("match foo::baz")
//         },
//         _ => {
//             println!("match others")
//         },
//     }
// }

// 9

// // // 就地修复错误
// fn main() {
//     let age = Some(30);
//     if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
//        assert_eq!(age, 30);
//     } // 新的 `age` 变量在这里超出作用域

//     match age {
//         // `match` 也能实现变量遮蔽
//         Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
//         _ => ()
//     }
//  }

// 解构 Option
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(x) => Some(x + 1),
//         None => None,
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

// 模式适用场景
// while let 条件循环

// fn main() {
//     let mut stack = Vec::new();
//     // 向尾部插入元素
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     // 向尾部弹出元素
//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }

// }

// use std::vec;

// fn main() {
//     let v = vec!['a', 'b','c'];
//     for (index, value) in v.iter().enumerate() {
//         println!("{} is at index {}", value, index);
//     }
// }

// 全模式列表

// 匹配字面值
// fn main() {
//     let x = 1;

//     match x {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }

// 匹配命名变量
// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Mached, y= {:?}",y),
//         _ => println!("Default case, x= {:?}",x),
//     }
//     println!("at the end: x= {:?}, y = {:?}", x,y);
// }

// 单分支多模式
// fn main() {
//     let x = 1;

//     match x {
//         1 | 2 => println!("one or two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }

// 通过序列 ..= 匹配值的范围
// fn main() {
//     // let x = 5;
//     // match x {
//     //     1..=5 => println!("one through five"),
//     //     _ => println!("someThing else"),
//     // }

//     let x = 'c';
//     match x {
//         'a'..='j' => println!("early ASCII letter"),
//         'k'..='z' => println!("late ASCII letter"),
//         _ => println!("someThing else"),
//     }
// }

// 解构并分解值
// // 解构结构体
// struct Point {
//     x:i32,
//     y:i32,
// }
// fn main() {
//     let p = Point{x:0, y:7};
//     // let Point{x:a,y:b} = p;
//     // assert_eq!(0,a);
//     // assert_eq!(7,b);

//     let Point{x,y} = p;
//     assert_eq!(0,x);
//     assert_eq!(7,y);
//  }

// fn main() {
//     let p = Point{x:0,y:7};

//     match p {
//         Point { x, y :0} =>println!("On the x axis ar {}", x),
//         Point { x: 0, y} =>println!("On the y axis ar {}", y),
//         Point { x, y } =>println!("On neither axis ar ({}, {})", x, y),
//     }
// }

// 解构枚举
// enum Message {
//     Quit,
//     Move {x:i32, y:i32},
//     Write(String),
//     ChangeColor(i32,i32,i32),
// }
// fn main() {
//  let msg = Message::ChangeColor(0, 160, 255);

//  match msg {
//      Message::Quit => {
//         println!("The Quit variant has no data to destructure.");
//      },
//      Message::Move { x, y } => {
//         println!(
//             "Move in the x direction {} and in the y direction {}",
//             x,
//             y
//         );
//      }
//      Message::Write(text) => println!("Text message: {}",text),
//      Message::ChangeColor(r, g, b) => {
//         println!(
//             "Change the color to red {}, green {}, and blue {}",
//             r,
//             g,
//             b
//         )
//      }
//  }
// }
// 解构嵌套的结构体和枚举
// enum Color {
//     Rgb(i32,i32,i32),
//     Hsv(i32,i32,i32),
// }
// enum Message {
//     Quit,
//     Move {x:i32, y:i32},
//     Write(String),
//     ChangeColor(Color),
// }
// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv((0), (160), (255)));

//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!(
//                 "Change the color to red {}, green {}, and blue {}",
//                 r,
//                 g,
//                 b
//             )
//         },
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!(
//                 "Change the color to hue {}, saturation {}, and value {}",
//                 h,
//                 s,
//                 v
//             )
//         },
//         _ => {}
//     }
// }
// 解构结构体和元组
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let ((feet, inches), Point {x,y}) = ((3,10), Point{x:3,y:-10});

// }

// 解构数组
// fn main() {
//     // 定长数组
//     // let arr: [u16;2] = [114,514];
//     // let [x,y] = arr;

//     // assert_eq!(x,114);
//     // assert_eq!(y,514);

//     // 不定长数组
//     let arr: &[u16] = &[114,514];
//     if let [x, ..] = arr {
//         assert_eq!(x, &114);
//     }
//     if let [.., y] = arr {
//         assert_eq!(y, &514);
//     }
//     let arr: &[u16] = &[];

//     assert!(matches!(arr, [..]));
//     assert!(!matches!(arr, [x, ..]));
// }

// 忽略模式中的值
// fn foo(_: i32, y: i32) {
//     println!("This code only uses the y parameter: {}", y);
// }
// fn main() {
//     foo(3, 4);
// }
// 使用嵌套的 _ 忽略部分值
// fn main() {
//     let mut setting_value = Some(5);
//     let mut new_setting_value = Some(10);

//     match (setting_value, new_setting_value) {
//         (Some(_), Some(_)) => {
//             println!("Can't overwrite an existing customized value");
//         },
//         _ => {
//             setting_value = new_setting_value;
//         }
//     }
//     println!("setting is {:?}", setting_value);
// }

// fn main() {
//     let number = (2,4,8,16,32);

//     match number {
//         (first, _, third, _, fifth) => {
//             println!("Some numbers: {}, {}, {}", first, third, fifth)
//         },
//     }
// }

// 使用下划线开头忽略未使用的变量
// fn main() {
//   // let _x = 5;
//   // let y = 10;

//   let s = Some(String::from("Hello!"));
//   // if let Some(_s) = s {
//   if let Some(_) = s {
//     println!("found a string");
//   }
//   println!("{:?}", s);

// }

// 用 .. 忽略剩余值
// struct Point {
//     x: i32,
//     y: i32,
//     z: i32,
// }

// fn main() {
//     // let origin = Point{x:0, y:0,z:0};
//     // match origin {
//     //     Point {x, ..} => {
//     //       println!("x is {}",x);
//     //     }
//     // }

//     let numbers = (2, 4, 8, 16, 32);
//     // match numbers {
//     //     (first, .., last) => {
//     //         println!("Some numbers {}, {}", first, last);
//     //     },
//     // }

//     match numbers {
//       (.., second, ..) => {
//           println!("Some numbers: {}", second)
//       },
//   }
// }

// 匹配守卫提供的额外条件
// fn main() {
//   let num = Some(4);

//   match num {
//       Some(x) if x<5 => println!("less than five {}",x),
//       Some(x) => println!("{}",x),
//       None => (),
//   }
// }

// fn main() {
//   let x = Some(5);
//   let y = 10;

//   match x {
//       Some(50) => println!("Got 50"),
//       Some(n) if n ==y => println!("Mached, n= {}", n),
//       _ => println!("Default case, x = {:?}",x),
//   }
//   println!("at the end: x={:?}, y = {}", x, y);
// }

// 匹配守卫的条件会作用于所有的模式
// fn main() {
//     let x = 4;
//     let y = false;

//     match x {
//         4 | 5 | 6 if y => println!("yes"),
//         _ => println!("no"),
//     }
// }

// @绑定
// enum Message {
//     Hello {id:i32},
// }
// fn main() {
//   let msg = Message::Hello { id: 5 };
//   match msg {
//       Message::Hello { id: id_variable @ 3..=7 } => {
//         println!("Found an id in range: {}", id_variable);
//       },
//       Message::Hello { id: 10..=12 } => {
//         println!("Founf an id in range");
//       },
//       Message::Hello { id } => {
//         println!("Found some other id: {}",id)
//       },
//   }
// }

// @前绑定后解构(Rust 1.56 新增)
// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     // 绑定新变量 `p`，同时对 `Point` 进行解构
//     let p @ Point { x: px, y: py } = Point { x: 10, y: 22 };
//     println!("x:{}, y:{}", px, py);
//     println!("{:?}", p);

//     let point = Point { x: 10, y: 5 };

//     if let p @ Point { x: 10, y } = point {
//         println!("x is 10 and y is {} in {:?}", y, p);
//     } else {
//         println!("x was not 10");
//     }
// }

// @新特性(Rust 1.53 新增)
// fn main() {
//   match 1 {
//       num @ (1 | 2) => {
//         println!("{}", num);
//       },
//       _ => {}
//   }
// }

// practice
// 1

// fn main() {
//     match_number(13);
// }
// fn match_number(n: i32) {
//     match n {
//         // 匹配一个单独的值
//         1 => println!("One!"),
//         // 使用 `|` 填空，不要使用 `..` 或 `..=`
//         2 | 3 | 4 | 5 => println!("match 2 -> 5"),
//         // 匹配一个闭区间的数值序列
//         6..=10 => {
//             println!("match 6 -> 10")
//         }
//         11..=i32::MAX => {
//             println!("match 11 -> +infinite")
//         }
//         _ => {}
//     }
// }

// 2

// struct Point {
//   x: i32,
//   y: i32,
// }

// fn main() {
//   // 填空，让 p 匹配第二个分支
//   let p = Point { x: 0, y: 10 };

//   match p {
//       Point { x, y: 0 } => println!("On the x axis at {}", x),
//       // 第二个分支
//       Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//       Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//   }
// }

// 3

// 修复错误
// enum Message {
//   Hello { id: i32 },
// }

// fn main() {
//   let msg = Message::Hello { id: 5 };

//   match msg {
//       Message::Hello {
//           id: id @ 3..=7,
//       } => println!("id 值的范围在 [3, 7] 之间: {}", id),
//       Message::Hello { id: newid@(10 | 11 | 12) } => {
//           println!("id 值的范围在 [10, 12] 之间: {}", newid)
//       }
//       Message::Hello { id } => println!("Found some other id: {}", id),
//   }
// }

// 4

// 填空让代码工作，必须使用 `split`
// fn main() {
//   let num = Some(4);
//   let split = 5;
//   match num {
//       Some(x) if x < split => assert!(x < split),
//       Some(x) => assert!(x >= split),
//       None => (),
//   }
// }

// 5

// 填空，让代码工作
// fn main() {
//   let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

//   match numbers {
//       (first, .. , last) => {
//          assert_eq!(first, 2);
//          assert_eq!(last, 2048);
//       }
//   }
// }

// 6

// 修复错误，尽量少地修改代码
// 不要移除任何代码行
// fn main() {
//   let mut v = String::from("hello,");
//   let r = &mut v;

//   match r {
//     // &mut value => value.push_str(" world!")
//     value => value.push_str(" world!")
//   }
// }

// 方法
// 方法定义
// struct Circle {
//   x: f64,
//   y: f64,
//   radius: f64,
// }

// impl Circle {
//     // new是Circle的关联函数，因为它的第一个参数不是self，且new并不是关键字
//     // 这种方法往往用于初始化当前结构体的实例

//     fn new(x: f64, y: f64, radius: f64) -> Circle{
//         Circle {
//           x: x,
//           y: y,
//           radius: radius,
//         }
//     }
//      // Circle的方法，&self表示借用当前的Circle结构体
//     fn area(&self) -> f64 {
//       std::f64::consts::PI * (self.radius * self.radius)
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//   width: u32,
//   height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32{
//       self.width * self.height
//     }
// }

// fn main() {
//   let rect1 = Rectangle {
//     width: 30,
//     height: 50,
//   };

//   println!(
//     "The area of the rectangle is {} square pixels.",
//     rect1.area())
// }

// 方法名跟结构体字段名相同
// impl Rectangle {
//     fn width(&self) -> bool {
//       self.width>0
//     }
// }

// fn main() {
//   let rect1 = Rectangle {
//     width: 30,
//     height: 50,
//   };

//   if rect1.width() {
//     println!("The rectangle has a nonzero width; it is {}", rect1.width);
//   }
//   // 使用 rect1.width() 时，我们调用的是它的方法，如果使用 rect1.width，则是访问它的字段。

// }
// 实现 getter 访问器
// pub struct Rectangle {
//   width: u32,
//   height: u32,
// }

// impl Rectangle {
//   pub fn new(width: u32, height: u32) -> Self{
//     Rectangle { width, height}
//   }
//   pub fn width(&self) ->u32 {
//     return self.width;
//   }
// }
// fn main() {
//   let rect1 = Rectangle::new(30, 50);
//   println!("{}", rect1.width())
// }

// 带有多个参数的方法
// impl Rectangle {
//     fn area(&self) -> u32 {
//       self.width*self.height
//     }
//     fn can_hold(&self, other: &Rectangle) -> bool {
//       self.width > other.width && self.height > other.height
//     }
// }
// fn main() {
//   let rect1 = Rectangle{width:30, height:50};
//   let rect2 = Rectangle{width:10, height:40};
//   let rect3 = Rectangle{width:60, height:45};

//   println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//   println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }

// 关联函数
// impl Rectangle {
//     fn new(w: u32, h: u32) -> Rectangle {
//       Rectangle{width:w, height:h}
//     }
// }

// fn main() {
//   let sq = Rectangle::new(3, 2);
// }

// 多个 impl 定义
// impl Rectangle {
//     fn area(&self) -> u32 {
//       self.width*self.height
//     }
// }
// impl Rectangle {
//   fn can_hold(&self, other: &Rectangle) -> bool {
//       self.width > other.width && self.height > other.height
//   }
// }

// 为枚举实现方法
// #![allow(unused)]
// enum Message {
//   Quit,
//   Move {x: i32, y:i32},
//   Write(String),
//   ChangeColor(i32,i32,i32),
// }

// impl Message {
//   fn call(&self) {
//       // 在这里定义方法体
//   }
// }

// fn main() {
//   let m = Message::Write(String::from("hello"));
//   m.call();
// }
// practice
// struct Point {
//   x: f64,
//   y: f64,
// }

// // `Point` 的关联函数都放在下面的 `impl` 语句块中
// impl Point {
//   // 关联函数的使用方法跟构造器非常类似
//   fn origin() -> Point {
//       Point { x: 0.0, y: 0.0 }
//   }

//   // 另外一个关联函数，有两个参数
//   fn new(x: f64, y: f64) -> Point {
//       Point { x: x, y: y }
//   }
// }

// struct Rectangle {
//   p1: Point,
//   p2: Point,
// }

// impl Rectangle {
//   // 这是一个方法
//   // `&self` 是 `self: &Self` 的语法糖
//   // `Self` 是当前调用对象的类型，对于本例来说 `Self` = `Rectangle`
//   fn area(&self) -> f64 {
//       // 使用点操作符可以访问 `self` 中的结构体字段
//       let Point { x: x1, y: y1 } = self.p1;
//       let Point { x: x2, y: y2 } = self.p2;

//       // `abs` 是一个 `f64` 类型的方法，会返回调用者的绝对值
//       ((x1 - x2) * (y1 - y2)).abs()
//   }

//   fn perimeter(&self) -> f64 {
//       let Point { x: x1, y: y1 } = self.p1;
//       let Point { x: x2, y: y2 } = self.p2;

//       2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
//   }

//   // 该方法要求调用者是可变的，`&mut self` 是 `self: &mut Self` 的语法糖
//   fn translate(&mut self, x: f64, y: f64) {
//       self.p1.x += x;
//       self.p2.x += x;

//       self.p1.y += y;
//       self.p2.y += y;
//   }
// }

// // `Pair` 持有两个分配在堆上的整数
// struct Pair(Box<i32>, Box<i32>);

// impl Pair {
//   // 该方法会拿走调用者的所有权
//   // `self` 是 `self: Self` 的语法糖
//   fn destroy(self) {
//       let Pair(first, second) = self;

//       println!("Destroying Pair({}, {})", first, second);

//       // `first` 和 `second` 在这里超出作用域并被释放
//   }
// }

// fn main() {
//   let rectangle = Rectangle {
//       // 关联函数的调用不是通过点操作符，而是使用 `::`
//       p1: Point::origin(),
//       p2: Point::new(3.0, 4.0),
//   };

//   // 方法才是通过点操作符调用
//   // 注意，这里的方法需要的是 `&self` 但是我们并没有使用 `(&rectangle).perimeter()` 来调用，原因在于：
//   // 编译器会帮我们自动取引用
//   //  `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
//   println!("Rectangle perimeter: {}", rectangle.perimeter());
//   println!("Rectangle area: {}", rectangle.area());

//   let mut square = Rectangle {
//       p1: Point::origin(),
//       p2: Point::new(1.0, 1.0),
//   };

//   // 错误！`rectangle` 是不可变的，但是这个方法要求一个可变的对象
//   //rectangle.translate(1.0, 0.0);
//   // TODO ^ 试着反注释此行，看看会发生什么

//   // 可以！可变对象可以调用可变的方法
//   square.translate(1.0, 1.0);

//   let pair = Pair(Box::new(1), Box::new(2));

//   pair.destroy();

//   // Error! 上一个 `destroy` 调用拿走了 `pair` 的所有权
//   pair.destroy();
//   // TODO ^ 试着反注释此行
// }

// 1
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // 完成 area 方法，返回矩形 Rectangle 的面积
//     fn area(&self) -> u32 {
//         self.height * self.width
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     assert_eq!(rect1.area(), 1500);
// }

// 2
// 只填空，不要删除任何代码行!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(&self)  {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light = TrafficLight{
//         color: "red".to_owned(),
//     };
//     // 不要拿走 `light` 的所有权
//     light.show_state();
//     // 否则下面代码会报错
//     println!("{:?}", light);
// }

// 3
// struct TrafficLight {
//   color: String,
// }

// impl TrafficLight {
//   // 使用 `Self` 填空
//   pub fn show_state(&self)  {
//       println!("the current state is {}", self.color);
//   }

//   // 填空，不要使用 `Self` 或其变体
//   pub fn change_state(&mut self) {
//       self.color = "green".to_string()
//   }
// }
// fn main() {}

// 4
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // 1. 实现下面的关联函数 `new`,
//     // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
//     // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
//     pub fn new() -> Self {
//       Self { color: "red".to_string() }

//     }

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }

// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");
// }

// 5

// struct Rectangle {
//   width: u32,
// }

// // 使用多个 `impl` 语句块重写下面的代码
// impl Rectangle {
//   fn area(&self) -> u32 {
//       self.width * self.height
//   }

//   fn can_hold(&self, other: &Rectangle) -> bool {
//       self.width > other.width && self.height > other.height
//   }
// }

// fn main() {}

// 6

// #[derive(Debug)]
// #[allow(unused)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }

// // 为 TrafficLightColor 实现所需的方法
// impl TrafficLightColor {
//     fn color(&self) -> String {
//       // TrafficLightColor::Yellow
//         match *self {
//             TrafficLightColor::Red => "red".to_string(),
//             TrafficLightColor::Yellow => "yellow".to_string(),
//             TrafficLightColor::Green => "Green".to_string(),
//         }
//     }
// }

// fn main() {
//     let c = TrafficLightColor::Yellow;

//     assert_eq!(c.color(), "yellow");

//     println!("{:?}",c);
// }

// 泛型
// fn add<T>(x: T, y: T) -> T {
//     x + y
// }
// fn main() {
//     println!("add i8: {}", add(2i8, 4i8));
//     println!("add i32: {}", add(20, 30));
//     println!("add i64: {}", add(1.23, 1.24));
// }
// fn largest<T> (list: &[T]) -> T{
//   let mut largest = list[0];
//   for &item in list.iter() {
//     if item > largest {
//       largest = item;
//     }
//   }
//   largest
// }
// fn main() {
//   let number_list = vec![34, 50, 25, 100, 65];

//   let result = largest(&number_list);
//   println!("The largest number is {}", result);

//   let char_list = vec!['y', 'm', 'a', 'q'];

//   let result = largest(&char_list);
//   println!("The largest char is {}", result);
// }

// // 结构体泛型
// struct Point<T>{
//   x: T,
//   y: T,
// }
// fn main() {
//   let interger = Point{x:2,y:4};
//   let float = Point{x:2.2,y:3.2};

// }
// struct Point<T, U>{
//   x: T,
//   y: U,
// }
// fn main() {
//   let i = Point{x:2,y:4.2};
//   // let float = Point{x:2.2,y:3.2};

// }

// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 方法中使用泛型
// struct Point<T> {
//   x: T,
//   y: T,
// }
// impl<T> Point<T> {
//   fn x(&self) -> &T {
//     &self.x
//   }

// }

// fn main() {
//   let p = Point{x: 22,y:43};
//   println!("p.x = {}", p.x());
// }

// struct Point<T,U> {
//   x: T,
//   y: U,
// }
// impl<T, U> Point<T, U> {
//   fn mixup<V, W>(self, other: Point<V,W>) -> Point<T,W> {
//     Point {
//       x: self.x,
//       y: other.y,
//     }
//   }
// }

// fn main() {
//   let p1 = Point{x: 3, y: 2.3};
//   let p2 = Point{x: "hello", y: 'c'};

//   let p3 = p1.mixup(p2);
//   println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

// }

// 为具体的泛型类型实现方法
// struct Point<T> {
//   x: T,
//   y: T,
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//       (self.x.powi(2)+ self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//   let p:Point<f32> = Point{x:3.0,y:4.0};
//   println!("{}", p.distance_from_origin())
//   }

// fn display_array(arr: &[i32]) {
//   println!("{:?}",arr);
// }

// fn main() {
//   let arr: [i32; 3] = [1, 2, 3];
//     display_array(&arr);

//     let arr: [i32;2] = [1,2];
//     display_array(&arr);
// }

// 将 i32 改成所有类型的数组
// fn display_array<T: std::fmt::Debug>(arr: &[T]) {
//   println!("{:?}",arr);
// }

// fn main() {
//   let arr: [i32; 3] = [1, 2, 3];
//     display_array(&arr);

//     let arr: [i32;2] = [1,2];
//     display_array(&arr);
// }

// fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//   println!("{:?}",arr);
// }

// fn main() {
//   let arr: [i32; 3] = [1, 2, 3];
//     display_array(arr);

//     let arr: [i32;2] = [1,2];
//     display_array(arr);
// }

// const 泛型表达式
// 目前只能在nightly版本下使用
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn something<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
//     //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
// {
//     //
// }

// fn main() {
//     something([0u8; 0]); // ok
//     something([0u8; 512]); // ok
//     something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制
// }

// // ---

// pub enum Assert<const CHECK: bool> {
//     //
// }

// pub trait IsTrue {
//     //
// }

// impl IsTrue for Assert<true> {
//     //
// }

// practice

// 1

// 填空
// struct A;          // 具体的类型 `A`.
// struct S(A);       // 具体的类型 `S`.
// struct SGen<T>(T); // 泛型 `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // 使用非泛型函数
//     reg_fn(S(A));          // 具体的类型
//     gen_spec_t(SGen((A)));   // 隐式地指定类型参数  `A`.
//     gen_spec_i32(SGen(2)); // 隐式地指定类型参数`i32`.

//     // 显式地指定类型参数 `char`
//     generic::<char>(SGen('s'));

//     // 隐式地指定类型参数 `char`.
//     generic(SGen('a'));
// }

// 实现下面的泛型函数 sum
// fn sum<T:std::ops::Add<Output = T>>(x: T, y:T) -> T{
//     x+y
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
// }

// 3

// 实现一个结构体 Point 让代码工作
// struct Point<T, V> {
//     x:T,
//     y:V,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// 4

// 修改以下结构体让代码工作
// struct Point<T, V> {
//     x: T,
//     y: V,
// }

// fn main() {
//     // 不要修改这行代码！
//     let p = Point{x: 5, y : "hello".to_string()};
// }

// 5

// 为 Val 增加泛型参数，不要修改 `main` 中的代码
// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }

// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }

// 6
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // 实现 mixup，不要修改其它代码！
//     fn mixup<V,W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};

//     // println!("{}",p1.x);
//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// }

// 7

// 修复错误，让代码工作
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p = Point{x: 5.0, y: 10.0};
//     println!("{}",p.distance_from_origin())
// }

// const
// pub struct MinSlice<T, const N: usize> {
//     pub head: [T; N],
//     pub tail: [T],
// }

// fn main() {
//     let slice: &[u8] = b"Hello, world";
//     let reference: Option<&u8> = slice.get(6);
//     // 我们知道 `.get` 返回的是 `Some(b' ')`
//     // 但编译器不知道
//     assert!(reference.is_some());

//     let slice: &[u8] = b"Hello, world";

//     // 当编译构建 MinSlice 时会进行长度检查，也就是在编译期我们就知道它的长度是 12
//     // 在运行期，一旦 `unwrap` 成功，在 `MinSlice` 的作用域内，就再无需任何检查
//     let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
//     let value: u8 = minslice.head[6];
//     assert_eq!(value, b' ')
// }

// 1
// 修复错误
// struct Array<T, const N: usize> {
//     data : [T; N]
// }

// fn main() {
//     let arrays = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 0, 0],
//         },
//         Array {
//             data: [1, 2, 2],
//         }
//     ];
// }

// 2

// 填空
// fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);

//     let arr = ["hello", "world"];
//     print_array(arr);
// }

// 3
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }

// // 修复 main 函数中的错误
// fn main() {
//     check_size([0u8; 767]);
//     check_size([0i32; 191]);
//     check_size(["hello你好"; 2]); // size of &str ?
//     check_size([(); __].map(|_| "hello你好".to_string()));  // size of String?
//     check_size(['中'; __]); // size of char ?
// }

// pub enum Assert<const CHECK: bool> {}

// pub trait IsTrue {}

// impl IsTrue for Assert<true> {}

// 定义特征
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// use std::str::pattern::SearchStep;

// 为类型实现特征
// pub struct Post {
//     pub title: String,   //标题
//     pub content: String, //内容
//     pub author: String,  //作者
// }
// pub struct Wibo {
//     pub username: String,
//     pub content: String,
// }
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章是:{},作者是{}",self.title, self.author)
//     }
// }

// impl Summary for Wibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post{title: "Rust语言".to_string(), content: "trait特征".to_string(), author: "me".to_string()};
//     let wibo = Wibo{username: "sunface".to_string(), content: "weibo不好用".to_string()};

//     println!("{}", post.summarize());
//     println!("{}", wibo.summarize());
// }
// // 默认实现
// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("read more..")
//     }
// }
// impl Summary for Post {}

// impl Summary for Wibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// use std::fmt::{Debug, Display};

// pub trait Summary {
//     fn summarize_author(&self) -> String;

//     fn summarize(&self) -> String {
//         format!("read more from..{}", self.summarize_author())
//     }
// }

// impl Summary for Wibo {
//     fn summarize_author(&self) -> String {
//         format!("@{}", self.username)
//     }
// }

// fn main() {
//     let wibo = Wibo{username: "sunface".to_string(), content: "weibo不好用".to_string()};
//     println!("i new wibo: {}", wibo.summarize());
// }
// 使用特征作为函数参数
// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// 特征约束(trait bound)
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// 多重约束
// pub fn notify(item: &(impl Summary + Display)) {}

// pub fn notify<T: Summary + Display>(item: &T) {}

// Where 约束
// fn some_function<T: Display+Clone, U: Clone + Debug>(t: &T, u: &U) ->i32{}

// fn some_function<T, U> (t: &T, u: &U) -> i32
//     where T: Display+Clone,
//           U: Clone+ Debug
//     { }
// fn main() {

// }

// 使用特征约束有条件地实现方法或特征

// use std::fmt::Display;

// struct Pair<T> {
//     x: T,
//     y: T,
// }
// impl <T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x>=self.y {
//             println!("The largest member is x = {}", self.x);

//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// 函数返回中的 impl Trait
// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// pub struct Post {
//     pub title: String, // 标题
//     pub author: String, // 作者
//     pub content: String, // 内容
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章{}, 作者是{}", self.title, self.author)
//     }
// }

// pub struct Weibo {
//     pub username: String,
//     pub content: String
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// // fn return_summarizable() -> impl Summary {
// //     Weibo {
// //         username: String::from("sunface"),
// //         content: String::from("
// //             电脑好卡
// //         ")
// //     }
// // }

// fn return_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Post {
//             title: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Weibo {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//         }
//     }
// }

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn main() {
//     let number_list = vec![100,22,34,55,43];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['w', 'e','c', 'g'];
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     &largest
// }

// fn main() {
//     let number_list = vec![100,22,34,55,43];
//     let result = largest(&number_list);
//     println!("The largest number is {}", &result);

//     let char_list = vec!['w', 'e','c', 'g'];
//     let result = largest(&char_list);
//     println!("The largest char is {}", &result);
// }

// 调用方法需要引入特征
// fn main() {
//     let a: i32 = 10;
//     let b: u16 = 100;
//     let b_ = b.try_into()
//             .unwrap();
//     if a < b_ {
//         println!("Ten is less than one hundred.");
//     }
// }

// use std::process::Output;

// // 示例
// // 为自定义类型实现 + 操作
// use std::ops::Add;
// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> {
//     x: T,
//     y: T,
// }
// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;

//     fn add(self, p: Point<T>) -> Point<T> {
//         Point {
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }
// fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// fn main() {
//     let p1 = Point {
//         x: 1.1f32,
//         y: 1.1f32,
//     };
//     let p2 = Point {
//         x: 2.1f32,
//         y: 2.1f32,
//     };
//     println!("{:?}", add(p1, p2));

//     let p3 = Point { x: 1i32, y: 1i32 };
//     let p4 = Point { x: 2i32, y: 2i32 };
//     println!("{:?}", add(p3, p4));
// }

// 自定义类型的打印输出

// #![allow(dead_code)]

// use core::fmt;
// use std::fmt::Display;

// #[derive(Debug,PartialEq)]
// enum FileState {
//     Open,
//     Closed,
// }

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<u8>,
//     state: FileState,
// }

// impl Display for FileState {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FileState::Open => write!(f, "OPEN"),
//             FileState::Closed => write!(f, "CLOSED"),
//         }
//     }
// }

// impl Display for File {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "<{} ({})>", self.name, self.state)
//     }
// }

// impl File {
//     fn new(name: &str) -> File{
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//             state: FileState::Closed,
//         }
//     }
// }

// fn main() {
//     let f6 = File::new("f6.txt");

//     println!("{:?}", f6);
//     println!("{}", f6);
// }

// practice

// struct Sheep {
//     naked: bool,
//     name: String,
// }

// impl Sheep {
//     fn is_naked(&self) -> bool{
//         self.naked
//     }

//     fn shear(&mut self) {
//         if self.is_naked() {
//             // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
//             println!("{} is already naked...", self.name);
//         } else {
//             println!("{} get a haircut!", self.name);

//             self.naked = true;
//         }
//     }
// }

// trait Animal {
//     // 关联函数签名；`Self` 指代实现者的类型
//     // 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，这里的 `Self` 指代的就是 `Pig` 类型
//     fn new(name: String) -> Self;

//     // 方法签名
//     fn name(&self) -> String;

//     fn noise(&self) -> String;

//     // 方法还能提供默认的定义实现
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }
// impl Animal for Sheep  {
//     // `Self` 被替换成具体的实现者类型： `Sheep`
//     fn new(name: String) -> Sheep {
//         Sheep { naked: false, name: name }
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn noise(&self) -> String {
//         if self.is_naked() {
//             "baaah?".to_string()
//         } else {
//             "baah!".to_string()
//         }
//     }

//     fn talk(&self) {
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }

// fn main() {
//     // 这里的类型注释时必须的
//     let mut dolly: Sheep = Animal::new("Dolly".to_string());

//     dolly.talk();
//     dolly.shear();
//     dolly.talk();
// }

// 1

// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }

//     fn say_something(&self) -> String;
// }

// struct Student {}
// impl Hello for Student {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }
// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("Hi, I'm your new teacher")
//     }
//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }

// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");

//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");

//     println!("Success!")
// }

// 2

// `Centimeters`, 一个元组结构体，可以被比较大小
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);

// `Inches`, 一个元组结构体可以被打印
// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }

// // 添加一些属性让代码工作
// // 不要修改其它代码！
// #[derive(Debug,PartialEq, PartialOrd)]
// struct Seconds(i32);

// fn main() {
//     let _one_second = Seconds(1);

//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = _one_second == _one_second;
//     let _this_is_false = _one_second > _one_second;

//     let foot = Inches(12);

//     println!("One foot equals {:?}", foot);

//     let meter = Centimeters(100.0);

//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };

//     println!("One foot is {} than one meter.", cmp);
// }

// 3

// use std::ops::Mul;

// // 实现 fn multiply 方法
// // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
// fn multiply<T: Mul<T, Output = T>>(a: T, b: T) -> T {
//     a * b
// }

// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));

//     println!("Success!")
// }

// 4

// 修复错误，不要修改 `main` 中的代码!
// use std::{ops, path};

// #[derive(Debug, PartialEq)]
// struct FooBar;

// struct Foo;
// struct Bar;

// #[derive(Debug, PartialEq)]
// struct BarFoo;

// // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }

// impl ops::Sub<Bar> for Foo {
//     type Output = BarFoo;

//     fn sub(self, _rhs: Bar) -> BarFoo {
//         BarFoo
//     }
// }

// fn main() {
//     // 不要修改下面代码
//     // 你需要为 FooBar 派生一些特征来让代码工作
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);

//     println!("Success!")
// }

// 5

// 实现 `fn summary`
// 修复错误且不要移除任何代码行
// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };

//     summary(&post);
//     summary(&weibo);

//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }

// // 在下面实现 `fn summary` 函数
// fn summary(item: &impl Summary) {
//     println!("Breaking news is {}", item.summarize());
// }

// 6

// use std::str::pattern::SearchStep;

// struct Sheep {}
// struct Cow {}

// trait Animal {
//     fn noise(&self) -> String;
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }

// 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
// fn random_animal(random_number: f64) -> impl Animal {
//     if random_number < 0.5 {
//         Sheep {}
//     } else {
//         Sheep{}
//     }
// }

// fn main() {
//     let random_number = 0.288;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }

// fn random_animal(random_number: f64) -> Box<dyn Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow {})
//     }
// }

// fn main() {
//     let random_number = 0.288;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }

// use std::process::Output;

// 7
// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }

// // 通过两种方法使用特征约束来实现 `fn sum`
// // use std::ops::Add;
// // fn sum<T: Add<Output=T>>(x: T, y: T) -> T {
// //     x + y
// // }
// use std::ops::Add;
// fn sum<T>(x: T, y: T) -> T
//     where
//         T: Add<Output=T>
//     {
//         x + y
//     }

// 8
// 修复代码中的错误
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }

// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }

// #[derive(Debug, PartialEq,PartialOrd)]
// struct Unit(i32);

// fn main() {
//     let pair = Pair{
//         x: Unit(1),
//         y: Unit(3)
//     };

//     pair.cmp_display();
// }

// 9

// // 填空
// fn example1() {
//     // `T: Trait` 是最常使用的方式
//     // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
//     struct Cacher<T: Fn(u32) -> u32> {
//         calculation: T,
//         value: Option<u32>,
//     }

//     impl<T: Fn(u32) -> u32> Cacher<T> {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }

//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }

//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(10), 11);
//     assert_eq!(cacher.value(15), 11);
// }

// fn example2() {
//     // 还可以使用 `where` 来约束 T
//     struct Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         calculation: T,
//         value: Option<u32>,
//     }

//     impl<T> Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }

//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }

//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(20), 21);
//     assert_eq!(cacher.value(25), 21);
// }

// fn main() {
//     example1();
//     example2();

//     println!("Success!")
// }

// 特征对象

// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8:{}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64:{}", *self)
//     }
// }
// fn draw1(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw2(x: &dyn Draw) {
//     x.draw();
// }

// fn main() {
//     let x = 1.1f64;
//     // do_something(&x);
//     let y = 8u8;

//     // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
//     // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
//     draw1(Box::new(x));
//     // 基于 y 的值创建一个 Box<u8> 类型的智能指针
//     draw1(Box::new(y));
//     draw2(&x);
//     draw2(&y);
// }

// #[derive(Debug)]
// enum UiObject {
//     Button,
//     SelectBox,
// }
// fn main() {
//     let objects = [
//         UiObject::Button,
//         UiObject::SelectBox,
//     ];

//     for o in objects {
//         draw(o);
//     }
// }

// fn draw(object: UiObject) {
//     println!("{:?}", object);
// }

// pub trait Draw {
//     fn draw(&self);
// }

// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button {
//     fn draw(&self) {

//     }
// }

// pub struct SelectBox {
//     pub width: u32,
//     pub height: u32,
//     pub label: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {

//     }
// }

// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw()
//         }
//     }
// }

// fn main() {
//     let screen = Screen{
//         components: vec![
//             Box::new(Button {
//                 width: 50,
//                 height: 10,
//                 label: "OK".to_string(),
//             }),
//             Box::new(SelectBox{
//                 width:75,
//                 height: 10,
//                 label: vec![
//                     String::from("yes"),
//                     String::from("maybe"),
//                     String::from("no"),
//                 ]
//             })
//         ]
//     };
//     screen.run()
// }

// Self 与 self
// trait Draw {
//     fn draw(&self) -> Self;
// }

// #[derive(Clone)]
// struct Button;

// impl Draw for Button {
//     fn draw(&self) -> Self {
//         return self.clone();
//     }
// }
// // self指代的就是当前的实例对象，也就是 button.draw() 中的 button 实例，Self 则指代的是 Button 类型。
// fn main() {
//     let button = Button;
//     let newb = button.draw();
// }

// 特征对象的限制

// 方法的返回类型不能是 Self
// 方法没有任何泛型参数

// pub struct Screen {
//     pub components: Vec<Box<dyn Clone>>,
// }

// practice

// 1

// use std::char::ParseCharError;

// trait Bird {
//     fn quack(&self) -> String;
// }

// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String{
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String{
//         "swan swan".to_string()
//     }
// }

// fn main() {
//     // 填空
//     let duck = Duck;
//     duck.swim();

//     let bird = hatch_a_bird(2);
//     // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
//     // bird.swim();
//     // 但它依然可以叫唤
//     assert_eq!(bird.quack(), "duck duck");

//     let bird = hatch_a_bird(1);
//     // 这只鸟儿忘了如何飞翔，因此以下代码会报错
//     // bird.fly();
//     // 但它也可以叫唤
//     assert_eq!(bird.quack(), "swan swan");

//     println!("Success!")
// }

// // 实现以下函数
// fn hatch_a_bird(item: u8) -> Box<dyn Bird>{
//     match item {
//         1=> Box::new(Swan),
//         2=> Box::new(Duck),
//         _ => panic!("Invalid input"),
//     }
// }

// 2
// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }

// fn main() {
//     // 填空
//     let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

//     for bird in birds {
//         bird.quack();
//         // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
//         // 因此，以下代码会报错
//         // bird.fly();
//     }
// }

// 3

// 填空
// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }

// fn main() {
//     let x = 1.1f64;
//     let y = 8u8;

//     // draw x
//     draw_with_box(Box::new(x));

//     // draw y
//     draw_with_ref(&y);

//     println!("Success!")
// }

// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw_with_ref(x: & dyn Draw) {
//     x.draw();
// }

// 4

// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String { format!("u8: {}", *self) }
// }

// impl Foo for String {
//     fn method(&self) -> String { format!("string: {}", *self) }
// }

// // 通过泛型实现以下函数
// fn static_dispatch<T: Foo>(x: T) {
//     x.method();
// }

// // 通过特征对象实现以下函数
// fn dynamic_dispatch(y: & dyn Foo) {
//     y.method();
// }

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success!")
// }

// 5

// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Self { 42 }
// }

// impl MyTrait for String {
//     fn f(&self) -> Self { self.clone() }
// }

// fn my_function(x: impl MyTrait) -> impl MyTrait {
//     x.f()
// }

// fn main() {
//     // my_function(Box::new(13_u32));
//     // my_function(Box::new(String::from("abc")));

//     my_function(13_u32);
//     my_function(String::from("abc"));

//     println!("Success!")
// }

// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
// trait MyTrait {
//     fn f(&self) -> Box<dyn MyTrait>;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Box<dyn MyTrait> {Box::new(42)}
// }

// impl MyTrait for String {
//     fn f(&self) -> Box<dyn MyTrait> {Box::new(self.clone())}
// }

// fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait>{
//     x.f()
// }

// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));

//     println!("Success!")
// }

// 深入了解特征
// pub trait Interator {
//     type Item;

//     fn next(&self) -> Option<Self::Item>;
// }

// // struct Counter{}
// impl Interator for Counter {
//     type Item = u32;
//     fn next(&self) -> Option<Self::Item> {

//     }
// }

// fn main() {
//     let c = Counter{..}
//     c.next();
// }

// 默认泛型类型参数
// trait Add<RHS=Self> {
//     type Output;
//     fn add(self, rhs:RHS) -> Self::Output;

// }

// example
// use std::ops::Add;
// #[derive(PartialEq, PartialOrd, Debug)]
// struct Point{
//     x:i32,
//     y:i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x:self.x + other.x,
//             y: self.y+other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//         Point { x: 3, y: 3 });
// }

// struct Millimeters(u32);
// struct Meters(u32);

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;
//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters(self.0 + (other.0 * 1000))
//     }
// }

// 调用同名的方法
// trait Pilot {
//     fn fly(&self);
// }
// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking.");
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }

// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }

// fn main() {
//     let person = Human;
//     // 默认调用该类型中定义的方法
//     person.fly();

//     Pilot::fly(&person); //调用Pilot特征上的方法
//     Wizard::fly(&person);

// }

// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;
// impl Dog {
//     fn baby_name() -> String{
//         String::from("spake")
//     }
// }
// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from("puppy")
//     }
// }
// // fn main() {
// //     println!("A baby dog is called a {}", Dog::baby_name());
// // }
// // 完全限定语法
// fn main() {
//     println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
//     // <Type as Trait>::function(receiver_if_method, next_arg, ...);

// }
// 特征定义中的特征约束

// use std::fmt::Display;
// trait OutLinePrint: Display {
//     fn OutLinePrint(&self) {
//         let output = self.to_string();
//         let len = output.len();

//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

// use std::fmt;

// struct Point {
//     x:i32,
//     y:i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// use std::fmt;

// // 在外部类型上实现外部特征(newtype)
// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }
// fn main() {
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }

// practice

// 1

// struct Container(i32, i32);

// // 使用关联类型实现重新实现以下特征
// // trait Contains {
// //    type A;
// //    type B;

// trait Contains<A, B> {
//     fn contains(&self, _: &A, _: &B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// impl Contains<i32, i32> for Container {
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }
//     // Grab the first number.
//     fn first(&self) -> i32 { self.0 }

//     // Grab the last number.
//     fn last(&self) -> i32 { self.1 }
// }

// fn c<A, B, C: Contains<A, B>>(container: &C) -> i32 {
//     container.last() - container.first()
// }

// fn main() {
//     let number_1 = 3;
//     let number_2 = 10;

//     let container = Container(number_1, number_2);

//     println!("Does container contain {} and {}: {}",
//         &number_1, &number_2,
//         container.contains(&number_1, &number_2));
//     println!("First number: {}", container.first());
//     println!("Last number: {}", container.last());

//     println!("The difference is: {}", difference(&container));
// }

// use std::process::Output;

// struct Container(i32, i32);

// trait Contains {
//     type A;
//     type B;

//     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
//     // fn first(&self) -> Self::A;
//     // fn last(&self) -> Self::B;

//     fn first(&self) -> i32;
//     fn last(&self) -> i32;

// }

// impl Contains for Container {
//     type A = i32;
//     type B = i32;

//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         (&self.0== number_1) && (&self.1 == number_2)
//     }
//     fn first(&self) -> i32 {
//         self.0
//     }
//     fn last(&self) -> i32 {
//         self.1
//     }
// }

// // fn difference<C: Contains>(container: &C) -> C::A
// //     where
// //         C::A: std::ops::Sub<Output=C::A> + Copy, {
// //             container.last() - container.first()
// //         }

// fn difference<C: Contains>(container: &C) -> i32 {
//     container.last() - container.first()
// }

// fn main() {
//     let number_1 = 3;
//     let number_2 = 10;

//     let container = Container(number_1, number_2);

//     println!("Does container contain {} and {}: {}",
//         &number_1, &number_2,
//         container.contains(&number_1, &number_2));
//     println!("First number: {}", container.first());
//     println!("Last number: {}", container.last());

//     println!("The difference is: {}", difference(&container));
// }

// 2

// use std::{ops::Sub, process::Output};

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// impl<T: Sub<Output=T>> Sub for Point<T> {
// // impl<T: Sub<Output=T>> Sub<Point<T>> for Point<T> {
// // impl<T: Sub<Output=T>> Sub<Self> for Point<T> {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 });

//     println!("Success!")
// }

// 3
// trait UsernameWidget {
//     fn get(&self) -> String;
// }

// trait AgeWidget {
//     fn get(&self) -> u8;
// }

// struct Form {
//     username: String,
//     age: u8,
// }

// impl UsernameWidget for Form {
//     fn get(&self) -> String {
//         self.username.clone()
//     }
// }

// impl AgeWidget for Form {
//     fn get(&self) -> u8 {
//         self.age
//     }
// }

// fn main() {
//     let form = Form{
//         username: "rustacean".to_owned(),
//         age: 28,
//     };

//     // 如果你反注释下面一行代码，将看到一个错误: Fully Qualified Syntax
//     // 毕竟，这里有好几个同名的 `get` 方法
//     //
//     // println!("{}", form.get());

//     let username = UsernameWidget::get(&form);
//     assert_eq!("rustacean".to_owned(), username);
//     let age = AgeWidget::get(&form); // 你还可以使用以下语法 `<Form as AgeWidget>::get`
//     assert_eq!(28, age);

//     println!("Success!")
// }

// 3
// trait Pilot {
//     fn fly(&self) -> String;
// }

// trait Wizard {
//     fn fly(&self) -> String;
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) -> String {
//         String::from("This is your captain speaking.")
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) -> String {
//         String::from("Up!")
//     }
// }

// impl Human {
//     fn fly(&self) -> String {
//         String::from("*waving arms furiously*")
//     }
// }

// fn main() {
//     let person = Human;

//     assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
//     assert_eq!(Wizard::fly(&person), "Up!");
//     assert_eq!(person.fly(), "*waving arms furiously*");

//     println!("Success!")
// }

// 4

// trait Person {
//     fn name(&self) -> String;
// }

// // Person 是 Student 的 supertrait .
// // 实现 Student 需要同时实现 Person.
// trait Student: Person {
//     fn university(&self) -> String;
// }

// trait Programmer {
//     fn fav_language(&self) -> String;
// }

// // CompSciStudent (computer science student) 是 Programmer
// // 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
// trait CompSciStudent: Programmer + Student {
//     fn git_username(&self) -> String;
// }

// fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
//     format!(
//         "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
//         student.name(),
//         student.university(),
//         student.fav_language(),
//         student.git_username()
//     )
// }

// struct CSStudent {
//     name: String,
//     university: String,
//     fav_language: String,
//     git_username: String
// }

// // 为 CSStudent 实现所需的特征
// impl Person for CSStudent {
//     fn name(&self) -> String {
//         self.name.clone()
//     }
// }

// impl Student for CSStudent {
//     fn university(&self) -> String {
//         self.university.clone()
//     }
// }

// impl Programmer for CSStudent {
//     fn fav_language(&self) -> String {
//         self.fav_language.clone()
//     }
// }

// impl CompSciStudent for CSStudent {
//     fn git_username(&self) -> String {
//         self.git_username.clone()
//     }
// }

// fn main() {
//     let student = CSStudent {
//         name: "Sunfei".to_string(),
//         university: "XXX".to_string(),
//         fav_language: "Rust".to_string(),
//         git_username: "sunface".to_string()
//     };

//     // 填空
//     println!("{}", comp_sci_student_greeting(&student));
// }

// 5
// use std::fmt;

// // 定义一个 newtype `Pretty`
// struct Pretty(String);

// impl fmt::Display for Pretty {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "\"{}\"", self.0.clone() + ", world")
//     }
// }

// fn main() {
//     let w = Pretty("hello".to_string());-//     println!("w = {}", w);
// }

// 动态数组
// fn main() {
//     // let mut v1 = Vec::new();
//     // v1.push(1);

//     // // Vec::with_capacity(capacity)

//     // // 宏 vec! 来创建数组，与 Vec::new 有所不同，前者能在创建同时给予初始化值
//     // let v2 = vec![1,2,3];

//     let v = vec![1,2,3,4,5];
//     let third = &v[2];
//     println!("第三个元素是: {}", third);

//     match v.get(2) {
//         Some(third) => println!("第三个元素是 {third}"),
//         None => println!("去你的第三个元素，根本没有！"),
//     }
// }

// &v[2] 表示借用 v 中的第三个元素，最终会获得该元素的引用。
// v.get(2) 也是访问第三个元素，但是有所不同的是，它返回了 Option<&T>，因此还需要额外的 match 来匹配解构出具体的值

// fn main() {

// let v = vec![1, 2, 3, 4, 5];

// let does_not_exist = &v[100];
// let does_not_exist = v.get(100);

// let mut v = vec![1, 2, 3, 4, 5];

// let x = &v[0];

// v.push(6);

// println!("The first element is: {x}");

// for i in &v {
//     println!("{i}");
// }

// for i in &mut v {
//     *i+=10;
// }

//    for i in &v {
//     println!("{i}");
// }

// 存储不同类型的元素

// }

// 枚举实现
// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let v = vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];  

//     for ip in &v {
//         println!("{:?}", ip);
//     }
// }


// // 特征对象实现
// trait IpAddr {
//     fn dispaly(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn dispaly(&self) {
//         println!("{:?}", self.0);
//     }
// }

// struct V6(String);
// impl IpAddr for V6 {
//     fn dispaly(&self) {
//         println!("{:?}", self.0);
//     }
// }


// fn main() {
//     let v: Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::.1".to_string())),
//     ];

//     for ip in v{
//         ip.dispaly();
//     }
// }


// Vector 常用方法
// 初始化
// fn main() { 
//     let v = vec![0;3];  //// 默认值为 0，初始长度为 3
//     let v_from = Vec::from([0,0,0]);

//     assert_eq!(v, v_from);
// }

// fn main() {
//     let mut v = Vec::with_capacity(10);
//     v.extend([1,2,3]);

//     println!("Vectory 的长度是{}, 容量是{}", v.len(), v.capacity());

//     v.reserve(100);   //调整v的容量 至少要有100
//     println!("Vectory(reserve)的长度是： {}, 容量是{}", v.len(), v.capacity());

//     v.shrink_to_fit();  //释放剩余的容量，一般情况下，不会主动去释放容量
//     println!("Vectory(shrink_to_fit)的长度是： {}, 容量是{}", v.len(), v.capacity());
// }

// 常见方法示例
// fn main() {
//     let mut v = vec![1,2];
//     assert!(!v.is_empty());    //检查是否为空

//     v.insert(2, 3);  //在指定索引插入数据，索引值不能大于 v 的长度
    
//     assert_eq!(v.remove(1), 2);    //移除指定位置的元素并返回
//     assert_eq!(v.pop(), Some(3));   //删除并返回 v 尾部的元素   
//     assert_eq!(v.pop(), Some(1));   //v:[]
//     assert_eq!(v.pop(), None);      // pop 方法返回的是 Option 枚举值
//     v.clear();   //清空v

//     let mut v1 = [11,22].to_vec();  // append 操作会导致 v1 清空数据，增加可变声明
//     v.append(&mut v1);    // 将 v1 中的所有元素附加到 v 中, v1: []
//     v.truncate(1);   // 截断到指定长度，多余的元素被删除, v: [11]
//     v.retain(|x| *x >10);   // 保留满足条件的元素，即删除不满足条件的元素

//     let mut v3 = vec![11,22,33,44,55];
//     let mut m: Vec<_> = v3.drain(1..=3).collect();  
//     // 删除指定范围的元素，同时获取被删除元素的迭代器, v3: [11, 55], m: [22, 33, 44]
//     println!("{:?}", m);

//     let v4 = m.split_off(1);   // 指定索引处切分成两个 vec,
//     println!("{:?}", v4);


// }

// fn main() {
//     let v = vec![11,22,33,44,55];
//     let silce = &v[1..=3];
//     assert_eq!(silce, &[22,33,44]);
// }

// Vector的排序
//整数数组的排序
// fn main() {
//     let mut v = vec![1, 5, 10, 2, 15];
//     v.sort_unstable();
//     assert_eq!(v, vec![1, 2, 5, 10, 15]);
// }


// 浮点数数组的排序
// fn main() {
//     let mut v = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
//     v.sort_unstable();
//     assert_eq!(v, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
// }

// fn main() {
//     let mut v = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
//     v.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());
//     assert_eq!(v, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
// }

// 对结构体数组进行排序
// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32,
// }

// impl Person {
//     fn new(name:String, age:u32) -> Person {
//         Person {
//             name,
//             age,
//         }
//     }
// }

// fn main() {
//     let mut people = vec![
//         Person::new("Zone".to_string(), 22),
//         Person::new("Alpha".to_string(), 54),
//         Person::new("John".to_string(), 63),
//     ];

//     people.sort_unstable_by(|a, b| b.age.cmp(&a.age));
 
//     println!("{:?}", people);
// }



// practice
// 1

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
    
//     let v = Vec::from(arr);
//     is_vec(&v);

//     let v = vec![1, 2, 3];
//     is_vec(&v);

//     // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
//     let v = vec!(1, 2, 3);
//     is_vec(&v);
    
//     // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
//     // 使用 Vec::new 和 `for` 来重写下面这段代码
//     // let v1 = vec!(arr);
//     // is_vec(v1);

//     let mut v1 = Vec::new();
//     for i in &v {
//         v1.push(*i);
//     }
//     is_vec(&v1);
//     assert_eq!(format!("{:?}",v), format!("{:?}",v1));
 
//     // assert_eq!(v, v1);

//     println!("Success!")
// }

// fn is_vec(v: &Vec<u8>) {}


// 2

// 填空
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
    
//     let mut v2 = Vec::new();
//     v2.extend([1,2,3]);

//     assert_eq!(v1, v2);

//     println!("Success!")
// }

// 3

// 填空
// fn main() {
//     // array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();
 
//     assert_eq!(v1, v2);
 
    
//     // String -> Vec
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = Vec::from(s);

//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);

//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);

//     // 迭代器 Iterators 可以通过 collect 变成 Vec
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success!")
//  }

// 4

// 修复错误并实现缺失的代码
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..=2 {
//         println!("{:?}", v[i])
//     }
//     v.clear();

//     for i in 0..5 {
//        // 实现这里的代码...
//        v.push(i+2);
//     }
    
//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!")
// }

// 5

// 修复错误
// fn main() {
//     let mut v = vec![1, 2, 3];

//     let slice1 = &v[..];
//     // 越界访问将导致 panic.
//     // 修改时必须使用 `v.len`
//     let slice2 = &v[0..v.len()];
    
//     assert_eq!(slice1, slice2);
    
//     // 切片是只读的
//     // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &mut v[0..4];
//     slice3[3] = 4;

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success!")
// }

// 6
// 修复错误
// fn main() {
//     let mut vec = Vec::with_capacity(10);

//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);

//     // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(),10);
//     assert_eq!(vec.capacity(), 10);

//     // ...但是下面的代码会造成新的内存分配
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);


//     // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(),100);
//     assert_eq!(vec.capacity(),100);
    
//     println!("Success!")
// }


// 7
// #[derive(Debug, PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     // 填空
//     let v : Vec<IpAddr>= vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];
    
//     // 枚举的比较需要派生 PartialEq 特征
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));

//     println!("Success!")
// }


// 8
// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}",self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}",self.0)
//     }
// }

// fn main() {
//     // 填空
//     let v: Vec<Box<dyn IpAddr>>= vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];

//     for ip in v {
//         ip.display();
//     }
// }

// use std::collections::HashMap;

// // HashMap
// fn main() {
//     // 创建一个HashMap，用于存储宝石种类和对应的数量
//     let mut my_gems = HashMap::new();
//     // 将宝石类型和对应的数量写入表中
//     my_gems.insert("红宝石", 1);
//     my_gems.insert("蓝宝石", 1);
//     my_gems.insert("河边捡的破石头", 1);
// }

// use std::collections::HashMap;


// // Vec<(String, u32)> 中的数据写入到 HashMap<String, u32>
// fn main() {
//     let team_list = vec![
//         ("中国队".to_string(), 100),
//         ("美国队".to_string(), 10),
//         ("日本队".to_string(), 50),
//     ];

//     // let mut team_hashMap = HashMap::new();
//     // for team in team_list {
//     //     team_hashMap.insert(team.0, team.1);
//     // }
//     // println!("{:?}", team_hashMap);

//     let team_hashMap: HashMap<_, _> = team_list.into_iter().collect();
//     println!("{:?}", team_hashMap);

// }

// fn main() {
//     use std::collections::HashMap;

//     let name = String::from("Sunface");
//     let age = 18;

//     let mut handsome_boys = HashMap::new();
//     handsome_boys.insert(name, age);

//     println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
//     println!("还有，他的真实年龄远远不止{}岁", age);
// }

// use std::collections::HashMap;

// // 查询 HashMap
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("red"), 20);
//     scores.insert(String::from("Yellow"), 50);

//     // let team_name = String::from("Blue");
//     // // let score = scores.get(&team_name);    //score: Option<&i32>
//     // // println!("{:?}", score);

//     // let score = scores.get(&team_name).copied().unwrap_or(0);
//     // println!("{}", score);

//     for (key, value) in &scores {
//         println!("{}, {}", key, value);
//     }


// }

// use std::collections::HashMap;

// // 更新 HashMap 中的值
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("Blue", 10);

//     // 覆盖
//     let old = scores.insert("Blue", 20);
//     assert_eq!(old, Some(10));

//     // 查询新值
//     let new = scores.get("Blue");
//     assert_eq!(new, Some(&20));

//     // 查询Yellow对应的值，若不存在则插入新值
//     let v = scores.entry("Yellow").or_insert(5);
//     assert_eq!(*v, 5); // 不存在，插入5

//     let v = scores.entry("Yellow").or_insert(50);
//     assert_eq!(*v, 50);

// }

// use std::collections::HashMap;

// // 在文本中统计词语出现的次数
// fn main() {
//     let text =  "hello world wonderful world";
//     let mut map = HashMap::new();
//     // 根据空格来切分字符串(英文单词都是通过空格切分)
//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count+=1;
//     }
//     println!("{:?}", map);
// }

// 哈希函数
// 高性能三方库
// use std::hash::BuildHasherDefault;
// use std::collections::HashMap;

// // 引入第三方哈希函数
// use twox_hash::XxHash64;
// fn main() {
//     // 指定HashMap使用第三方的哈希函数XxHash64
//     let mut hash: HashMap<_, _,  BuildHasherDefault<XxHash64>> = Default::default();
//     hash.insert(42, "the answer");
//     assert_eq!(hash.get(&42), Some(&"the answer"));
// }

// practixe
// 1

// 填空并修复错误
// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69);
//     scores.insert("Katie", 58);

//     // get 返回一个 Option<&V> 枚举值
//     let score = scores.get("Sunface");
//     assert_eq!(score, Some(&98));

//     if scores.contains_key("Daniel") {
//         // 索引返回一个值 V
//         let score = scores["Daniel"];
//         assert_eq!(score, 95);
//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), 3);

//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score)
//     }
// }

// 2

// use std::collections::HashMap;
// fn main() {
//     let teams = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];

//     let mut teams_map1 = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }
// let teams_map2 = HashMap::from(teams);

//     // 使用两种方法实现 team_map2
//     // 提示:其中一种方法是使用 `collect` 方法
//     // let mut teams_map2 =  HashMap::new();
//     // for team in &teams {
//     //     teams_map2.insert(team.0, team.1);
//     // }

//     let teams_map2 = teams.into_iter().collect();

//     assert_eq!(teams_map1, teams_map2);

//     println!("Success!")
// }

// 3

// 填空
// use std::collections::HashMap;
// fn main() {
//     // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
//     let mut player_stats = HashMap::new();

//     // 查询指定的 key, 若不存在时，则插入新的 kv 值
//     player_stats.entry("health").or_insert(100);

//     assert_eq!(player_stats["health"], 100);

//     // 通过函数来返回新的值
//     player_stats.entry("health").or_insert_with(random_stat_buff);
//     assert_eq!(player_stats["health"], 100);

//     let health = player_stats.entry("health").or_insert(50);
//     assert_eq!(*health, 100);
//     assert_eq!(health, &100);
//     *health -= 50;
//     assert_eq!(*health, 50);

//     println!("Success!")
// }

// fn random_stat_buff() -> u8 {
//     // 为了简单，我们没有使用随机，而是返回一个固定的值
//     42
// }

//4  ?

// 修复错误
// 提示: `derive` 是实现一些常用特征的好办法
// use std::collections::HashMap;

// #[derive(PartialEq, Eq, Debug, Hash)]
// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// fn main() {
//     // 使用 HashMap 来存储 viking 的生命值
//     let vikings = HashMap::from([
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ]);

//     // 使用 derive 的方式来打印 viking 的当前状态
//     for (viking, health) in &vikings {
//         println!("{:?} has {} hp", viking, health);
//     }
// } 

// 5
// 容量
// use std::collections::HashMap;
// fn main() {
//     let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
//     map.insert(1, 2);
//     map.insert(3, 4);
//     // 事实上，虽然我们使用了 100 容量来初始化，但是 map 的容量很可能会比 100 更多
//     assert!(map.capacity() >= 100);

//     // 对容量进行收缩，你提供的值仅仅是一个允许的最小值，实际上，Rust 会根据当前存储的数据量进行自动设置，当然，这个值会尽量靠近你提供的值，同时还可能会预留一些调整空间

//     map.shrink_to(50);
//     assert!(map.capacity() >= 50);

//     // 让 Rust  自行调整到一个合适的值，剩余策略同上
//     map.shrink_to_fit();
//     assert!(map.capacity() >= 2);
//     println!("Success!")
// }

// 5
// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
// use std::collections::HashMap;
// fn main() {
//   let v1 = 10;
//   let mut m1 = HashMap::new();
//   m1.insert(v1, v1);
//   println!("v1 is still usable after inserting to hashmap : {}", v1);

// //   let v2 = "hello".to_string();
//   let v2 = "hello";
// //   let mut m2 = HashMap::new();
// //   // 所有权在这里发生了转移
// //   m2.insert(v2.clone(), v1);
//   let mut m2 = HashMap::new();
//   m2.insert(v2, v1);

//   assert_eq!(v2, "hello");

//    println!("Success!")
// }



// 生命周期
// 悬垂指针
// fn main() {
//     let v1;
//     {
//         let x = 2;
//         v1 = &x;
//     }
//     println!("v1: {}", v1);
// }

// 函数中的生命周期
// fn main() {
//     let string1 = String::from("Hello");
//     let string2 = "rust";
//     let result = longest(string1.as_str(), string2);
//     println!("{}", result);
// // }
// fn longest<'a>(str1: & 'a str, str2: & 'a str) -> & 'a str{
//     if str1.len()> str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// fn main() {
//     let string1 = String::from("long string is long");

//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }

// fn longest<'a>(_str1: &str, _str2: &str) -> String {
//     String::from("really long string")
// }

// fn main() {
//     let s = longest("not", "import");
//     println!("{}", s);
// }

// 结构体中的生命周期
// struct ImportException<'a> {
//     part: & 'a str,
// }
// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.' ");
//     let i= ImportException {
//         part:first_sentence,
//     };
// }

// struct ImportException<'a> {
//     part: & 'a str,
// }
// fn main() {
//     let i;
//     {
//         let novel = String::from("Call me Ishmael. Some years ago...");
//         let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//         i = ImportantExcerpt {
//             part: first_sentence,
//         };
//     }
//     println!("{:?}",i);
// }



// 生命周期消除
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }
// fn main() {
//     first_word("s");
// }

// 方法中的生命周期
// struct Point<T> {
//     x:T,
//     y:T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// use std::fmt::Display;

// struct ImportantExcerpt<'a> {
//     part: & 'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// 第一规则，给予每个输入参数一个生命周期
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// impl<'a: 'b, 'b> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str 
//         where 'a: 'b,
//       {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }

// 静态生命周期
// fn main() {
//     let s: &'static str = "我没啥优点，就是活得久，嘿嘿";

// }

// 一个复杂例子: 泛型、特征约束
// use std::fmt::Display;
// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T,
// ) -> &'a str 
//     where 
//         T: Display, 
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// practice
// 1
/* 为 `i` 和 `borrow2` 标注合适的生命周期范围 */


// `i` 拥有最长的生命周期，因为它的作用域完整的包含了 `borrow1` 和 `borrow2` 。
// 而 `borrow1` 和 `borrow2` 的生命周期并无关联，因为它们的作用域没有重叠
// fn main() {
//     let i = 3;                                             
//     {                                                    
//         let borrow1 = &i; // `borrow1` 生命周期开始. ──┐
//         //                                                │
//         println!("borrow1: {}", borrow1); //              │
//     } // `borrow1` 生命周期结束. ──────────────────────────────────┘
//     {                                                    
//         let borrow2 = &i; 
                                                        
//         println!("borrow2: {}", borrow2);               
//     }                                                   
// }   

// 2
// {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {}", r); //   |       |
//                           // --+       |
// }                         // ----------+

/* 像上面的示例一样，为 `r` 和 `x` 标准生命周期，然后从生命周期的角度. */

// fn main() {  
//     {
//         let r;                // ---------+-- 'a
//                               //          |
//         {                     //          |
//             let x = 5;        // -+-- 'b  |
//             r = &x;           //  |       |
//         }                     // -+       |
//                               //          |
//         println!("r: {}", r); //          |
//     }                         // ---------+
// }

// // 引用参数中的生命周期 'a 至少要跟函数活得一样久
// fn print_one<'a>(x: &'a i32) {
//     println!("`print_one`: x is {}", x);
// }

// // 可变引用依然需要标准生命周期
// fn add_one<'a>(x: &'a mut i32) {
//     *x += 1;
// }

// // 下面代码中，每个参数都拥有自己独立的生命周期，事实上，这个例子足够简单，因此它们应该被标记上相同的生命周期 `'a`，但是对于复杂的例子而言，独立的生命周期标注是可能存在的
// fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("`print_multi`: x is {}, y is {}", x, y);
// }

// // 返回一个通过参数传入的引用是很常见的，但是这种情况下需要标注上正确的生命周期
// fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }

// fn main() {
//     let x = 7;
//     let y = 9;
    
//     print_one(&x);
//     print_multi(&x, &y);
    
//     let z = pass_x(&x, &y);
//     print_one(z);

//     let mut t = 3;
//     add_one(&mut t);
//     print_one(&t);
// }

// 3
/* 添加合适的生命周期标注，让下面的代码工作 */
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {}

// 4
/* 使用三种方法修复下面的错误  */
// fn invalid_output<'a>(x: &'a String ) -> &'a String { 
//     &String::from("foo") 
// }

// fn main() {
// }

// fn invalid_output<'a>() -> String {  
//     String::from("foo") 
// }

// fn main() {
// }

// fn invalid_output() -> &'static str { 
//     "foo" 
// }

// fn main() {
// }

// fn invalid_output<'a>(x: &'a String ) -> &'a String { 
//     x
// }

// fn main() {
// }

// 5
// `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("x is {} and y is {}", x, y);
// }

// /* 让下面的代码工作 */
// fn failed_borrow<'a>() {
//     let _x = 12;

//     // ERROR: `_x` 活得不够久does not live long enough
//     let y: & i32 = &_x;

//     // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
//     // 你不能将一个小的生命周期强转成大的
// }

// fn main() {
//     let (four, nine) = (4, 9);
    

//     print_refs(&four, &nine);
//     // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长
    
//     failed_borrow();
//     // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
// }

// 6
/* 增加合适的生命周期标准，让代码工作 */

// `i32` 的引用必须比 `Borrowed` 活得更久
// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32);

// // 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
// #[derive(Debug)]
// struct NamedBorrowed<'a> {
//     x: &'a i32,
//     y: &'a i32,
// }

// #[derive(Debug)]
// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {
//     let x = 18;
//     let y = 15;

//     let single = Borrowed(&x);
//     let double = NamedBorrowed { x: &x, y: &y };
//     let reference = Either::Ref(&x);
//     let number    = Either::Num(y);

//     println!("x is borrowed in {:?}", single);
//     println!("x and y are borrowed in {:?}", double);
//     println!("x is borrowed in {:?}", reference);
//     println!("y is *not* borrowed in {:?}", number);
// }

// 7 --
/* 让代码工作 */

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }

// fn main()
// { 
//   let var_a = 35;
//   let example: Example;
  
  
//     let var_b = NoCopyType {};
    
//     /* 修复错误 */
//     example = Example { a: &var_a, b: &var_b };
  
  
//   println!("(Success!) {:?}", example);
// }

// 8

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }

// /* 修复函数的签名 */
// fn fix_me<'b>(foo: &Example<'_, 'b>) -> &'b NoCopyType
// { foo.b }

// fn main()
// {
//     let no_copy = NoCopyType {};
//     let example = Example { a: &1, b: &no_copy };
//     fix_me(&example);
//     println!("Success!")
// }

// struct Owner(i32);

// impl Owner {
//     fn add_one<'a>(&'a mut self) { self.0 += 1; }
//     fn print<'a>(&'a self) {
//         println!("`print`: {}", self.0);
//     }
// }

// fn main() {
//     let mut owner = Owner(18);

//     owner.add_one();
//     owner.print();
// }

// 9
/* 添加合适的生命周期让下面代码工作 */
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&'a self) ->  i32 {
//         3
//     }
// }

// fn main() {}

// 10

/* 移除所有可以消除的生命周期标注 */

// fn nput(x: &i32) {
//     println!("`annotated_input`: {}", x);
// }

// fn pass(x: &i32) -> & i32 { x }

// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     x
// }

// struct Owner(i32);

// impl Owner {
//     fn add_one(& mut self) { self.0 += 1; }
//     fn print(&self) {
//         println!("`print`: {}", self.0);
//     }
// }

// struct Person<'a> {
//     age: u8,
//     name: &'a str,
// }

// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {}


// 返回值和错误处理

// panic! 与不可恢复错误
// 被动触发
// fn main() {
//     let v = vec![1,2,3,4];

//     v[99];  //数组访问越界
// }

// 主动调用
// fn main() {
//     panic!("crash and burn");
// }




// 统计文本中的单词
// use std::collections::HashMap;
// use std::env; 
// use std::fs::File; 
// use std::io::prelude::BufRead; 
// use std::io::BufReader; 
// #[derive(Debug)] 
// struct WordCounter(HashMap<String, u64>); 
// impl WordCounter {  
//     fn new() -> WordCounter {  
//         WordCounter(HashMap::new())
//      }   
//     fn increment(& mut self, word: &str) { 
//          let key = word.to_string();  
//          let count = self.0.entry(key).or_insert(0);  
//          *count += 1;
//     }  
//     fn display(self) { 
//          for (key, value) in self.0.iter()  {  
//             println!("{}: {}", key, value);  
//         }  
//     }
// }  
// fn main() {  
//     let arguments: Vec<String> = env::args().collect();  

//     if arguments.len() < 2 {
//         eprintln!("Usage: {} <filename>", arguments[0]);
//         return;
//     }
//     // 获取第二个命令行参数作为文件名
//     let filename = &arguments[1];

//     println!("Processing file: {}", filename);  
//     let file = File::open(filename).expect("Could not open file");  
//     let reader = BufReader::new(file);  
//     let mut word_counter = WordCounter::new();   
//     for line in reader.lines() { 
//          let line = line.expect("Could not read line"); 
//           let words = line.split(" ");  
//           for word in words { 
//              if word == "" {
//                   continue
//                   }
//               else { 
//                  word_counter.increment(word); 
//                  } 
//               }  
//             }  
//     word_counter.display(); 
// }


// backtrace 栈展开
// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// }

// 何时该使用 panic!
// enum Result<T, E> {
//     OK(T),
//     Err(E),
// }

// use std::net::IpAddr;
// fn  main() {
//     let home: IpAddr = "127.0.0.1".parse().unwrap();
//     println!("{}", home);
// }

// practice
// 1

// 填空
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // 实现下面的代码
//         panic!("drinked, duang.....peng!")
//      }

//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     let str = "lemonade";
//     drink(str);

//     println!("Exercise Failed if printing out this line!");
// }

// 2
// 修复所有的 panic，让代码工作
// fn main() {
//     assert_eq!("abc".as_bytes(), [97, 98, 99]);

//     let v = vec![1, 2, 3];
//     let ele = v[1];
//     let ele = v.get(1).unwrap();

//     // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
//     let v = production_rate_per_hour(2);

//     divide(15, 1);

//     println!("Success!")
// }

// fn divide(x:u8, y:u8) {
//     println!("{}", x / y)
// }

// fn production_rate_per_hour(speed: u32) -> f64 {
//     let cph: u32 = 221;
//     match speed {
//         1..=4 => (speed * cph) as f64,
//         5..=8 => (speed * cph) as f64 * 0.9,
//         9..=10 => (speed * cph) as f64 * 0.77,
//         _ => 0 as f64,
//     }
// }

// pub fn working_items_per_minute(speed: u8) -> u32 {
//     (production_rate_per_hour(speed) / 60 as f64) as u32
// }

// 3


// 可恢复的错误 Result
// enum Result<T, E> {
//     OK(T),
//     Err(E),
// }

// use std::{error, io::ErrorKind, };
// use::std::fs::File;
// fn main() {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("problem creating the file: {:?}", e),
//             },
//             other_error => panic!("problem opening the file: {:?}", other_error),
           
//         }
//     };
// }


// 失败就 panic: unwrap 和 expect

// use std::fs::File;

// fn main() {
//     // let f = File::open("hello.txt").unwrap();


//     let f = File::open("hello.txt").expect("Faild to open hello.txt");

// }

// use std::{fs::File, io::{self, Read}};


// //从文件中读取用户名，然后返回结果
// fn read_username_from_file() -> Result<String, io::Error> {
//     // 打开文件，f是`Result<文件句柄,io::Error>`
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         // 打开文件成功，将file句柄赋值给f
//         Ok(file) => file,
//         // 打开文件失败，将错误返回(向上传播)
//         Err(e) => return Err(e), 
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         // 将错误向上传播
//         Err(e)  => Err(e),
//     }

// }

// use std::{fs::File, io::{self, Read}};


// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// use std::{fs::File, io::{self, Read}};
// use std::{fs, io};

// fn read_username_from_file()-> Result<String, io::Error> {
//         // let mut s = String::new();
//         // File::open("hello.txt")?.read_to_string(&mut s)?;
//         // Ok(s)

//          // read_to_string是定义在std::io中的方法，因此需要在上面进行引用
//         fs::read_to_string("hello.txt")

// }

// fn first(arr: &[i32]) -> Option<&i32> {
//     let v = arr.get(2)?;
//     Some(v)
// }

// 带返回值的 main 函数

// use std::{error::Error, fs::File};

// // fn main() {
// //     let f = File::open("hello.txt")?;
// // }

// fn main() -> Result<(), Box<dyn Error>>{
//     let f = File::open("hello.txt")?;
//     Ok(())
// }

// try!
// macro_rules! try {
//     ($e:expr) => (match $e {
//         Ok(val) => val,
//         Err(err) => return Err(::std::convert::From::from(err)),
//     });
// }


// practice

// 1
// 填空并修复错误
// use std::num::ParseIntError;

// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError>{
//     let n1 = n1_str.parse::<i32>();
//     let n2 = n2_str.parse::<i32>();
//     Ok(n1.unwrap() * n2.unwrap())
// }

// fn main() {
//     let result = multiply("10", "2");
//     assert_eq!(result, Ok(20));

//     let result = multiply("4", "2");
//     assert_eq!(result.unwrap(), 8);

//     println!("Success!")
// }

// 2

// use std::num::ParseIntError;

// // 使用 `?` 来实现 multiply
// // 不要使用 unwrap !
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     let n1 = n1_str.parse::<i32>()?;
//     let n2 =   n2_str.parse::<i32>()?;
//     Ok(n1* n2)
// }

// fn main() {
//     assert_eq!(multiply("3", "4").unwrap(), 12);
//     println!("Success!")
// }

// 3

// use std::fs::File;
// use std::io::{self, Read};

// fn read_file1() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// // 填空
// // 不要修改其它代码
// fn read_file2() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn main() {
//     assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
//     println!("Success!")
// }

// 4
// use std::num::ParseIntError;

// // 使用两种方式填空: map, and then
// fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
//    n_str.parse::<i32>().map(|i| i+2)
// }

// fn main() {
//     assert_eq!(add_two("4").unwrap(), 6);

//     println!("Success!")
// }

// use std::num::ParseIntError;

// // 使用两种方式填空: map, and then
// fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
//    n_str.parse::<i32>().and_then(|i| Ok(i+2))
// }

// fn main() {
//     assert_eq!(add_two("4").unwrap(), 6);

//     println!("Success!")
// }

// 5
// use std::num::ParseIntError;

// // 使用 Result 重写后，我们使用模式匹配的方式来处理，而无需使用 `unwrap`
// // 但是这种写法实在过于啰嗦..
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     match n1_str.parse::<i32>() {
//         Ok(n1)  => {
//             match n2_str.parse::<i32>() {
//                 Ok(n2)  => {
//                     Ok(n1 * n2)
//                 },
//                 Err(e) => Err(e),
//             }
//         },
//         Err(e) => Err(e),
//     }
// }

// // 重写上面的 `multiply` ，让它尽量简洁
// // 提示：使用 `and_then` 和 `map`
// fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     // 实现...
//     n1_str.parse::<i32>().and_then(|n1| {
//         n2_str.parse::<i32>().map(|n2| n1 * n2)
//     })
  
    
// }

// fn print(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn main() {
//     let twenty = multiply1("10", "2");
//     print(twenty);

//     // 下面的调用会提供更有帮助的错误信息
//     let tt = multiply("t", "2");
//     print(tt);

//     println!("Success!")
// }


// use std::num::ParseIntError;

// // 填空
// type Res<T> = std::result::Result<T, ParseIntError>;

// // 使用上面的别名来引用原来的 `Result` 类型
// fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
//     first_number_str.parse::<i32>().and_then(|first_number| {
//         second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
//     })
// }

// // 同样, 这里也使用了类型别名来简化代码
// fn print(result: Res<i32>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn main() {
//     print(multiply("10", "2"));
//     print(multiply("t", "2"));

//     println!("Success!")
// }   



// use std::num::ParseIntError;

// fn main() -> Result<(), ParseIntError> {
//     let number_str = "10";
//     let number = match number_str.parse::<i32>() {
//         Ok(number)  => number,
//         Err(e) => return Err(e),
//     };
//     println!("{}", number);
//     Ok(())
// }


// package
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }


// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }



// use std::{cmp::Ordering, io};

// use rand::Rng;
// // 猜数字
// fn main() {

//     let number = rand::thread_rng().gen_range(0..=100);
//     println!("secret number is: {number}");

//     loop {
//         let mut input = String::new();
//         // 获取输入
//         println!("please input your guess(0-100):>");
//         io::stdin().read_line(&mut input).expect("Faild to read line");

//         println!("your guess {input}");

//         let guess: i32= match input.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         match guess.cmp(& number) {
//             Ordering::Less => println!("Too small"),
//             Ordering::Greater => println!("Too big"),
//             Ordering::Equal => {
//                 println!("you win!");
//                 break;
//         }
//             }
//     }


// }

// 按空格划分语句
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     let s = String::from("Hello World");
//     let n = first_word(&s);
//     println!("{n}");
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i]
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let s = String::from("Hello World");
//     let n = first_word(&s);
//     println!("{n}");
// }

// 格式化输出
// fn main() {
//     // let s = "hello";
//     // println!("{} world", s);
//     // let s1 = format!("{}, world", s);
//     // print!("{}", s1);
//     // print!("{}\n", "!");

//     // 输出错误信息
//     eprintln!("Error: Could not complete task")


// }


// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8
// }

// fn main() {
//     let i = 3.1415926;
//     let s = String::from("hello");
//     let v = vec![1, 2, 3];
//     let p = Person{name: "sunface".to_string(), age: 18};
//     println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
// }

// use std::fmt;

// // 为自定义类型实现 Display 特征
// struct Person {
//     name: String,
//     age: u8,
// }

// impl fmt::Display for Person {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
//             self.name, self.age
//         )
//     }
// }

// fn main() {
//     let p = Person{
//         name: "sunface".to_string(),
//         age: 22,
//     };

//     println!("{}", p);
// }

// use std::fmt;


// // 为外部类型实现 Display 特征
// struct Array(Vec<i32>);

// impl fmt::Display for Array {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f, 
//             "数组是：{:?}", self.0
//         )
//     }
// }

// fn main() {
//     let arr = Array(vec![1,2,3]);
//     println!("{}", arr);
// }

// 位置参数
// fn main() {
//     println!("{}{}", 1, 2); // =>"12"
//     println!("{1}{0}", 1, 2); // =>"21"
//     // => Alice, this is Bob. Bob, this is Alice
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
//     println!("{1}{}{0}{}", 1, 2); // => 2112
// }

// 具名参数
// fn main() {
//     println!("{argument}", argument = "test"); // => "test"
//     println!("{name} {}", 1, name = 2); // => "2 1"
//     println!("{a} {c} {b}", a = "a", b = 'b', c = 3); // => "a 3 b"
    // println!("{abc} {1}", abc = "def", 2);

// }

// 格式化参数
// fn main() {
//     let v = 3.1415926;
//     // Display => 3.14
//     println!("{:.2}", v);
//         // Debug => 3.14
//     println!("{:.2?}", v);
// }   

// 宽度
// fn main() {
//     //-----------------------------------
//     // 以下全部输出 "Hello x    !"
//     // 为"x"后面填充空格，补齐宽度5
//     println!("Hello {:5}!", "x");
//     // 使用参数5来指定宽度
//     println!("Hello {:1$}!", "x", 5);
//     // 使用x作为占位符输出内容，同时使用5作为宽度
//     println!("Hello {1:0$}!", 5, "x");
//     // 使用有名称的参数作为宽度
//     println!("Hello {:width$}!", "x", width = 5);
//     //-----------------------------------

//     // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
//     println!("Hello {:1$}!{}", "x", 5);
// }



// rust高阶
// &'static 和 T: 'static
// fn main() {
//     let mark_twain = "hello world";
//     print_author(mark_twain);
// }

// fn print_author(author: &'static str) {
//     println!("{}", author);
// }

// use std::fmt::Display;

// fn main() {
//     let mark_twain = "hello world";
//     print(&mark_twain);
// }

// fn print<T: Display + 'static>(message: &T) {
//     println!("{}", message);
// }

// use std::{slice::from_raw_parts, str::from_utf8_unchecked};

// fn get_memory_location() -> (usize, usize) {
//   // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
//   // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
//   let string = "Hello World";
//   let pointer = string.as_ptr() as usize;
//   let length = string.len();
//   (pointer, length)
//   // `string` 在这里被 drop 释放
//   // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
// }

// fn get_str_at_location(pointer: usize, length: usize) -> &'static str{
//     // 使用裸指针需要 `unsafe{}` 语句块
//     unsafe {from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))}
// }


// fn main() {
//     let (pointer, length) = get_memory_location();
//     let message = get_str_at_location(pointer, length);
//     println!(
//         "The {} bytes as 0x{:X} stored: {}", 
//         length, pointer, message
//     );
//     // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
//     // let message = get_str_at_location(1000, 10);

// }

// T: 'static
// use std::fmt::Debug;

// // fn print_it<T: Debug + 'static>(input: T) {
// //     println!("'static value passed in is: {:?}", input);
// // }

// fn print_it<T: Debug + 'static>(input: &T) {
//     println!("'static value passed in is: {:?}", input);
// }

// fn print_it1(input: impl Debug + 'static) {
//     println!("'static value passed in is: {:?}", input);
// }

// fn main() {
//     let i = 5;
//     print_it(&i);
//     // print_it1(&i);
// }

// use std::fmt::Display;


// fn main() {
//     let r1;
//     let r2;
//     {
//         static STATIC_EXAMPLE: i32 = 44;
//         r1 = &STATIC_EXAMPLE;
//         let x = "'static str";
//         r2 = x;
//         // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
//     }

//     println!("'static i32: {}", r1);
//     println!("'static str: {}", r2);

//     let r3: &str;
//     {
//         let s1 = "String".to_string();
//         // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
//         // 充分说明这个约束是多么的弱。。
//         static_bound(&s1);
//         // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
//         r3 = &s1;
//         // s1 在这里被 drop
//     }
//     println!("{}", r3);
//     fn static_bound<T: Display+ 'static>(t: &T) {
//         println!("{}", t);
//     }

// }

// fn main() {
//     {
//         let static_string = "I'm in read-only memory";
//         println!("static_string: {}", static_string);

//         // 当 `static_string` 超出作用域时，该引用不能再被使用，但是数据依然会存在于 binary 所占用的内存中
//     }

//     println!("static_string reference remains alive: {}", static_string);
// }

// practice
// 1

/* 使用两种方法填空 */
// fn main() {
//     let v = "hello";
//     need_static(v);

//     println!("Success!")
// }

// fn need_static(r : &'static str) {
//     assert_eq!(r, "hello");
// }



/* 使用两种方法填空 */
// fn main() {
//     const v:&str = "hello";
//     need_static(v);

//     println!("Success!")
// }

// fn need_static(r : &'static str) {
//     assert_eq!(r, "hello");
// }

// 2
// #[derive(Debug)]
// struct Config {
//     a: String,
//     b: String,
// }
// static mut config: Option<&mut Config> = None;

// /* 让代码工作，但不要修改函数的签名 */
// fn init() -> Option<&'static mut Config> {
//     let c = Box::new(Config {
//         a: "A".to_string(),
//         b: "B".to_string(),
//         // a: Box::leak(Box::new("A")).to_string(),
//         // b: Box::leak(Box::new("B")).to_string(),
//     });
//     Some(Box::leak(c))
// }


// fn main() {
//     unsafe {
//         config = init();

//         println!("{:?}",config)
//     }
// }

// 3
// fn main() {
//     {
//         // 字符串字面量能跟程序活得一样久，因此 `static_string` 的生命周期是 `'static`
//         let static_string = "I'm in read-only memory";
//         println!("static_string: {}", static_string);

//         // 当 `static_string` 超出作用域时，该引用就无法再被使用，但是引用指向的数据( 字符串字面量 ) 依然保存在二进制 binary 所占用的内存中
//     }

//     println!("static_string reference remains alive: {}", static_string);
// }

// 4
// 声明一个 static 常量，它拥有 `'static` 生命周期.
// static NUM: i32 = 18;

// // 返回常量 `Num` 的引用，注意，这里的生命周期从 `'static` 强转为 `'a`
// fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
//     &NUM
// }

// fn main() {
//     {
//         let lifetime_num = 9;

//         let coerced_static = coerce_static(&lifetime_num);

//         println!("coerced_static: {}", coerced_static);
//     }

//     println!("NUM: {} stays accessible!", NUM);
// }

// 5
/* 让代码工作 */
// use std::fmt::Debug;

// fn print_it<T: Debug + 'static>( input: T) {
//     println!( "'static value passed in is: {:?}", input );
// }

// fn print_it1( input: impl Debug + 'static ) {
//     println!( "'static value passed in is: {:?}", input );
// }


// fn print_it2<T: Debug + 'static>( input: &T) {
//     println!( "'static value passed in is: {:?}", input );
// }

// fn main() {
//     // i 是有所有权的数据，并没有包含任何引用，因此它是 'static
//     let i = 5;
//     print_it(i);

//     // 但是 &i 是一个引用，生命周期受限于作用域，因此它不是 'static
//     print_it(i);

//     print_it1(i);

//     // 但是下面的代码可以正常运行 !
//     print_it2(&i);
// }


// 6
// use std::fmt::Display;

// fn main() {
//   let mut string = "First".to_owned();

//   string.push_str(string.to_uppercase().as_str());
//   print_a(&string);
//   print_b(&string);
//   print_c(&string); // Compilation error
//   print_d(&string); // Compilation error
//   print_e(&string);
//   print_f(&string);
//   print_g(&string); // Compilation error
// }

// fn print_a<T: Display + 'static>(t: &T) {
//   println!("{}", t);
// }

// fn print_b<T>(t: &T)
// where
//   T: Display + 'static,
// {
//   println!("{}", t);
// }

// fn print_c(t: &'static dyn Display) {
//   println!("{}", t)
// }

// fn print_d(t: &'static impl Display) {
//   println!("{}", t)
// }

// fn print_e(t: &(dyn Display + 'static)) {
//   println!("{}", t)
// }

// fn print_f(t: &(impl Display + 'static)) {
//   println!("{}", t)
// }

// fn print_g(t: &'static String) {
//   println!("{}", t);
// }

// Closure
// fn main() {
//     let x = 1;
//     let sum = |y| x+y;

//     assert_eq!(3, sum(2));
// }

// use std::{thread, time::Duration};

// // 传统函数实现
// // 开始健身，好累，我得发出声音：muuuu...
// fn muuuuu(intensity: u32) -> u32 {
//     println!("muuuuu..");
//     thread::sleep(Duration::from_secs(2));
//     intensity  //强度
// }

// fn workout(intensity: u32, random_number: u32) {
//     if intensity< 25 {
//         println!("今天活力满满，先做{}个俯卧撑",muuuuu(intensity));
//         println!("旁边有妹子，再来{}组卧推", muuuuu(intensity));
//     } else if random_number == 3 {
//         println!("昨天训练过度，今天休息下");
//     } else {
//         println!("今天有氧运动{}分钟", muuuuu(intensity));
//     }
// }

// fn main() {
//     // 强度
//     let intensity = 10;
//     // 随机值来决定某个选择
//     let random_number = 7;

//     workout(intensity, random_number);
// }

// 闭包实现
// use std::{thread, time::Duration};

// 传统函数实现
// 开始健身，好累，我得发出声音：muuuu...
// fn muuuuu(intensity: u32) -> u32 {
//     println!("muuuuu..");
//     thread::sleep(Duration::from_secs(2));
//     intensity  //强度
// }

// fn workout(intensity: u32, random_number: u32) {
//     let action = || {
//         println!("muuuuu..");
//         thread::sleep(Duration::from_secs(2));
//         intensity  //强度
//     };
//     if intensity< 25 {
//         println!("今天活力满满，先做{}个俯卧撑",action());
//         println!("旁边有妹子，再来{}组卧推", action());
//     } else if random_number == 3 {
//         println!("昨天训练过度，今天休息下");
//     } else {
//         println!("今天有氧运动{}分钟", action());
//     }
// }

// fn main() {
//     // 强度
//     let intensity = 10;
//     // 随机值来决定某个选择
//     let random_number = 7;

//     workout(intensity, random_number);
// }

// fn main() {
//     // 闭包类型标注
//     // let sum = |x: i32, y:i32| -> i32{ x + y };
//     let sum = |x, y| x + y;
//     let v = sum(1, 2);
// }

// 结构体中的闭包
// struct Catch<T> 
// where 
//     T: Fn(u32) -> u32,
// {
//     query: T,
//     value: Option<u32>,
// }

// impl<T> Catch<T> 
// where 
//     T: Fn(u32) -> u32,
// {
//     fn new(query: T) -> Catch<T> {
//         Catch {
//             query,
//             value: None,
//         }
//     }
//     //  先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
//     fn value(&mut self, args: u32) -> u32{
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query)(args);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }

// }

// 捕获作用域中的值
// fn main() {
//     let x = 4;
//     let equal_to_x = |y| y == x;

//     let z = 4;

//     assert!(equal_to_x(z));
// }

// 三种 Fn 特征
// FnOnce
// fn fn_once<F>(func: F) 
// where 
//     F: FnOnce(usize) -> bool + Copy
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     let x = vec![1, 2, 3];
//     fn_once(|z| z== x.len());
// }

// use std::thread;

// fn main() {
//     let v = vec![1,2,3];
//     let handle = thread::spawn(move || {
//         println!("here the vector is {:?}", v);
//     });
//     handle.join().unwrap();
// }

// FnMut
// fn main() {
//     let mut s = String::new();
//     let mut update_string = |str| s.push_str(str); 
//     update_string("hello");
//     println!("{:?}", s);

// }


// fn main() {
//     let mut s = String::new();
//     let update_string = |str| s.push_str(str);
//     exec(update_string);
//     println!("{:?}", s);
// }
// // fn exec<'a, F: FnMut(&'a str)> (mut f: F) {
// //     f("hello");
// // }

// fn exec<'a, F: Fn(&'a str)> (mut f: F) {
//     f("hello");
// }

// fn main() {
//     let s = "hello".to_string();
//     let update_string = |str| println!("{}, {}", s, str);
//     exec(update_string);
//     println!("{:?}", s);
// }

// fn exec<'a, F: Fn(&'a str)> (f: F) {
//     f("hello");
// }

// 三种 Fn 的关系
// fn main() {
//     let s = String::new();
//     let update_string = || println!("{}", s);

//     exec(update_string);
//     exec1(update_string);
//     exec2(update_string);
// }

// fn exec<F: FnOnce()>(f: F) {
//     f()
// }

// fn exec1<F: FnMut()>(mut f: F) {
//     f()
// }

// fn exec2<F: Fn()>(f: F) {
//     f()
// }

// fn main() {
//     let mut s = String::new();
//     let update_string = |str| -> String {s.push_str(str); s};

//     exec(update_string);
// }

// fn exec<'a, F: FnMut(&'a str)-> String>(mut f: F) {
//     f("hello");
// }

// 闭包作为函数返回值
// fn factory() -> Fn(i32) -> i32 {
//     let mut num= 5;
//     |x| x + num 
// }

// fn main() {
//     let f = factory(1);
//     // let answer = f(1);
//     // assert_eq!(6, answer);
// }


// fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
//     let num = 5;
//     if x > 1 {
//         Box::new(|x| x + num)
//     } else {
//         Box::new(|x| x - num)
//     }
// }


// 迭代器
// 惰性初始化
// fn main() {
//     let v1 = vec![1,2,3];
//     let v1_iter = v1.iter();
//     for var in v1_iter {
//         println!("{}", var)
//     }
// }

// next
// fn main() {
//     // let v = vec![1,2,3];

//     // let v_iter = v.iter();

//     let arr = [1,2,3];
//     let mut arr_iter = arr.into_iter();

//     assert_eq!(arr_iter.next(), Some(1));
//     assert_eq!(arr_iter.next(), Some(2));
//     assert_eq!(arr_iter.next(), Some(3));

// }

// 模拟实现for循环
// fn main() {
    

// }

// fn my_for() ->() {
//     let values = [1,2,3];
//     {
//         let result = match IntoIterator::into_iter(values) {
//             mut iter => loop {
//                 match iter.next() {
//                     Some(x) => println!("{}", x),
//                     None => break,
//                 }
//             },
//         };
//         result
//     }
// }

// IntoIterator 特征
// fn main() {
//     let values = vec![1,2,3];

//     // into_iter 会夺走所有权
//     for v in values.into_iter() {
//         println!("{}", v);
//     }

//     // println!("{:?}", values);

//     let values = vec![1,2,3];
//     let _values_iter = values.iter();

//     // values_iter 只是借用了 values 中的元素
//     println!("{:?}", _values_iter);

//     let mut values = vec![1,2,3,4];
//     let mut values_mut_iter = values.iter_mut();

//     // 取出第一个元素，并修改为0
//     if let Some(v) = values_mut_iter.next() {
//         *v = 0;
//     };

//     println!("{:?}", values);

// }


// 消费者与适配器
// 消费者适配器
// fn main() {
//     let v1 = vec![1,2,3,4];
//     let v1_inter = v1.iter();

//     let sum = v1_inter.sum();
//     assert_eq!(10, sum);

//     println!("{:?}", v1);
//     println!("{:?}", v1_inter);
// }

// 迭代器适配器
// fn main() {
//     let v1 = vec![1,2,3];
//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
//     println!("{:?}", v2);

// }

// use std::collections::HashMap;


// fn main() {
//     let names = ["dazhang", "daming"];
//     let ages = [22,24];

//     let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    
//     println!("{:?}", folks);
// }

// 闭包作为适配器参数/
// struct Shoe {
//     size: u32,
//     style: String,
// }

// fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
//     shoes.into_iter().filter(|s| s.size == shoe_size).collect()
// }
// 实现 Iterator 特征
// 实现iterator特征
// 创建一个计数器
// struct Counter {
//     count: u32,
// }

// impl  Counter{
//     fn new() -> Counter {
//         Counter {
//             count: 0
//         }
//     }
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;
//         if self.count < 6 {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }


// fn main() {
//     // let mut counter = Counter::new();
//     // println!("{:?}", counter.next());
//     // println!("{:?}", counter.next());
//     // println!("{:?}", counter.next());
//     // println!("{:?}", counter.next());
//     // println!("{:?}", counter.next());
//     // println!("{:?}", counter.next());

//     // 实现 Iterator 特征的其它方法

//     let sum: u32 = Counter::new()
//                         .zip(Counter::new().skip(1))
//                         .map(|(a, b)| a * b)
//                         .filter(|x| x % 3 == 0)
//                         .sum();
//     assert_eq!(18, sum);
// }

// enumerate
// fn main() {
//     let v = vec![1u64, 2,3,4,5,6];
//     for (i, v) in v.iter().enumerate() {
//         println!("第{}个值是{}", i,v);
//     }
// }
// fn main() {
//     let v = vec![1u64, 2,3,4,5,6];
//     let val = v.iter().enumerate()
//     .filter(|&(idx, _)| idx % 2 == 0)
//     .map(|(_, val)| val)
//     .fold(0u64, |sum, acm| sum + acm);
// }
// 迭代器的性能

// fn sum_for(x: &[f64]) -> f64 {
//     let mut result = 0f64;
//     for i in 0..x.len() {
//         result += x[i];
//     }
//     result
// }

// fn sum_iter(x: &[f64]) -> f64{
//     x.iter().sum::<f64>()
// }


// // 避免栈上数据的拷贝
// fn main() {
//     // 在栈上创建一个长度为1000的数组
//     let arr = [0; 1000];
//     // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
//     let arr1 = arr;
//     // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
//     println!("{}",arr.len());
//     println!("{}",arr1.len());

//     // 在堆上创建长度为1000的数组，然后用智能指针指向它
//     let arr3 = Box::new([0; 1000]);
//     // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
//     // 所有权顺利转移给 arr4，arr 不再拥有所有权\
//     let arr4 = arr3;
//     println!("{}", arr4.len());
//     // 由于 arr3 不再拥有底层数组的所有权，因此下面代码将报错
//     // println!("{}", arr3.len());

// }

// 将动态大小类型变为 Sized 固定大小类型
// 递归类型
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use core::str;

// // 特征对象
// trait Draw {
//     fn draw(&self);
// }

// struct Button {
//     id: u32,
// }

// impl Draw for Button {
//     fn draw(&self) {
//         println!("这是屏幕上的第{}号按钮", self.id);
//     }
// }

// struct Select{
//     id: u32,
// }

// impl Draw for Select {
//     fn draw(&self) {
//         println!("这是一个选择框{}", self.id);
//     }
// }

// fn main() {
//     let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button{id: 1}), Box::new(Select{id: 2})];

//     for e in elems {
//         e.draw();
//     }
// }

// Box 内存布局
// fn main() {
//     let arr = vec![Box::new(1), Box::new(2)];
//     let (first, second) = (&arr[0], &arr[1]);
//     let sum = **first + **second;
//     println!("{sum}");
// }

// fn main() {
//     let s = gen_static_str();
//     println!("{}", s);
// }

// fn gen_static_str() -> &'static str{
//     let mut s = String::new();
//     s.push_str("hello world");

//     Box::leak(s.into_boxed_str())
// }

// 智能指针解引用
// fn main() {
//     let s = Box::new(1);
//     let sum = 1 + *s;
//     println!("{sum}");
// }

// 定义自己的智能指针
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
// // 为智能指针实现 Deref 特征
// use std::ops::Deref;
// impl<T> Deref for MyBox<T>{
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// fn main() {
//     let s = MyBox::new(2);
//     let sum = *s + 1;
//     println!("{}", sum);
// }

// 三种 Deref 转换
// struct MyBox<T> {
//     v: T
// }

// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox { v: x }
//     }
// }

// use std::ops::Deref;
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.v
//     }
// }

// use std::ops::DerefMut;
// impl<T> DerefMut for MyBox<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.v
//     }
// }

// fn main() {
//     let mut s = MyBox::new(String::from("hello, "));
//     display(&mut s);
// }

// fn display(s: &mut String) {
//     s.push_str("world");
//     println!("{}", s);
// }

// Drop
// struct HasDrop1;
// struct HasDrop2;

// impl Drop for HasDrop1 {
//     fn drop(&mut self) {
//         println!("Dropping HasDrop1");
//     }
// }
// impl Drop for HasDrop2 {
//     fn drop(&mut self) {
//         println!("Dropping HasDrop2");
//     }
// }

// struct HasTwoDrops {
//     one: HasDrop1,
//     two: HasDrop2,
// }

// impl Drop for HasTwoDrops {
//     fn drop(&mut self) {
//         println!("Dropping HasTwoDrops!")
//     }
// }

// struct Foo;

// impl Drop for Foo {
//     fn drop(&mut self) {
//         println!("Dropping Foo");
//     }
// }

// fn main() {
//     let _x: HasTwoDrops = HasTwoDrops {
//         one: HasDrop1,
//         two: HasDrop2,
//     };
//     let _foo = Foo;
//     println!("Running...");
// }

// 手动回收
// #[derive(Debug)]
// struct Foo;

// impl Drop for Foo {
//     fn drop(&mut self) {
//         println!("Droppong Foo!");
//     }
// }

// fn main() {
//     let foo = Foo;
//    drop(foo);
//     // println!("Running!: {:?}", foo);
// }
// 互斥的 Copy 和 Drop
// #[derive(Copy)]
// struct Foo;

// impl Drop for Foo {
//     fn drop(&mut self) {
//         println!("Dropping Foo!");
//     }
// }

// Rc<T>
// use std::rc::Rc;
// fn main() {
//     let a = Rc::new(String::from("hello world"));
//     let b = Rc::clone(&a);

//     assert_eq!(2, Rc::strong_count(&a));
//     assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

// }

// use std::rc::Rc;
// fn main() {
//     let a = Rc::new(String::from("test ref counting"));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Rc::clone(&a);
//     println!("count after creating b = {}", Rc::strong_count(&a));

//     {
//         let c = Rc::clone(&a);
//         println!("count after creating c = {}", Rc::strong_count(&c));
//     }
//     println!("count after c gose of scope = {}", Rc::strong_count(&a));
// }

// use std::rc::Rc;

// struct Owner {
//     name: String,
// }

// struct Gadget {
//     id:i32,
//     owner: Rc<Owner>
// }

// fn main() {
//     // 创建一个基于引用计数的Owner
//     let gadget_owner = Rc::new(Owner{name: "Gadget Man".to_string()});

//     // 创建两个不同的工具，他们属于同一主人
//     let gadget1 = Gadget {
//         id: 1,
//         owner: Rc::clone(&gadget_owner),
//     };

//     let gadget2 = Gadget {
//         id:2,
//         owner: Rc::clone(&gadget_owner),
//     };

//     // 释放掉第一个Rc<Owner>
//     drop(gadget_owner);
//     //尽管在上面我们释放了 gadget_owner，但是依然可以在这里使用 owner 的信息
//     // 原因是在 drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
//     // drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据，外面还有两个
//     // 引用指向底层的 owner 数据，引用计数尚未清零
//     // 因此 owner 数据依然可以被使用
//     println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
//     println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

//     // 在函数最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，随后底层
//     // 数据也被清理释放

// }

// 多线程场景使用 Rc<T>
// use std::rc::Rc;
// use std::thread;

// fn main() {
//     let s = Rc::new(String::from("多线程漫游者"));

//     for _ in 0..10 {
//         let s = Rc::clone(&s);
//         let handle = thread::spawn(move || {
//             println!("{}", s)
//         });
//     }
// }

// Arc
// use std::sync::Arc;
// use std::thread;

// fn main() {
//     let s = Arc::new(String::from("多线程漫游者"));
//     for _ in 1..10 {
//         let s = Arc::clone(&s);
//         let handle = thread::spawn(move || {
//             println!("{}", s)
//         }
//     );
//     }
// }
// Cell
// use std::cell::Cell;
// fn main() {
//     let c = Cell::new("wwas");
//     // &str实现了copy特征
//     let one = c.get();
//     c.set("qwer");
//     let two = c.get();
//     println!("{}, {}", one, two);
// }

// RefCell
// use std::cell::RefCell;

// fn main() {
//     let s = RefCell::new(String::from("hello world"));
//     let s1 = s.borrow();
//     let s2 = s.borrow_mut();
// }

// use std::cell::Cell;
// fn main() {
//     let x = Cell::new(1);
//     let y =  &x;
//     let z = &x;
//     x.set(2);
//     y.set(3);
//     z.set(4);
//     println!("{}", x.get());
// }

// fn main() {
//     let mut x = 1;
//     let y = &mut x;
//     let z = &mut x;
//     x = 2;
//     *y = 3;
//     *z = 4;
//     println!("{}", x);
// }
// 内部可变性

// 定义在外部库中的特征
// use std::cell::RefCell;
// pub trait Messenger {
//     fn send(&self, msg: String);
// }

// struct MsgQueue {
//     msg_cache: RefCell<Vec<String>>,
// }

// impl Messenger for MsgQueue {
//     fn send(&self, msg: String) {
//         self.msg_cache.borrow_mut().push(msg);
//     }
// }

// fn main() {
//     let mq = MsgQueue {
//         msg_cache: RefCell::new(Vec::new())
//     };
//     mq.send("hello".to_string());
// }

// Rc + RefCell 组合使用
// use std::rc::Rc;
// use std::cell::RefCell;

// fn main() {
//     let s = Rc::new(RefCell::new("我很善变，还有多个主人".to_string()));

//     let s1 = s.clone();
//     let s2 = s.clone();

//     s2.borrow_mut().push_str(", oh year!");

//     println!("{:?}\n{:?}\n{:?}", s,s1,s2);
// }

// fn is_even(i:i32) -> bool {
//     i%2 ==0
// }

// fn retain_even(nums: &mut Vec<i32>) {
//     let mut i = 0;
//     for num in nums.iter().filter(|&num| is_even(*num)) {
//         nums[i] = *num;
//         i+=1;
//     }
//     nums.truncate(i);
// }

// 循环引用与自引用


// 并发和并行
// 创建线程
// use std::thread;
// use std::time::Duration;

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} for thr spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread", i);
//         thread::sleep(Duration::from_millis(1));
//     } 
// }

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} for thr spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     // 让当前线程阻塞，直到它等待的子线程的结束
//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("hi number {} from the main thread", i);
//         thread::sleep(Duration::from_millis(1));
//     } 
// }

// use std::thread;

// fn main() {
//     let v = vec![1,2,3];

//     let handle = thread::spawn(move || {
//         println!("here's a vector: {:?}", v)
//     });

//     handle.join().unwrap();

//     // println!("{:?}", v);
// }

// use std::thread;
// use std::time::Duration;

// fn main() {
//     // 创建一个线程A
//     let new_thread = thread::spawn(move ||  {
//         // 创建一个线程B
//         thread::spawn(move || {
//             loop {
//                 print!("i am a new thread");
//             }
//         })
//     });
//     // 等待新创建的线程执行完成
//     new_thread.join().unwrap();
//     println!("child thread finish");

//     // 休眠一段时间，看子线程创建的子线程是否还在运行
//     thread::sleep(Duration::from_millis(100));
// }

// 线程屏障(Barrier)

// use std::{sync::{Arc, Barrier}, thread};

// fn main() {
//     let mut handles = Vec::with_capacity(6);
//     let barrier = Arc::new(Barrier::new(6));

//     for _ in 0..6 {
//         let b = barrier.clone();
//         handles.push(thread::spawn(move || {
//             println!("before wait");
//             b.wait();
//             println!("after wait");
//         }));
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }
// }

// 线性局部变量
    
// use std::cell::RefCell;
// use std::thread;

// use rand::thread_rng;



// fn main() {
//     thread_local! {static FOO: RefCell<u32> = RefCell::new(1)};

// FOO.with(|f| {
//     assert_eq!(*f.borrow(), 1);
//     *f.borrow_mut() = 2;
// });

//     // 每个线程开始时都会拿到线程局部变量FOO的初始值
//     let t = thread::spawn(move|| {
//         FOO.with(|f| {
//             assert_eq!(*f.borrow(), 1);
//             *f.borrow_mut() = 3
//         })
//     });

//     // 等待线程完成
//     t.join().unwrap();

//     // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
//     FOO.with(|f| {
//         assert_eq!(*f.borrow(), 2);
//     });

// }

// 在结构体中使用线程局部变量
// use std::cell::RefCell;

// struct Foo;
// impl Foo {
//     thread_local! {
//         static FOO: RefCell<usize> = RefCell::new(1);
//     }
// }

// fn main() {
//     Foo::FOO.with(|f| println!("{:?}", f));
// }

// 通过引用的方式使用局部变量
// use std::cell::RefCell;
// use std::thread::LocalKey;

// thread_local! {static FOO: RefCell<usize> = RefCell::new(3)}

// struct Bar {
//     foo: & 'static LocalKey<RefCell<usize>>
// }

// impl Bar {
//     fn constructor() -> Self{
//         Self { foo: &FOO }
//     }
// }

// fn main() {

// }


// use std::sync::Arc;
// // 三方库 thread-local 它允许每个线程持有值的独立拷贝
// use std::{cell, thread};
// use std::cell::Cell;
// use thread_local::ThreadLocal;

// fn main() {
//     let tls = Arc::new(ThreadLocal::new());
//     let mut v = vec![];
//     // 创建多个线程
//     for _ in 0..5 {
//         let tls2 = tls.clone();
//         let handle = thread::spawn(move || {
//         // 将计数器加1
//         // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
//         // 这只能在线程退出后发生，因此不会导致任何竞争条件
//         let cell = tls2.get_or(|| Cell::new(0));
//         cell.set(cell.get() + 1);
//         });
//         v.push(handle);
//     }
//     for handle in v {
//         handle.join().unwrap();
//     }

//     // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
//     let tls = Arc::try_unwrap(tls).unwrap();
//     let total = tls.into_iter().fold(0, |x, y| {
//     // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
//     // 因为一些线程已退出，并且其他线程会回收退出线程的对象
//     println!("x: {}, y: {}", x, y.get());
//     x + y.get()
//     });

//     assert_eq!(total, 5);
// }


// 线程间的消息传递
// 多发送者，单接收者
// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     // 创建一个消息通道，返回一个元组
//     let (tx, rx) = mpsc::channel();

//     // 创建线程并发送消息
//     thread::spawn(move || {
//         // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
//         tx.send(1).unwrap();
//         // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
//         // tx.send(Some(1)).unwrap()
//     });

//     // 在主线程中接收子线程发送的消息并输出
//     println!("revice: {}", rx.recv().unwrap());
// }

// // 不阻塞的 try_recv 方法
// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let(tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         tx.send(1).unwrap();
//     });
//     println!("revice: {}", rx.try_recv().unwrap());
// }
// 传输具有所有权的数据

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let(tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         let str = String::from("我走了");
//         tx.send(str).unwrap();
//         println!("the value is: {:?}", str);
//     });
//     let reviced = rx.recv().unwrap();
//     println!("Got: {:?}", reviced);
// }

// 使用 for 进行循环接收
// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hello"),
//             String::from("you"),
//             String::from("me"),
//             String::from("him"),
//         ];
//         tx.send(vals).unwrap();
//     });
//     let reviced = rx.recv().unwrap();
//     for rec in reviced {
//         println!("reviced: {:?}", rec);
//     }
// }

// 使用多发送者
// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         tx.send(String::from("hi from raw tx")).unwrap();
//     });

//     thread::spawn(move || {
//         tx1.send(String::from("hi from cloned tx")).unwrap();
//     });

//     for recived in rx {
//         println!("Got: {}", recived);
//     }
// }

// 同步和异步通道
// 异步通道
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx)= mpsc::channel();

//     let handle = thread::spawn(move || {
//         println!("发送之前");
//         tx.send(1).unwrap();
//         println!("发送之后");
//     });

//     println!("睡眠之前");
//     thread::sleep(Duration::from_secs(3));
//     println!("睡眠之后");

//     println!("revice: {}", rx.recv().unwrap());
//     handle.join().unwrap();

    
// }

// // 同步通道
// fn main() {
//     let (tx, rx)= mpsc::sync_channel(0);

//     let handle = thread::spawn(move || {
//         println!("发送之前");
//         tx.send(1).unwrap();
//         println!("发送之后");
//     });

//     println!("睡眠之前");
//     thread::sleep(Duration::from_secs(3));
//     println!("睡眠之后");

//     println!("revice: {}", rx.recv().unwrap());
//     handle.join().unwrap();

    
// }

// 传输多种类型的数据

use std::sync::mpsc;
enum Fruit {
    Apple(u8),
    Orange(String)
}
fn main() {
    let (tx, rx) = mpsc::channel();
    tx.send(Fruit::Orange("Sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(3)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("recived {} apples", count),
            Fruit::Orange(flavor) => println!("revice {} oranges", flavor),
        }
    }
}