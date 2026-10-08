/// Splits text into optimal chunks for TTS models, respecting sentence boundaries
/// and avoiding splitting on common abbreviations.
pub struct ChunkerConfig {
    pub max_chars: usize,
    pub min_chars: usize,
}

impl Default for ChunkerConfig {
    fn default() -> Self {
        Self {
            max_chars: 350,
            min_chars: 40,
        }
    }
}

pub fn split_into_chunks(text: &str, config: &ChunkerConfig) -> Vec<String> {
    let mut chunks = Vec::new();
    let paragraphs = text.split("\n\n");

    for para in paragraphs {
        let para = para.trim();
        if para.is_empty() {
            continue;
        }

        let sentences = split_sentences(para);
        let mut current_chunk = String::new();

        for sentence in sentences {
            let sentence = sentence.trim();
            if sentence.is_empty() {
                continue;
            }

            // If a single sentence is longer than max_chars, split it by secondary punctuation
            if sentence.len() > config.max_chars {
                if !current_chunk.is_empty() {
                    chunks.push(current_chunk.trim().to_string());
                    current_chunk.clear();
                }
                let sub_chunks = split_long_sentence(sentence, config.max_chars);
                chunks.extend(sub_chunks);
                continue;
            }

            if current_chunk.is_empty() {
                current_chunk.push_str(sentence);
            } else if current_chunk.len() + 1 + sentence.len() <= config.max_chars {
                current_chunk.push(' ');
                current_chunk.push_str(sentence);
            } else {
                chunks.push(current_chunk.trim().to_string());
                current_chunk = sentence.to_string();
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk.trim().to_string());
        }
    }

    chunks
}

/// Splits a paragraph into sentences, checking for common abbreviations.
pub fn split_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    let mut i = 0;
    while i < len {
        let c = chars[i];
        if is_sentence_terminator(c) {
            // Check for ellipsis (e.g. "...")
            let mut end_punct = i;
            while end_punct + 1 < len && is_sentence_terminator(chars[end_punct + 1]) {
                end_punct += 1;
            }

            // Check if followed by whitespace or quote or end of text
            let next_idx = end_punct + 1;
            let followed_by_quote = next_idx < len && (chars[next_idx] == '"' || chars[next_idx] == '”' || chars[next_idx] == '’');
            let check_idx = if followed_by_quote { next_idx + 1 } else { next_idx };

            let is_boundary = check_idx >= len || chars[check_idx].is_whitespace();

            if is_boundary {
                let slice: String = chars[start..=if followed_by_quote { next_idx } else { end_punct }].iter().collect();

                // Verify it's not a common abbreviation (like "np.", "dr.", "Mr.")
                if !is_abbreviation(&slice) {
                    let trimmed = slice.trim().to_string();
                    if !trimmed.is_empty() {
                        sentences.push(trimmed);
                    }
                    start = if followed_by_quote { next_idx + 1 } else { next_idx };
                    i = start;
                    continue;
                }
            }
            i = end_punct + 1;
        } else {
            i += 1;
        }
    }

    if start < len {
        let remaining: String = chars[start..len].iter().collect();
        let trimmed = remaining.trim().to_string();
        if !trimmed.is_empty() {
            sentences.push(trimmed);
        }
    }

    sentences
}

fn is_sentence_terminator(c: char) -> bool {
    c == '.' || c == '!' || c == '?' || c == '…'
}

fn is_abbreviation(text: &str) -> bool {
    let lower = text.trim().to_lowercase();
    const ABBREVIATIONS: &[&str] = &[
        "np.", "tzn.", "itd.", "itp.", "m.in.", "dr.", "prof.", "ul.", "al.", "pl.",
        "nr.", "art.", "str.", "godz.", "min.", "sek.", "tys.", "mln.", "mld.",
        "mr.", "mrs.", "ms.", "dr.", "vs.", "etc.", "e.g.", "i.e.", "approx.",
    ];

    for abbr in ABBREVIATIONS {
        if lower.ends_with(abbr) {
            return true;
        }
    }

    // Number followed by a dot (e.g. "1." or "3.14")
    if let Some(last_word) = lower.split_whitespace().last() {
        let without_dot = last_word.trim_end_matches('.');
        if !without_dot.is_empty() && without_dot.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }

    false
}

fn split_long_sentence(sentence: &str, max_chars: usize) -> Vec<String> {
    let mut parts = Vec::new();
    let separators = ["; ", ", ", " - ", ": "];

    let mut current = sentence;

    while current.len() > max_chars {
        let mut split_pos = None;

        for sep in separators {
            if let Some(pos) = current[..max_chars].rfind(sep) {
                split_pos = Some(pos + sep.len());
                break;
            }
        }

        // Fallback: split on nearest space before max_chars
        let split_at = split_pos.or_else(|| current[..max_chars].rfind(' ').map(|p| p + 1));

        match split_at {
            Some(pos) if pos > 0 => {
                parts.push(current[..pos].trim().to_string());
                current = current[pos..].trim();
            }
            _ => {
                // If no space found, force split at max_chars
                parts.push(current[..max_chars].trim().to_string());
                current = current[max_chars..].trim();
            }
        }
    }

    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_simple_sentences() {
        let text = "Hello world! This is a test. Another sentence here?";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0], "Hello world!");
        assert_eq!(sentences[1], "This is a test.");
        assert_eq!(sentences[2], "Another sentence here?");
    }

    #[test]
    fn test_abbreviations_not_split() {
        let text = "Poszedł do dr. Kowalskiego po receptę. Było np. pięć osób w kolejce.";
        let sentences = split_sentences(text);
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[0], "Poszedł do dr. Kowalskiego po receptę.");
        assert_eq!(sentences[1], "Było np. pięć osób w kolejce.");
    }

    #[test]
    fn test_chunks_grouping() {
        let text = "Short one. Short two. Short three.";
        let config = ChunkerConfig {
            max_chars: 100,
            min_chars: 10,
        };
        let chunks = split_into_chunks(text, &config);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], "Short one. Short two. Short three.");
    }

    #[test]
    fn test_long_sentence_splitting() {
        let text = "This is a very long sentence that has multiple clauses, connected by commas, so that we can test if the chunker properly breaks it down into smaller parts when exceeding the limit.";
        let config = ChunkerConfig {
            max_chars: 80,
            min_chars: 10,
        };
        let chunks = split_into_chunks(text, &config);
        assert!(chunks.len() > 1);
        for chunk in &chunks {
            assert!(chunk.len() <= 80, "Chunk exceeded max_chars: '{}'", chunk);
        }
    }
}
