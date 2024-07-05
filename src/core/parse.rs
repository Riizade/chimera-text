use anyhow::{Context, Result};
use epub::doc::EpubDoc;
use regex::Regex;
use xvii::Roman;

use super::data::{Chapter, EpubText};
use scraper::{node::Element, selectable::Selectable, Html, Node, Selector};

#[derive(Clone, Debug)]
pub struct Section {
    header: String,
    body: String,
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

    let html = Html::parse_document(&complete_text);
    let root = html.tree.root();
    let mut sections: Vec<Section> = vec![];
    let mut current_section: Option<Section> = None;
    for node in root.descendants() {
        match node.value() {
            Node::Comment(_) => todo!(),
            Node::Document => todo!(),
            Node::Fragment => todo!(),
            Node::Doctype(_) => todo!(),
            Node::Text(_) => todo!(),
            Node::Element(e) => {
                // if the element is a header, we finish the previous section and start the next section
                if is_header(e) {
                    if let Some(section) = &current_section {
                        sections.push(section.clone());
                    }
                    current_section = Some(Section {
                        header = e
                    })
                }
            }
            Node::ProcessingInstruction(_) => todo!(),
        }
    }
    let header_selector = Selector::parse("h1,h2,h3,h4,h5").unwrap();
    let headers = html.select(&header_selector);
    for h in headers {
        let header_text = h.text().collect::<String>();
        let chapter_number = get_chapter_number(&header_text);
        log::info!("header: {header_text}\nchapter: {chapter_number:#?}");
        log::info!("--------------------------------------------");
    }

    Ok(EpubText::default())
}

fn is_header(e: &Element) -> bool {
    let header_classes = ["h1", "h2", "h3", "h4", "h5"];
    for class in header_classes {
        if e.has_class(class, scraper::CaseSensitivity::AsciiCaseInsensitive) {
            return true;
        }
    }

    return false;
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
