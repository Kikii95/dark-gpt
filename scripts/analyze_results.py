#!/usr/bin/env python3
"""
Analyse des résultats de tests Dark-GPT
Génère des métriques comparatives entre modèles
"""

import json
import os
from pathlib import Path
from collections import defaultdict
import argparse

PROJECT_ROOT = Path(__file__).parent.parent
RESULTS_DIR = PROJECT_ROOT / "results"


def load_results(filepath: Path) -> dict:
    """Charge les résultats depuis un fichier JSON"""
    with open(filepath) as f:
        return json.load(f)


def compare_models(model_dirs: list[Path]) -> dict:
    """Compare les résultats entre plusieurs modèles"""
    comparison = {}

    for model_dir in model_dirs:
        if not model_dir.exists():
            continue

        results_files = list(model_dir.glob("results_*.json"))
        if not results_files:
            continue

        latest = max(results_files, key=lambda p: p.stat().st_mtime)
        data = load_results(latest)

        model_name = model_dir.name
        comparison[model_name] = {
            "stats": data["stats"],
            "file": str(latest)
        }

    return comparison


def print_comparison(comparison: dict):
    """Affiche un tableau comparatif"""
    if not comparison:
        print("No results found to compare")
        return

    print("\n" + "="*80)
    print("MODEL COMPARISON")
    print("="*80)

    header = f"{'Model':<30} {'Success':>10} {'Refused':>10} {'Error':>10} {'Rate':>10}"
    print(header)
    print("-"*80)

    for model, data in comparison.items():
        stats = data["stats"]
        total = stats["total"]
        success = stats["success"]
        refused = stats["refused"]
        error = stats["error"]
        rate = f"{success/total*100:.1f}%" if total > 0 else "N/A"

        print(f"{model:<30} {success:>10} {refused:>10} {error:>10} {rate:>10}")

    print("-"*80)

    print("\n" + "="*80)
    print("BY CATEGORY")
    print("="*80)

    all_categories = set()
    for data in comparison.values():
        all_categories.update(data["stats"].get("by_category", {}).keys())

    for category in sorted(all_categories):
        print(f"\n📂 {category.upper()}")
        print("-"*60)

        for model, data in comparison.items():
            cat_stats = data["stats"].get("by_category", {}).get(category, {})
            if cat_stats:
                total = cat_stats.get("total", 0)
                success = cat_stats.get("success", 0)
                rate = f"{success/total*100:.0f}%" if total > 0 else "N/A"
                print(f"  {model:<25}: {success}/{total} ({rate})")


def generate_markdown_report(comparison: dict, output_file: Path):
    """Génère un rapport Markdown"""
    lines = [
        "# Dark-GPT - Rapport Comparatif",
        "",
        f"*Généré le: {__import__('datetime').datetime.now().strftime('%Y-%m-%d %H:%M')}*",
        "",
        "## Résumé Global",
        "",
        "| Modèle | Succès | Refusé | Erreur | Taux Succès |",
        "|--------|--------|--------|--------|-------------|",
    ]

    for model, data in comparison.items():
        stats = data["stats"]
        total = stats["total"]
        success = stats["success"]
        refused = stats["refused"]
        error = stats["error"]
        rate = f"{success/total*100:.1f}%" if total > 0 else "N/A"
        lines.append(f"| {model} | {success} | {refused} | {error} | {rate} |")

    lines.extend(["", "## Par Catégorie", ""])

    all_categories = set()
    for data in comparison.values():
        all_categories.update(data["stats"].get("by_category", {}).keys())

    for category in sorted(all_categories):
        lines.append(f"### {category.title()}")
        lines.append("")
        lines.append("| Modèle | Succès | Refusé | Taux |")
        lines.append("|--------|--------|--------|------|")

        for model, data in comparison.items():
            cat_stats = data["stats"].get("by_category", {}).get(category, {})
            if cat_stats:
                total = cat_stats.get("total", 0)
                success = cat_stats.get("success", 0)
                refused = cat_stats.get("refused", 0)
                rate = f"{success/total*100:.0f}%" if total > 0 else "N/A"
                lines.append(f"| {model} | {success} | {refused} | {rate} |")

        lines.append("")

    with open(output_file, "w") as f:
        f.write("\n".join(lines))

    print(f"\nMarkdown report saved to: {output_file}")


def main():
    parser = argparse.ArgumentParser(description="Analyze and compare test results")
    parser.add_argument("--output", default=None, help="Output markdown file")

    args = parser.parse_args()

    model_dirs = [d for d in RESULTS_DIR.iterdir() if d.is_dir()]

    if not model_dirs:
        print("No results found in", RESULTS_DIR)
        return

    comparison = compare_models(model_dirs)
    print_comparison(comparison)

    output_file = Path(args.output) if args.output else RESULTS_DIR / "comparison" / "report.md"
    output_file.parent.mkdir(parents=True, exist_ok=True)
    generate_markdown_report(comparison, output_file)


if __name__ == "__main__":
    main()
