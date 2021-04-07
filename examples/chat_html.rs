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

fn urldecode(s: &str) -> Result<String, String> {
    let mut bytes = vec![];
    let mut skip = 0;
    for (i, byte) in s.as_bytes().iter().cloned().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if byte == b'%' {
            let num_str = s.get(i + 1 .. i + 3)
                .ok_or_else(|| format!("%-encoded character cut short"))?;
            let n = u8::from_str_radix(num_str, 16)
                .map_err(|e| format!("invalid %-encoded character: {}", e))?;
            bytes.push(n);
            skip = 2;
        } else {
            bytes.push(byte);
        }
    }

    String::from_utf8(bytes)
        .map_err(|e| format!("{}", e))
}

fn find_local_file(url: &str, base_path: &Path) -> Option<PathBuf> {
    let url_filename = url
        .rsplit_terminator('/')
        .next().unwrap();
    let decoded_filename = urldecode(url_filename)
        .map_err(|e| {
            eprintln!("bad URL {:?}: {}", url_filename, e);
            e
        })
        .ok()?;

    let mut filename = decoded_filename.clone();
    let mut localpath = base_path.join(&filename);

    loop {
        if localpath.exists() {
            return Some(localpath);
        }

        if localpath.extension().is_none() {
            localpath.set_extension("jpg");
            if localpath.exists() {
                return Some(localpath);
            }
        }

        // Try unwrapping another layer of urlencoding.
        if let Ok(decoded) = urldecode(&filename) {
            if decoded == filename {
                break;
            }
            filename = decoded;
            localpath.set_file_name(&filename);
        } else {
            break;
        }
    }

    // Try again, additionally replacing some characters because this is what Google does sometimes.
    filename = decoded_filename;
    loop {
        filename = filename.replace('+', " ");
        filename = filename.replace('?', "_");
        localpath.set_file_name(&filename);

        if localpath.exists() {
            return Some(localpath);
        }

        if localpath.extension().is_none() {
            localpath.set_extension("jpg");
            if localpath.exists() {
                return Some(localpath);
            }
        }

        // Try unwrapping another layer of urlencoding.
        if let Ok(decoded) = urldecode(&filename) {
            if decoded == filename {
                break;
            }
            filename = decoded;
        } else {
            break;
        }
    }

    None
}

fn file_url(path: &Path) -> String {
    let s = path.to_str().expect("non-utf8 path");
    if cfg!(windows) && s.starts_with(r"\\?\") {
        // Browsers don't like "\\?\" paths; remove the prefix.
        format!("file:///{}", &s[4..])
    } else {
        format!("file:///{}", s)
    }
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
                p.fallback_name.as_ref() == Some(&participant_name)))
        .unwrap_or_else(|| {
            eprintln!("No matching conversation found with a person named {:?}", participant_name);
            std::process::exit(1);
        });

    let names: HashMap<raw::ParticipantId, String> = convo.header.details.participant_data
        .iter()
        .map(|p| (p.id.clone(), p.fallback_name.as_deref().unwrap_or("[unknown]").to_owned()))
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
                            println!("<img src=\"{}\"", file_url(&path));
                        } else {
                            println!("<img src=\"{}\"", photo.url);
                        }
                        // TODO: maybe make a thumbnail: smaller image and link to full one?
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

            raw::EventData::MembershipChange { .. } => {
                println!("<i>[membership change: {:#?}]</i>", event.data);
            }

            raw::EventData::ConversationRename { ref old_name, ref new_name } => {
                println!("<i>[conversation renamed from {:?} to {:?}]</i>", old_name, new_name);
            }
        }
        println!("<hr />");
    }
    println!("</body></html>");
    Ok(())
}
