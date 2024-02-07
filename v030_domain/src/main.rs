mod configuration;
mod app_builder;
mod domain;

use clap::Parser;
use crate::app_builder::run_app;
use crate::configuration::Configuration;

fn main() {
    let configuration = Configuration::parse();
    run_app(configuration);
}
