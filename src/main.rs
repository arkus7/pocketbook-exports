mod parse;

use parse::PocketBookNotesExport;

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
    let export: PocketBookNotesExport = notes_str.parse().unwrap();

    println!(
        "Notes from '{}' book by {}",
        export.book.title, export.book.author
    );
    for note in export.notes.iter().filter(|n| !n.is_bookmark()) {
        println!("{}", note.highlight);
        if let Some(note) = &note.comment {
            println!("*Note*: {}", note);
        }
        println!("Page: {}", note.page);
    }
}
