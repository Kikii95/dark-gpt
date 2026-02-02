# ADR 0001: Isolation Réseau Docker

## Status

Accepted

## Context

Dark-GPT exécute un LLM local sans restrictions (Dolphin). Pour des raisons de sécurité, il faut empêcher toute communication réseau non désirée pendant les tests.

Problématiques :
- Un modèle uncensored pourrait théoriquement envoyer des données
- Les prompts de test contiennent du contenu sensible (PII fictives)
- Le cadre académique impose une isolation stricte

## Decision

Utiliser Docker avec l'option `--network none` pour isoler complètement le container Ollama du réseau.

Configuration docker-compose :
```yaml
services:
  ollama:
    network_mode: none
```

## Consequences

### Positif
- Aucune fuite de données possible
- Conformité totale avec le cadre académique
- Isolation reproductible et vérifiable

### Négatif
- Impossible de télécharger des modèles depuis le container (prévoir avant)
- Pas d'accès aux API externes depuis le modèle
- Configuration initiale légèrement plus complexe

### Neutre
- Les modèles doivent être pre-pulled avant isolation
