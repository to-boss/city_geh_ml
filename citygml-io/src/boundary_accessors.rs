use citygml_types::construction::{
    CeilingSurface, DoorSurface, FloorSurface, GroundSurface, InteriorWallSurface,
    OuterCeilingSurface, OuterFloorSurface, RoofSurface, WallSurface, WindowSurface,
};
use citygml_types::core::{AbstractSpaceBoundary, ClosureSurface};

/// Typed accessor methods for filtering `AbstractSpaceBoundary` slices by surface type.
///
/// Works on any struct with a `boundary: Vec<AbstractSpaceBoundary>` field:
/// ```ignore
/// let walls: Vec<&WallSurface> = building.boundary.wall_surfaces().collect();
/// ```
pub trait BoundaryAccessors {
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface>;
    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface>;
    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface>;
    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface>;
    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface>;
    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface>;
    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface>;
    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface>;
    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface>;
    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface>;
    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface>;
}

impl BoundaryAccessors for [AbstractSpaceBoundary] {
    fn wall_surfaces(&self) -> impl Iterator<Item = &WallSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::WallSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn roof_surfaces(&self) -> impl Iterator<Item = &RoofSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::RoofSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn ground_surfaces(&self) -> impl Iterator<Item = &GroundSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::GroundSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn ceiling_surfaces(&self) -> impl Iterator<Item = &CeilingSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::CeilingSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn floor_surfaces(&self) -> impl Iterator<Item = &FloorSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::FloorSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn interior_wall_surfaces(&self) -> impl Iterator<Item = &InteriorWallSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::InteriorWallSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn outer_ceiling_surfaces(&self) -> impl Iterator<Item = &OuterCeilingSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::OuterCeilingSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn outer_floor_surfaces(&self) -> impl Iterator<Item = &OuterFloorSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::OuterFloorSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn closure_surfaces(&self) -> impl Iterator<Item = &ClosureSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::ClosureSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn door_surfaces(&self) -> impl Iterator<Item = &DoorSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::DoorSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }

    fn window_surfaces(&self) -> impl Iterator<Item = &WindowSurface> {
        self.iter().filter_map(|b| match b {
            AbstractSpaceBoundary::WindowSurface(v) => Some(v.as_ref()),
            _ => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_slice_returns_no_surfaces() {
        let boundaries: Vec<AbstractSpaceBoundary> = vec![];
        assert_eq!(boundaries.wall_surfaces().count(), 0);
        assert_eq!(boundaries.roof_surfaces().count(), 0);
        assert_eq!(boundaries.ground_surfaces().count(), 0);
    }

    #[test]
    fn filters_by_variant() {
        let boundaries = vec![
            AbstractSpaceBoundary::WallSurface(Box::new(WallSurface::default())),
            AbstractSpaceBoundary::RoofSurface(Box::new(RoofSurface::default())),
            AbstractSpaceBoundary::WallSurface(Box::new(WallSurface::default())),
            AbstractSpaceBoundary::GroundSurface(Box::new(GroundSurface::default())),
        ];
        assert_eq!(boundaries.wall_surfaces().count(), 2);
        assert_eq!(boundaries.roof_surfaces().count(), 1);
        assert_eq!(boundaries.ground_surfaces().count(), 1);
        assert_eq!(boundaries.ceiling_surfaces().count(), 0);
    }
}
