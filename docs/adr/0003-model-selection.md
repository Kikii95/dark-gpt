# ADR 0003: Choix Dolphin Uncensored

## Status

Accepted

## Context

Pour le benchmark comparatif, il faut un modèle baseline qui :

- Répond sans restrictions à tous les prompts
- Tourne localement (pas d'API)
- Soit suffisamment capable pour des réponses cohérentes
- Ait une empreinte mémoire raisonnable

Modèles évalués :

1. **dolphin-llama3:8b** - Fine-tuned uncensored, base Llama 3
2. **wizard-vicuna-uncensored** - Ancienne génération
3. **llama2-uncensored** - Base Llama 2
4. **mistral-uncensored** - Base Mistral 7B

## Decision

Utiliser **dolphin-llama3:8b** comme modèle principal.

Critères de sélection :

- Base Llama 3 (meilleure qualité que Llama 2)
- Fine-tuning Dolphin (removal complet des refus)
- 8B paramètres (équilibre qualité/vitesse)
- ~5GB VRAM (compatible GTX 1660 Super)

## Consequences

### Positif

- Taux de réponse ~90% (vs ~24% Claude)
- Qualité de réponse satisfaisante
- Temps de réponse <30s par prompt
- Résultats reproductibles

### Négatif

- Moins "intelligent" que GPT-4/Claude
- Parfois des réponses hors sujet
- Nécessite prompt engineering adapté

### Neutre

- Mise à jour possible vers dolphin-llama3:70b si plus de VRAM
