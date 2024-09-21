fn main() {
    let x: i8 = 10;

    println!("{}", x);

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';
    println!("{}", byte);

    let y = 2.0; // f64 default
    let z: f32 = 1.0;

    println!("{}", y);
    println!("{}", z);

    let t = true;
    let _f: bool = false;

    println!("{}", t);
    // println!("{}", f);

    let c = 'c';
    println!("{}", c);


    // +, -, *, /, %
    let a = 10;
    let b = 7;

    let remainder = a%b;
    println!("{}", remainder)
}
