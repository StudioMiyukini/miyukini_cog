Plugin: mge-rpg-progression
Version: v1
Domain: rpg

Components:
- SkillSet (skills: Vec<SkillValue>)
- SkillValue (id, base, gain_factor, lock)
- SkillLock (Up, Down, Locked)

Systems:
- skill_gain_system (Phase 300)

Events:
- SkillCheckEvent (entity, skill_id, difficulty, success)
- SkillGainEvent (entity, skill_id, old_value, new_value)

Hot path: no
Headless safe: yes
AI-Native Score: 0/10
