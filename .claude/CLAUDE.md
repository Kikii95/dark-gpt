# Dark-GPT - Instructions Projet

## 🎯 Objectif

Projet académique M1/M2 Cybersécurité : créer et documenter un LLM local sans restrictions pour comparer avec les modèles alignés (Claude, ChatGPT).

## 🏗️ Architecture

```
dark-gpt/
├── docker/          # Ollama isolé (--network none)
├── data/            # Dataset de test (prompts sensibles)
├── results/         # Résultats par modèle
├── scripts/         # Scripts de test automatisés
├── docs/            # Documentation + rapport
├── logs/            # Logs de session JSONL
└── charts/          # Graphiques comparatifs
```

## 🔧 Stack Technique

| Composant | Technologie                                   |
| --------- | --------------------------------------------- |
| LLM Local | Ollama + dolphin-llama3:8b                    |
| Isolation | Docker (--network none)                       |
| Scripts   | Python 3.11+                                  |
| Logs      | JSONL (timestamp, prompt, response, category) |
| Analyse   | pandas + matplotlib                           |

## 🛡️ Règles de Sécurité

1. **JAMAIS de vraies PII** dans les tests
2. **Docker isolé** (--network none) obligatoire
3. **Logs locaux uniquement** (pas de cloud logging)
4. **Cleanup** : script de purge après rapport final
5. **Cadre académique** : aucune cible réelle

## 📚 Docs à Maintenir

### Docs Git (source) → sync Obsidian

| Doc       | Chemin Git     | Quand mettre à jour |
| --------- | -------------- | ------------------- |
| Backlog   | `BACKLOG.md`   | Nouvelles tâches    |
| Roadmap   | `ROADMAP.md`   | Planning            |
| Changelog | `CHANGELOG.md` | Chaque milestone    |

### Docs Obsidian-only

| Doc         | Chemin Obsidian                                           | Quand mettre à jour |
| ----------- | --------------------------------------------------------- | ------------------- |
| Faisabilité | `~/obsidian-vault/Projects/Perso/dark-gpt/Faisabilite.md` | Jamais (figé)       |
| Results     | `~/obsidian-vault/Projects/Perso/dark-gpt/Results.md`     | Après tests         |
| Logs        | `~/obsidian-vault/Projects/Perso/dark-gpt/_Logs/`         | Chaque session      |

## 🚀 Commandes Rapides

```bash
# Démarrer Ollama isolé
docker compose -f docker/docker-compose.yml up -d

# Tester le modèle
ollama run dolphin-llama3

# Lancer tests automatisés
python scripts/test_prompts.py

# Analyser résultats
python scripts/analyze_results.py

# Générer graphiques
python scripts/generate_charts.py
```

## ⚠️ Rappels

- Projet **perso** (pas école) → auto-push OK
- Modèle local = 100% sans censure
- Claude Code Max = pour documenter les REFUS (comparaison)
- Budget API : 0€ (tout inclus)
