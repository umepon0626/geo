use super::Intersects;
use crate::contains::Contains;
use crate::kernels::*;
use crate::utils::{coord_pos_relative_to_ring, CoordPos};
use crate::*;

impl<T> Intersects<Coordinate<T>> for Polygon<T>
where
    T: HasKernel,
{
    fn intersects(&self, p: &Coordinate<T>) -> bool {
        coord_pos_relative_to_ring(*p, &self.exterior()) != CoordPos::Outside
            && self
                .interiors()
                .iter()
                .all(|int| coord_pos_relative_to_ring(*p, int) != CoordPos::Inside)
    }
}

impl<T> Intersects<Point<T>> for Polygon<T>
where
    T: HasKernel,
{
    fn intersects(&self, p: &Point<T>) -> bool {
        self.intersects(&p.0)
    }
}

impl<T> Intersects<Line<T>> for Polygon<T>
where
    T: HasKernel,
{
    fn intersects(&self, p: &Line<T>) -> bool {
        self.exterior().intersects(p)
            || self.interiors().iter().any(|inner| inner.intersects(p))
            || self.contains(&p.start)
            || self.contains(&p.end)
    }
}

impl<T> Intersects<LineString<T>> for Polygon<T>
where
    T: HasKernel,
{
    fn intersects(&self, linestring: &LineString<T>) -> bool {
        // line intersects inner or outer polygon edge
        if self.exterior().intersects(linestring)
            || self
                .interiors()
                .iter()
                .any(|inner| inner.intersects(linestring))
        {
            true
        } else {
            // or if it's contained in the polygon
            linestring.points_iter().any(|point| self.contains(&point))
        }
    }
}

impl<T> Intersects<Rect<T>> for Polygon<T>
where
    T: HasKernel,
{
    fn intersects(&self, rect: &Rect<T>) -> bool {
        let p = Polygon::new(
            LineString::from(vec![
                (rect.min().x, rect.min().y),
                (rect.min().x, rect.max().y),
                (rect.max().x, rect.max().y),
                (rect.max().x, rect.min().y),
                (rect.min().x, rect.min().y),
            ]),
            vec![],
        );
        self.intersects(&p)
    }
}

impl<T> Intersects<Polygon<T>> for Polygon<T>
where
    T: HasKernel,
{
    fn intersects(&self, polygon: &Polygon<T>) -> bool {
        // self intersects (or contains) any line in polygon
        self.intersects(polygon.exterior()) ||
            polygon.interiors().iter().any(|inner_line_string| self.intersects(inner_line_string)) ||
            // self is contained inside polygon
            polygon.intersects(self.exterior())
    }
}
