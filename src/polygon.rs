use voracious_radix_sort::{RadixSort, Radixable};

pub struct Vector2D {
    pub x: i64,
    pub y: i64,

    pub h_neighbor: Option<usize>,
    pub v_neighbor: Option<usize>,

    pub h_chord: Option<usize>,
    pub v_chord: Option<usize>,
}

impl Vector2D {
    fn new(x: i64, y: i64) -> Self {
        Vector2D {
            x,
            y,
            h_neighbor: None,
            v_neighbor: None,
            h_chord: None,
            v_chord: None,
        }
    }
}

impl std::fmt::Debug for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug)]
struct Key {
    key: u64,
    index: usize,
}

impl Key {
    fn new(key: u64, index: usize) -> Self {
        Key { key, index }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl Radixable<u64> for Key {
    type Key = u64;
    fn key(&self) -> u64 {
        self.key
    }
}

pub struct RectilinearPolygon {
    pub vertices: Vec<Vector2D>,
    pub start: usize,
    pub boundary: Vec<usize>,
    pub min_x: i64,
    pub max_x: i64,
    pub min_y: i64,
    pub max_y: i64,
}

impl RectilinearPolygon {
    pub fn from_vertices(vertices: &[(i64, i64)]) -> Self {
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;

        let size = vertices.len();

        let mut vs = Vec::with_capacity(size);

        for (x, y) in vertices {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }

            vs.push(Vector2D::new(*x, *y));
        }

        let x_range = (max_x - min_x) as u64;
        let y_range = (max_y - min_y) as u64;

        // Create keys for vertical sorting
        let mut e_vs: Vec<Key> = (0..size)
            .map(|i| {
                let v = &vs[i];
                Key::new(
                    (v.x - min_x) as u64 * (y_range + 1) + (v.y - min_y) as u64,
                    i,
                )
            })
            .collect();
        e_vs.voracious_sort();

        // Assign vertical neighbors
        for i in 0..size - 1 {
            let a = e_vs[i].index;
            let b = e_vs[i + 1].index;

            if vs[a].v_neighbor.is_none() {
                debug_assert!(vs[a].x == vs[b].x);

                vs[a].v_neighbor = Some(b);
                vs[b].v_neighbor = Some(a);
            }
        }

        // Update keys for horizontal sorting
        for key in e_vs.iter_mut() {
            let v = &vs[key.index];
            key.key = (v.y - min_y) as u64 * (x_range + 1) + (v.x - min_x) as u64;
        }
        e_vs.voracious_sort();

        // Assign horizontal neighbors
        for i in 0..size - 1 {
            let a = e_vs[i].index;
            let b = e_vs[i + 1].index;

            if vs[a].h_neighbor.is_none() {
                debug_assert!(vs[a].y == vs[b].y);

                vs[a].h_neighbor = Some(b);
                vs[b].h_neighbor = Some(a);
            } else {
                if vs[a].y == vs[b].y {
                    vs[a].h_chord = Some(b);
                    vs[b].h_chord = Some(a);
                }
            }
        }

        // Check that all vertices have a horizontal and vertical neighbor
        for v in &vs {
            debug_assert!(v.h_neighbor.is_some());
            debug_assert!(v.v_neighbor.is_some());
        }

        let mut rp = RectilinearPolygon {
            vertices: vs,
            start: e_vs[0].index,
            boundary: Vec::with_capacity(size),
            min_x,
            max_x,
            min_y,
            max_y,
        };

        rp.compute_boundary();

        rp
    }

    pub fn compute_boundary(&mut self) {
        let vs = &self.vertices;

        let boundary = &mut self.boundary;
        let start = self.start;
        let mut next = start;
        let mut is_h = true;

        boundary.clear();

        loop {
            boundary.push(next);

            if is_h {
                next = vs[next].h_neighbor.unwrap();
            } else {
                next = vs[next].v_neighbor.unwrap();
            }
            is_h = !is_h;

            if next == start {
                break;
            }
        }
    }

    pub fn find_vchords(&mut self) {
        let mut vstack = Vec::new();

        let vs = &mut self.vertices;
        let boundary = &self.boundary;

        // Find vertical chords in the right chain
        for i in 0..boundary.len() - 2 {
            let a = boundary[i];
            let b = boundary[i + 1];
            let c = boundary[i + 2];

            if vs[a].y > vs[b].y {
                break;
            }

            if vs[a].x == vs[b].x && vs[b].x < vs[c].x {
                // Start of a possible vertical chord
                vstack.push(b);
            }

            if vs[a].x > vs[b].x && vs[b].x == vs[c].x {
                // End of a possible vertical chord
                for j in (0..vstack.len()).rev() {
                    let d = vstack[j];

                    if vs[b].x == vs[d].x {
                        vs[b].v_chord = Some(d);
                        vs[d].v_chord = Some(b);
                        vstack.truncate(j);
                    } else if vs[b].x < vs[d].x {
                        vstack.truncate(j);
                    }
                }
            }
        }

        // Find vertical chords in the left chain
        for i in (2..boundary.len()).rev() {
            let a = boundary[i];
            let b = boundary[i - 1];
            let c = boundary[i - 2];

            if vs[a].y > vs[b].y {
                break;
            }

            if vs[a].x == vs[b].x && vs[b].x > vs[c].x {
                // Start of a possible vertical chord
                vstack.push(b);
            }

            if vs[a].x < vs[b].x && vs[b].x == vs[c].x {
                // End of a possible vertical chord
                for j in (0..vstack.len()).rev() {
                    let d = vstack[j];

                    if vs[b].x == vs[d].x {
                        vs[b].v_chord = Some(d);
                        vs[d].v_chord = Some(b);
                        vstack.truncate(j);
                    } else if vs[b].x > vs[d].x {
                        vstack.truncate(j);
                    }
                }
            }
        }
    }

    pub fn make_cuts(&mut self) {
        let vs = &mut self.vertices;
        let boundary = &self.boundary;

        for &i in boundary {
            if let Some(v_chord) = vs[i].v_chord {
                let outer_a = vs[i].v_neighbor.unwrap();
                let outer_b = vs[v_chord].v_neighbor.unwrap();

                vs[outer_a].v_neighbor = Some(outer_b);
                vs[outer_b].v_neighbor = Some(outer_a);
            }
        }
    }
}
