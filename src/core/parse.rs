use anyhow::Result;
use epub::doc::EpubDoc;

use super::data::EpubText;

pub fn parse_epub<R>(epub: &mut EpubDoc<R>) -> Result<EpubText>
where
    R: std::io::Read + std::io::Seek,
{
    Ok(EpubText::default())
}
