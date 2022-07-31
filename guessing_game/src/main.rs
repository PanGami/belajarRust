use std::io;
use rand::Rng; // trait
use std::cmp::Ordering;

fn main() {
    println!("Permainan Menebak Angka!");

    let angka_rahasia = rand::thread_rng().gen_range(1..=100);
    println!("Angka rahasianya adalah:{}", angka_rahasia);

    loop{
        println!("Mohon masukkan / input angka 1-100 tebakan anda!");
        let mut tebakan = String::new();

        io::stdin()
            .read_line(&mut tebakan)
            .expect("Gagal membaca input!");
        let tebakan: u32 = match tebakan.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Tebakan anda: {}", tebakan);
    
        match tebakan.cmp(&angka_rahasia){
            Ordering::Less => println!("Tebakan anda terlalu rendah!"),
            Ordering::Greater => println!("Tebakan anda terlalu tinggi!"),
            Ordering::Equal => {
                println!("Tebakan anda benar!");
                break;
            },
        }
    }
    
}
