use anyhow::{anyhow, Context, Result};
use epub::doc::EpubDoc;
use epub_builder::{EpubBuilder, EpubContent, Result as EpubBuilderResult, ZipLibrary};

use super::data::MergeType;

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
    let mut idx = 0;
    loop {
        let (a_text, _) = a.get_current_str().unwrap();
        let (b_text, _) = b.get_current_str().unwrap();
        log::trace!("adding text from chapter {idx}\na: {a_text}\nb: {b_text}");
        builder
            .add_content(EpubContent::new(format!("a_{idx}"), a_text.as_bytes()))
            .map_err(|e| anyhow!(e.to_string()))?;
        builder
            .add_content(EpubContent::new(format!("b_{idx}"), b_text.as_bytes()))
            .map_err(|e| anyhow!(e.to_string()))?;
        if !a.go_next() || !b.go_next() {
            break;
        }
        idx += 1;
    }

    log::debug!("finished reading at chapter index {idx}");
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
