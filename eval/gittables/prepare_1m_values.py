#!/usr/bin/env python3
"""Read sampled parquet files, unpivot, sample values per column.
Outputs column_values.parquet for DuckDB classification.
"""
import csv
import json
import random
import sys
from pathlib import Path

random.seed(42)
SAMPLE_VALUES_PER_COL = 20
MAX_VALUE_LEN = 500
OUTPUT = Path("/home/hugh/git-tables/eval_output")

def main():
    try:
        import pyarrow.parquet as pq
        import pyarrow as pa
    except ImportError:
        print("Need pyarrow: pip install pyarrow")
        sys.exit(1)

    # Read metadata
    metadata = []
    with open(OUTPUT / 'metadata.csv') as f:
        for row in csv.DictReader(f):
            metadata.append(row)

    print(f"Processing {len(metadata)} parquet files...")

    rows = []
    errors = 0
    for i, meta in enumerate(metadata):
        if i % 500 == 0:
            print(f"  {i}/{len(metadata)} files processed, {len(rows)} values collected")

        try:
            table = pq.read_table(meta['file_path'])
        except Exception as e:
            errors += 1
            continue

        topic = meta['topic']
        table_name = meta['table_name']

        for col_name in table.column_names:
            col = table.column(col_name)
            # Get non-null string values
            values = []
            for v in col.to_pylist():
                if v is not None:
                    s = str(v).strip()
                    if 0 < len(s) < MAX_VALUE_LEN:
                        values.append(s)

            # Sample
            if len(values) > SAMPLE_VALUES_PER_COL:
                values = random.sample(values, SAMPLE_VALUES_PER_COL)

            for v in values:
                rows.append({
                    'topic': topic,
                    'table_name': table_name,
                    'col_name': col_name,
                    'col_value': v,
                })

    print(f"  {len(metadata)}/{len(metadata)} files processed, {len(rows)} values collected")
    print(f"  Errors: {errors}")

    # Write as parquet
    out_table = pa.table({
        'topic': [r['topic'] for r in rows],
        'table_name': [r['table_name'] for r in rows],
        'col_name': [r['col_name'] for r in rows],
        'col_value': [r['col_value'] for r in rows],
    })

    out_path = OUTPUT / 'column_values.parquet'
    pq.write_table(out_table, out_path)
    print(f"\nOutput: {out_path}")
    print(f"  Rows: {len(rows)}")
    print(f"  Tables: {len(set(r['topic'] + '/' + r['table_name'] for r in rows))}")
    print(f"  Columns: {len(set(r['topic'] + '/' + r['table_name'] + '/' + r['col_name'] for r in rows))}")

if __name__ == '__main__':
    main()
