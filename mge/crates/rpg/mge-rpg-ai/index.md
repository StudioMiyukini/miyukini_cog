Plugin: mge-rpg-ai
Version: v1
Domain: rpg

Components:
- CreatureState (enum: Idle, Chase, Attack, Return)
- AIState (state, spawn_point, aggro_radius, leash_radius, target, attack_range)
- AiTargetable (marker)

Systems:
- ai_tick_system (phase 500)

Events:
- AiAttackRequestEvent (attacker, target)

Hot path: yes
Headless safe: yes
AI-Native Score: 8/10
