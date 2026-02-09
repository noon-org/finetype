#!/usr/bin/env python3
"""Fix taxonomy method fields to match keys."""

import yaml
from pathlib import Path

def main():
    taxonomy_path = Path(__file__).parent.parent / "labels" / "definitions.yaml"
    
    with open(taxonomy_path) as f:
        taxonomy = yaml.safe_load(f)
    
    fixes = []
    for key, defn in taxonomy.items():
        provider, method = key.split('.', 1)
        if defn.get('provider') != provider or defn.get('method') != method:
            old_provider = defn.get('provider')
            old_method = defn.get('method')
            defn['provider'] = provider
            defn['method'] = method
            fixes.append(f"{key}: {old_provider}.{old_method} → {provider}.{method}")
    
    with open(taxonomy_path, "w") as f:
        yaml.dump(taxonomy, f, default_flow_style=False, allow_unicode=True, sort_keys=True, width=120)
    
    print(f"Fixed {len(fixes)} entries:")
    for fix in fixes:
        print(f"  {fix}")

if __name__ == "__main__":
    main()
