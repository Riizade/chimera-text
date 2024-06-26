use clap::ValueEnum;

/// associates a particular location in the text with a unique identifier
/// this is useful to match corresponding locations between two parallel texts
pub struct AnchorPoint {
    identifier: String,
    anchor_type: AnchorPointType,
}

pub enum AnchorPointType {
    ChapterStart,
    ParagraphStart,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, ValueEnum)]
pub enum MergeType {
    /// the merged text will alternate between text a and b every chapter
    /// e.g., A Chapter 1, B Chapter 1, A Chapter 2, B Chapter 2, ...
    AlternateChapters,
    /// the merged text will alternate between text a and b every paragraph
    /// e.g., A P1, B P1, A P2, B P2, ...
    AlternateParagraphs,
    /// the merged text will have all of text A, then all of text B, but will insert hyperlinks to navigate between them for every chapter
    HyperlinkChapters,
    /// the merged text will have all of text A, then all of text B, but will insert hyperlinks to navigate between them for every paragraph
    HyperlinkParagraphs,
}

#[derive(Debug, Clone, Default)]
pub struct EpubText {
    pub table_of_contents: Option<String>,
    pub preamble: Option<String>,
    pub chapters: Vec<Chapter>,
    pub epilogue: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Chapter {
    pub title: String,
    pub subtitle: Option<String>,
    pub paragraphs: Vec<String>,
}
