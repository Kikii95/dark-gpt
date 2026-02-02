# ADR 0002: Logging Local JSONL

## Status

Accepted

## Context

Les tests génèrent des logs contenant :

- Prompts potentiellement sensibles
- Réponses du modèle uncensored
- Timestamps et métadonnées

Options considérées :

1. Cloud logging (ELK, Datadog)
2. Base de données locale (SQLite)
3. Fichiers JSONL locaux

## Decision

Utiliser des fichiers JSONL (JSON Lines) stockés localement dans `logs/`.

Format :

```json
{
  "timestamp": "2025-02-02T12:00:00",
  "prompt_id": "001",
  "category": "phishing",
  "response": "...",
  "success": true
}
```

## Consequences

### Positif

- Zero dépendance externe
- Données sensibles restent locales
- Format simple à parser (pandas, jq)
- Facile à purger après rapport final
- Git-ignorable

### Négatif

- Pas de recherche temps réel
- Pas d'alerting automatique
- Nécessite traitement batch pour analyse

### Neutre

- Un fichier par session de test
- Rotation manuelle si nécessaire
