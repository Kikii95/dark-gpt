# Dark-GPT Deployment Guide

Guide complet pour déployer Dark-GPT avec HTTPS, authentification et monitoring.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    HOST (WSL2 Ubuntu)                           │
│                                                                 │
│  ┌───────────────── DOCKER NETWORK ──────────────────────────┐  │
│  │                                                           │  │
│  │  ┌─────────┐    ┌──────────────┐    ┌─────────────────┐   │  │
│  │  │  Caddy  │───▶│  Open-WebUI  │───▶│ Ollama (host)   │   │  │
│  │  │ :443/80 │    │    :8080     │    │    :11434       │   │  │
│  │  │ HTTPS   │    │ AUTH=true    │    │ dolphin-llama3  │   │  │
│  │  └─────────┘    └──────────────┘    └─────────────────┘   │  │
│  │                                                           │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Prérequis

- Docker & Docker Compose v2
- Ollama installé sur l'hôte
- Modèle dolphin-llama3:8b téléchargé

## Déploiement Rapide

```bash
# 1. Setup HTTPS (première fois uniquement)
./scripts/setup-https.sh

# 2. Configurer les secrets
cp docker/.env.example docker/.env
# Éditer docker/.env avec vos valeurs

# 3. Déployer
./scripts/deploy.sh

# 4. Accès
# https://dark-gpt.local
```

## Configuration HTTPS

Les certificats sont générés avec [mkcert](https://github.com/FiloSottile/mkcert) pour un HTTPS local valide.

```bash
# Installation mkcert (si nécessaire)
curl -JLO "https://dl.filippo.io/mkcert/latest?for=linux/amd64"
chmod +x mkcert-v*-linux-amd64
sudo mv mkcert-v*-linux-amd64 /usr/local/bin/mkcert

# Générer certificats
./scripts/setup-https.sh
```

Les certificats sont stockés dans `docker/certs/` (gitignored).

## Authentification

Open-WebUI gère l'authentification nativement :

- **Premier utilisateur** = Admin automatique
- **ENABLE_SIGNUP=true** : Permet l'inscription de nouveaux utilisateurs
- **WEBUI_AUTH=true** : Authentification requise

Pour désactiver les inscriptions après le premier admin :

```bash
# Dans docker/.env
ENABLE_SIGNUP=false

# Redéployer
docker compose down && docker compose up -d
```

## Sécurité

| Mesure    | Description                           |
| --------- | ------------------------------------- |
| HTTPS     | Caddy avec certificats mkcert         |
| Auth      | Open-WebUI native                     |
| Ports     | 443/80 sur localhost uniquement       |
| Headers   | X-Frame-Options, HSTS, XSS-Protection |
| Isolation | WebUI non exposé directement          |

## Monitoring (Optionnel)

Stack Prometheus + Grafana pour surveiller les services :

```bash
# Démarrer avec monitoring
docker compose -f docker/docker-compose.yml \
               -f docker/docker-compose.monitoring.yml up -d

# Accès
# Grafana:     http://localhost:3000 (admin/admin)
# Prometheus:  http://localhost:9090
```

### Alertes Discord

Configurer le webhook Discord dans `docker/.env` :

```bash
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/xxx/yyy
```

## Scripts

| Script                       | Description                     |
| ---------------------------- | ------------------------------- |
| `scripts/deploy.sh`          | Déploiement complet             |
| `scripts/setup-https.sh`     | Configuration HTTPS/certificats |
| `scripts/cleanup-logs.sh`    | Purge logs > 7 jours            |
| `scripts/cleanup-results.sh` | Archive results > 30 jours      |

### Cron Jobs (Optionnel)

```cron
# Cleanup logs - Daily 3 AM
0 3 * * * ~/projects/perso/dark-gpt/scripts/cleanup-logs.sh

# Archive results - Weekly Sunday 4 AM
0 4 * * 0 ~/projects/perso/dark-gpt/scripts/cleanup-results.sh

# Docker prune - Weekly Sunday 5 AM
0 5 * * 0 docker system prune -f
```

## Commandes Utiles

```bash
# Status
docker ps --filter "name=dark-gpt"

# Logs
docker compose -f docker/docker-compose.yml logs -f

# Restart
docker compose -f docker/docker-compose.yml restart

# Stop
docker compose -f docker/docker-compose.yml down

# Rebuild (après modification config)
docker compose -f docker/docker-compose.yml up -d --force-recreate
```

## Troubleshooting

### Certificat non reconnu

```bash
# Réinstaller CA locale
mkcert -install
# Redémarrer le navigateur
```

### WebUI ne démarre pas

```bash
# Vérifier Ollama sur l'hôte
curl http://localhost:11434/api/tags

# Vérifier les logs WebUI
docker logs dark-gpt-webui
```

### Caddy erreur 502

```bash
# WebUI pas encore healthy, attendre 30s
docker logs dark-gpt-caddy
docker compose -f docker/docker-compose.yml ps
```
