use std::io;

fn main() {
    println!("Calculator");

    loop{
        println!("Mohon inputkan nomor pertama:");
        let mut operand1 = String::new();
        io::stdin()
            .read_line(&mut operand1)
            .expect("Gagal membaca input!");

        let operand1: i32 = match operand1.trim().parse() { //use signed integer so we can use negative number
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Mohon inputkan nomor kedua:");
        let mut operand2 = String::new();
        io::stdin()
            .read_line(&mut operand2)
            .expect("Gagal membaca input!");

        let operand2: i32 = match operand2.trim().parse() { //use signed integer so we can use negative number
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Mohon inputkan operasi (ex +,-,*,etc) :");
        let mut operator = String::new();
        io::stdin()
            .read_line(&mut operator)
            .expect("Gagal membaca input!");

        let operator: char = match operator.trim().parse() { //use signed integer so we can use negative number
            Ok(num) => num,
            Err(_) => continue,
        };
        processing(operator, operand1, operand2);
        break;
    }    
}

//method processing calculation calculator
fn processing(operator: char, operand1: i32, operand2: i32){ //use signed integer so we can use negative number
    match operator {
        '+' => println!("{}",operand1 + operand2),
        '-' => println!("{}",operand1 - operand2),
        '*' => println!("{}",operand1 * operand2),
        '/' => {
            if operand2 == 0 {
                println!("Tidak bisa dibagi dengan nol!");
            } else {
                println!("{}",operand1 / operand2);
            }            
        },
        '%' => {
            if operand2 == 0 {
                println!("Tidak bisa dibagi dengan nol!");
            } else {
                println!("{}",operand1 % operand2);
            }            
        },
        _ => println!("Operator {} tidak dikenali", operator),
    }
}