

fn main() -> anyhow::Result<()>{
    use std::io::{self, BufRead};

    let mut votants : Vec<&str> = vec![];
    let mut scores : Vec<&str> = vec![];

    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    println!("Commandes valides :");
    println!("- \"voter Tux NixOS\" : Tux vote pour NixOS");
    println!("- \"voter Tux\" : Tux vote blanc");
    println!("- \"votants\" : affiche la liste des votants");
    println!("- \"scores\" : affiche les scores pour tous les candidats");

    for line in lines {
        match line {
            Ok(line_content) => {
                let mut vector = vec![];
                vector = line_content.as_str().split_whitespace().collect();
                match vector[0] {
                    "voter" => {
                        if vector.len()==2 {
                            votants.push(vector[1]);
                            scores.push(vector[2]);
                        }
                        else {
                            votants.push(vector[1]);
                        }
                        println!("Commande trouvé");
                    }
                    "votants" => {
                        println!("{:?}",votants);
                        println!("Commande trouvé");
                    }
                    "scores" => {
                        println!("{:?}",scores);
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
