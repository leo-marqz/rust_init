
fn main()
{
    let mut names: Vec<String> = Vec::new();
    let numbers:[i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    
    for i in 0..5
    {
        
        println!("Ingresa tu nombre: ");
        let mut name: String = String::new();
        std::io::stdin()
            .read_line(&mut name)
            .unwrap();
        names.push(name.trim().to_string());
    }

    println!("{:?}", names);
    println!("First Name: {}", names[0]);
}