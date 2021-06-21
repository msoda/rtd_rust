use std::io;
use rand::Rng;

fn main() {
    loop {
        println!("--Input--");
        println!("Enter dice type to roll:");

        let mut d_type: String = String::new();
        io::stdin().read_line(&mut d_type)
            .expect("Failed to read line");

        let d_type: u32 = match d_type.trim().parse() {
            Ok(typ) => typ,
            Err(_) => {
                println!("Must be a number.");
                continue;
            },
        };

        println!("Enter amount of dice to roll:");

        let mut d_amt: String = String::new();
        io::stdin().read_line(&mut d_amt)
            .expect("Failed to read line");

        let d_amt: u32 = match d_amt.trim().parse() {
            Ok(amt) => amt,
            Err(_) => {
                println!("Must be a number.");
                continue;
            },
        };

        let mut vec: Vec<u32> = Vec::new();

        for _i in 0..d_amt {
            let r_num: u32 = rand::thread_rng()
                .gen_range(1, d_type+1);

            vec.push(r_num);
        }

        println!("\n--Output--");

        for r in 0..vec.len() {
            println!("Roll #{}: {}", r+1, vec[r]);
        }
        break;
    }
}
