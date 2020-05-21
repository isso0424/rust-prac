fn main() {
    println!("please input x");
    let x = &read_int();
    println!("x: {}", *x);

    println!("please input y");
    let y = &read_int();
    println!("y: {}", *y);

    let result = calc(x, y);
    println!("result: {}", result);
}

fn read_int() -> i32 {
    let mut inputted = String::new();
    std::io::stdin().read_line(&mut inputted).ok();
    inputted.trim().parse().ok().unwrap()
}

fn read_sign() -> std::string::String {
    let signs: Vec<std::string::String> = ["+", "-", "*", "/"]
        .iter()
        .map(|x| String::from(*x))
        .collect();

    loop {
        println!("please input sign");
        let mut inputted = String::new();
        std::io::stdin().read_line(&mut inputted).ok();
        inputted = inputted.trim().to_string();
        if signs.contains(&inputted) {
            return inputted;
        }
        println!("invalid sign: {}", inputted);
    }
}

fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}

fn sub(x: &i32, y: &i32) -> i32 {
    *x - *y
}

fn multi(x: &i32, y: &i32) -> i32 {
    *x * *y
}

fn div(x: &i32, y: &i32) -> i32 {
    *x / *y
}

fn calc(x: &i32, y: &i32) -> i32 {
    let sign = read_sign();
    match sign.as_str() {
        "+" => add(x, y),
        "-" => sub(x, y),
        "*" => multi(x, y),
        "/" => div(x, y),
        _ => 0,
    }
}
