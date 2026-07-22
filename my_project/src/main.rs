fn main() {
    println!("Hello, world!");
    //  variables in rust
    let x = 21;
    let mut m = 55;
    println!("{}",m);
    m = 32;
        // shadoing
    let shadow = 55;
    let shadow = shadow - 40;
    let shadow = shadow + 100;

    println!("{}",x);
    println!("{}",m);
    println!("{}",shadow);

    // Datatypes
    let a: i8 = 10;
    let b: i16 = 100;
    let c: i32 = 1000;
    let d: i64 = 10000;
    let e: i128 = 100000;
    let u: u32 = 100;
    let f: f32 = 3.14;
    let g: f64 = 3.14159;
    let t: bool = true;
    let ch: char = 'A';
    let st: &str = "Richard Fyenman";
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{}",d);
    println!("{}",e);
    println!("{}",f);
    println!("{}",g);
    println!("{}",t);
    println!("{}",ch);
    println!("{}",st);

    const PI: f32 = 3.14159;

    // type casting
    let x = 121;
    let y = x as f64;
    let z = x as str;
    println!({},[x,y,z]);
}
