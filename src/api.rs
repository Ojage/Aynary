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

        // Fall back to prefix matches if no exact entry exists
        if matches.is_empty() {
            matches = OFFLINE_ENTRIES
                .iter()
                .filter(|entry| entry.word.to_lowercase().starts_with(&lower))
                .cloned()
                .collect();
        }

        if matches.is_empty() {
            anyhow::bail!("No offline entry found for '{}'.", term);
        }

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
    let raw = include_str!("../data/dictionary.json");
    serde_json::from_str(raw)
        .context("Failed to parse bundled offline dictionary data")
        .unwrap_or_else(|_| Vec::new())
}
