fn main() -> anyhow::Result<()>{
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    for line in lines {
        match line {
            Ok(line_content) => {
                match line_content.as_str() {
                    "voter" => {
                        println!("Commande trouvé");
                    }
                    "votants" => {
                        println!("Commande trouvé");
                    }
                    "scores" => {
                        println!("Commande trouvé");
                    }
                    _ => {
                        println!("Commande inconnue");
                    }

                }
            }
            Err(err) => {
                eprintln!("Erreur de lecture: {}", err);
            }
        }
    }
    Ok(())
}
