use std::path::Path;
use epub::doc::EpubDoc;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub content: String,
    pub word_count: usize,
    pub selected: bool,
}

#[derive(Debug, Clone)]
pub struct BookMetadata {
    pub title: String,
    pub author: String,
    pub cover_image: Option<Vec<u8>>,
    pub cover_mime: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Book {
    pub metadata: BookMetadata,
    pub chapters: Vec<Chapter>,
}

pub fn parse_epub<P: AsRef<Path>>(path: P) -> Result<Book, String> {
    let mut doc = EpubDoc::new(path.as_ref())
        .map_err(|e| format!("Błąd otwierania pliku EPUB: {e}"))?;

    let title = doc
        .mdata("title")
        .map(|m| m.value.clone())
        .unwrap_or_else(|| "Nieznany tytuł".to_string());

    let author = doc
        .mdata("creator")
        .map(|m| m.value.clone())
        .unwrap_or_else(|| "Nieznany autor".to_string());

    let (cover_image, cover_mime) = match doc.get_cover() {
        Some(cover) => (Some(cover.0), Some(cover.1)),
        None => (None, None),
    };

    let mut chapters = Vec::new();
    let num_chapters = doc.get_num_chapters();

    for i in 0..num_chapters {
        doc.set_current_chapter(i);
        if let Some((page_content, mime)) = doc.get_current() {
            if mime.contains("html") || mime.contains("xml") {
                let text_raw = String::from_utf8_lossy(&page_content).to_string();
                let clean_text = clean_html_content(&text_raw);

                if clean_text.trim().len() > 30 {
                    let page_title = extract_title(&text_raw).unwrap_or_else(|| {
                        format!("Rozdział {}", chapters.len() + 1)
                    });

                    let word_count = clean_text.split_whitespace().count();

                    chapters.push(Chapter {
                        id: format!("chap_{i}"),
                        title: page_title,
                        content: clean_text,
                        word_count,
                        selected: true,
                    });
                }
            }
        }
    }

    if chapters.is_empty() {
        return Err("Nie znaleziono czytelnych rozdziałów w pliku EPUB.".to_string());
    }

    Ok(Book {
        metadata: BookMetadata {
            title,
            author,
            cover_image,
            cover_mime,
        },
        chapters,
    })
}

pub fn clean_html_content(raw_html: &str) -> String {
    let fragment = Html::parse_fragment(raw_html);
    let p_selector = Selector::parse("p, h1, h2, h3, h4, h5, h6, li, blockquote").unwrap();

    let mut paragraphs = Vec::new();

    for element in fragment.select(&p_selector) {
        if element.ancestors().any(|a| {
            if let Some(el) = a.value().as_element() {
                el.name() == "script" || el.name() == "style" || el.name() == "head"
            } else {
                false
            }
        }) {
            continue;
        }

        let text = element.text().collect::<Vec<_>>().join(" ");
        let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");

        if !cleaned.is_empty() {
            paragraphs.push(cleaned);
        }
    }

    if paragraphs.is_empty() {
        let text = fragment.root_element().text().collect::<Vec<_>>().join(" ");
        return text.split_whitespace().collect::<Vec<_>>().join(" ");
    }

    paragraphs.join("\n\n")
}

fn extract_title(raw_html: &str) -> Option<String> {
    let fragment = Html::parse_fragment(raw_html);
    let title_selector = Selector::parse("h1, h2, title").ok()?;

    for element in fragment.select(&title_selector) {
        let title = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
        if !title.is_empty() && title.len() < 120 {
            return Some(title);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_html_content() {
        let sample = r#"
            <html>
                <head><style>p { color: red; }</style></head>
                <body>
                    <h1>Rozdział 1</h1>
                    <p>To jest pierwszy akapit książki.</p>
                    <p>A to jest drugi akapit, z polskimi znakami: ą, ć, ę, ł, ń, ó, ś, ź, ż.</p>
                </body>
            </html>
        "#;

        let cleaned = clean_html_content(sample);
        assert!(cleaned.contains("Rozdział 1"));
        assert!(cleaned.contains("To jest pierwszy akapit książki."));
        assert!(cleaned.contains("polskimi znakami"));
        assert!(!cleaned.contains("color: red"));
    }
}
