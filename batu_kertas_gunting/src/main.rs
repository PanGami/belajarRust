use std::io;
use rand::Rng;

fn main() {
    let pilihan_user = user_input();
    // processing(pilihan_user);
}

fn processing(user_input: char){
    print!("{}", user_input);
}

fn user_input(){    
    let mut user_input = String::new();    
    loop{       
        println!("Masukkan huruf awal pilihan! ");
        println!("Kertas : k, Gunting : g, Batu : b");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Gagal membaca input!");    
                
        // println!("TEST CONVERTED INPUT ! {}", converted_input);
        // println!("TEST USER INPUT ! {}", user_input);
        let mut converted_input = user_input.to_lowercase().chars().next().unwrap();
        match converted_input {
            'b' =>{
                println!("Anda memilih batu!");
                break;
            },
            'k' =>{
                println!("Anda memilih kertas!");
                break;
            },
            'g' =>{
                println!("Anda memilih gunting!");
                break;
            },
            _ =>{
                println!("Input tidak valid!");
                continue;
            }
        }                
        user_input;     
        break;   
    }
}