fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        match line {
            Ok(line_content) => {
                println!("{}", line_content);
            }
            Err(err) => {
                eprintln!("Erreur de lecture: {}", err);
            }
        }
    }
}
