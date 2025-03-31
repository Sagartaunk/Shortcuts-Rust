
use serde::{Deserialize, Serialize};

pub mod cli;
pub mod functions;

#[derive(Serialize , Deserialize , Debug , Clone)]
pub struct Shortcut{
    pub name : String ,
    pub command : String,
}