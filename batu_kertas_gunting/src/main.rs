use std::io;
use rand::Rng;

fn main() {
    let pilihan_user = user_input();
    let pilihan_komputer = komputer_input();

    println!("Pilihan user: {}", formatter(pilihan_user));
    println!("Pilihan komputer: {}", formatter(pilihan_komputer));
    
    //let hasil = hasil_game(pilihan_user, pilihan_komputer);
}

fn user_input() -> u32{    
    let value;     
    loop{               
        println!("Input pilihan anda! Batu[0], Kertas[1], Gunting[2]");
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

fn komputer_input() -> u32{
    let random_number = rand::thread_rng().gen_range(0..=2);    
    return random_number;
}

fn formatter(pilihan_angka: u32) -> String{
    let hasil = match pilihan_angka {
        0 => "Batu".to_string(),
        1 => "Kertas".to_string(),
        2 => "Gunting".to_string(),
        _ => "Tidak ada".to_string(),
    };
    return hasil;
}