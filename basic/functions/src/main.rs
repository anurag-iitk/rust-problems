fn main() {
    print("Hello");
    println!("{}", check_boolean(false));
}

fn print(name: &str){
    println!("{}", name)
}

fn check_boolean(flag: bool) -> bool {
    if flag == true{
        true
    } else {
        false
    }
}
