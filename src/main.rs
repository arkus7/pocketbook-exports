mod parse;

use parse::PocketBookNotes;

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

fn main() {
    let notes_str = include_str!("../notes/basb.html");
    let notes: PocketBookNotes = notes_str.parse().unwrap();

    println!(
        "Notes from '{}' book by {}",
        notes.book.title, notes.book.author
    );
    for ele in notes.notes.iter().filter(|n| !n.is_page_bookmark()) {
        println!("{}", ele.text);
        if let Some(note) = &ele.note {
            println!("*Note*: {}", note);
        }
        println!("Page: {}", ele.page);
    }
}
