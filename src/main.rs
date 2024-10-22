fn main() {
    const B: i32 = 123;
    let a = 12;
    println!("a is {}, b is {}", a, add(a, B));
    let number = 7;
    if number < 5 {
        println!("小于5");
    } else {
        println!("大于5");
    }

    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10{
            break;
        }
    }
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

