use std::io;

pub fn read_num() -> u32 {
    loop {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        match response.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("! Please enter a number !");
            }
        };
    }
}
