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
    println!("{:#?}",os)
}
