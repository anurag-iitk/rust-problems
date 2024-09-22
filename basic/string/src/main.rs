fn main() {
    let name = String::from("Anurag");
    let course = "Rust".to_string();
    let new_name = name.replace("Anurag", "AS");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    let str1 = "hello"; // &str
    println!("{}", &str1);

    let str2 = str1.to_string();
    println!("{}", str2);

    let str3 = &str2;
    println!("{}", str3);

    println!("{}", str1.to_lowercase() == str2)

}
