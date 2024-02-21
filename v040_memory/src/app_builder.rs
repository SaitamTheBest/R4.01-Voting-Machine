use std::io;
use std::io::BufRead;
use anyhow::Result;
use clap::error::ContextValue::None;
use crate::configuration::Configuration;
use crate::domain::{BallotPaper, Candidate, Voter, VotingMachine};

pub fn run_app(configuration:Configuration) -> anyhow::Result<()>{
    let stdin = io::stdin();
    let handle = stdin.lock().lines();
    let list_v: Vec<Voter> = Vec::new();
    let mut list_c: Vec<Candidate> = Vec::new();
    list_c.insert(0,Candidate("Lucas".to_string()));
    list_c.insert(0,Candidate("Matias".to_string()));
    list_c.insert(0,Candidate("Alex".to_string()));
    list_c.insert(0,Candidate("Trump".to_string()));
    let mut voting_machine = VotingMachine::new(list_v, list_c);

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
                        let votant = Voter(votant.to_string());

                        let ballot_paper = BallotPaper{
                            voter : votant,
                            candidate : Option::None
                        };

                        voting_machine.vote(ballot_paper);
                    }
                    ["voter", votant, candidat_votant] => {
                        let votant = Voter(votant.to_string());
                        let candidat = Candidate(candidat_votant.to_string());

                        let ballot_paper = BallotPaper{
                            voter : votant,
                            candidate : Option::from(candidat)
                        };

                        voting_machine.vote(ballot_paper);
                    }
                    ["votants"] => {
                        println!("{:?}", voting_machine.get_voters());
                    }
                    ["scores"] => {
                        println!("{:?}", voting_machine.get_scoreboard());
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
    Ok(())
}