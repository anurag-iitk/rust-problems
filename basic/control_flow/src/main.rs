fn main() {
    let one = 1;

    // if else

    if one > 10 {
        println!("Greater")
    } else {
        println!("Lower");
    }


    // loop

    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;

        loop{
            println!("Decrease: {}", decrease);
            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }

    // while loop

    let mut num = 0;

    while num < 5 {
        println!("{}", num);
        num += 1;
    }

    // for loop

    let vec: Vec<i8> = (6..10).collect();

    for ele in vec {
        println!("{}", ele);
    }

    for num in (20..25).rev() {
        println!("{}", num)
    }
}
