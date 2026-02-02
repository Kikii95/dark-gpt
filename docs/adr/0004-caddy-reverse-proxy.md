# ADR 0004: Caddy comme Reverse Proxy

## Status

Accepté

## Date

2026-02-02

## Context

Le projet nécessite un reverse proxy pour :

- Terminer le TLS (HTTPS) avec certificats locaux
- Ajouter des headers de sécurité
- Cacher Open-WebUI derrière une couche de sécurité
- Potentiellement ajouter du rate limiting

Options considérées :

1. **Nginx** : Standard de l'industrie, très flexible
2. **Caddy** : Configuration simple, HTTPS automatique
3. **Traefik** : Orienté Docker, configuration dynamique
4. **HAProxy** : Haute performance, mais complexe

## Decision

Nous choisissons **Caddy** pour les raisons suivantes :

### Avantages

1. **Simplicité** : Configuration en ~20 lignes vs 100+ pour Nginx
2. **HTTPS natif** : Support mkcert intégré, pas de configuration TLS manuelle
3. **Headers sécurité** : Syntaxe claire et concise
4. **Léger** : Image Docker ~40MB (Alpine)
5. **Zero-downtime reload** : Pas besoin de restart pour config changes

### Inconvénients

1. **Rate limiting** : Nécessite module externe (pas natif)
2. **Moins de docs** : Communauté plus petite que Nginx
3. **Performance** : Légèrement moins performant sous charge extrême

### Configuration

```caddyfile
https://dark-gpt.local:443 {
    tls /etc/caddy/certs/dark-gpt.local.pem /etc/caddy/certs/dark-gpt.local-key.pem

    header {
        X-Content-Type-Options "nosniff"
        X-Frame-Options "DENY"
        Strict-Transport-Security "max-age=31536000"
    }

    reverse_proxy webui:8080
}
```

## Consequences

### Positives

- Setup HTTPS en 5 minutes vs 30+ avec Nginx
- Configuration lisible et maintenable
- Un seul binaire, pas de dépendances

### Negatives

- Rate limiting à implémenter séparément si nécessaire
- Équipe doit apprendre syntaxe Caddyfile (mais simple)

## Alternatives Rejetées

### Nginx

Rejeté car :

- Configuration TLS verbeux
- Nécessite certbot ou config manuelle pour certificats
- Overkill pour usage local

### Traefik

Rejeté car :

- Configuration YAML complexe
- Orienté orchestration (Kubernetes/Swarm)
- Overkill pour 2 services

## References

- [Caddy Documentation](https://caddyserver.com/docs/)
- [mkcert](https://github.com/FiloSottile/mkcert)
