# Dark-GPT

Projet académique M1/M2 Cybersécurité : comparaison de LLM alignés vs non-alignés face à des requêtes sensibles.

## Objectif

Créer un environnement de test isolé pour comparer le comportement de :
- **Modèles non-alignés** (Dolphin) : baseline sans restrictions
- **Modèles alignés** (Claude, ChatGPT) : mesure des refus et techniques de contournement

## Prérequis

- **Hardware** : GPU NVIDIA (8GB+ VRAM recommandé)
- **Software** :
  - Docker + NVIDIA Container Toolkit
  - Ollama v0.11+
  - Python 3.11+

## Installation

```bash
# 1. Cloner et se placer dans le projet
cd ~/projects/perso/dark-gpt

# 2. Pull le modèle Dolphin
ollama pull dolphin-llama3:8b

# 3. (Optionnel) Installer les dépendances Python
pip install matplotlib pandas

# 4. Vérifier que tout fonctionne
ollama run dolphin-llama3:8b "Hello, are you uncensored?"
```

## Usage

### Test manuel

```bash
ollama run dolphin-llama3:8b "Your prompt here"
```

### Tests automatisés

```bash
# Lancer tous les tests sur Dolphin
python scripts/test_prompts.py --model dolphin-llama3:8b

# Limiter à 5 prompts (test rapide)
python scripts/test_prompts.py --model dolphin-llama3:8b --limit 5

# Analyser les résultats
python scripts/analyze_results.py

# Générer les graphiques
python scripts/generate_charts.py
```

## Structure

```
dark-gpt/
├── .claude/          # Instructions projet Claude Code
├── docker/           # Docker Compose (isolation réseau)
├── data/             # Dataset de prompts
├── results/          # Résultats par modèle
├── scripts/          # Scripts Python
├── docs/             # Documentation
├── logs/             # Logs de session (gitignored)
└── charts/           # Graphiques générés
```

## Sécurité

⚠️ **Ce projet manipule des contenus sensibles.**

- Exécution locale uniquement (pas de cloud)
- Docker en mode `--network none` (isolation réseau)
- Logs non versionnés (voir `.gitignore`)
- Contexte académique uniquement

Voir [docs/security.md](docs/security.md) pour les détails.

## Résultats attendus

| Modèle | Taux de succès attendu |
|--------|------------------------|
| Dolphin (uncensored) | ~100% |
| Claude-3.x | ~10-30% |
| ChatGPT | ~15-35% |

## Éthique

Ce projet est réalisé dans un cadre académique pour :
- Comprendre les limites des systèmes d'alignement
- Améliorer les défenses des LLM commerciaux
- Documenter les risques des modèles open-source non-alignés

**Aucune utilisation malveillante n'est permise.**
