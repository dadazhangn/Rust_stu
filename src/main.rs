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