use std::io;
use rand::Rng;

fn main() {
    let pilihan_user = user_input();
    println!("Test {}", pilihan_user);
}

fn user_input() -> u32{    
    let value;     
    loop{               
        println!("Input pilihan anda! Kertas[0], Gunting[1], Batu[2]");
        let mut user_input = String::new();   
        io::stdin()
            .read_line(&mut user_input)
            .expect("Gagal membaca input!");    
                
        let converted_input:u32 = match user_input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Input tidak valid!");
                continue
            },
        };

        if converted_input < 3{
            value = converted_input;
            break;
        } else {
            println!("Mohon input angka antara 1-3!");
            continue;
        }
    }
    return value;
}

fn computer_input() -> u32{
    let mut random_number = rand::thread_rng().gen_range(0..=2);    
    return random_number;
}