use std::{path::Path, sync::Arc};

use chrono::{DateTime, NaiveDate, Utc};
use light_magic::{
    atomic::{AtomicDatabase, DataStore},
    table::{PrimaryKey, Table},
};
use serde::{Deserialize, Serialize};
use serenity::all::prelude::TypeMapKey;

use crate::config::Config;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Database {
    pub birthdays: Table<Birthday>,
    pub citations: Table<Message>,
    pub config: Config,
}

impl light_magic::atomic::DataStore for Database {}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Birthday {
    pub user: User,
    pub date: NaiveDate,
}

impl PrimaryKey for Birthday {
    type PrimaryKeyType = u64;

    fn primary_key(&self) -> &Self::PrimaryKeyType {
        &self.user.id
    }
}

impl Birthday {
    pub fn new(user: User, date: NaiveDate) -> Self {
        Self { user, date }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: u64,
    pub user: User,
    pub content: String,
    pub utc_timestamp: DateTime<Utc>,
}

impl PrimaryKey for Message {
    type PrimaryKeyType = u64;

    fn primary_key(&self) -> &Self::PrimaryKeyType {
        &self.id
    }
}

impl Message {
    pub fn new(id: u64, user: User, content: String, utc_timestamp: DateTime<Utc>) -> Self {
        Self {
            id,
            user,
            content,
            utc_timestamp,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
}

impl User {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}

pub struct SerenityDatabase {
    pub inner: Arc<AtomicDatabase<Database>>,
}

impl SerenityDatabase {
    pub fn open<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            inner: Arc::new(DataStore::open(path)),
        }
    }
}

impl TypeMapKey for SerenityDatabase {
    type Value = SerenityDatabase;
}
