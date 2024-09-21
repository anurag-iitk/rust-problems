fn main() {
    let mut tup = (500, 500, "hi", false);

    println!("{}", tup.0);

    tup.0 = 200;

    println!("{}", tup.0);

    let (_a, _b, c, _d) = tup;

    println!("{}", c)
}
