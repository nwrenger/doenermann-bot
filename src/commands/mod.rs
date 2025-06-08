use csv::{Reader, Writer};

use crate::{error::Error, BIRTHDAYS_PATH};

pub mod count;
pub mod delete_birthday;
pub mod doener;
pub mod next_birthdays;
pub mod set_birthday;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct BirthdayRow {
    birthday: String,
    user: u64,
}

/// Load all birthday rows from `path`.
pub fn load_birthdays() -> Result<Vec<BirthdayRow>, Error> {
    let mut rdr = Reader::from_path(BIRTHDAYS_PATH).map_err(|_| Error::ReadCSV)?;
    // Deserialize each row, falling back to default on malformed lines
    let rows = rdr.deserialize().map(|r| r.unwrap_or_default()).collect();
    Ok(rows)
}

/// Overwrite `path` with exactly the rows in `rows`.
pub fn save_birthdays(rows: &[BirthdayRow]) -> Result<(), Error> {
    let mut wtr = Writer::from_path(BIRTHDAYS_PATH).map_err(|_| Error::WriteCSV)?;
    for row in rows {
        wtr.serialize(row)
            .map_err(|e| Error::OtherCSV(e.to_string()))?;
    }
    wtr.flush().map_err(|e| Error::OtherCSV(e.to_string()))?;
    Ok(())
}
