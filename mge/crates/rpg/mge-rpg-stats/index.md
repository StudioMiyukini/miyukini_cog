Plugin: mge-rpg-stats
Version: v1
Domain: rpg

Components:
- StatBlock (values: [f64; 16], indexé par StatId)
- DerivedStats (hp_max, mp_max, end_max, aggro, weight_max)
- Health (current, max, regen_rate)
- DeadTag (marqueur entité morte)

Events:
- StatChangedEvent (entity, stat_id, old_value, new_value)
- EntityDeathEvent (entity)

Systems:
- (à implémenter)

Hot path: no
Headless safe: yes
AI-Native Score: 5/10
