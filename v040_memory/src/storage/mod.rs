pub mod memory;

use async_trait::async_trait;

use crate::domain::VotingMachine;

#[async_trait]
pub trait Storage {
    async fn getVotingMachine(&self) -> anyhow::Result<VotingMachine>;
    async fn pubVotingMachine(&self, machine: VotingMachine) -> anyhow::Result<()>;
}