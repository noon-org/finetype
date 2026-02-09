#!/usr/bin/env python3
"""Evaluate CharCNN model on test set and generate confusion matrix."""

import json
import subprocess
from collections import defaultdict
import sys

def main():
    # Read test data
    print("Loading test data...", file=sys.stderr)
    test_samples = []
    with open("data/test.ndjson") as f:
        for line in f:
            if line.strip():
                record = json.loads(line)
                test_samples.append({
                    "text": record["text"],
                    "label": record["classification"]
                })
    
    print(f"Loaded {len(test_samples)} test samples", file=sys.stderr)
    
    # Write texts to temp file
    with open("/tmp/char_test_texts.txt", "w") as f:
        for sample in test_samples:
            f.write(sample["text"] + "\n")
    
    # Run predictions using the CLI (we need to update CLI to support char model)
    # For now, let's output the data for analysis
    
    # Get actual labels
    actual_labels = [s["label"] for s in test_samples]
    
    # Count by class
    class_counts = defaultdict(int)
    for label in actual_labels:
        class_counts[label] += 1
    
    print(f"\nTest set class distribution ({len(class_counts)} classes):")
    for label, count in sorted(class_counts.items(), key=lambda x: -x[1])[:20]:
        print(f"  {label}: {count}")
    
    print(f"\n... and {len(class_counts) - 20} more classes")
    
    # Save for external analysis
    with open("/tmp/char_test_actual.txt", "w") as f:
        for label in actual_labels:
            f.write(label + "\n")
    
    print(f"\nActual labels saved to /tmp/char_test_actual.txt")
    print(f"Test texts saved to /tmp/char_test_texts.txt")

if __name__ == "__main__":
    main()
