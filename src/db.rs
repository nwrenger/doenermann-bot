use std::sync::Arc;

use chrono::{DateTime, NaiveDate, Utc};
use light_magic::{
    atomic::{AtomicDatabase, DataStore},
    table::{PrimaryKey, Table},
};
use serde::{Deserialize, Serialize};
use serenity::all::prelude::TypeMapKey;
use url::Url;

use crate::api::RadomCharacterResponse;

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
    pub characters: Table<Character>,
}

impl PrimaryKey for Collection {
    type PrimaryKeyType = u64;

    fn primary_key(&self) -> &Self::PrimaryKeyType {
        &self.user_id
    }
}

impl Collection {
    pub fn new(user_id: u64) -> Self {
        Self {
            user_id,
            characters: Table::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub mal_id: u32,
    pub name: String,
    pub goon_credits: u32,
    pub image: Url,
}

impl PrimaryKey for Character {
    type PrimaryKeyType = u32;

    fn primary_key(&self) -> &Self::PrimaryKeyType {
        &self.mal_id
    }
}

impl From<RadomCharacterResponse> for Character {
    fn from(r: RadomCharacterResponse) -> Self {
        Self {
            mal_id: r.data.mal_id,
            name: r.data.name,
            goon_credits: r.data.favorites,
            image: r.data.images.webp.image_url,
        }
    }
}

impl Character {
    pub fn from_payload(payload: &str) -> Option<Self> {
        let (mal_id, payload) = payload.split_once(',')?;
        let (payload, image) = payload.rsplit_once(',')?;
        let (name, goon_credits) = payload.rsplit_once(',')?;

        Some(Self {
            mal_id: mal_id.parse().ok()?,
            name: name.to_string(),
            goon_credits: goon_credits.parse().ok()?,
            image: Url::parse(image).ok()?,
        })
    }
}
