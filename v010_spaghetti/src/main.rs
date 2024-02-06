use std::io::{self, BufRead};
use std::collections::{BTreeMap as Map, BTreeMap, BTreeSet};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock().lines();
    let mut list_votants : BTreeSet<String> = BTreeSet::new();
    let mut scores:BTreeMap<String,i32> = Map::new();

    for line in handle {
        match line {
            Ok(text) => {
                if text.is_empty() {
                    println!("- \"voter Tux NixOS\" : Tux vote pour NixOS");
                    println!("- \"voter Tux\" : Tux vote blanc");
                    println!("- \"votants\" : affiche la liste des votants");
                    println!("- \"scores\" : affiche les scores pour tous les candidats");
                    continue;
                }

                let command: Vec<&str> = text.split_whitespace().collect();

                match command.as_slice() {
                    ["voter", votant] => {
                        let mut deja_vote = false;
                        for votants in &list_votants{
                            if *votants == votant.to_string(){
                                deja_vote = true;
                            }
                        }
                        if deja_vote {
                            println!("Cet utilisateur a déjà voter !")
                        }
                        else {
                            list_votants.insert(votant.to_string());
                            println!("{} a voté blanc", votant);
                        }
                    }
                    ["voter", votant, candidat_votant] => {
                        let mut deja_vote = false;
                        for votants in &list_votants{
                            if *votants == votant.to_string(){
                                deja_vote = true;
                            }
                        }
                        if deja_vote {
                            println!("Cet utilisateur a déjà voter !")
                        }
                        else {
                            list_votants.insert(votant.to_string());
                            let score = scores.entry(candidat_votant.to_string()).or_insert(0);
                            *score += 1;
                            println!("{} a voté pour {}.", votant, candidat_votant);
                        }
                    }
                    ["votants"] => {
                        for votant in &list_votants {
                            println!("{}", votant);
                        }
                    }
                    ["scores"] => {
                        if scores.is_empty(){
                            println!("Aucune donnée entrée !")
                        }
                        else {
                            for (candidat, score) in &scores {
                                println!("{} : {}", candidat, score);
                            }
                        }
                    }
                    _ => {
                        eprintln!("Commande inconnue : {}", text);
                    }
                }
            }
            Err(error) => {
                eprintln!("Erreur de lecture : {}", error);
            }
        }
    }
}
