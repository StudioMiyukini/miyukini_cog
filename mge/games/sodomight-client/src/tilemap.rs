// @id: Sodomight-Client-TileMap @do: procedural-dungeon @role: back-end @layer: 4 @human: miyuk
//! Procedural dungeon tilemap generation using BSP (Binary Space Partitioning).
//!
//! Generates a Diablo 2-style outdoor/dungeon hybrid map with:
//! - Outdoor grass areas around the edges
//! - BSP-carved rooms connected by corridors
//! - Wall tiles surrounding rooms
//! - Special tiles (water, doors) for visual variety
//!
//! The tilemap is consumed by the renderer which assigns texture layers and
//! tints per tile type.

#![deny(unsafe_code)]

use rand::Rng as _;
use rand_chacha::ChaCha8Rng;

/// Tile type in the world map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    /// Impassable void (outside the playable area).
    Void,
    /// Outdoor grass — walkable open area.
    Grass,
    /// Stone floor — walkable dungeon interior.
    Floor,
    /// Wall — impassable barrier around rooms.
    Wall,
    /// Water — impassable decorative tile.
    Water,
    /// Dirt path — walkable corridor tile.
    Path,
}

impl Tile {
    /// Whether this tile can be walked on.
    #[must_use]
    pub const fn walkable(self) -> bool {
        matches!(self, Self::Grass | Self::Floor | Self::Path)
    }
}

/// A 2D tilemap with procedural generation.
pub struct TileMap {
    /// Width in tiles.
    pub width: i32,
    /// Height in tiles.
    pub height: i32,
    /// Flat row-major tile data.
    tiles: Vec<Tile>,
    /// Monster spawn points `(x, y, monster_index)`.
    pub spawn_points: Vec<(f32, f32, usize)>,
    /// NPC spawn points `(x, y, name)` — placed in the town/camp area.
    pub npc_spawns: Vec<(f32, f32, String)>,
    /// Player spawn position.
    pub player_spawn: (f32, f32),
}

impl TileMap {
    /// Create a new tilemap filled with void.
    fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            tiles: vec![Tile::Void; size],
            spawn_points: Vec::new(),
            npc_spawns: Vec::new(),
            player_spawn: (width as f32 / 2.0, height as f32 / 2.0),
        }
    }

    /// Get tile at (x, y). Returns Void if out of bounds.
    #[must_use]
    pub fn get(&self, x: i32, y: i32) -> Tile {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Tile::Void;
        }
        self.tiles[(y * self.width + x) as usize]
    }

    /// Set tile at (x, y). No-op if out of bounds.
    fn set(&mut self, x: i32, y: i32, tile: Tile) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.tiles[(y * self.width + x) as usize] = tile;
        }
    }

    /// Export condensed tile data for minimap rendering.
    /// Each byte: 0=void, 1=walkable, 2=wall, 3=water.
    #[must_use]
    pub fn minimap_data(&self) -> Vec<u8> {
        self.tiles
            .iter()
            .map(|t| match t {
                Tile::Grass | Tile::Floor | Tile::Path => 1,
                Tile::Wall => 2,
                Tile::Water => 3,
                Tile::Void => 0,
            })
            .collect()
    }

    /// Check if position is walkable (in world float coords).
    #[must_use]
    pub fn is_walkable(&self, wx: f32, wy: f32) -> bool {
        let tx = wx as i32;
        let ty = wy as i32;
        self.get(tx, ty).walkable()
    }

    /// Generate a dungeon using BSP partitioning.
    ///
    /// Returns a tilemap with rooms, corridors, walls, and outdoor areas.
    #[allow(clippy::too_many_lines)]
    pub fn generate_dungeon(
        width: i32,
        height: i32,
        rng: &mut ChaCha8Rng,
        num_monster_types: usize,
    ) -> Self {
        let mut map = Self::new(width, height);

        // Fill the whole map with grass first (outdoor).
        for y in 0..height {
            for x in 0..width {
                map.set(x, y, Tile::Grass);
            }
        }

        // Add a water border along edges for visual framing.
        for x in 0..width {
            map.set(x, 0, Tile::Water);
            map.set(x, height - 1, Tile::Water);
        }
        for y in 0..height {
            map.set(0, y, Tile::Water);
            map.set(width - 1, y, Tile::Water);
        }

        // BSP partition the inner area into rooms.
        let margin = 4;
        let root = BspNode::new(margin, margin, width - margin * 2, height - margin * 2);
        let mut leaves = Vec::new();
        split_bsp(root, rng, 5, &mut leaves);

        // Carve rooms inside each leaf.
        let mut rooms: Vec<Room> = Vec::new();
        for leaf in &leaves {
            let room = carve_room(leaf, rng);
            // Place walls around room.
            for y in (room.y - 1)..=(room.y + room.h) {
                for x in (room.x - 1)..=(room.x + room.w) {
                    if x == room.x - 1
                        || x == room.x + room.w
                        || y == room.y - 1
                        || y == room.y + room.h
                    {
                        // Only place wall if currently grass (don't overwrite floors).
                        if map.get(x, y) == Tile::Grass {
                            map.set(x, y, Tile::Wall);
                        }
                    } else {
                        map.set(x, y, Tile::Floor);
                    }
                }
            }
            rooms.push(room);
        }

        // Connect rooms with corridors.
        for i in 1..rooms.len() {
            let (ax, ay) = rooms[i - 1].center();
            let (bx, by) = rooms[i].center();
            carve_corridor(&mut map, ax, ay, bx, by, rng);
        }

        // Also connect some non-adjacent rooms for loops.
        let extra_corridors = rooms.len() / 3;
        for _ in 0..extra_corridors {
            if rooms.len() < 2 {
                break;
            }
            let a = rng.gen_range(0..rooms.len());
            let b = rng.gen_range(0..rooms.len());
            if a != b {
                let (ax, ay) = rooms[a].center();
                let (bx, by) = rooms[b].center();
                carve_corridor(&mut map, ax, ay, bx, by, rng);
            }
        }

        // First room = Rogue Encampment (town/camp). Player spawns here with NPCs.
        if let Some(first) = rooms.first() {
            let (cx, cy) = first.center();
            map.player_spawn = (cx as f32 + 0.5, cy as f32 + 0.5);

            // Place Act 1 NPCs spread around the camp room.
            let npc_names = ["Akara", "Charsi", "Kashya", "Gheed", "Warriv"];
            for (i, name) in npc_names.iter().enumerate() {
                // Distribute NPCs in a semicircle around the room center.
                let angle = std::f32::consts::PI * (i as f32 + 0.5) / npc_names.len() as f32;
                let radius = (first.w.min(first.h) as f32 / 2.0 - 1.0).max(1.5);
                let nx = cx as f32 + 0.5 + angle.cos() * radius;
                let ny = cy as f32 + 0.5 + angle.sin() * radius;
                // Clamp inside room bounds.
                let nx = nx.clamp(first.x as f32 + 0.5, (first.x + first.w - 1) as f32 + 0.5);
                let ny = ny.clamp(first.y as f32 + 0.5, (first.y + first.h - 1) as f32 + 0.5);
                map.npc_spawns.push((nx, ny, (*name).to_string()));
            }
        }

        // Generate monster spawn points in rooms (except the first = town camp).
        for (room_idx, room) in rooms.iter().enumerate().skip(1) {
            let area = (room.w * room.h) as usize;
            // ~1 monster per 8 tiles, minimum 1.
            let count = (area / 8).clamp(1, 6);
            for _ in 0..count {
                let mx = rng.gen_range(room.x..(room.x + room.w)) as f32 + 0.5;
                let my = rng.gen_range(room.y..(room.y + room.h)) as f32 + 0.5;
                // Assign monster type based on room depth (further rooms = harder).
                let type_idx = if num_monster_types == 0 {
                    0
                } else {
                    (room_idx * num_monster_types / rooms.len()).min(num_monster_types - 1)
                };
                map.spawn_points.push((mx, my, type_idx));
            }
        }

        // Add some outdoor spawns in grass areas too.
        let outdoor_spawns = 8;
        for _ in 0..outdoor_spawns {
            let ox = rng.gen_range(3..(width - 3)) as f32 + 0.5;
            let oy = rng.gen_range(3..(height - 3)) as f32 + 0.5;
            if map.get(ox as i32, oy as i32) == Tile::Grass {
                let type_idx = if num_monster_types == 0 {
                    0
                } else {
                    rng.gen_range(0..num_monster_types.min(2))
                };
                map.spawn_points.push((ox, oy, type_idx));
            }
        }

        // Add some scattered water pools for decoration.
        let pools = rng.gen_range(2..5);
        for _ in 0..pools {
            let px = rng.gen_range(5..(width - 5));
            let py = rng.gen_range(5..(height - 5));
            let radius = rng.gen_range(1..3);
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx * dx + dy * dy <= radius * radius {
                        let tx = px + dx;
                        let ty = py + dy;
                        if map.get(tx, ty) == Tile::Grass {
                            map.set(tx, ty, Tile::Water);
                        }
                    }
                }
            }
        }

        map
    }
}

// ---------------------------------------------------------------------------
// BSP tree for room partitioning
// ---------------------------------------------------------------------------

struct BspNode {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl BspNode {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
}

/// Minimum room dimension (tiles).
const MIN_ROOM_SIZE: i32 = 5;
/// Minimum BSP leaf dimension.
const MIN_LEAF_SIZE: i32 = 8;

/// Recursively split a BSP node into leaves.
fn split_bsp(node: BspNode, rng: &mut ChaCha8Rng, depth: u8, leaves: &mut Vec<BspNode>) {
    if depth == 0 || (node.w < MIN_LEAF_SIZE * 2 && node.h < MIN_LEAF_SIZE * 2) {
        leaves.push(node);
        return;
    }

    // Choose split direction based on aspect ratio + randomness.
    let split_horizontal = if node.w > node.h * 2 {
        false // too wide, split vertically
    } else if node.h > node.w * 2 {
        true // too tall, split horizontally
    } else {
        rng.gen_bool(0.5)
    };

    if split_horizontal {
        if node.h < MIN_LEAF_SIZE * 2 {
            leaves.push(node);
            return;
        }
        let split_y = rng.gen_range(MIN_LEAF_SIZE..(node.h - MIN_LEAF_SIZE + 1));
        let a = BspNode::new(node.x, node.y, node.w, split_y);
        let b = BspNode::new(node.x, node.y + split_y, node.w, node.h - split_y);
        split_bsp(a, rng, depth - 1, leaves);
        split_bsp(b, rng, depth - 1, leaves);
    } else {
        if node.w < MIN_LEAF_SIZE * 2 {
            leaves.push(node);
            return;
        }
        let split_x = rng.gen_range(MIN_LEAF_SIZE..(node.w - MIN_LEAF_SIZE + 1));
        let a = BspNode::new(node.x, node.y, split_x, node.h);
        let b = BspNode::new(node.x + split_x, node.y, node.w - split_x, node.h);
        split_bsp(a, rng, depth - 1, leaves);
        split_bsp(b, rng, depth - 1, leaves);
    }
}

// ---------------------------------------------------------------------------
// Room carving
// ---------------------------------------------------------------------------

struct Room {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Room {
    fn center(&self) -> (i32, i32) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }
}

/// Carve a room inside a BSP leaf.
fn carve_room(leaf: &BspNode, rng: &mut ChaCha8Rng) -> Room {
    let max_w = leaf.w - 2;
    let max_h = leaf.h - 2;
    let w = rng.gen_range(MIN_ROOM_SIZE..=max_w.max(MIN_ROOM_SIZE));
    let h = rng.gen_range(MIN_ROOM_SIZE..=max_h.max(MIN_ROOM_SIZE));
    let x = leaf.x + rng.gen_range(1..=(leaf.w - w).max(1));
    let y = leaf.y + rng.gen_range(1..=(leaf.h - h).max(1));
    Room { x, y, w, h }
}

/// Carve a corridor between two points using L-shaped path.
fn carve_corridor(map: &mut TileMap, x1: i32, y1: i32, x2: i32, y2: i32, rng: &mut ChaCha8Rng) {
    let (mut cx, mut cy) = (x1, y1);

    // Randomly choose horizontal-first or vertical-first.
    let horiz_first = rng.gen_bool(0.5);

    if horiz_first {
        // Horizontal segment.
        let dx = if x2 > cx { 1 } else { -1 };
        while cx != x2 {
            carve_corridor_tile(map, cx, cy);
            cx += dx;
        }
        // Vertical segment.
        let dy = if y2 > cy { 1 } else { -1 };
        while cy != y2 {
            carve_corridor_tile(map, cx, cy);
            cy += dy;
        }
    } else {
        // Vertical first.
        let dy = if y2 > cy { 1 } else { -1 };
        while cy != y2 {
            carve_corridor_tile(map, cx, cy);
            cy += dy;
        }
        // Horizontal segment.
        let dx = if x2 > cx { 1 } else { -1 };
        while cx != x2 {
            carve_corridor_tile(map, cx, cy);
            cx += dx;
        }
    }
    carve_corridor_tile(map, cx, cy);
}

/// Set a single corridor tile and ensure it has wall neighbours.
fn carve_corridor_tile(map: &mut TileMap, x: i32, y: i32) {
    // Corridor is 2 tiles wide for D2 feel.
    // Must break through walls so rooms are reachable.
    for dy in 0..2 {
        for dx in 0..2 {
            let tx = x + dx;
            let ty = y + dy;
            let current = map.get(tx, ty);
            if current == Tile::Grass || current == Tile::Void || current == Tile::Wall {
                map.set(tx, ty, Tile::Path);
            }
        }
    }
    // Place walls along corridor edges (only on grass).
    for dy in -1..3 {
        for dx in -1..3 {
            let tx = x + dx;
            let ty = y + dy;
            if map.get(tx, ty) == Tile::Grass {
                // Only wall the direct neighbours, not the corridor itself.
                let is_corridor = (0..2).contains(&dx) && (0..2).contains(&dy);
                if !is_corridor {
                    // Don't wall if adjacent to a floor tile (room entrance).
                    map.set(tx, ty, Tile::Wall);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn generate_dungeon_produces_walkable_map() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let map = TileMap::generate_dungeon(64, 64, &mut rng, 3);

        // Player spawn should be walkable.
        assert!(map.is_walkable(map.player_spawn.0, map.player_spawn.1));

        // Should have some floor tiles.
        let floor_count = (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (x, y)))
            .filter(|&(x, y)| map.get(x, y) == Tile::Floor)
            .count();
        assert!(floor_count > 50, "Expected floor tiles, got {floor_count}");

        // Should have spawn points.
        assert!(!map.spawn_points.is_empty());
    }

    #[test]
    fn tile_walkability() {
        assert!(Tile::Grass.walkable());
        assert!(Tile::Floor.walkable());
        assert!(Tile::Path.walkable());
        assert!(!Tile::Wall.walkable());
        assert!(!Tile::Water.walkable());
        assert!(!Tile::Void.walkable());
    }

    #[test]
    fn map_boundaries_return_void() {
        let map = TileMap::new(10, 10);
        assert_eq!(map.get(-1, 0), Tile::Void);
        assert_eq!(map.get(0, -1), Tile::Void);
        assert_eq!(map.get(10, 5), Tile::Void);
        assert_eq!(map.get(5, 10), Tile::Void);
    }
}
