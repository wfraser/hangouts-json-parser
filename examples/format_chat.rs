extern crate chrono;
extern crate serde_json;
extern crate hangouts_json_parser;

use std::collections::hash_map::*;
use std::fs::File;
use std::env;
use std::io::{self, BufReader};
use hangouts_json_parser::{raw, Hangouts};

fn usage() {
    eprintln!("usage: {} <json path> <participant name>", env::args().nth(0).unwrap());
}

fn chrono(parts: Result<(i64, u32), std::num::ParseIntError>) -> chrono::DateTime<chrono::Utc> {
    let (secs, nsecs) = parts.expect("bad timestamp");
    chrono::TimeZone::timestamp(&chrono::Utc, secs, nsecs)
}

fn main() -> Result<(), io::Error> {
    let path = env::args_os().nth(1).unwrap_or_else(|| {
        usage();
        std::process::exit(2);
    });

    let participant_name = env::args().nth(2).unwrap_or_else(|| {
        usage();
        std::process::exit(2);
    });

    let mut hangouts: Hangouts = serde_json::from_reader(BufReader::new(File::open(path)?))?;

    let convo = hangouts.conversations
        .iter_mut()
        .find(|convo|
            convo.header.details.participant_data.iter().any(|p|
                p.fallback_name == participant_name))
        .unwrap_or_else(|| {
            eprintln!("No matching conversation found with a person named {:?}", participant_name);
            std::process::exit(1);
        });

    let names: HashMap<raw::ParticipantId, String> = convo.header.details.participant_data
        .iter()
        .map(|p| (p.id.clone(), p.fallback_name.clone()))
        .collect();

    convo.events.sort_unstable_by_key(|event| event.header.timestamp.clone());
    for event in &convo.events {
        let dt = chrono(event.header.timestamp()).format("%Y-%m-%d %H:%M:%S");
        let name = &names[&event.header.sender_id];
        let text = match event.data {
            raw::EventData::ChatMessage { ref message_content, .. } => {
                let mut combined = String::new();

                for segment in &message_content.segments {
                    match segment {
                        raw::ChatSegment::Text { ref text, formatting: _ } => {
                            //combined += &format!("[text: {}]", text);
                            combined += text;
                        }
                        raw::ChatSegment::Link {
                            text: _,
                            ref link_data,
                            formatting: _,
                        } => {
                            combined += &format!("[link: {:?}]", link_data);
                        }
                        raw::ChatSegment::LineBreak { ref text } => {
                            if let Some(text) = text {
                                combined += text;
                            } else {
                                combined.push('\n');
                            }
                        }
                    }
                }

                for attachment in &message_content.attachments {
                    combined += &format!("[an attachment: {:#?}]", attachment);
                }

                combined
            }

            raw::EventData::HangoutEvent { ref data, .. } => {
                match data {
                    raw::HangoutEvent::StartHangout => "[call started]".to_owned(),
                    raw::HangoutEvent::EndHangout { ref hangout_duration_secs } => {
                        format!("[call ended; duration was {} seconds", hangout_duration_secs)
                    }
                }
            }
        };

        eprintln!("[{}] {}: {}", dt, name, text);
    }

    Ok(())
}
