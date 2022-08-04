use std::io;
use rand::Rng;

/*  
    =============== HERE Short Docs ==================
    Computer get choices from rand::Rng (random number)
    User input with io::stdin()        (standard input)
    ================ About Functions ==================
    Make functions to make main function more readable
    main()           : Is the core of the program and handles all functions
    user_input()     : Handle input(u32 / unsigned integer) and handle error
    computer_input() : Handle computer choices(with rand)
    formatter()      : Handle from(u32 / unsigned integer) to (String / "batu kertas gunting")
    game_process()   : Handle winner and loser
*/
fn main() {
    let pilihan_user = user_input();
    let pilihan_komputer = computer_input();

    println!("Pilihan user: {}", formatter(pilihan_user));
    println!("Pilihan komputer: {}", formatter(pilihan_komputer));    
    
    println!("{}",game_process(pilihan_user, pilihan_komputer));
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

fn computer_input() -> u32{
    let random_number = rand::thread_rng().gen_range(0..=2);    
    return random_number;
}

fn formatter(pilihan_angka: u32) -> &'static str{
    let hasil = match pilihan_angka {
        0 => "Batu",
        1 => "Kertas",
        2 => "Gunting",
        _ => "Tidak ada",
    };
    return hasil;
}

fn game_process(pilihan_user: u32, pilihan_komputer: u32) -> String{
    if pilihan_user == pilihan_komputer{
        return "Draw".to_string();
    } else if pilihan_user == 0 && pilihan_komputer == 2{
        return "Menang".to_string();
    } else if pilihan_user == 1 && pilihan_komputer == 0{
        return "Menang".to_string();
    } else if pilihan_user == 2 && pilihan_komputer == 1{
        return "Menang".to_string();
    } else {
        return "Kalah".to_string();
    }
}