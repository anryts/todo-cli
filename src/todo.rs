use std::{fmt, str::FromStr};

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: Status 
}

impl Todo {
    pub fn new (id: i32,
            title: String,
            description: String) -> Todo {
        return Todo {
            id,
            title,
            description,
            status: Status::Todo
        };
    }
}

#[derive(Debug, Clone, Parser, ValueEnum, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Todo = 0,
    Done = 1,
    InProgress = -1
}

impl FromStr for Status {
    type  Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "t" | "todo" => Ok(Status::Todo),
            "1" | "d" | "done" => Ok(Status::Done),
            "-1" | "p" | "in progress" => Ok(Status::InProgress),
            _ => Err(())
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Status::Todo => write!(f, "todo"),
            Status::Done => write!(f, "done"),
            Status::InProgress => write!(f, "in progress")
        }
    }
}