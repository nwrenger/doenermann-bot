use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::CreateEmbed;

use crate::db::Database;
use crate::error::Result;
use crate::util::ResponseContent;

pub fn run(_db: Arc<AtomicDatabase<Database>>, _user_id: u64) -> Result<ResponseContent> {
    Ok(ResponseContent::new_only_embed(CreateEmbed::new()))
}
