use std::collections::BTreeSet as Set;
use std::collections::BTreeMap as Map;
use crate::domain::VoteOutcome::{AcceptedVote, BlankVote};

#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub struct Voter(pub String);
#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub struct Candidate(pub String);
#[derive(Debug)]
pub struct Score(pub usize);
#[derive(Debug)]
pub struct AttendenceSheet(pub Set<Voter>);

#[derive(Debug)]
pub struct Scoreboard{
    pub scores : Map<Candidate,Score>,
    pub blank_score : Score,
    pub invalid_score : Score,
}
#[derive(Debug, Clone)]
pub struct BallotPaper{
    pub voter : Voter,
    pub candidate : Option<Candidate>,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum VoteOutcome{
    AcceptedVote(Voter,Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

pub struct VotingMachine{
    voters : AttendenceSheet,
    scoreboard : Scoreboard,
}

impl Scoreboard{
    pub fn new(candidates: Vec<Candidate>) -> Self{
        let mut scores = Map::new();
        for candidate in candidates {
            scores.insert(candidate, Score(0));
        }

        let blank_score = Score(0);
        let invalid_score = Score(0);

        Scoreboard {
            scores,
            blank_score,
            invalid_score,
        }
    }
}

impl VotingMachine{
    pub fn new(voter: Vec<Voter>,  candidates: Vec<Candidate>) -> Self{
        let mut voters : Set<Voter> = Set::new();

        for voter_user in voter {
            voters.insert(voter_user);
        }
        let mut voters = AttendenceSheet(voters);

        VotingMachine {
            voters,
            scoreboard : Scoreboard::new(candidates)
        }
    }
    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome{

        if self.voters.0.contains(&ballot_paper.voter){
            println!("Le votant {:?} a déjà voté", ballot_paper.voter);
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
        }

        match ballot_paper.candidate {
            Some(candidate) => {
                if self.scoreboard.scores.contains_key(&candidate) {
                    self.scoreboard.scores.insert(candidate.clone(), Score(1));
                    self.voters.0.insert(ballot_paper.voter.clone());
                    println!("Le votant {:?} a voté {:?}", ballot_paper.voter, candidate);
                    return AcceptedVote(ballot_paper.voter, candidate);
                }
                else {
                    self.scoreboard.invalid_score.0 = self.scoreboard.invalid_score.0 + 1;
                    println!("Une erreur est survenue pendant l'envoie du vote");
                    return VoteOutcome::InvalidVote(ballot_paper.voter);
                }
            }
            None => {
                self.voters.0.insert(ballot_paper.voter.clone());
                self.scoreboard.blank_score.0 = self.scoreboard.blank_score.0 + 1;
                    println!("Le votant {:?} a voté blanc", ballot_paper.voter);
                return BlankVote(ballot_paper.voter);
            }
            _ =>{
                self.scoreboard.invalid_score.0 = self.scoreboard.invalid_score.0 + 1;
                println!("Une erreur est survenue pendant l'envoie du vote");
                return VoteOutcome::InvalidVote(ballot_paper.voter);
            }
        }

    }
    pub fn get_scoreboard(&mut self) -> &Scoreboard{
        return &self.scoreboard;
    }

    pub fn get_voters(&mut self) -> &AttendenceSheet{
        return &self.voters;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet as Set;
    use crate::domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine};

    #[test]
    fn itWorks(){
        assert_eq!(1+1,2);
    }

    #[test]
    fn test1(){
        let voterSet = Vec::new();
        let mut votingMachine = VotingMachine::new(voterSet, vec![Candidate("matia".parse().unwrap())]);
        let ballotPaper = BallotPaper{
            voter: Voter("Lucas".to_string()),
            candidate: Option::from(Candidate("matia".to_string())),
        };
        assert_eq!(votingMachine.vote(ballotPaper.clone()), VoteOutcome::AcceptedVote(ballotPaper.voter.clone(), Candidate("matia".parse().unwrap())));

    }
    #[test]
    fn test2(){
        let mut voterSet = Vec::new();
        voterSet.insert(0,Voter("Lucas".parse().unwrap()));
        let mut votingMachine = VotingMachine::new(voterSet, vec![Candidate("matia".parse().unwrap())]);
        let ballotPaper = BallotPaper{
            voter: Voter("Lucas".to_string()),
            candidate: Option::from(Candidate("matia".to_string())),
        };
        assert_eq!(votingMachine.vote(ballotPaper.clone()), VoteOutcome::HasAlreadyVoted(ballotPaper.voter.clone()));
    }
    #[test]
    fn test3(){
        let voterSet = Vec::new();
        let mut votingMachine = VotingMachine::new(voterSet, vec![Candidate("matia".parse().unwrap())]);
        let ballotPaper = BallotPaper{
            voter: Voter("Lucas".to_string()),
            candidate: None,
        };
        assert_eq!(votingMachine.vote(ballotPaper.clone()), VoteOutcome::BlankVote(ballotPaper.voter.clone()));
    }
    #[test]
    fn test4(){
        let voterSet = Vec::new();
        let mut votingMachine = VotingMachine::new(voterSet, vec![Candidate("matia".parse().unwrap())]);
        let ballotPaper = BallotPaper{
            voter: Voter("Lucas".to_string()),
            candidate: Option::from(Candidate("bibi".to_string())),
        };
        assert_eq!(votingMachine.vote(ballotPaper.clone()), VoteOutcome::InvalidVote(ballotPaper.voter.clone()));
    }
}