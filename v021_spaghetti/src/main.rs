mod configuration;
mod app_builder;

use std::io::{self, BufRead};
use std::collections::{BTreeMap as Map, BTreeMap, BTreeSet};
use clap::Parser;
use crate::app_builder::run_app;
use crate::configuration::Configuration;

fn main() {
    let configuration = Configuration::parse();
    run_app(configuration);
}
