use anyhow::{anyhow, Context, Result};
use epub::doc::EpubDoc;
use epub_builder::{EpubBuilder, EpubContent, Result as EpubBuilderResult, ZipLibrary};

use super::{data::MergeType, parse::parse_epub};

pub fn merge<R>(
    text_a: &mut EpubDoc<R>,
    text_b: &mut EpubDoc<R>,
    merge_type: MergeType,
) -> Result<Vec<u8>>
where
    R: std::io::Read + std::io::Seek,
{
    let zip_library = ZipLibrary::new().map_err(|e| anyhow!(e.to_string()))?;
    let mut builder = EpubBuilder::new(zip_library).map_err(|e| anyhow!(e.to_string()))?;
    set_metadata(&mut builder, text_a)?;
    add_content(&mut builder, text_a, text_b, merge_type)?;
    let mut write_buffer = Vec::<u8>::new();
    builder
        .inline_toc()
        .generate(&mut write_buffer)
        .map_err(|e| anyhow!(e.to_string()))?;

    Ok(write_buffer)
}

fn set_metadata<R>(builder: &mut EpubBuilder<ZipLibrary>, primary_text: &EpubDoc<R>) -> Result<()>
where
    R: std::io::Read + std::io::Seek,
{
    primary_text
        .metadata
        .get("author")
        .map(|x| builder.add_author(x.join(", ")));

    primary_text
        .metadata
        .get("description")
        .map(|x| builder.add_description(x.join(", ")));

    Ok(())
}

fn add_content<R>(
    builder: &mut EpubBuilder<ZipLibrary>,
    a: &mut EpubDoc<R>,
    b: &mut EpubDoc<R>,
    merge_type: MergeType,
) -> Result<()>
where
    R: std::io::Read + std::io::Seek,
{
    log::info!("a");
    let text = parse_epub(a)?;
    log::info!("b");
    let text = parse_epub(b)?;

    todo!();
    match merge_type {
        MergeType::AlternateChapters => add_content_alternating_chapters(builder, a, b),
        MergeType::AlternateParagraphs => add_content_alternating_paragraphs(builder, a, b),
        MergeType::HyperlinkChapters => add_content_hyperlink_chapters(builder, a, b),
        MergeType::HyperlinkParagraphs => add_content_hyperlink_paragraphs(builder, a, b),
    }?;
    Ok(())
}

fn add_content_alternating_chapters<R>(
    builder: &mut EpubBuilder<ZipLibrary>,
    a: &mut EpubDoc<R>,
    b: &mut EpubDoc<R>,
) -> Result<()>
where
    R: std::io::Read + std::io::Seek,
{
    Ok(())
}

fn add_content_alternating_paragraphs<R>(
    builder: &mut EpubBuilder<ZipLibrary>,
    a: &mut EpubDoc<R>,
    b: &mut EpubDoc<R>,
) -> Result<()>
where
    R: std::io::Read + std::io::Seek,
{
    unimplemented!();
    Ok(())
}

fn add_content_hyperlink_chapters<R>(
    builder: &mut EpubBuilder<ZipLibrary>,
    a: &mut EpubDoc<R>,
    b: &mut EpubDoc<R>,
) -> Result<()>
where
    R: std::io::Read + std::io::Seek,
{
    unimplemented!();
    Ok(())
}

fn add_content_hyperlink_paragraphs<R>(
    builder: &mut EpubBuilder<ZipLibrary>,
    a: &mut EpubDoc<R>,
    b: &mut EpubDoc<R>,
) -> Result<()>
where
    R: std::io::Read + std::io::Seek,
{
    unimplemented!();
    Ok(())
}
