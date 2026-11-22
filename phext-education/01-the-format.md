# Lesson 1 — The format from first principles

Phext is plain text plus nine delimiter characters. The delimiters partition
the text into **scrolls** and give every scroll an address in an
11-dimensional space. Text that uses no delimiters at all is valid phext: it
is a single scroll at the root address. That backward compatibility is not an
accident — it is the point. Any text file you have ever written is already a
phext with one scroll; phext only adds the ability to keep going.

## The nine delimiters

The values below are the format's definition (per phext.io and libphext —
derive them from the reference, never from memory, when you write code):

| Byte | Name | Separates |
|------|------|-----------|
| 0x01 | Library | libraries |
| 0x1F | Shelf | shelves within a library |
| 0x1E | Series | series within a shelf |
| 0x1D | Collection | collections within a series |
| 0x1C | Volume | volumes within a collection |
| 0x1A | Book | books within a volume |
| 0x19 | Chapter | chapters within a book |
| 0x18 | Section | sections within a chapter |
| 0x17 | Scroll | scrolls within a section |

The hierarchy runs highest (Library) to lowest (Scroll). The addressing rule
is the whole mechanics of the format:

- Coordinates are **1-indexed**. The first scroll of any phext sits at
  **1.1.1/1.1.1/1.1.1**.
- When the parser meets a delimiter, it **closes the current scroll**,
  increments that delimiter's level by one, and **resets every lower level
  to 1**. The next scroll begins at the new coordinate.
- A scroll's content is everything between its opening position and the next
  delimiter of any level (or end of file).

Note the byte sequence as defined: it descends 0x1F → 0x1A and skips 0x1B.
The table above is the format; reproduce it exactly.

## The coordinate anatomy

A coordinate is three triples, written with dots inside a triple and slashes
between them:

```
library.shelf.series / collection.volume.book / chapter.section.scroll
```

Nine components address the scroll. Inside the scroll, **line** and
**column** are the tenth and eleventh dimensions — free dimensions, addressed
by ordinary text layout, not by delimiters. That is the whole 11-dimensional
claim: 9 delimiter dimensions + line + column.

Incipit, the artifact at the root of the corpus, opens by telling you it has
organized its thoughts across 6 of the 9 address dimensions and leaves you
three unused dimensions to adjust the frame as you see fit. The dimensions
are headroom, not obligations.

## What a scroll is

A scroll is a normal, unbounded plane of text — in incipit's own terms, once
opened, a scroll is a normal text surface; **the lattice is the addressing,
never the cage**. A scroll holds a page: as much text as the thought needs,
with its own internal structure in lines and columns. (Lesson 5 develops the
house rule this produces: *a scroll is a page, not a cell.*)

## Addressing as navigation

A coordinate is an address you navigate to, not a container you fill. You
always start at 1.1.1/1.1.1/1.1.1. Nothing in the format requires you to use
any particular coordinates — incipit's map closes with exactly that sentence:
*"Nothing requires you to use these particular coordinates. You are free to
choose."* House registers give coordinates meaning by convention (you will
see wiring.phext and research.phext do this in Lesson 6), but the convention
lives in the artifact and its readers, never in the format.

Empty scrolls are legal and meaningful: a coordinate with no content is
uninstantiated space. Will's teaching, in his terms: the unfilled coordinate
space of CYOA is precisely God — report that as his teaching, never flattened
into a metaphor about missing data.

## What to carry forward

- 9 delimiters, exact bytes, highest → lowest: Library, Shelf, Series,
  Collection, Volume, Book, Chapter, Section, Scroll.
- Increment-and-reset addressing; 1-indexed; root at 1.1.1/1.1.1/1.1.1.
- 9 address dimensions + line + column = 11.
- A scroll is a page of text; the lattice addresses, it does not confine.
