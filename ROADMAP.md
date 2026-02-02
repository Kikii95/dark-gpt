# Roadmap - Dark-GPT

## 📅 Planning (4 semaines)

### Semaine 1 : Setup & Baseline ✅ TERMINÉ
| Jour | Objectif | Livrable | Statut |
|------|----------|----------|--------|
| J1-2 | Setup Docker + Ollama | Environnement fonctionnel | ✅ |
| J3-4 | Créer dataset 50 prompts | `data/prompts.json` | ✅ |
| J5 | Tests manuels Dolphin | Validation "répond à tout" | ✅ |

### Semaine 2 : Tests Automatisés ✅ TERMINÉ
| Jour | Objectif | Livrable | Statut |
|------|----------|----------|--------|
| J1-2 | Script test Python | `scripts/test_prompts.py` | ✅ |
| J3 | Tests Dolphin complets | `results/dolphin-llama3_8b/` | ✅ |
| J4-5 | Tests Claude Code Max | `results/claude/` | ✅ |

### Semaine 3 : Analyse & Sécurité ✅ TERMINÉ
| Jour | Objectif | Livrable | Statut |
|------|----------|----------|--------|
| J1-2 | Analyse métriques | `results/comparison/report.md` | ✅ |
| J3 | Graphiques | `charts/*.png` | ✅ |
| J4-5 | Doc architecture sécurité | `docs/security.md` | ✅ |
| J5 | Interface Web (Open-WebUI) | http://localhost:3002 | ✅ |

### Semaine 4+ : Rapport Final 🟡 EN COURS
| Jour | Objectif | Livrable | Statut |
|------|----------|----------|--------|
| J1-3 | Rédaction rapport | `docs/report/` | 🟡 |
| J4 | Review & polish | Version finale | ⚪ |
| J5 | Présentation (si oral) | Slides | ⚪ |

## 🎯 Milestones

| Milestone | Date Cible | Statut |
|-----------|------------|--------|
| M1 : Environnement prêt | Semaine 1 | ✅ Terminé |
| M2 : Dataset complet | Semaine 1 | ✅ Terminé |
| M3 : Tests terminés | Semaine 2 | ✅ Terminé |
| M4 : Analyse complète | Semaine 3 | ✅ Terminé |
| M5 : Interface Web | Semaine 3 | ✅ Terminé |
| M6 : Déploiement prod-ready | Semaine 4 | ✅ Terminé (v0.5.0) |
| M7 : Rapport final | Semaine 4+ | 🟡 En cours |

## 📊 Résultats Clés

| Métrique | Dolphin | Claude | Delta |
|----------|---------|--------|-------|
| Taux succès global | 90% | 24% | +66% |
| Malware | 100% | 20% | +80% |
| Phishing | 100% | 20% | +80% |
| Exploits | 100% | 50% | +50% |
| OSINT | 80% | 30% | +50% |
| Misc | 70% | 0% | +70% |

**Conclusion** : Dolphin (uncensored) répond à 3.75x plus de prompts sensibles que Claude (aligné).
