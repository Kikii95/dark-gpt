# Changelog - Dark-GPT

Toutes les modifications notables de ce projet sont documentées ici.

## [0.6.0] - 2026-02-02

### Added
- **Desktop Application** : GUI Tauri portable (Windows + Linux)
  - Setup wizard avec détection prérequis (Docker, Ollama)
  - Téléchargement modèle avec progress bar
  - Interface Open-WebUI intégrée (iframe)
  - One-click start/stop services
- **Backend Rust** :
  - `commands/docker.rs` : Gestion Docker Compose
  - `commands/ollama.rs` : API Ollama (list, pull avec events)
  - `commands/health.rs` : Health checks services
  - `commands/setup.rs` : Détection prérequis, wizard logic
- **Frontend Svelte 5** :
  - Dark theme avec Tailwind CSS
  - Event listening Tauri pour progress
  - Responsive UI avec loading states
- **CI/CD GitHub Actions** :
  - `ci.yml` : Lint et check sur PRs
  - `release.yml` : Build multi-plateforme sur tags
- **Documentation** :
  - `docs/adr/0005-tauri-gui.md` : ADR architecture Tauri

### Changed
- Structure projet : ajout `src-tauri/` (Rust) et `src/` (Svelte)
- Version bump : 0.5.0 → 0.6.0

### Technical
- Tauri 2 + Svelte 5 + Rust + Tailwind CSS
- Binaries : ~15-20MB (vs ~150MB Electron)
- Build time CI : ~8-12 min (avec cache)

## [0.5.0] - 2026-02-02

### Added
- **HTTPS** : Reverse proxy Caddy avec certificats mkcert
  - Accès sécurisé : https://dark-gpt.local
  - Headers sécurité (HSTS, X-Frame-Options, etc.)
- **Authentification** : Open-WebUI native (`WEBUI_AUTH=true`)
  - Premier utilisateur = admin automatique
  - Gestion utilisateurs intégrée
- **Scripts déploiement** :
  - `scripts/deploy.sh` : Orchestration complète
  - `scripts/setup-https.sh` : Installation certificats mkcert
  - `scripts/cleanup-logs.sh` : Purge logs > 7 jours
  - `scripts/cleanup-results.sh` : Archive results > 30 jours
- **Monitoring stack** (optionnel) :
  - Prometheus : Collecte métriques
  - Grafana : Dashboards
  - Alertmanager : Alertes Discord
- **Documentation** :
  - `docs/deployment.md` : Guide déploiement complet
  - `docs/adr/0004-caddy-reverse-proxy.md` : ADR Caddy

### Changed
- `docker-compose.yml` : WebUI exposé uniquement via Caddy (port 8080 interne)
- Secrets externalisés dans `docker/.env`

### Security
- Port 443/80 bindés sur localhost uniquement
- WebUI non exposé directement (via Caddy uniquement)
- Cleanup automatique disponible via cron

## [0.4.0] - 2026-02-02

### Added
- **Tests unitaires** : Structure `tests/` complète avec pytest
  - `conftest.py` avec fixtures réutilisables
  - Tests pour `test_prompts.py`, `analyze_results.py`, `generate_charts.py`
- **ADRs** : Architecture Decision Records (`docs/adr/`)
  - `0001-network-isolation.md` : Isolation réseau Docker
  - `0002-local-logging.md` : Logging local JSONL
  - `0003-model-selection.md` : Choix Dolphin uncensored
- **Scripts centralisés** : `~/tools/scripts/dark-gpt/`
  - `dark-gpt-dev.sh` : Démarrage Ollama + Open-WebUI
  - `dark-gpt-stop.sh` : Arrêt services
  - `dark-gpt-status.sh` : Status check
  - `dark-gpt-restart.sh` : Redémarrage complet
- **Wrappers** : `.claude/*.sh` (start, stop, status, restart)
- **Pre-commit** : Configuration `.pre-commit-config.yaml`
  - Ruff (lint + format)
  - Trailing whitespace, EOF fixer, YAML/JSON check
  - Private key detection
- **Requirements** : `requirements.txt` avec dépendances
- **Sync Obsidian** : Docs Git synchronisés vers Obsidian (Trigger #21)

### Changed
- Grade audit v14 : 🟠 B (8/18) → 🟢 A (14/18)

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
