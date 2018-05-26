extern crate chrono;
extern crate serde_json;
extern crate hangouts_json_parser;

use std::char;
use std::collections::hash_map::*;
use std::ffi::OsStr;
use std::fs::File;
use std::env;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::str;
use hangouts_json_parser::{raw, Hangouts};

fn usage() {
    eprintln!("usage: {} <json path> <participant name>", env::args().nth(0).unwrap());
}

fn chrono(parts: Result<(i64, u32), std::num::ParseIntError>) -> chrono::DateTime<chrono::Utc> {
    let (secs, nsecs) = parts.expect("bad timestamp");
    chrono::TimeZone::timestamp(&chrono::Utc, secs, nsecs)
}

fn start_formatting(formatting: &raw::Formatting) {
    if formatting.bold { print!("<b>"); }
    if formatting.italics { print!("<i>"); }
    if formatting.strikethrough { print!("<s>"); }
    if formatting.underline { print!("<ul>"); }
}

fn end_formatting(formatting: &raw::Formatting) {
    if formatting.bold { print!("</b>"); }
    if formatting.italics { print!("</i>"); }
    if formatting.strikethrough { print!("</s>"); }
    if formatting.underline { print!("</ul>"); }
}

fn parent_path(s: &OsStr) -> Result<PathBuf, io::Error> {
    let mut canonical = Path::new(s).canonicalize()?;
    canonical.pop();
    Ok(canonical)
}

fn urldecode(s: &str) -> String {
    let mut res = String::new();
    let mut number_start = 0;
    let mut in_percent = false;
    for (i, byte) in s.as_bytes().iter().cloned().enumerate() {
        let c = if byte < 0x80 {
            byte as char
        } else {
            panic!("bad URL (not ASCII)");
        };

        if in_percent {
            if number_start == i - 1 {
                let num_str = &s[number_start..=i];
                let n = u32::from_str_radix(num_str, 16)
                    .expect("bad URL (bad number after %)");
                let c = char::from_u32(n)
                    .expect("bad URL (invalid %-encoded character)");
                res.push(c);
                in_percent = false;
            }
        } else if c == '%' {
            in_percent = true;
            number_start = i + 1;
        } else {
            res.push(c);
        }
    }
    res
}

fn find_local_file(url: &str, base_path: &Path) -> Option<PathBuf> {
    let url_filename = url
        .rsplit_terminator('/')
        .next().unwrap();
    let filename = urldecode(url_filename);

    let localpath = base_path.join(&filename);
    if localpath.exists() {
        Some(localpath)
    } else {
        // hack
        let filename2 = filename.replace('+', " ");
        let localpath2 = base_path.join(filename2);
        if localpath2.exists() {
            Some(localpath2)
        } else {
            None
        }
    }

    // TODO: also sometimes the URL is just missing the extension, so check
    // that too.
}

fn main() -> Result<(), io::Error> {
    let path = env::args_os()
        .nth(1)
        .unwrap_or_else(|| {
            usage();
            std::process::exit(2);
        });

    let base_path = parent_path(&path).unwrap_or_else(|e| {
            eprintln!("Error: could not canonicalize path {:?}: {}", path, e);
            std::process::exit(2);
        });

    let participant_name = env::args()
        .nth(2)
        .unwrap_or_else(|| {
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

    println!("<!DOCTYPE html>");
    println!("<html>");
    println!("<head><meta charset=\"utf-8\"/></head>");
    println!("<body>");

    convo.events.sort_unstable_by_key(|event| event.header.timestamp.clone());
    for event in &convo.events {
        let dt = chrono(event.header.timestamp()).format("%Y-%m-%d %H:%M:%S");
        let name = &names[&event.header.sender_id];
        println!("[{}] {}: ", dt, name);
        match event.data {
            raw::EventData::ChatMessage { ref message_content, .. } => {
                for segment in &message_content.segments {
                    match segment {
                        raw::ChatSegment::Text { ref text, formatting, } => {
                            start_formatting(&formatting);
                            println!("{}", text);
                            end_formatting(&formatting);
                        }
                        raw::ChatSegment::Link { text, link_data, formatting } => {
                            start_formatting(&formatting);
                            println!("<a href=\"{}\">{}</a>",
                                link_data.link_target, text);
                            end_formatting(&formatting);
                        }
                        raw::ChatSegment::LineBreak { text: _ } => {
                            println!("<br />");
                        }
                    }
                }

                for attachment in &message_content.attachments {
                    if let Some(ref photo) = attachment.embed_item.plus_photo {
                        if let Some(path) = find_local_file(
                            &photo.url, &base_path)
                        {
                            println!("<img src=\"{}\"", path.to_string_lossy());
                        } else {
                            println!("remote image at <a href=\"{}\">{}</a>",
                                photo.url, photo.url);
                            println!("<img src=\"{}\"", photo.url);
                        }
                        println!("width=\"100%\"/>");
                    } else {
                        println!("[an attachment: <pre>{:#?}</pre>]", attachment);
                    }
                }
            }

            raw::EventData::HangoutEvent { ref data, .. } => {
                match data {
                    raw::HangoutEvent::StartHangout => {
                        println!("<i>[call started]</i><br />");
                    }
                    raw::HangoutEvent::EndHangout { ref hangout_duration_secs } => {
                        println!("<i>[call ended; duration was {} seconds]</i><br />",
                            hangout_duration_secs);
                    }
                }
            }
        }
        println!("<hr />");
    }
    println!("</body></html>");
    Ok(())
}