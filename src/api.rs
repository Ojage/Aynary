use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static OFFLINE_ENTRIES: Lazy<Vec<DictionaryEntry>> = Lazy::new(load_dictionary);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub phonetic: Option<String>,
    pub phonetics: Vec<Phonetic>,
    pub meanings: Vec<Meaning>,
    pub license: Option<License>,
    #[serde(rename = "sourceUrls")]
    pub source_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phonetic {
    pub text: Option<String>,
    pub audio: Option<String>,
    #[serde(rename = "sourceUrl")]
    pub source_url: Option<String>,
    pub license: Option<License>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definition {
    pub definition: String,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: String,
}

#[derive(Clone)]
pub struct DictionaryClient {
}

impl DictionaryClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn lookup(&self, word: &str) -> Result<Vec<DictionaryEntry>> {
        // #region agent log
        use std::fs::OpenOptions;
        use std::io::Write;
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
            let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"D\",\"location\":\"api.rs:59\",\"message\":\"lookup called\",\"data\":{{\"word\":\"{}\",\"offline_entries_count\":{}}},\"timestamp\":{}}}", word, OFFLINE_ENTRIES.len(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
        }
        // #endregion
        let term = word.trim();
        if term.is_empty() {
            anyhow::bail!("Please enter a word to look up.");
        }

        let lower = term.to_lowercase();

        // Prefer exact matches first
        let mut matches: Vec<DictionaryEntry> = OFFLINE_ENTRIES
            .iter()
            .filter(|entry| entry.word.eq_ignore_ascii_case(term))
            .cloned()
            .collect();

        // #region agent log
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
            let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"D\",\"location\":\"api.rs:70\",\"message\":\"Exact match search complete\",\"data\":{{\"word\":\"{}\",\"match_count\":{}}},\"timestamp\":{}}}", word, matches.len(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
        }
        // #endregion

        // Fall back to prefix matches if no exact entry exists
        if matches.is_empty() {
            matches = OFFLINE_ENTRIES
                .iter()
                .filter(|entry| entry.word.to_lowercase().starts_with(&lower))
                .cloned()
                .collect();
            // #region agent log
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
                let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"D\",\"location\":\"api.rs:78\",\"message\":\"Prefix match search complete\",\"data\":{{\"word\":\"{}\",\"match_count\":{}}},\"timestamp\":{}}}", word, matches.len(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
            }
            // #endregion
        }

        if matches.is_empty() {
            // #region agent log
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
                let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"B\",\"location\":\"api.rs:84\",\"message\":\"No offline entry found\",\"data\":{{\"word\":\"{}\",\"offline_entries_count\":{}}},\"timestamp\":{}}}", word, OFFLINE_ENTRIES.len(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
            }
            // #endregion
            anyhow::bail!("No offline entry found for '{}'.", term);
        }

        // #region agent log
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
            let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"D\",\"location\":\"api.rs:92\",\"message\":\"lookup returning matches\",\"data\":{{\"word\":\"{}\",\"match_count\":{}}},\"timestamp\":{}}}", word, matches.len(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
        }
        // #endregion

        Ok(matches)
    }

    pub fn format_entry(&self, entries: &[DictionaryEntry]) -> String {
        if entries.is_empty() {
            return String::from("No definitions found.");
        }

        let mut formatted = String::new();

        for entry in entries {
            formatted.push_str(&format!("{}\n", entry.word));

            if let Some(phonetic) = &entry.phonetic {
                formatted.push_str(&format!("{}\n\n", phonetic));
            }

            for meaning in &entry.meanings {
                formatted.push_str(&format!(
                    "{}\n",
                    meaning.part_of_speech.replace('-', " ")
                ));

                for (idx, definition) in meaning.definitions.iter().enumerate() {
                    formatted.push_str(&format!("{}. {}\n", idx + 1, definition.definition));

                    if let Some(example) = &definition.example {
                        formatted.push_str(&format!("   Example: {}\n", example));
                    }
                }

                if !meaning.synonyms.is_empty() {
                    formatted.push_str(&format!(
                        "   Synonyms: {}\n",
                        meaning.synonyms.join(", ")
                    ));
                }

                formatted.push_str("\n");
            }
        }

        formatted
    }
}

impl Default for DictionaryClient {
    fn default() -> Self {
        Self::new()
    }
}

fn load_dictionary() -> Vec<DictionaryEntry> {
    // #region agent log
    use std::fs::OpenOptions;
    use std::io::Write;
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
        let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"B\",\"location\":\"api.rs:139\",\"message\":\"load_dictionary called\",\"data\":{{\"timestamp\":{}}},\"timestamp\":{}}}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
    }
    // #endregion
    let raw = include_str!("../data/dictionary.json");
    // #region agent log
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
        let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"C\",\"location\":\"api.rs:141\",\"message\":\"Dictionary data loaded from include_str\",\"data\":{{\"raw_length\":{},\"first_100_chars\":\"{}\"}},\"timestamp\":{}}}", raw.len(), raw.chars().take(100).collect::<String>().replace("\"", "\\\"").replace("\n", "\\n"), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
    }
    // #endregion
    let result: Result<Vec<DictionaryEntry>, _> = serde_json::from_str(raw)
        .context("Failed to parse bundled offline dictionary data");
    // #region agent log
    match &result {
        Ok(entries) => {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
                let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"B\",\"location\":\"api.rs:144\",\"message\":\"Dictionary parsed successfully\",\"data\":{{\"entry_count\":{}}},\"timestamp\":{}}}", entries.len(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
            }
        }
        Err(e) => {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
                let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"B\",\"location\":\"api.rs:147\",\"message\":\"Dictionary parse failed\",\"data\":{{\"error\":\"{}\"}},\"timestamp\":{}}}", format!("{:?}", e).replace("\"", "\\\"").replace("\n", "\\n"), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
            }
        }
    }
    // #endregion
    result.unwrap_or_else(|e| {
        // #region agent log
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/home/salathiel/House Projects/.cursor/debug.log") {
            let _ = writeln!(file, "{{\"sessionId\":\"debug-session\",\"runId\":\"run1\",\"hypothesisId\":\"E\",\"location\":\"api.rs:152\",\"message\":\"Dictionary load returning empty Vec due to error\",\"data\":{{\"error\":\"{}\"}},\"timestamp\":{}}}", format!("{:?}", e).replace("\"", "\\\"").replace("\n", "\\n"), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
        }
        // #endregion
        Vec::new()
    })
}
