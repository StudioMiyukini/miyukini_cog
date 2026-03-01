// @id: MGE-Collision-Tests @do: test @role: back-end @layer: 2 @human: miyuk

//! Tests for mge-collision.

#[cfg(test)]
mod tests {
    use mge_math::{Aabb, Vec2};

    use crate::collider::{Collider, CollisionPair};
    use crate::grid::SpatialGrid;
    use crate::layer::CollisionLayer;
    use crate::world::CollisionWorld;

    // ===================================================================
    // CollisionLayer
    // ===================================================================

    #[test]
    fn collision_layer_none_contains_nothing() {
        assert!(!CollisionLayer::NONE.contains(CollisionLayer::PLAYER));
        assert!(!CollisionLayer::NONE.contains(CollisionLayer::MONSTER));
        assert!(!CollisionLayer::NONE.contains(CollisionLayer::STATIC));
        assert!(!CollisionLayer::NONE.contains(CollisionLayer::NONE));
    }

    #[test]
    fn collision_layer_union() {
        let combined = CollisionLayer::PLAYER.union(CollisionLayer::MONSTER);
        assert!(combined.contains(CollisionLayer::PLAYER));
        assert!(combined.contains(CollisionLayer::MONSTER));
        assert!(!combined.contains(CollisionLayer::PROJECTILE));
    }

    #[test]
    fn collision_layer_contains() {
        assert!(CollisionLayer::PLAYER.contains(CollisionLayer::PLAYER));
        assert!(!CollisionLayer::PLAYER.contains(CollisionLayer::MONSTER));
        let all = CollisionLayer::ALL;
        assert!(all.contains(CollisionLayer::PLAYER));
        assert!(all.contains(CollisionLayer::TRIGGER));
    }

    // ===================================================================
    // Collider
    // ===================================================================

    fn make_aabb(x: f32, y: f32, w: f32, h: f32) -> Aabb {
        Aabb::new(Vec2::new(x, y), Vec2::new(x + w, y + h))
    }

    #[test]
    fn collider_intersects_same_layer() {
        let a = Collider::new(
            make_aabb(0.0, 0.0, 10.0, 10.0),
            CollisionLayer::PLAYER,
            CollisionLayer::PLAYER,
        );
        let b = Collider::new(
            make_aabb(5.0, 5.0, 10.0, 10.0),
            CollisionLayer::PLAYER,
            CollisionLayer::PLAYER,
        );
        assert!(a.intersects(&b));
    }

    #[test]
    fn collider_no_intersect_different_layer_mask() {
        // Player collider with mask=PLAYER only, vs Monster collider
        let a = Collider::new(
            make_aabb(0.0, 0.0, 10.0, 10.0),
            CollisionLayer::PLAYER,
            CollisionLayer::PLAYER, // only interacts with PLAYER
        );
        let b = Collider::new(
            make_aabb(5.0, 5.0, 10.0, 10.0),
            CollisionLayer::MONSTER,
            CollisionLayer::MONSTER, // only interacts with MONSTER
        );
        // AABBs overlap, but layers don't match masks
        assert!(!a.intersects(&b));
    }

    #[test]
    fn collider_trigger_flag() {
        let t = Collider::trigger(make_aabb(0.0, 0.0, 10.0, 10.0), CollisionLayer::TRIGGER);
        assert!(t.is_trigger);
        assert_eq!(t.layer, CollisionLayer::TRIGGER);
        assert_eq!(t.mask, CollisionLayer::ALL);
    }

    // ===================================================================
    // SpatialGrid
    // ===================================================================

    #[test]
    fn spatial_grid_insert_and_query() {
        let mut grid = SpatialGrid::new(64.0);
        let aabb = make_aabb(10.0, 10.0, 20.0, 20.0);
        grid.insert(42, &aabb);

        // Query overlapping region
        let results = grid.query(&make_aabb(15.0, 15.0, 5.0, 5.0));
        assert!(results.contains(&42));

        // Query non-overlapping region (far away)
        let results_far = grid.query(&make_aabb(500.0, 500.0, 10.0, 10.0));
        assert!(!results_far.contains(&42));
    }

    #[test]
    fn spatial_grid_clear() {
        let mut grid = SpatialGrid::new(64.0);
        grid.insert(1, &make_aabb(0.0, 0.0, 10.0, 10.0));
        grid.insert(2, &make_aabb(32.0, 32.0, 10.0, 10.0));

        grid.clear();

        let results = grid.query(&make_aabb(0.0, 0.0, 100.0, 100.0));
        assert!(results.is_empty());
    }

    #[test]
    fn spatial_grid_query_empty_returns_empty() {
        let grid = SpatialGrid::new(64.0);
        let results = grid.query(&make_aabb(0.0, 0.0, 100.0, 100.0));
        assert!(results.is_empty());
    }

    // ===================================================================
    // CollisionWorld
    // ===================================================================

    #[test]
    fn collision_world_insert_remove() {
        let mut world = CollisionWorld::new();
        assert!(world.is_empty());

        let c = Collider::new(
            make_aabb(0.0, 0.0, 10.0, 10.0),
            CollisionLayer::PLAYER,
            CollisionLayer::ALL,
        );
        world.insert(1, c);
        assert_eq!(world.len(), 1);

        world.remove(1).unwrap();
        assert!(world.is_empty());

        // Removing a non-existent entity returns an error
        assert!(world.remove(999).is_err());
    }

    #[test]
    fn collision_world_detect_overlapping_pair() {
        let mut world = CollisionWorld::new();

        let layer = CollisionLayer::PLAYER;
        let mask = CollisionLayer::PLAYER;

        // Two overlapping colliders
        world.insert(1, Collider::new(make_aabb(0.0, 0.0, 20.0, 20.0), layer, mask));
        world.insert(2, Collider::new(make_aabb(10.0, 10.0, 20.0, 20.0), layer, mask));

        let pairs = world.detect_all();
        assert_eq!(pairs.len(), 1);

        let pair = &pairs[0];
        // Pair is ordered (entity_a < entity_b)
        assert!(pair.entity_a < pair.entity_b);
        // Overlap vector should be non-zero
        assert!(pair.overlap.length() > 0.0);

        // Verify penetration: overlap on both axes is 10.0,
        // so the min penetration is 10.0 on either axis
        let pen = pair.overlap;
        assert!(
            (pen.x.abs() - 10.0).abs() < f32::EPSILON
                || (pen.y.abs() - 10.0).abs() < f32::EPSILON,
            "Expected penetration of 10.0 on one axis, got {:?}",
            pen
        );
    }

    #[test]
    fn collision_world_detect_no_overlap() {
        let mut world = CollisionWorld::new();

        let layer = CollisionLayer::PLAYER;
        let mask = CollisionLayer::PLAYER;

        // Two non-overlapping colliders
        world.insert(1, Collider::new(make_aabb(0.0, 0.0, 10.0, 10.0), layer, mask));
        world.insert(2, Collider::new(make_aabb(100.0, 100.0, 10.0, 10.0), layer, mask));

        let pairs = world.detect_all();
        assert!(pairs.is_empty());
    }

    #[test]
    fn collision_world_query_area() {
        let mut world = CollisionWorld::new();

        world.insert(
            1,
            Collider::new(
                make_aabb(0.0, 0.0, 10.0, 10.0),
                CollisionLayer::PLAYER,
                CollisionLayer::ALL,
            ),
        );
        world.insert(
            2,
            Collider::new(
                make_aabb(200.0, 200.0, 10.0, 10.0),
                CollisionLayer::MONSTER,
                CollisionLayer::ALL,
            ),
        );

        // Query area overlapping only entity 1
        let hits = world.query_area(&make_aabb(0.0, 0.0, 15.0, 15.0));
        assert!(hits.contains(&1));
        assert!(!hits.contains(&2));
    }

    #[test]
    fn collision_world_is_empty() {
        let world = CollisionWorld::new();
        assert!(world.is_empty());
        assert_eq!(world.len(), 0);
    }

    // ===================================================================
    // Bonus: CollisionPair is well-formed
    // ===================================================================

    #[test]
    fn collision_pair_fields_accessible() {
        let pair = CollisionPair {
            entity_a: 1,
            entity_b: 2,
            overlap: Vec2::new(5.0, 0.0),
        };
        assert_eq!(pair.entity_a, 1);
        assert_eq!(pair.entity_b, 2);
        assert!((pair.overlap.x - 5.0).abs() < f32::EPSILON);
    }
}
