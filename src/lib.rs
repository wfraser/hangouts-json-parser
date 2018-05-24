extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod raw;
pub use raw::Hangouts;

use std::collections::hash_map::*;

impl Hangouts {
    pub fn participants(&self) -> HashMap<raw::ParticipantId, raw::ParticipantData> {
        let mut map = HashMap::new();

        for convo in &self.conversations {
            for participant in &convo.header.details.participant_data {
                if let Entry::Vacant(mut entry) = map.entry(participant.id.clone()) {
                    entry.insert(participant.clone());
                }
            }
        }

        map
    }
}

impl raw::EventHeader {
    pub fn timestamp(&self) -> Result<(i64, u32), std::num::ParseIntError> {
        let usecs: u32 = self.timestamp[self.timestamp.len() - 6..].parse()?;
        let secs: i64 = self.timestamp[0..(self.timestamp.len() - 6)].parse()?;
        Ok((secs, usecs * 1_000))
    }
}
