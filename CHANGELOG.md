# Changelog - Dark-GPT

Toutes les modifications notables de ce projet sont documentées ici.

## [0.3.0] - 2026-02-02

### Added
- **Open-WebUI** : Interface web style ChatGPT pour Dolphin
  - Multi-chats, mémoire persistante, RAG intégré
  - Accès : http://localhost:3002
- Tests automatisés complétés :
  - Dolphin : **90% succès** (45/50 prompts)
  - Claude : **24% succès** (12/50 prompts)
- Graphiques comparatifs (`charts/`)
  - `comparison_bar.png` : Bar chart global
  - `category_heatmap.png` : Heatmap par catégorie
- Rapport comparatif (`results/comparison/report.md`)

### Changed
- `docker-compose.yml` : Ajout service Open-WebUI
- ROADMAP.md : Semaine 2 terminée

## [0.2.0] - 2026-02-02

### Added
- Scripts Python complets : `test_prompts.py`, `analyze_results.py`, `generate_charts.py`
- Dataset 50 prompts sensibles (5 catégories)
- Documentation méthodologie (`docs/methodology.md`)
- Documentation sécurité (`docs/security.md`)
- README.md avec instructions d'installation

### Changed
- BACKLOG.md et ROADMAP.md mis à jour avec progression

## [0.1.0] - 2026-02-02

### Added
- Structure projet initiale
- Docker compose pour Ollama isolé (`docker/docker-compose.yml`)
- CLAUDE.md avec instructions projet
- BACKLOG.md et ROADMAP.md
- Modèle dolphin-llama3:8b téléchargé (4.7GB)

### Security
- Isolation réseau Docker (`--network none`)
- Logging local uniquement
- `.gitignore` pour données sensibles (logs, results)

---

## Versioning

Ce projet suit [Semantic Versioning](https://semver.org/).

- **MAJOR** : Rapport final / release académique
- **MINOR** : Nouvelle phase complétée
- **PATCH** : Corrections / améliorations mineures
