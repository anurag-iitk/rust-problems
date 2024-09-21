fn main() {
    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);

    println!("{:?}", vec1);

    vec1.reverse();

    println!("{:?}", vec1);

    let mut vec2 = Vec::new();

    vec2.push("Hello");
    vec2.push("Anurag");

    println!("{:?}", vec2);

    let mut vec3 = Vec::<i32>::with_capacity(3);

    println!("{:?}", vec3.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

}
