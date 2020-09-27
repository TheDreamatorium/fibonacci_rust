use std::io;

fn main() {
    let mut nth: String = String::new();
    let mut restart: String = String::new();

    loop {
        if nth.len() > 0 {
            nth.clear();
        }

        println!("Please enter the nth fibonacci number you want to display");
        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read input");

        clear_screen();

        // the number of recursive iterations
        let iterations = (nth.trim().parse::<u32>().expect("Not a valid integer")) - 1;

        println!(
            "The {}th number from the sequence is {}.",
            nth.trim(),
            find_fibonacci(iterations)
        );

        loop {
            restart.clear();
            println!("Do you want to continue ? (y/n)");
            io::stdin().read_line(&mut restart).expect("Input Error");

            if restart.trim().to_uppercase() == "Y" {
                clear_screen();
                break;
            } else if restart.trim().to_uppercase() == "N" {
                println!("Bye! :-)");
                std::process::exit(0);
            } else {
                println!("Please choose Y/N");
            }
        }
    }

    // recursively find the nth (n + 1) number from the fibonacci sequence
    fn find_fibonacci(n: u32) -> u32 {
        if n == 1 || n == 0 {
            return n;
        }

        return find_fibonacci(n - 1) + find_fibonacci(n - 2);
    }

    fn clear_screen() {
        std::process::Command::new("clear").status().unwrap();
    }
}
