use select::{
    document::Document,
    node::{Data, Node},
    predicate::{Class, Element, Name},
};

#[derive(Debug)]
enum HighlightCategory {
    Books,
}

#[derive(Debug)]
enum HighlightLocationType {
    Page,
}

#[derive(Debug)]
struct ReadwiseBookHighlight {
    text: String,
    title: Option<String>,
    author: Option<String>,
    source_type: Option<String>,
    category: Option<HighlightCategory>,
    note: Option<String>,
    location: Option<usize>,
    location_type: Option<HighlightLocationType>,
    highlighted_at: Option<String>,
}

#[derive(Debug)]
struct PocketBookHighlight {
    text: String,
    note: Option<String>,
    page: usize,
}

impl PocketBookHighlight {
    const PAGE_BOOKMARK_CONTENT: &'static str = "Bookmark";

    pub fn is_page_bookmark(&self) -> bool {
        self.text == Self::PAGE_BOOKMARK_CONTENT && self.note.is_none()
    }
}

#[derive(Debug)]
struct BookAuthor(String);
#[derive(Debug)]
struct NotesExportDate(String);
#[derive(Debug)]
struct BookTitle(String);

#[derive(Debug)]
struct Page(usize);

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

#[derive(Debug)]
struct BookmarkContent(String);

#[derive(Debug)]
struct BookmarkNote(String);

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
        let bookmark_note: Option<String> = value.try_into().ok().map(|n: BookmarkNote| n.0);
        Ok(Self {
            page: page.0,
            text: bookmark_content.0,
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
fn main() {
    let document = Document::from(include_str!("../notes/basb.html"));

    let mut nodes = document.find(Class("bookmark"));

    let title_node = nodes
        .next()
        .ok_or("Expected at least one HTML node with 'bookmark' class")
        .unwrap();

    let export_date: Option<NotesExportDate> = title_node.try_into().ok();
    let book_title: Option<BookTitle> = title_node.try_into().ok();
    let book_author: Option<BookAuthor> = nodes
        .next()
        .ok_or("Expected another HTML node with 'bookmark' class")
        .unwrap()
        .try_into()
        .ok();

    println!("{book_title:?}");
    println!("{book_author:?}");
    println!("{export_date:?}");

    println!("Highlighted:");

    for node in nodes {
        let highlight: PocketBookHighlight = node.try_into().unwrap();
        if highlight.is_page_bookmark() {
            continue;
        }
        println!("{highlight:#?}");
    }
}
