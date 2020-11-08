use crate::{PathI32, PathF64, PointF64, Spline};

pub struct CompoundPath {
    pub paths: Vec<CompoundPathElement>,
}

pub enum CompoundPathElement {
    PathI32(PathI32),
    PathF64(PathF64),
    Spline(Spline),
}

impl Default for CompoundPath {
    fn default() -> Self {
        Self::new()
    }
}

impl CompoundPath {

    pub fn new() -> Self {
        Self {
            paths: vec![]
        }
    }

    pub fn append(&mut self, mut other: Self) {
        self.paths.append(&mut other.paths);
    }

    pub fn add_path_i32(&mut self, path: PathI32) {
        self.paths.push(CompoundPathElement::PathI32(path));
    }

    pub fn add_path_f64(&mut self, path: PathF64) {
        self.paths.push(CompoundPathElement::PathF64(path));
    }

    pub fn add_spline(&mut self, path: Spline) {
        self.paths.push(CompoundPathElement::Spline(path));
    }

    pub fn to_svg_string(&self, close: bool, offset: PointF64) -> (String, PointF64) {
        let origin = if !self.paths.is_empty() {
            match &self.paths[0] {
                CompoundPathElement::PathI32(p) => -p.path[0].to_point_f64(),
                CompoundPathElement::PathF64(p) => -p.path[0],
                CompoundPathElement::Spline(p) => -p.points[0],
            }
        } else {
            PointF64::default()
        };

        let string = self.paths.iter().map(|p| {
            match p {
                CompoundPathElement::PathI32(p) => p.to_svg_string(close, &origin.to_point_i32()),
                CompoundPathElement::PathF64(p) => p.to_svg_string(close, &origin),
                CompoundPathElement::Spline(p) => p.to_svg_string(close, &origin),
            }
        }).collect::<String>();

        (string, offset - origin)
    }
}

#[cfg(test)]
mod tests {
    use crate::PointI32;
    use super::*;

    #[test]
    fn test_to_svg_string() {
        let mut paths = CompoundPath::new();
        let mut path = PathI32::new();
        path.add(PointI32 { x: 1, y: 1 });
        path.add(PointI32 { x: 2, y: 1 });
        path.add(PointI32 { x: 2, y: 2 });
        path.add(PointI32 { x: 1, y: 1 });
        paths.add_path_i32(path);

        let (string, offset) = paths.to_svg_string(true, PointF64 { x: 0.0, y: 0.0 });
        assert_eq!("M0,0 L1,0 L1,1 Z ", string);
        assert_eq!(offset, PointF64 { x: 1.0, y: 1.0 });
    }

    #[test]
    fn test_to_svg_string_compound() {
        let mut paths = CompoundPath::new();

        let mut path1 = PathI32::new();
        path1.add(PointI32 { x: 1, y: 1 });
        path1.add(PointI32 { x: 2, y: 1 });
        path1.add(PointI32 { x: 2, y: 2 });
        path1.add(PointI32 { x: 1, y: 1 });
        paths.add_path_i32(path1);

        let mut path2 = PathI32::new();
        path2.add(PointI32 { x: 3, y: 3 });
        path2.add(PointI32 { x: 4, y: 3 });
        path2.add(PointI32 { x: 4, y: 4 });
        path2.add(PointI32 { x: 3, y: 3 });
        paths.add_path_i32(path2);

        let (string, offset) = paths.to_svg_string(true, PointF64 { x: 1.0, y: 1.0 });
        assert_eq!("M0,0 L1,0 L1,1 Z M2,2 L3,2 L3,3 Z ", string);
        assert_eq!(offset, PointF64 { x: 2.0, y: 2.0 });
    }
}