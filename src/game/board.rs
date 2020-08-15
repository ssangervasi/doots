pub use crate::game::basic_types::{dot, edge, BoardSize, Dot, DotBox, Edge};
use crate::game::box_drawings::{lookup, BoxChar, LINE_H, LINE_V};
use crate::players::player::PlayerId;
use crate::utils::{pad_end, pad_out};

#[derive(Clone, Debug)]
pub struct Board {
    size: BoardSize,
    /* Edges are kept in the order they were drawn. */
    edges: Vec<Edge>,
    player_ids: Vec<PlayerId>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            size: 2,
            edges: vec![],
            player_ids: vec![PlayerId::One, PlayerId::Two],
        }
    }
}

impl Board {
    pub fn new(size: BoardSize) -> Board {
        Board {
            size,
            ..Default::default()
        }
    }

    pub fn size(&self) -> BoardSize {
        self.size
    }

    /* The number of dots in a row (equal to column) */
    pub fn dot_size(&self) -> BoardSize {
        self.size + 1
    }

    /* The number of dots in the whole board */
    pub fn dot_count(&self) -> BoardSize {
        let (sq, is_over) = (self.dot_size()).overflowing_pow(2);
        if is_over {
            // As if this will ever happened:
            BoardSize::MAX
        } else {
            sq
        }
    }

    pub fn edge_count(&self) -> BoardSize {
        2 * self.size * self.dot_size()
    }

    pub fn is_full(&self) -> bool {
        (self.edge_count() as usize) <= self.edges.len()
    }

    pub fn draw(&mut self, edge: Edge) -> Result<Edge, String> {
        let validation = self.validate_draw(edge);
        if validation.is_ok() {
            self.edges.push(edge);
        }
        validation
    }

    pub fn draw_many(&mut self, edges: Vec<Edge>) -> Result<BoardSize, String> {
        let mut success_count = 0;
        for edge in &edges {
            let validation = self.validate_draw(*edge);
            if validation.is_err() {
                return Err(validation.unwrap_err());
            }
            success_count += 1
        }
        for edge in edges {
            self.edges.push(edge);
        }
        Ok(success_count)
    }

    pub fn validate_draw(&self, edge: Edge) -> Result<Edge, String> {
        if !edge.is_valid() {
            return Err(format!("Cannot draw invalid edge: {}", edge));
        } else if !self.edge_fits(edge) {
            return Err(format!(
                "Edge {:?} does not fit in board of size {}",
                edge, self.size
            ));
        } else if self.is_drawn(edge) {
            return Err(format!("Cannot redraw edge: {}", edge));
        }
        Ok(edge)
    }

    pub fn is_free(&self, Edge(d1, d2): Edge) -> bool {
        for connected in self.find_connected(d1).iter() {
            if d2 == *connected {
                return false;
            }
        }
        true
    }

    pub fn is_drawn(&self, edge: Edge) -> bool {
        !self.is_free(edge)
    }

    /*
     * Which PlayerId drew the edge. At the moment, the owner methods are all
     * O(n) where n = number of edges drawn. This could be sped up by storing
     * owner information at draw time.
     */
    pub fn edge_owner(&self, edge: Edge) -> Option<PlayerId> {
        let indexes = self.indexes_of(vec![edge]);
        if indexes.len() < 1 {
            return None;
        }
        self.edge_index_owner(indexes[0])
    }

    fn edge_index_owner(&self, edge_index: usize) -> Option<PlayerId> {
        if self.edges.len() <= edge_index {
            return None;
        }
        let player_index = edge_index % self.player_ids.len();
        Some(self.player_ids[player_index])
    }

    fn indexes_of(&self, edges_to_find: Vec<Edge>) -> Vec<usize> {
        let mut found: Vec<usize> = vec![];
        for (i, &drawn_edge) in self.edges.iter().enumerate() {
            for &to_find in edges_to_find.iter() {
                if drawn_edge == to_find {
                    found.push(i)
                }
            }
            if found.len() == edges_to_find.len() {
                break;
            }
        }
        found
    }

    /*
     * Who owns a drawn box. Boxes are always identified by their upper-left corner,
     * so this method returns None if:
     *  - Fewer than 4 edges are drawn to complete the box,
     *  - Or the specified dot does not match a box corner: it is on the right
     *    or bottom edge.
     */
    pub fn box_owner(&self, corner: Dot) -> Option<PlayerId> {
        let dot_box = DotBox(corner);
        let indexes = self.indexes_of(dot_box.edges());
        if indexes.len() < 4 {
            return None;
        }
        self.edge_index_owner(indexes[3])
    }

    /* Whether the dot fits in this board. */
    pub fn dot_fits(&self, Dot { row, col }: Dot) -> bool {
        // Note that comparison to zero is unnecessary due to
        // unsigned integer type.
        row < self.dot_size() && col < self.dot_size()
    }

    /* Whether the edge fits in this board. */
    pub fn edge_fits(&self, Edge(d1, d2): Edge) -> bool {
        self.dot_fits(d1) && self.dot_fits(d2)
    }

    /* Iterate across all dots in order of left-to-right, top-to-botom. */
    pub fn iter_dots(&self) -> impl Iterator<Item = Dot> {
        let size = self.dot_size();
        (0..size).flat_map(move |row| (0..size).map(move |col| dot(row, col)))
    }

    /* Iterate across all edges in the same order as iter_dots.
     * All edges are relative to the upper-left of a square,
     * meaning every edge will be one of these two forms:
     *   - (upper_left_dot, upper_right_dot)
     *   - (upper_left_dot, lower_left_dot)
     * This also means that right-most and bottom-most edges will
     * appear only as second entries of an edge.
     */
    pub fn iter_edges(&self) -> impl Iterator<Item = Edge> {
        let max_dot_index = self.dot_size() - 1;
        self.iter_dots().flat_map(move |d| {
            let mut d_edges = vec![];
            // Right
            if d.col < max_dot_index {
                d_edges.push(Edge(d, d + dot(0, 1)));
            }
            // Down
            if d.row < max_dot_index {
                d_edges.push(Edge(d, d + dot(1, 0)));
            }

            d_edges
        })
    }

    /**
     * O(n). Should be able to make this O(1) by mapping...
     */
    pub fn find_edges(&self, dot: Dot) -> Vec<Edge> {
        let mut edges: Vec<Edge> = vec![];
        for edge in self.edges.iter() {
            if edge.has_dot(dot) {
                edges.push(*edge)
            }
        }
        edges
    }

    pub fn find_connected(&self, dot: Dot) -> Vec<Dot> {
        // let mut dots: Vec<Dot> = vec![];
        self.find_edges(dot)
            .iter()
            .map(|Edge(d1, d2)| if dot == *d1 { *d2 } else { *d1 })
            .collect()
    }

    pub fn to_string(&self) -> String {
        let cell_width = 3;
        let dot_size = self.dot_size();
        let mut grid: Vec<String> = Vec::new();

        // Header guide row:
        let mut row_string = " ".repeat(cell_width);
        for col in 0..dot_size {
            row_string.push_str(&pad_end(&col.to_string(), " ", cell_width));
        }
        grid.push(row_string);

        for row in 0..dot_size {
            // Left guide column:
            let mut dot_row_string = pad_out(&row.to_string(), " ", cell_width);
            let mut fill_row_string = pad_out("", " ", cell_width);

            for col in 0..dot_size {
                // Pick the appropriate box intersection:
                let entry = self.choose_char(dot(row, col));

                // Extend right to account for horizontal space:
                let spacer_h = if entry.right { LINE_H } else { ' ' };
                dot_row_string.push_str(&pad_end(
                    &entry.value.to_string(),
                    &spacer_h.to_string(),
                    cell_width,
                ));

                // Extend down for fill-in row:
                let spacer_v = if entry.down { LINE_V } else { ' ' };
                fill_row_string.push_str(&pad_end(&spacer_v.to_string(), " ", cell_width))
            }
            grid.push(dot_row_string);
            grid.push(fill_row_string);
        }
        // Dropping the extra is easier than checking the edges.
        grid.pop();

        grid.join("\n")
    }

    pub fn choose_char(&self, dot: Dot) -> BoxChar {
        let mut box_char = BoxChar::default();
        for connected in self.find_connected(dot).iter() {
            // Note that only one of these can be true because dots are not
            // allowed to connect diagonally.
            if connected.row < dot.row {
                box_char.up = true
            } else if dot.col < connected.col {
                box_char.right = true
            } else if dot.row < connected.row {
                box_char.down = true
            } else if connected.col < dot.col {
                box_char.left = true
            }
        }
        lookup(box_char)
    }
}