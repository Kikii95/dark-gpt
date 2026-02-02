#!/usr/bin/env python3
"""
Génération de graphiques comparatifs pour Dark-GPT
Utilise matplotlib pour créer des visualisations
"""

import json
from pathlib import Path
import argparse

PROJECT_ROOT = Path(__file__).parent.parent
RESULTS_DIR = PROJECT_ROOT / "results"
CHARTS_DIR = PROJECT_ROOT / "charts"


def check_matplotlib():
    """Vérifie si matplotlib est installé"""
    try:
        import matplotlib
        return True
    except ImportError:
        print("matplotlib not installed. Install with: pip install matplotlib")
        return False


def load_latest_results(model_dir: Path) -> dict | None:
    """Charge les derniers résultats d'un modèle"""
    results_files = list(model_dir.glob("results_*.json"))
    if not results_files:
        return None

    latest = max(results_files, key=lambda p: p.stat().st_mtime)
    with open(latest) as f:
        return json.load(f)


def generate_comparison_bar_chart(comparison: dict, output_file: Path):
    """Génère un bar chart comparatif"""
    import matplotlib.pyplot as plt

    models = list(comparison.keys())
    success_rates = []
    refused_rates = []
    error_rates = []

    for model in models:
        stats = comparison[model]["stats"]
        total = stats["total"]
        success_rates.append(stats["success"] / total * 100 if total > 0 else 0)
        refused_rates.append(stats["refused"] / total * 100 if total > 0 else 0)
        error_rates.append(stats["error"] / total * 100 if total > 0 else 0)

    x = range(len(models))
    width = 0.25

    fig, ax = plt.subplots(figsize=(12, 6))

    bars1 = ax.bar([i - width for i in x], success_rates, width, label='Success', color='#22c55e')
    bars2 = ax.bar(x, refused_rates, width, label='Refused', color='#f59e0b')
    bars3 = ax.bar([i + width for i in x], error_rates, width, label='Error', color='#ef4444')

    ax.set_xlabel('Model')
    ax.set_ylabel('Percentage (%)')
    ax.set_title('Dark-GPT: Model Comparison - Response Rates')
    ax.set_xticks(x)
    ax.set_xticklabels(models, rotation=45, ha='right')
    ax.legend()
    ax.set_ylim(0, 100)

    for bars in [bars1, bars2, bars3]:
        for bar in bars:
            height = bar.get_height()
            if height > 0:
                ax.annotate(f'{height:.1f}%',
                           xy=(bar.get_x() + bar.get_width() / 2, height),
                           xytext=(0, 3),
                           textcoords="offset points",
                           ha='center', va='bottom', fontsize=8)

    plt.tight_layout()
    plt.savefig(output_file, dpi=150, bbox_inches='tight')
    plt.close()

    print(f"Bar chart saved to: {output_file}")


def generate_category_heatmap(comparison: dict, output_file: Path):
    """Génère une heatmap par catégorie"""
    import matplotlib.pyplot as plt
    import numpy as np

    all_categories = set()
    for data in comparison.values():
        all_categories.update(data["stats"].get("by_category", {}).keys())

    categories = sorted(all_categories)
    models = list(comparison.keys())

    matrix = np.zeros((len(categories), len(models)))

    for j, model in enumerate(models):
        for i, category in enumerate(categories):
            cat_stats = comparison[model]["stats"].get("by_category", {}).get(category, {})
            if cat_stats:
                total = cat_stats.get("total", 0)
                success = cat_stats.get("success", 0)
                matrix[i, j] = (success / total * 100) if total > 0 else 0

    fig, ax = plt.subplots(figsize=(10, 8))

    im = ax.imshow(matrix, cmap='RdYlGn', aspect='auto', vmin=0, vmax=100)

    ax.set_xticks(range(len(models)))
    ax.set_yticks(range(len(categories)))
    ax.set_xticklabels(models, rotation=45, ha='right')
    ax.set_yticklabels(categories)

    for i in range(len(categories)):
        for j in range(len(models)):
            text = ax.text(j, i, f'{matrix[i, j]:.0f}%',
                          ha='center', va='center', color='black', fontsize=10)

    ax.set_title('Dark-GPT: Success Rate by Category and Model')
    fig.colorbar(im, ax=ax, label='Success Rate (%)')

    plt.tight_layout()
    plt.savefig(output_file, dpi=150, bbox_inches='tight')
    plt.close()

    print(f"Heatmap saved to: {output_file}")


def main():
    parser = argparse.ArgumentParser(description="Generate comparison charts")
    args = parser.parse_args()

    if not check_matplotlib():
        return

    model_dirs = [d for d in RESULTS_DIR.iterdir() if d.is_dir() and d.name != "comparison"]

    if not model_dirs:
        print("No results found to visualize")
        return

    comparison = {}
    for model_dir in model_dirs:
        data = load_latest_results(model_dir)
        if data:
            comparison[model_dir.name] = data

    if not comparison:
        print("No valid results found")
        return

    CHARTS_DIR.mkdir(parents=True, exist_ok=True)

    generate_comparison_bar_chart(comparison, CHARTS_DIR / "comparison_bar.png")
    generate_category_heatmap(comparison, CHARTS_DIR / "category_heatmap.png")

    print(f"\nAll charts saved to: {CHARTS_DIR}")


if __name__ == "__main__":
    main()
