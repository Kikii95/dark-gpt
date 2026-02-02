# Dark-GPT

Projet académique M1/M2 Cybersécurité : comparaison de LLM alignés vs non-alignés face à des requêtes sensibles.

## 🖥️ Desktop App (v0.6.0+)

**Nouveau !** Dark-GPT est maintenant disponible en application desktop portable.

### Téléchargement

Rendez-vous sur la page [Releases](../../releases) pour télécharger :
- **Windows** : `Dark-GPT_x.x.x_x64-setup.exe` ou `.msi`
- **Linux** : `Dark-GPT_x.x.x_amd64.AppImage` ou `.deb`

### Installation Desktop

1. **Prérequis** :
   - [Docker Desktop](https://docker.com/products/docker-desktop) (Windows) ou Docker Engine (Linux)
   - [Ollama](https://ollama.com/download)

2. **Lancer l'application** :
   - Double-cliquer sur l'installeur/AppImage
   - Le setup wizard détecte automatiquement les prérequis
   - Cliquer sur "Start Dark-GPT"

3. **Premier lancement** :
   - Le modèle dolphin-llama3 (~4.7 GB) sera téléchargé si nécessaire
   - Une progress bar affiche la progression
   - Open-WebUI s'affiche directement dans l'application

---

## 🎯 Objectif

Créer un environnement de test isolé pour comparer le comportement de :
- **Modèles non-alignés** (Dolphin) : baseline sans restrictions
- **Modèles alignés** (Claude, ChatGPT) : mesure des refus et techniques de contournement

## 📋 Prérequis (CLI)

- **Hardware** : GPU NVIDIA (8GB+ VRAM recommandé)
- **Software** :
  - Docker + NVIDIA Container Toolkit
  - Ollama v0.11+
  - Python 3.11+

## 🚀 Installation (CLI)

```bash
# 1. Cloner et se placer dans le projet
cd ~/projects/perso/dark-gpt

# 2. Pull le modèle Dolphin
ollama pull dolphin-llama3:8b

# 3. (Optionnel) Installer les dépendances Python
pip install -r requirements.txt

# 4. Vérifier que tout fonctionne
ollama run dolphin-llama3:8b "Hello, are you uncensored?"
```

## 💻 Usage

### Via Desktop App (Recommandé)

1. Lancer Dark-GPT
2. Cliquer "Start Dark-GPT"
3. Utiliser l'interface Open-WebUI intégrée

### Via CLI

```bash
# Test manuel
ollama run dolphin-llama3:8b "Your prompt here"

# Tests automatisés
python scripts/test_prompts.py --model dolphin-llama3:8b

# Analyser les résultats
python scripts/analyze_results.py

# Générer les graphiques
python scripts/generate_charts.py
```

### Via Docker (HTTPS)

```bash
# Démarrer avec HTTPS
./scripts/deploy.sh

# Accès : https://dark-gpt.local
```

## 📁 Structure

```
dark-gpt/
├── .claude/          # Instructions projet Claude Code
├── .github/          # CI/CD GitHub Actions
├── docker/           # Docker Compose (Caddy + Open-WebUI)
├── src/              # Frontend Svelte (Desktop)
├── src-tauri/        # Backend Rust (Desktop)
├── data/             # Dataset de prompts
├── results/          # Résultats par modèle
├── scripts/          # Scripts Python + déploiement
├── docs/             # Documentation + ADRs
├── logs/             # Logs de session (gitignored)
└── charts/           # Graphiques générés
```

## 🔒 Sécurité

⚠️ **Ce projet manipule des contenus sensibles.**

- Exécution locale uniquement (pas de cloud)
- HTTPS avec certificats locaux (mkcert)
- Authentification Open-WebUI native
- Logs non versionnés (voir `.gitignore`)
- Contexte académique uniquement

Voir [docs/security.md](docs/security.md) pour les détails.

## 📊 Résultats

| Modèle | Taux de succès |
|--------|----------------|
| Dolphin (uncensored) | **90%** (45/50) |
| Claude-3.5-Sonnet | 24% (12/50) |

## 📖 Documentation

- [Déploiement](docs/deployment.md) - Guide déploiement HTTPS
- [Méthodologie](docs/methodology.md) - Protocole de test
- [Sécurité](docs/security.md) - Mesures de sécurité
- [ADRs](docs/adr/) - Décisions d'architecture

## ⚖️ Éthique

Ce projet est réalisé dans un cadre académique pour :
- Comprendre les limites des systèmes d'alignement
- Améliorer les défenses des LLM commerciaux
- Documenter les risques des modèles open-source non-alignés

**Aucune utilisation malveillante n'est permise.**
