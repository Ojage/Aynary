use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const DICTIONARY_API_BASE: &str = "https://api.dictionaryapi.dev/api/v2/entries/en";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub phonetic: Option<String>,
    pub phonetics: Vec<Phonetic>,
    pub meanings: Vec<Meaning>,
    pub license: Option<License>,
    pub source_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phonetic {
    pub text: Option<String>,
    pub audio: Option<String>,
    pub source_url: Option<String>,
    pub license: Option<License>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meaning {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub title: String,
    pub message: String,
    pub resolution: Option<String>,
}

#[derive(Clone)]
pub struct DictionaryClient {
    client: reqwest::Client,
}

impl DictionaryClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn lookup(&self, word: &str) -> Result<Vec<DictionaryEntry>> {
        let word_encoded = urlencoding::encode(word);
        let url = format!("{}/{}", DICTIONARY_API_BASE, word_encoded);

        let response = self
            client
            .get(&url)
            .send()
            .await
            .context("Failed to send request to dictionary API")?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            let error: ErrorResponse = response
                .json()
                .await
                .context("Failed to parse error response")?;
            return Err(anyhow::anyhow!("{}: {}", error.title, error.message));
        }

        response
            .error_for_status()
            .context("Dictionary API returned an error")?
            .json::<Vec<DictionaryEntry>>()
            .await
            .context("Failed to parse dictionary response")
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

