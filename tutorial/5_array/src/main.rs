fn main() {
    // array must have same type

    let arr = [1, 2, 3];

    println!("{}", arr[0]);

    let mut arr2: [i32; 3] = [100, 200, 300];

    println!("{}", arr2[0]);

    arr2[0] = 500;

    println!("{}", arr2[0]);
    println!("{:?}", &arr2[..2]);
    println!("{:#?}", &arr2[1..]);
    println!("{:?}", &arr2[1..2]);
    println!("{:?}", &arr2[..]);
    println!("{:?}", arr2);
    println!("{:?}", &arr2);
    println!("{}", arr2[arr2.len()-1]);
}
