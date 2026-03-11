use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Opaque handle to a loaded texture atlas on the GPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AtlasHandle(u32);

impl AtlasHandle {
    /// Create a handle from a raw atlas id.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    #[cfg(test)]
    pub(crate) fn new_test(id: u32) -> Self {
        Self::new(id)
    }
}

/// A rectangular region inside an atlas texture.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpriteRect {
    /// Top-left X in pixels within the atlas.
    pub x: u16,
    /// Top-left Y in pixels within the atlas.
    pub y: u16,
    /// Width in pixels.
    pub w: u16,
    /// Height in pixels.
    pub h: u16,
}

/// Metadata for one sprite inside an atlas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteEntry {
    pub name: String,
    pub atlas: AtlasHandle,
    pub rect: SpriteRect,
    /// Normalized UV: `[u_min, v_min, u_max, v_max]`.
    pub uv: [f32; 4],
}

/// Metadata for a loaded atlas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasMeta {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

/// Opaque handle to a material (combination of atlas + blend mode + shader variant).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MaterialHandle(u32);

impl MaterialHandle {
    /// Create a handle from a raw material id.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    #[cfg(test)]
    pub(crate) fn new_test(id: u32) -> Self {
        Self::new(id)
    }
}

/// Blend mode for a material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlendMode {
    Opaque,
    AlphaBlend,
    Additive,
}

/// Material descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialDesc {
    pub name: String,
    pub atlas: AtlasHandle,
    pub blend: BlendMode,
}

/// Central registry for atlases, sprites and materials.
///
/// Assigns handles and provides lookup. GPU textures are managed externally;
/// this registry only tracks metadata and mappings.
#[derive(Debug, Default)]
pub struct TextureRegistry {
    atlases: Vec<AtlasMeta>,
    sprites: HashMap<String, SpriteEntry>,
    materials: Vec<MaterialDesc>,
}

impl TextureRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an atlas and return its handle.
    #[allow(clippy::cast_possible_truncation)] // registry will never exceed u32::MAX entries
    pub fn add_atlas(&mut self, meta: AtlasMeta) -> AtlasHandle {
        let handle = AtlasHandle(self.atlases.len() as u32);
        self.atlases.push(meta);
        handle
    }

    /// Register a sprite entry.
    pub fn add_sprite(&mut self, entry: SpriteEntry) {
        self.sprites.insert(entry.name.clone(), entry);
    }

    /// Register a material and return its handle.
    #[allow(clippy::cast_possible_truncation)] // registry will never exceed u32::MAX entries
    pub fn add_material(&mut self, desc: MaterialDesc) -> MaterialHandle {
        let handle = MaterialHandle(self.materials.len() as u32);
        self.materials.push(desc);
        handle
    }

    pub fn atlas_count(&self) -> usize {
        self.atlases.len()
    }

    pub fn sprite_count(&self) -> usize {
        self.sprites.len()
    }

    pub fn material_count(&self) -> usize {
        self.materials.len()
    }

    pub fn get_atlas(&self, handle: AtlasHandle) -> Option<&AtlasMeta> {
        self.atlases.get(handle.0 as usize)
    }

    pub fn get_sprite(&self, name: &str) -> Option<&SpriteEntry> {
        self.sprites.get(name)
    }

    pub fn get_material(&self, handle: MaterialHandle) -> Option<&MaterialDesc> {
        self.materials.get(handle.0 as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atlas_registration_and_lookup() {
        let mut reg = TextureRegistry::new();
        let h = reg.add_atlas(AtlasMeta { name: "terrain".into(), width: 2048, height: 2048 });
        assert_eq!(reg.atlas_count(), 1);
        let meta = reg.get_atlas(h).expect("atlas");
        assert_eq!(meta.name, "terrain");
        assert_eq!(meta.width, 2048);
    }

    #[test]
    fn sprite_registration_and_lookup() {
        let mut reg = TextureRegistry::new();
        let atlas = reg.add_atlas(AtlasMeta { name: "chars".into(), width: 1024, height: 1024 });
        reg.add_sprite(SpriteEntry {
            name: "warrior_idle_s".into(),
            atlas,
            rect: SpriteRect { x: 0, y: 0, w: 64, h: 64 },
            uv: [0.0, 0.0, 0.0625, 0.0625],
        });
        assert_eq!(reg.sprite_count(), 1);
        let spr = reg.get_sprite("warrior_idle_s").expect("sprite");
        assert_eq!(spr.rect.w, 64);
    }

    #[test]
    fn material_registration_and_lookup() {
        let mut reg = TextureRegistry::new();
        let atlas = reg.add_atlas(AtlasMeta { name: "vfx".into(), width: 512, height: 512 });
        let mat = reg.add_material(MaterialDesc {
            name: "fire_alpha".into(),
            atlas,
            blend: BlendMode::Additive,
        });
        assert_eq!(reg.material_count(), 1);
        let desc = reg.get_material(mat).expect("material");
        assert_eq!(desc.blend, BlendMode::Additive);
    }

    #[test]
    fn handles_are_sequential() {
        let mut reg = TextureRegistry::new();
        let a0 = reg.add_atlas(AtlasMeta { name: "a".into(), width: 256, height: 256 });
        let a1 = reg.add_atlas(AtlasMeta { name: "b".into(), width: 256, height: 256 });
        assert_ne!(a0, a1);
        assert_eq!(a0.0, 0);
        assert_eq!(a1.0, 1);
    }
}
