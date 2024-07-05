use anyhow::{Context, Result};
use epub::doc::EpubDoc;
use regex::Regex;
use xvii::Roman;

use super::data::{Chapter, EpubText};
use html_parser::{Dom, Element};

#[derive(Clone, Debug)]
pub struct Section {
    header_html: String,
    header_text: String,
    body_html: String,
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

    let mut sections: Vec<Section> = vec![];
    let mut current_section: Option<Section> = None;
    let html = Dom::parse(&complete_text)?;
    for child in html.children {
        if let Some(element) = child.element() {
            // if the element is a header, we finish the previous section and start the next section
            if is_header(element) {
                if let Some(section) = &current_section {
                    sections.push(section.clone());
                }
                current_section = Some(Section {
                    header_html: display_element(element),
                    header_text: element.source_span.text.clone(),
                    body_html: "".to_string(),
                });
            } else {
                // otherwise, add the element to the section body
                if let Some(section) = &mut current_section {
                    section.body_html += &display_element(element);
                }
            }
        }
    }
    // add the last section to the vector
    sections.push(current_section.unwrap());

    for section in sections {
        log::info!("{0}", section.header_html);
        log::info!("{0}", section.header_text);
        log::info!("{0:#?}", get_chapter_number(&section.header_text));
        log::info!("--------------------------------------------");
    }

    Ok(EpubText::default())
}

fn is_header(e: &Element) -> bool {
    let header_classes = ["h1", "h2", "h3", "h4", "h5"];
    for class in header_classes {
        if e.classes.contains(&class.to_string()) {
            return true;
        }
    }

    return false;
}

fn display_element(e: &Element) -> String {
    format!("{e:#?}")
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
