use std::{env::consts::OS, ffi::OsString, os::windows};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum OS {
    Windows,
    Mac,
    Linux(String),
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    let x = 50;
    if x == 50 {
        println!("{}", x);
        x.to_string();
    } else {
        println!("no");
        "no".to_string();
    }
    println!("{}", x);
    let point = Point { x: 32, y: 32 };
    println!("{:#?}", point);
    let os = OS::Linux("Ubuntu".to_string());
    println!("{:#?}", os);
    let mut t = (0, 1, 2);
    t.0 = 10;
    const N: usize = 1;
    let i = 9;
    let a = [i; N];
    let a_0 = a[0];
    a_0 == a[0];
    let b = a_0;
    println!("{},{}", a_0, b);
    let a2 = (0..100).collect::<Vec<i32>>();
    let arr2: [i32; 5] = [0; 5].map(|x| x + 1);
    let [arr0, arr1, arr2, arr3, arr4] = arr2;
    println!(
        "{:?}
    {:?}",
        a2, arr2
    );
    let mut v = vec![OS::Windows, OS::Mac, OS::Mac];
    let v0 = &v[0];
    println!("{:#?}", v.pop());
    println!("{:#?}", v[0]);
    let mut vd = std::collections::VecDeque::new();
    vd.push_back(0);
    vd.push_back(1);
    let s = "abcde".to_string().as_bytes();
    let vs = &v[..];
    let vs2: &mut [OS] = &mut v;
    vs2[0] = OS::Windows;
    println!("vs2:{:#?}", vs2);
    let win: OS = OS::Windows;
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{:?}", home);
    struct IPv4(u8, u8, u8, u8);
    let ipaddress = IPv4(192, 168, 0, 0);
    let hoge = OsString::from("from &str");
    let mut vt = Vec::new();
    vt.push(0u8);
    let i = 0i64;
    let ip = &i as *const i64;
    let ll = ip as u64;
    let llp = ll as *const u64;
    let ll2 = llp as i32;
    println!("{}", ll2);
    let p = 0xff00 as *const u64;
    unsafe {
        // println!("{}",*p)
    }
    let a_ = [0; 3];
    let b = a_;
    println!("{:?}", a_);
    let vv = [0, 1, 2];
    let vv2 = vv;
    vv;
    panic!("hogehoge{}", i);
}
