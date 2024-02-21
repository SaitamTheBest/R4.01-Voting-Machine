use tokio::sync::RwLock;
use crate::domain::VotingMachine;
struct MemoryStore {
    machine: RwLock<VotingMachine>,
}

trait Storage {
    fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>;
    fn save_voting_machine(&self, machine: VotingMachine) -> anyhow::Result<()>;
}

impl MemoryStore {
    pub fn new(machine: VotingMachine) -> Self {
        MemoryStore { machine: RwLock::new(machine) }
    }
}