#!/usr/bin/env python3
"""
Convert dict.csv to dictionary.json format for Aynary dictionary app.

This script reads the CSV file with word,definition columns and converts
it to the JSON format expected by the Aynary application.
"""

import csv
import json
import sys
from pathlib import Path

def convert_csv_to_json(csv_path: Path, json_path: Path):
    """Convert CSV dictionary file to JSON format."""
    entries = []
    
    print(f"Reading CSV from {csv_path}...")
    with open(csv_path, 'r', encoding='utf-8') as f:
        reader = csv.DictReader(f)
        row_count = 0
        
        for row in reader:
            word = row['word'].strip()
            definition = row['definition'].strip()
            
            # Skip empty entries
            if not word or not definition:
                continue
            
            # Create DictionaryEntry structure
            entry = {
                "word": word,
                "phonetic": None,
                "phonetics": [],
                "meanings": [
                    {
                        "partOfSpeech": "noun",  # Generic default since CSV doesn't specify
                        "definitions": [
                            {
                                "definition": definition,
                                "synonyms": [],
                                "antonyms": [],
                                "example": None
                            }
                        ],
                        "synonyms": [],
                        "antonyms": []
                    }
                ],
                "license": None,
                "sourceUrls": []
            }
            
            entries.append(entry)
            row_count += 1
            
            # Progress indicator for large files
            if row_count % 10000 == 0:
                print(f"Processed {row_count} entries...")
    
    print(f"Total entries: {row_count}")
    print(f"Writing JSON to {json_path}...")
    
    # Write JSON file
    with open(json_path, 'w', encoding='utf-8') as f:
        json.dump(entries, f, indent=2, ensure_ascii=False)
    
    # Get file size
    json_size = json_path.stat().st_size
    print(f"Conversion complete! JSON file size: {json_size / 1024 / 1024:.2f} MB")
    
    return row_count

def main():
    # Determine paths
    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    csv_path = project_root / "data" / "dict.csv"
    json_path = project_root / "data" / "dictionary.json"
    
    # Check if CSV exists
    if not csv_path.exists():
        print(f"Error: CSV file not found at {csv_path}", file=sys.stderr)
        sys.exit(1)
    
    # Backup existing JSON if it exists
    if json_path.exists():
        backup_path = json_path.with_suffix('.json.bak')
        print(f"Backing up existing dictionary.json to {backup_path.name}...")
        json_path.rename(backup_path)
    
    # Convert
    try:
        count = convert_csv_to_json(csv_path, json_path)
        print(f"\nSuccessfully converted {count} entries!")
        print(f"Output written to: {json_path}")
    except Exception as e:
        print(f"Error during conversion: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()

