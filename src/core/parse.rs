use anyhow::{Context, Result};
use epub::doc::EpubDoc;
use regex::Regex;
use xvii::Roman;

use super::data::{Chapter, EpubText};
use xml::reader::{EventReader, XmlEvent};

#[derive(Clone, Debug)]
pub struct Section {
    header_xhtml: String,
    header_text: String,
    body_xhtml: String,
}

pub fn parse_epub<R>(epub: &mut EpubDoc<R>) -> Result<EpubText>
where
    R: std::io::Read + std::io::Seek,
{
    let mut complete_text: String = "".to_string();
    // add all epub sections to the complete text
    // epub sections are just the .xhtml file divisions, they do not carry organizational significance in some cases
    // so we can't rely on them for detecting chapters/sections
    loop {
        let (text, _) = epub
            .get_current_str()
            .context("encountered section with no text")?;
        complete_text.push_str(&text);
        if !epub.go_next() {
            break;
        }
    }

    log::info!("pre-parse");
    // println!("{complete_text}");
    let parser = xml::EventReader::new(complete_text.as_bytes());
    log::info!("post-parse");

    for xml_event in parser {
        match xml_event {
            Ok(XmlEvent::StartDocument {
                version,
                encoding,
                standalone,
            }) => (),
            Ok(XmlEvent::EndDocument) => (),
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            }) => (),
            Ok(XmlEvent::EndElement { name }) => (),
            Ok(XmlEvent::Whitespace(s)) => (),
            Ok(XmlEvent::CData(s)) => (),
            Ok(XmlEvent::Characters(s)) => println!("{s}"),
            Ok(XmlEvent::ProcessingInstruction { name, data }) => (),
            Ok(XmlEvent::Comment(s)) => (),
            Err(e) => log::error!("encountered parsing error: {e}"),
        }
    }

    Ok(EpubText::default())
}

fn get_chapter_number(text: &str) -> Option<i32> {
    // a regex that captures numerals
    let regex = Regex::new(r"\b(?<numerals>(?<arabic>[0-9]+)|(?<roman>[IVXLCivxlc]+)|(?<kanji>[一二三四五六七八九十〇０0]+))\b")
        .unwrap();
    let captures = regex.captures(text);
    if let Some(caps) = captures {
        if let Some(cap) = caps.name("arabic") {
            return Some(i32::from_str_radix(cap.into(), 10).unwrap());
        }

        if let Some(cap) = caps.name("roman") {
            let roman: Roman = cap.as_str().parse().unwrap();
            return Some(roman.value() as i32);
        }

        if let Some(cap) = caps.name("kanji") {
            // TODO: handle non-digit style kanji as well (currently handles 一五四, needs to also handle 百五十四)
            let s = cap
                .as_str()
                .replace("一", "1")
                .replace("二", "2")
                .replace("三", "3")
                .replace("四", "4")
                .replace("五", "5")
                .replace("六", "6")
                .replace("七", "7")
                .replace("八", "8")
                .replace("九", "9")
                .replace("〇", "0")
                .replace("０", "0");
            return Some(i32::from_str_radix(&s, 10).unwrap());
        }
    }

    None
}

/// determines the longest run of chapters whose numbers increase by 1
/// returns the starting and ending indices of those chapters
fn get_main_chapter_indices(chapters: &Vec<Chapter>) -> (usize, usize) {
    todo!();
}
