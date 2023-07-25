// use std::io::stdin;

// fn main() {
    
//     println!("El siguiente programa te permitira genera un arreglo o conjunto de numero. 
//     Seras capaz de decidir desde que numero hasta que numero deseas que llegue.
//     Ademas podras elegir si quieres que incremente de 2 en 2 o lo que tu quieras. ");

//     let mut start: u64 = 0;
//     let mut start_string: String = String::new();
//     let mut end: u64 = 1;
//     let mut end_string: String = String::new();
//     let mut increment: u64 = 1;
//     let mut increment_string: String = String::new();
//     let mut numbers: Vec<u64> = Vec::new();
//     let mut current_number: u64 = 0;

//     println!("\nIngresa el numero inicial: ");
//     stdin().read_line(&mut start_string).unwrap();

//     println!("Ingresa el numero final: ");
//     stdin().read_line(&mut end_string).unwrap();
        
//     println!("Ingrese el numero de incremento: ");
//     stdin().read_line(&mut increment_string).unwrap();

//     start = start_string.trim().parse().unwrap();
//     end = end_string.trim().parse().unwrap();
//     increment = increment_string.trim().parse().unwrap();
//     current_number = start;

//     println!("Start: {}, End: {}, Increment: {}, Current Number: {}", start, end, increment, current_number);

//     loop {
//         if current_number > end {
//             break;
//         }
//         numbers.push(current_number);
//         current_number = current_number + increment;
//     }

//     println!("Tu conjunto de numeros: {:?}", numbers);

    
// }

// // fn capitalize(mut value: String) -> String {
// //     if let Some(first_letter) = value.chars().next() {
// //         let _ = value.remove(0);
// //         value.insert(0, first_letter.to_ascii_uppercase());
// //     }
// //     return value;
// // }
