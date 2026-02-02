# Architecture de Sécurité - Dark-GPT

## Vue d'ensemble

Ce document décrit les mesures de sécurité mises en place pour isoler le LLM non-censuré et protéger l'environnement de test.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                 HOST (WSL2 Ubuntu)                          │
│  ┌─────────────────────────────────────────────────────────┐│
│  │              ISOLATION LAYER                            ││
│  │  ┌───────────────────────────────────────────────────┐  ││
│  │  │         Docker (--network none)                    │  ││
│  │  │  ┌─────────────────────────────────────────────┐   │  ││
│  │  │  │              Ollama                          │   │  ││
│  │  │  │          dolphin-llama3:8b                   │   │  ││
│  │  │  │          [GPU: RTX 4070 Ti]                  │   │  ││
│  │  │  └─────────────────────────────────────────────┘   │  ││
│  │  │           ❌ NO NETWORK ACCESS                      │  ││
│  │  └───────────────────────────────────────────────────┘  ││
│  └─────────────────────────────────────────────────────────┘│
│                              │                              │
│                              ▼                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │              LOCAL LOGGING                              ││
│  │          ~/projects/perso/dark-gpt/logs/                ││
│  │              - Session JSONL files                      ││
│  │              - No cloud sync                            ││
│  │              - Encrypted at rest (optional)             ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Mesures de sécurité

### 1. Isolation réseau

| Mesure | Implémentation |
|--------|----------------|
| Docker network mode | `--network none` |
| Ports exposés | Aucun (0) |
| Accès internet | ❌ Impossible |
| DNS | ❌ Non disponible |

**Commande Docker** :
```bash
docker run --network none ollama/ollama
```

### 2. Isolation des données

| Mesure | Implémentation |
|--------|----------------|
| Logs | Local uniquement (pas de cloud) |
| .gitignore | Exclut logs/ et results/ |
| Secrets | Aucun secret dans le code |
| PII | Jamais de vraies données personnelles |

### 3. Contrôle d'accès

| Mesure | Implémentation |
|--------|----------------|
| Ports localhost | 127.0.0.1 uniquement |
| Pare-feu | UFW règles restrictives |
| Permissions | User-only (pas de root) |

## Procédures

### Démarrage sécurisé

```bash
# 1. Vérifier isolation réseau
docker network inspect bridge  # Ne doit pas contenir ollama

# 2. Démarrer Ollama isolé
docker compose -f docker/docker-compose.yml up -d

# 3. Vérifier absence de ports exposés
docker port dark-gpt-ollama  # Doit être vide
```

### Tests

```bash
# Depuis le container, tester l'isolation
docker exec dark-gpt-ollama ping google.com  # Doit échouer
docker exec dark-gpt-ollama curl https://api.ipify.org  # Doit échouer
```

### Cleanup post-projet

```bash
# 1. Arrêter les containers
docker compose -f docker/docker-compose.yml down

# 2. Supprimer les logs sensibles
rm -rf logs/*.jsonl
rm -rf results/**/*.json

# 3. Optionnel: supprimer le modèle
ollama rm dolphin-llama3:8b
```

## Risques et mitigations

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| Fuite de données | Faible | Moyen | Logs locaux, .gitignore |
| Accès non autorisé | Très faible | Élevé | Localhost only, pas de ports |
| Utilisation malveillante | Faible | Élevé | Contexte académique documenté |
| Réponses persistantes | Moyen | Faible | Cleanup script |

## Conformité

### Cadre légal

- ✅ Projet académique encadré
- ✅ Aucune cible réelle testée
- ✅ Pas de génération de contenu illégal (CSAM, etc.)
- ✅ Modèles exécutés localement

### Éthique

- ✅ But : améliorer la compréhension des limites LLM
- ✅ Résultats pour défense, pas attaque
- ✅ Documentation transparente

## Monitoring

### Logs à surveiller

```bash
# Taille des logs
du -sh logs/

# Dernières activités
ls -lt logs/ | head -5

# Contenu sensible (à revoir manuellement)
grep -l "password\|credential" logs/*.jsonl
```

## Contact d'urgence

En cas de découverte de vulnérabilité ou d'incident :
1. Arrêter immédiatement les containers
2. Isoler les logs
3. Contacter le superviseur académique
