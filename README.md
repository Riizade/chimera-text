# chimera-text

A work-in-progress command-line utility to combine two ebooks into a single text; primarily for reading a book in two languages simultaneously for language learning purposes.

This project does not exist yet! This repository contains notes on how I might implement this project in the future. Feel free to take these ideas and make your own whole thing, or fork the repository.

# Dependencies

## Development

- git-lfs
- rust
- cargo

# Notes

## Epub

Epub structures are not reliable. See the organization of Alice's Adventures in Wonderland.it.epub for an example. It has only 3 "chapters" (xhtml pages), which are unrelated to the actual structure of the book. Chapters are instead managed entirely via formatting and hyperlinks.

In order for this to work, we're going to have to try to automatically detect chapters via formatting such as headers.

We also need some methods to line up chapters. One approach is to try to detect chapters along with their index by looking for numerics literally. Then we can match chapters between the two texts via their parsed numerals.

Numeric support would need to include:

- arabic numerals
- Japanese kanji numerals (百五十六 = 156)
- Japanese kanji digits (e.g., 一五六 = 156, 一 0 六 = 106)
- Mandarin hanzi numerals (e.g., 一百五十六 = 156)
- Mandarin hanzi digits (e.g., 一五六 = 156)
- Roman numerals (e.g., XII)

Probably what we'll want to do is parse to a structure consisting of

- Table of Contents
- Preamble (optional, includes, foreword, etc)
- Chapters
- Epilogue (optional, includes actual epilogue, as well as any post-novel text such as acknowledgments that come after the final chapter)

Once we've lined up the chapters, we can attempt to line up smaller units of text, e.g., paragraphs by the presence of `<p>` tags or extra newlines/line breaks.

Parsers to try out:

- https://github.com/servo/html5ever
- https://crates.io/crates/kuchiki
- https://crates.io/crates/scraper
- https://docs.rs/html_parser/latest/html_parser/

## Word Match Anchor Points

We can explore matching locations between parallel texts via translation as well.

Probably what we'll want to do is get dictionaries between as many pairwise combinations of languages as possible. For pairwise combinations that are unavailable, we can try using a more popular language as an intermediary, such as English or Mandarin.

Next, we build n-gram lookup tables from each dictionary. (This will be a many-to-many relationship, because words do not translate directly from one language to another.)

Then we chunk the texts into a hierarchy of chunk sizes, something like chapter -> paragraph -> n-gram/word/atomic-lexeme.

We do a naive alignment of text by analyzing chapter/paragraph structure, and then start pairwise comparisons of chunk to chunk, and sliding the alignment backward or forward by some amount to try to find the correct pairwise chunks (roughly).

We compare chunks by taking each atomic lexeme that is part of that chunk and using the dictionary lookup table. If the atomic lexeme has a matching atomic lexeme in the parallel chunk, that's a hit. If not, that's a miss.

Calculate the ratio of hit-to-miss for each chunk-to-chunk comparison. I expect direct translations to have a hit ratio of something like 50-70%, and unrelated passages to have a hit ratio of like 0-20%.

Use high-confidence chunks to align the text by anchoring those chunks to each other.

Then fill in the gaps. For texts where they are structurally similar and direct translations, a naive linear interpolation approach should work. For translations where portions were removed or added, or even rearranged, we'll need to compare things like expected text size, and rely on the anchor points to make a best effort guess as to which portions of the text may not have corresponding text in the other epub.

Once we've aligned the texts as best we can, we move on to the epub creation stage.

## Manual Anchor Points

We can also allow users to create manual anchor points if they're technically inclined and willing to delve into the epub structure.

Creating anchor points should be a separate, independent module from the usage of the anchor points to link the epub.

This allows us to let users manually specify anchor points in case something about their book is substantially wonky, or the automatic detection doesn't work well for the two languages they've chosen.

As a stretch goal, we could have this utility render each of the two input epubs side-by-side and offer a GUI for creating matching anchor points between the two texts.

## Epub Creation

We should offer a few different options here

- Alternating text for different chunks (A, then B, then A)
  - Chapter-by-chapter
  - Paragraph-by-paragraph
- Parallel columns of side-by-side text
- All of A's text, then all of B's text, with hyperlinks between anchor points to facilitate navigation between the texts

## Dictionary Sources and Tools

Places where we can find resources that might be useful for this project.

- https://github.com/kujirahand/EJDict
- https://github.com/argosopentech/argos-translate
- https://freedict.org/ (LGPL 3+)
- https://github.com/open-dsl-dict/wiktionary-dict (bilingual DSL dictionaries under Creative Commons Attribution-ShareAlike 3.0 Unported or GNU Free Documentation License)
- https://lib.rs/crates/harlaw (converts DSL dictionaries to JSON)
- https://github.com/freedict/libdict (reads dictd format dictionaries)

# License

MIT Licensed

Works in the `test-books/` directory are not covered by the license. These are public domain works fetched from Project Gutenberg.
