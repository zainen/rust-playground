// const OUR_COURSE: &str = "Rust with AutoGPT";

mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;
mod m7_async;
mod m8_collections;
mod m9_decl_macros;
mod m10_proc_macro;
mod m11_smart_pointers;
mod m12_concurrency;

fn main() {
    // println!("Welcome to this course on {}!", OUR_COURSE);

    // let x: i32;
    // x = 2;
    // println!("x is {}", x);

    // let y: i32 = 4;
    // println!("y is {}", y);

    // for i in 0..=y {
    //     if i != 4 {
    //         println!("{}, ", i);
    //     } else {
    //         println!("{}, ", i);
    //     }
    // }

    // let mut z: i32 = 5;
    // println!("z was {}", z);

    // z = 10;
    // println!("but now is {}", z);

    // let freezing_temp: f64 = -2.4;
    // println!("freezing_temp is {}", freezing_temp);

    // let is_zero_remainder: bool = 10 % 4 != 0;
    // println!("is_zer_remainder is {}", is_zero_remainder);

    // let my_char: char = 'z';
    // println!("my_char is {}", my_char);

    // let emoji_char = '🥺';
    // println!("emoji_char is {}", emoji_char);

    // let my_floats: [f32; 10] = [0.0; 10];
    // println!("my_floats is {:?}", my_floats);

    // let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    // println!("my_floats_new is {:?}", my_floats_new);

    // let name: &str = "Zainen";
    // println!("name is {:?}", name);

    // let dynamic_name: String = String::from("Zainen Suzuki");
    // println!("dynamic_name is {:?}", dynamic_name);
    // println!("dynamic_name stored in memory {:p}", &dynamic_name);

    // let str_slice: &str = &dynamic_name[0..5];
    // println!("str_slice is {:?}", str_slice);

    // let mut chars: Vec<char> = Vec::new();
    // chars.insert(0, 'h');
    // chars.insert(1, 'e');
    // chars.insert(2, 'l');
    // chars.push('l');
    // chars.push('o');
    // chars.push('.');
    // println!("chars is {:?}", chars);
    // dbg!(&chars);
    
    // let removed_char: char = chars.pop().unwrap();
    // println!("removed_char is {:?}", removed_char);
    // println!("char is {:?}", chars);

    // chars.iter().for_each(|c| print!("{} ", c));

    // let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    // dbg!(&chars_again);

    // let collected: String = chars_again.iter().collect();
    // dbg!(collected);

    // for c in chars_again {
    //     print!("{}", c);
    //     if c == 'o' {
    //         print!(" world");
    //     }
    // }

    // let num: i32 = 5;
    // let add_num = |x: i32| x + num;
    // let new_num: i32 = add_num(7);
    // dbg!(new_num);

    //  println!("Big Number is {}", 98_222_000);
    //  println!("hex is {}", 0xff);
    //  println!("octal is {}", 0o77);
    //  println!("binary is {}", 0b1111_0000);
    //  println!("bytes 'A' is {}", b'A');

    //  let text: &str = r#"{\"message is\"}"#;
    //  dbg!(text);

    // let a: u8 = 0b_1010_1010;
    // let b: u8 = 0b_0101_0101;
    // println!("a {}", a);
    // println!("b {}", b);

    // println!("a binary is {:08b}", a);
    // println!("b binary is {:08b}", b);

    // println!("AND {:08b}", a & b);
    // println!("OR {:08b}", a | b);
    // println!("XOR {:08b}", a ^ b);
    // println!("NOT {:08b}", !a);

    // println!("a << 1 {:08b}", a << 1);
    // println!("a << 1 {}", a << 1);

    // println!("a >> 1 {:08b}", a >> 1);
    // println!("a >> 1 {}", a >> 1);


    // let n: u16 = 0x1234;
    // println!("n is: {:?}", n);

    // let big_endian = n.to_be_bytes();
    // let little_endian = n.to_le_bytes();

    // println!("n in big endian {:02X} {:02X}", big_endian[0], big_endian[1]);
    // println!("n in little endian {:02X} {:02X}", little_endian[0], little_endian[1]);
}
