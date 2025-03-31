
use serde::{Deserialize, Serialize};

pub mod cli;
pub mod functions;

#[derive(Debug, Serialize, Deserialize , Clone)]
pub struct Shortcut{
    pub name : String ,
    pub command : String,
}