use std::str::FromStr;

use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};

#[derive(Debug)]
pub struct PocketBookHighlight {
    pub text: BookmarkContent,
    pub note: Option<BookmarkNote>,
    pub page: Page,
}

impl PocketBookHighlight {
    pub fn is_page_bookmark(&self) -> bool {
        const PAGE_BOOKMARK_CONTENT: &'static str = "Bookmark";
        self.text.0 == PAGE_BOOKMARK_CONTENT && self.note.is_none()
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
pub struct NotesExportDate(String);

impl std::fmt::Display for NotesExportDate {
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

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
pub struct BookmarkContent(String);

impl std::fmt::Display for BookmarkContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
pub struct BookmarkNote(String);

impl std::fmt::Display for BookmarkNote {
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
            .ok_or("Expected text inside <span> element")?
            .text();

        Ok(Self(author_name))
    }
}

impl<'a> TryFrom<Node<'a>> for NotesExportDate {
    type Error = &'static str;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let h1_node = value
            .children()
            .find(|c| c.is(Name("h1")))
            .ok_or("Expected <h1> element with export date and book title inside")?;
        let content = h1_node
            .first_child()
            .ok_or("Expected text inside <h1> element")?
            .text();
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
            .ok_or("Expected text inside <h1> element")?
            .text();
        let book_title = content
            .split_once(" - ")
            .ok_or("Expected text with ' - ' inside to delimit export date and book title")?
            .1
            .to_owned();

        Ok(Self(book_title))
    }
}

impl<'a> TryFrom<Node<'a>> for BookmarkNote {
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

        let content = paragraph_node.text();
        let content = content.trim();
        Ok(Self(content.to_owned()))
    }
}

impl<'a> TryFrom<Node<'a>> for BookmarkContent {
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
        let content = content.trim();
        let content = content
            .replace('“', "\"")
            .replace('”', "\"")
            .replace('’', "'");
        Ok(Self(content.to_owned()))
    }
}

impl<'a> TryFrom<Node<'a>> for PocketBookHighlight {
    type Error = String;

    fn try_from(value: Node<'a>) -> Result<Self, Self::Error> {
        let bookmark_content: BookmarkContent = value
            .try_into()
            .map_err(|e| format!("Expected bookmark to include highlight/content: {e}"))?;
        let page: Page = value.try_into()?;
        let bookmark_note: Option<BookmarkNote> = value.try_into().ok();
        Ok(Self {
            page,
            text: bookmark_content,
            note: bookmark_note,
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
pub struct PocketBookNotes {
    pub book: Book,
    pub export_date: NotesExportDate,
    pub notes: Vec<PocketBookHighlight>,
}

impl FromStr for PocketBookNotes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let document = Document::from(s);

        let mut nodes = document.find(Class("bookmark"));

        let title_node = nodes
            .next()
            .ok_or("Expected at least one HTML node with 'bookmark' class")?;
        let export_date: NotesExportDate = title_node.try_into()?;
        let title: BookTitle = title_node.try_into()?;

        let author_node = nodes
            .next()
            .ok_or("Expected another HTML node with 'bookmark' class")?;
        let author: BookAuthor = author_node.try_into()?;

        let notes: Vec<PocketBookHighlight> = nodes.flat_map(|n| n.try_into()).collect();

        Ok(Self {
            book: Book { author, title },
            export_date,
            notes,
        })
    }
}
