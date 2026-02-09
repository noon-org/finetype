#!/usr/bin/env python3
"""
Merge duplicate taxonomy classes and document aliases.

This script updates the taxonomy to:
1. Merge semantic duplicates (federal_subject/province/state/region → subdivision)
2. Merge calling_code/isd_code → calling_code
3. Merge timestamp/unix_timestamp → unix_timestamp
4. Remove ambiguous person.first_name and person.last_name
5. Document aliases for all merged classes
"""

import yaml
from pathlib import Path

def main():
    taxonomy_path = Path(__file__).parent.parent / "labels" / "definitions.yaml"
    
    with open(taxonomy_path) as f:
        taxonomy = yaml.safe_load(f)
    
    # Track changes
    removed = []
    merged = []
    
    # 1. Merge address subdivisions → address.subdivision
    subdivision_classes = [
        "address.federal_subject",
        "address.province", 
        "address.state",
        "address.region",
        "address.prefecture",
    ]
    
    # Keep address.subdivision as the canonical, merge others into aliases
    if "address.province" in taxonomy:
        # Use province as base (most samples)
        base = taxonomy["address.province"].copy()
        base["aliases"] = ["federal_subject", "state", "region", "prefecture"]
        base["title"] = "Administrative subdivision (state, province, region, prefecture, etc.)"
        base["description"] = "A sub-national administrative division. Aliases: federal_subject, state, region, prefecture."
        taxonomy["address.subdivision"] = base
        merged.append("address.subdivision (from province, state, region, federal_subject, prefecture)")
        
        # Remove the old entries
        for cls in subdivision_classes:
            if cls in taxonomy:
                del taxonomy[cls]
                removed.append(cls)
    
    # 2. Merge calling_code/isd_code → calling_code
    if "address.calling_code" in taxonomy and "address.isd_code" in taxonomy:
        base = taxonomy["address.calling_code"].copy()
        base["aliases"] = ["isd_code"]
        base["description"] = "International calling code (ISD code). Alias: isd_code."
        taxonomy["address.calling_code"] = base
        del taxonomy["address.isd_code"]
        removed.append("address.isd_code")
        merged.append("address.calling_code (absorbed isd_code)")
    
    # 3. Merge timestamp variants → datetime.unix_timestamp
    timestamp_classes = ["datetime.timestamp", "datetime.unix_timestamp"]
    if "datetime.unix_timestamp" in taxonomy:
        base = taxonomy["datetime.unix_timestamp"].copy()
        base["aliases"] = ["timestamp"]
        base["description"] = "Unix timestamp (seconds since epoch). Alias: timestamp."
        taxonomy["datetime.unix_timestamp"] = base
        if "datetime.timestamp" in taxonomy:
            del taxonomy["datetime.timestamp"]
            removed.append("datetime.timestamp")
            merged.append("datetime.unix_timestamp (absorbed timestamp)")
    
    # 4. Remove ambiguous person names
    ambiguous_classes = ["person.first_name", "person.last_name"]
    for cls in ambiguous_classes:
        if cls in taxonomy:
            del taxonomy[cls]
            removed.append(cls)
    
    # 5. Consider EAN vs Unix epoch milliseconds
    # Keep both but add notes about confusion
    if "code.ean" in taxonomy:
        taxonomy["code.ean"]["notes"] = "EAN-8 (8 digits) and EAN-13 (13 digits). Note: EAN-13 may be confused with unix_epoch_in_milliseconds."
    
    if "datetime.unix_epoch_in_milliseconds" in taxonomy:
        taxonomy["datetime.unix_epoch_in_milliseconds"]["notes"] = "Unix timestamp in milliseconds (13 digits). Note: May be confused with EAN-13 barcodes."
    
    # Write updated taxonomy
    output_path = Path(__file__).parent.parent / "labels" / "definitions_v2.yaml"
    with open(output_path, "w") as f:
        yaml.dump(taxonomy, f, default_flow_style=False, allow_unicode=True, sort_keys=True, width=120)
    
    print("=== TAXONOMY MERGE SUMMARY ===")
    print(f"\nRemoved classes ({len(removed)}):")
    for cls in removed:
        print(f"  - {cls}")
    
    print(f"\nMerged classes ({len(merged)}):")
    for desc in merged:
        print(f"  + {desc}")
    
    print(f"\nOriginal classes: {len(taxonomy) + len(removed)}")
    print(f"New classes: {len(taxonomy)}")
    print(f"\nOutput: {output_path}")

if __name__ == "__main__":
    main()
