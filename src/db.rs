use std::sync::Arc;

use chrono::{DateTime, NaiveDate, Utc};
use light_magic::{
    atomic::{AtomicDatabase, DataStore},
    table::{PrimaryKey, Table},
};
use serde::{Deserialize, Serialize};
use serenity::all::prelude::TypeMapKey;
use url::Url;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Database {
    pub birthdays: Table<Birthday>,
    pub citations: Table<Message>,
    pub collections: Table<Collection>,
}

impl DataStore for Database {}

impl TypeMapKey for Database {
    type Value = Arc<AtomicDatabase<Database>>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Birthday {
    pub user_id: u64,
    pub date: NaiveDate,
}

impl PrimaryKey for Birthday {
    type PrimaryKeyType = u64;

    fn primary_key(&self) -> &Self::PrimaryKeyType {
        &self.user_id
    }
}

impl Birthday {
    pub fn new(user_id: u64, date: NaiveDate) -> Self {
        Self { user_id, date }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub user_id: u64,
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
    pub fn new(id: u64, user_id: u64, content: String, utc_timestamp: DateTime<Utc>) -> Self {
        Self {
            id,
            user_id,
            content,
            utc_timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub user_id: u64,
    pub waifus: Table<Waifu>,
}

impl PrimaryKey for Collection {
    type PrimaryKeyType = u64;

    fn primary_key(&self) -> &Self::PrimaryKeyType {
        &self.user_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waifu {
    pub mal_id: u64,
    pub name: String,
    pub image: Url,
}

impl PrimaryKey for Waifu {
    type PrimaryKeyType = u64;

    fn primary_key(&self) -> &Self::PrimaryKeyType {
        &self.mal_id
    }
}
