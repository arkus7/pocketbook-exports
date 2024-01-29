use std::str::FromStr;

use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};

#[derive(Debug)]
pub struct Note {
    pub highlight: NoteHighlight,
    pub comment: Option<NoteComment>,
    pub page: Page,
}

impl Note {
    pub fn is_bookmark(&self) -> bool {
        const BOOKMARK_CONTENT: &'static str = "Bookmark";
        self.highlight.0 == BOOKMARK_CONTENT && self.comment.is_none()
    }
}

#[derive(Debug)]
pub struct BookAuthor(String);

impl std::fmt::Display for BookAuthor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct ExportDate(String);

impl std::fmt::Display for ExportDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct BookTitle(String);

impl std::fmt::Display for BookTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
pub struct Page(usize);

impl AsRef<usize> for Page {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
pub struct NoteHighlight(String);

impl std::fmt::Display for NoteHighlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct NoteComment(String);

impl std::fmt::Display for NoteComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> TryFrom<Node<'a>> for BookAuthor {
    type Error = &'static str;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let author_node = value
            .children()
            .find(|c| c.is(Name("span")))
            .ok_or("Expected <span> element with book author inside")?;
        let author_name = author_node
            .first_child()
            .ok_or("Expected contents in <span> element")?
            .as_text()
            .ok_or("Expected text inside <span> element")?
            .to_owned();

        Ok(Self(author_name))
    }
}

impl<'a> TryFrom<Node<'a>> for ExportDate {
    type Error = &'static str;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let h1_node = value
            .children()
            .find(|c| c.is(Name("h1")))
            .ok_or("Expected <h1> element with export date and book title inside")?;
        let content = h1_node
            .first_child()
            .ok_or("Expected contents inside <h1> element")?
            .as_text()
            .ok_or("Expected text inside <h1> element")?;
        let export_date = content
            .split_once(" - ")
            .ok_or("Expected text with ' - ' inside to delimit export date and book title")?
            .0
            .to_owned();

        Ok(Self(export_date))
    }
}

impl<'a> TryFrom<Node<'a>> for BookTitle {
    type Error = &'static str;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let h1_node = value
            .children()
            .find(|c| c.is(Name("h1")))
            .ok_or("Expected <h1> element with export date and book title inside")?;
        let content = h1_node
            .first_child()
            .ok_or("Expected contents inside <h1> element")?
            .as_text()
            .ok_or("Expected text inside <h1> element")?;
        let book_title = content
            .split_once(" - ")
            .ok_or("Expected text with ' - ' inside to delimit export date and book title")?
            .1
            .to_owned();

        Ok(Self(book_title))
    }
}

impl<'a> TryFrom<Node<'a>> for NoteComment {
    type Error = &'static str;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let text_node = value
            .children()
            .find(|c| c.is(Class("bm-note")))
            .ok_or("Expected element with 'bm-note' class")?;
        let paragraph_node = text_node
            .find(Name("p"))
            .next()
            .ok_or("Expected one 'p' element inside '.bm-note'")?;

        let comment = paragraph_node.text();
        let comment = comment.trim();
        Ok(Self(comment.to_owned()))
    }
}

impl<'a> TryFrom<Node<'a>> for NoteHighlight {
    type Error = &'static str;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let text_node = value
            .children()
            .find(|c| c.is(Class("bm-text")))
            .ok_or("Expected element with 'bm-text' class")?;
        let paragraph_node = text_node
            .find(Name("p"))
            .next()
            .ok_or("Expected one 'p' element inside '.bm-text'")?;

        let mut content = String::new();
        recur(&paragraph_node, &mut content);

        fn recur(node: &Node, string: &mut String) {
            if let Some(text) = node.as_text() {
                string.push_str(text);
            }
            for child in node.children() {
                if child.is(Name("br")) {
                    string.push_str("\n");
                }
                recur(&child, string)
            }
        }
        let content = content
            .trim()
            .replace('“', "\"")
            .replace('”', "\"")
            .replace('’', "'");
        Ok(Self(content.to_owned()))
    }
}

impl<'a> TryFrom<Node<'a>> for Note {
    type Error = String;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let content: NoteHighlight = value
            .try_into()
            .map_err(|e| format!("Expected bookmark to include highlight/content: {e}"))?;
        let page: Page = value.try_into()?;
        let comment: Option<NoteComment> = value.try_into().ok();
        Ok(Self {
            page,
            highlight: content,
            comment,
        })
    }
}

impl<'a> TryFrom<Node<'a>> for Page {
    type Error = String;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let page_node = value
            .children()
            .find(|c| c.is(Class("bm-page")))
            .ok_or("Expected element with 'bm-page' class")?;
        let page = page_node
            .first_child()
            .ok_or("Expected contents inside element with 'bm-page' class")?
            .as_text()
            .ok_or("Expected contents to be text")?
            .parse()
            .map_err(|e| format!("Expected text to be an positive integer: {e}"))?;

        Ok(Self(page))
    }
}

#[derive(Debug)]
pub struct Book {
    pub author: BookAuthor,
    pub title: BookTitle,
}

#[derive(Debug)]
pub struct PocketBookNotesExport {
    pub book: Book,
    pub export_date: ExportDate,
    pub notes: Vec<Note>,
}

impl FromStr for PocketBookNotesExport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let document = Document::from(s);

        let mut nodes = document.find(Class("bookmark"));

        let title_node = nodes
            .next()
            .ok_or("Expected at least one HTML node with 'bookmark' class")?;
        let export_date: ExportDate = title_node.try_into()?;
        let title: BookTitle = title_node.try_into()?;

        let author_node = nodes
            .next()
            .ok_or("Expected another HTML node with 'bookmark' class")?;
        let author: BookAuthor = author_node.try_into()?;

        let notes: Vec<Note> = nodes.flat_map(|n| n.try_into()).collect();

        Ok(Self {
            book: Book { author, title },
            export_date,
            notes,
        })
    }
}
