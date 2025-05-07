use crate::Point2D;

use super::Edge;

/// A continuous series of edges (i.e. lines, arcs, ...).
pub struct Path {
    cursor: Point2D,
    edges: Vec<Edge>,
}
impl Path {
    /// Construct an empty `Path` at a given starting point.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Path, Point2D};
    ///
    /// let path = Path::at(Point2D::from_m(1., 2.));
    /// assert_eq!(path.start(), Point2D::from_m(1., 2.))
    /// ```
    pub fn at(start: Point2D) -> Self {
        Self {
            cursor: start,
            edges: vec![],
        }
    }

    /// Add a line to the end of this `Path` that ends at a specified point.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Path, Point2D};
    ///
    /// let path = Path::at(Point2D::from_m(1., 2.)).line_to(Point2D::from_m(3., 4.));
    /// assert_eq!(path.end(), Point2D::from_m(3., 4.))
    /// ```
    pub fn line_to(&self, point: Point2D) -> Self {
        self.add_edge(Edge::Line(self.cursor, point))
    }

    /// Return the starting point of the `Path`.
    ///
    /// If the path does not have any edges, the cursor is returned.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Path, Point2D};
    ///
    /// let path = Path::at(Point2D::from_m(1., 2.)).line_to(Point2D::origin()).line_to(Point2D::from_m(3., 4.));
    /// assert_eq!(path.start(), Point2D::from_m(1., 2.));
    ///
    /// let empty_path = Path::at(Point2D::from_m(5., 6.));
    /// assert_eq!(empty_path.start(), Point2D::from_m(5., 6.));
    /// ```
    pub fn start(&self) -> Point2D {
        match self.edges.first() {
            Some(edge) => edge.start(),
            None => self.cursor,
        }
    }

    /// Return the ending point of the `Path`.
    ///
    /// If the path does not have any edges, the cursor is returned.
    ///
    /// # Example
    /// ```rust
    /// use anvil::{Path, Point2D};
    ///
    /// let path = Path::at(Point2D::from_m(1., 2.)).line_to(Point2D::origin()).line_to(Point2D::from_m(3., 4.));
    /// assert_eq!(path.end(), Point2D::from_m(3., 4.));
    ///
    /// let empty_path = Path::at(Point2D::from_m(5., 6.));
    /// assert_eq!(empty_path.end(), Point2D::from_m(5., 6.));
    /// ```
    pub fn end(&self) -> Point2D {
        match self.edges.iter().last() {
            Some(edge) => edge.end(),
            None => self.cursor,
        }
    }

    fn add_edge(&self, edge: Edge) -> Self {
        if edge.start() != self.end() {
            panic!("path is not continuous");
        }

        let new_cursor = edge.end();
        let mut new_edges = self.edges.clone();
        new_edges.push(edge);

        Self {
            cursor: new_cursor,
            edges: new_edges,
        }
    }
}
