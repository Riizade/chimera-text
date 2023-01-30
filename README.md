# chimera-text

A work-in-progress command-line utility to combine two ebooks into a single text; primarily for reading a book in two languages simultaneously for language learning purposes.

This project does not exist yet! This repository contains notes on how I might implement this project in the future. Feel free to take these ideas and make your own whole thing, or fork the repository.

# Dependencies

## Development

- git-lfs
- rust
- cargo

## Executable

None?

# Notes

## Epub

investigate https://docs.rs/epub-builder/latest/epub_builder/ and https://docs.rs/epub/latest/epub/ for manipulating epub structure

## Parallel Text Alignment

Probably what we'll want to do is get dictionaries between as many pairwise combinations of languages as possible.

Next, we build n-gram lookup tables from each dictionary. (This will be a many-to-many relationship, because words do not translate directly from one language to another.)

Then we chunk the texts into a hierarchy of chunk sizes, something like chapter -> paragraph -> n-gram/word/atomic-lexeme.

We do a naive alignment of text, and then start pairwise comparisons of chunk to chunk, and sliding the alignment backward or forward by some amount to try to find the correct pairwise chunks (roughly).

We compare chunks by taking each atomic lexeme that is part of that chunk and using the dictionary lookup table. If the atomic lexeme has a matching atomic lexeme in the parallel chunk, that's a hit. If not, that's a miss.

Calculate the ratio of hit-to-miss for each chunk-to-chunk comparison. I expect direct translations to have a hit ratio of something like 50-70%, and unrelated passages to have a hit ratio of like 0-20%.

Use high-confidence chunks to align the text by anchoring those chunks to each other.

Then fill in the gaps. For texts where they are structurally similar and direct translations, a naive linear interpolation approach should work. For translations where portions were removed or added, or even rearranged, we'll need to compare things like expected text size, and rely on the anchor points to make a best effort guess as to which portions of the text may not have corresponding text in the other epub.

Once we've aligned the texts as best we can, we move on to the epub creation stage.

## Epub Creation

We should offer a few different options here

- Alternating text for different chunks (A, then B, then A)
  - Chapter-by-chapter
  - Paragraph-by-paragraph
  - Sentence-by-sentence
- Parallel columns of side-by-side text
- All of A's text, then all of B's text, with hyperlinks between anchor points to facilitate navigation between the texts

# License

MIT Licensed

Works in the `test-books/` directory are not covered by the license. These are public domain worked fetched from Project Gutenberg.
