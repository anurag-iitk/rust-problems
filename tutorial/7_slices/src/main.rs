fn main() {
    let vec1: Vec<i32> = (0..5).collect();

    let sv: &[i32] = &vec1;

    println!("{:?}", &sv[1..3]);
    println!("{:?}", &sv[..3]);
    println!("{:?}", &sv[1..]);
    println!("{:?}", &sv[..]);
}
