mod parse;

use parse::PocketBookNotesExport;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum HighlightCategory {
    Books,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum HighlightLocationType {
    Page,
}

#[derive(Debug, Serialize)]
struct ReadwiseBookHighlight {
    text: String,
    title: Option<String>,
    author: Option<String>,
    source_type: Option<String>,
    category: Option<HighlightCategory>,
    note: Option<String>,
    location: Option<usize>,
    location_type: Option<HighlightLocationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlighted_at: Option<String>,
}

fn main() {
    let notes_str = include_str!("../notes/basb.html");
    let export: PocketBookNotesExport = notes_str.parse().unwrap();

    println!(
        "Notes from '{}' book by {}",
        export.book.title, export.book.author
    );
    let notes = export.notes.iter().filter(|n| !n.is_bookmark());
    // for note in notes {
    //     println!("{}", note.highlight);
    //     if let Some(note) = &note.comment {
    //         println!("*Note*: {}", note);
    //     }
    //     println!("Page: {}", note.page);
    // }

    let highlights = notes
        .map(|n| ReadwiseBookHighlight {
            text: n.highlight.to_string(),
            title: Some(export.book.title.to_string()),
            author: Some(export.book.author.to_string()),
            source_type: Some("PocketBookImporter-rs-arkus7".into()),
            category: Some(HighlightCategory::Books),
            note: n.comment.as_ref().map(|c| c.to_string()),
            location: Some(*n.page.as_ref()),
            location_type: Some(HighlightLocationType::Page),
            // TODO: handle dates with chrono crate
            highlighted_at: None, // Some(export.export_date.to_string().replace(' ', "T")),
        })
        .collect::<Vec<_>>();

    println!("{}", serde_json::to_string_pretty(&highlights).unwrap());
}
