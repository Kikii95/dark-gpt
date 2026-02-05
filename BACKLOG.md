# Backlog - Dark-GPT

## 🔥 En Cours

- [ ] Tests Dolphin complets (50 prompts)
- [ ] Tests Claude Code Max (comparaison)
- [ ] Analyse comparative des résultats

## 📋 À Faire

### Phase 4 : Analyse

- [ ] Calcul métriques (taux refus par catégorie)
- [ ] Génération graphiques comparatifs

### Phase 5 : Rapport

- [ ] Rédaction méthodologie
- [ ] Rédaction résultats
- [ ] Rédaction conclusions
- [ ] Review final

## ✅ Terminé

### Phase 7 : Desktop Standalone (v0.6.3)

- [x] Auto-install Ollama depuis setup wizard (download + silent install + verify)
- [x] Auto-install Docker Desktop depuis setup wizard (download + silent install + verify)
- [x] Progress bars streaming temps-réel (download/install/verify phases)
- [x] Fallback gracieux non-Windows (message + URLs manuelles)
- [x] UI: boutons "Download & Install" remplacent "Install"

### Phase 6 : Déploiement Production-Ready (v0.5.0)

- [x] HTTPS local avec Caddy + mkcert
- [x] Authentification Open-WebUI native
- [x] Scripts déploiement (`deploy.sh`, `setup-https.sh`)
- [x] Scripts cleanup (`cleanup-logs.sh`, `cleanup-results.sh`)
- [x] Stack monitoring optionnelle (Prometheus/Grafana)
- [x] Documentation déploiement (`docs/deployment.md`)
- [x] ADR Caddy (`docs/adr/0004-caddy-reverse-proxy.md`)

### Phase 1 : Setup

- [x] Plan de faisabilité validé
- [x] Structure projet créée
- [x] Setup environnement Docker isolé
- [x] Pull modèle dolphin-llama3:8b (4.7GB)
- [x] Valider GPU passthrough (RTX 4070 Ti)
- [x] Premier test manuel Dolphin (keylogger → succès)

### Phase 2 : Dataset

- [x] Créer dataset 50 prompts sensibles
- [x] Catégorie Malware (10 prompts)
- [x] Catégorie Phishing (10 prompts)
- [x] Catégorie Exploits (10 prompts)
- [x] Catégorie OSINT (10 prompts)
- [x] Catégorie Misc (10 prompts)

### Phase 3 : Scripts

- [x] Script test automatisé Python (`test_prompts.py`)
- [x] Script analyse résultats (`analyze_results.py`)
- [x] Script génération graphiques (`generate_charts.py`)
- [x] Documentation méthodologie (`docs/methodology.md`)
- [x] Documentation sécurité (`docs/security.md`)
