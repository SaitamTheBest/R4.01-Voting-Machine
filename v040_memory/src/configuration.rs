use clap::Parser;
#[derive(Parser)]
pub struct Configuration{
    #[arg(short='c', long="candidates", num_args = 1..)]
    pub(crate) candidates: Vec<String>,
}