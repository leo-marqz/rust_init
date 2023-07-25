use regex::Regex;

fn main() {
    println!("Hi, Welcome to my Calculator.");
    println!("My name is 'Leo' and I`m programming in Rust.");

    //Regex
    // (\d+)\s?\+\s?(\d+)  '2+3' or '2 + 3'= true
    let reg_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap(); //multiplication
    let reg_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap(); //divition
    let reg_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap(); //addition
    let reg_subt = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap(); //subtract

    //User Data
    println!("Por favor introduce tu expresion: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    //Apply Operations
    //multiplication
    println!("Resultado[init]: {}", expression);
    loop {
        let capts = reg_mult.captures(expression.as_str());
        if capts.is_none() {
            break;
        }

        let capts = capts.unwrap();
        let capt_expression = capts.get(0).unwrap().as_str();
        let left_value:i32 = capts.get(1).unwrap().as_str().parse().unwrap();
        let right_value:i32 = capts.get(2).unwrap().as_str().parse().unwrap();

        let multiplication:i32 = left_value * right_value;

        expression = expression.replace(capt_expression, &multiplication.to_string());

    }

    println!("Resultado[mult]: {}", expression);
    //divition
    loop {
        let capts = reg_div.captures(expression.as_str());
        if capts.is_none() {
            break;
        }
        let capts = capts.unwrap();
        let capt_expression = capts.get(0).unwrap().as_str();
        let left_value:i32 = capts.get(1).unwrap().as_str().parse().unwrap();
        let right_value:i32 = capts.get(2).unwrap().as_str().parse().unwrap();

        let divition:i32 = left_value / right_value;

        println!("div: {}", divition);

        expression = expression.replace(capt_expression, &divition.to_string());

    }

    println!("Resultado[div]: {}", expression);
    //addition
    loop {
        let capts = reg_add.captures(expression.as_str());
        if capts.is_none() {
            break;
        }

        let capts = capts.unwrap();
        let capt_expression = capts.get(0).unwrap().as_str();
        let left_value:i32 = capts.get(1).unwrap().as_str().parse().unwrap();
        let right_value:i32 = capts.get(2).unwrap().as_str().parse().unwrap();

        let addition:i32 = left_value + right_value;

        expression = expression.replace(capt_expression, &addition.to_string());

    }

    println!("Resultado[addition]: {}", expression);
    //subtract
    loop {
        let capts = reg_subt.captures(expression.as_str());
        if capts.is_none() {
            break;
        }

        let capts = capts.unwrap();
        let capt_expression = capts.get(0).unwrap().as_str();
        let left_value:i32 = capts.get(1).unwrap().as_str().parse().unwrap();
        let right_value:i32 = capts.get(2).unwrap().as_str().parse().unwrap();

        let subtract:i32 = left_value - right_value;

        expression = expression.replace(capt_expression, &subtract.to_string());

    }
    println!("Resultado[sub]: {}", expression);

    //Show Results
    println!("Resultado[final]: {}", expression);
}

