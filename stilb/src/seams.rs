use std::collections::HashSet;

use crate::math::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Edge {
    pub a: u32,
    pub b: u32,
}

impl Edge {
    #[inline]
    fn new_sorted(i0: u32, i1: u32) -> Self {
        if i0 < i1 {
            Self { a: i0, b: i1 }
        } else {
            Self { a: i1, b: i0 }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Seam {
    // Edge0
    pub e0p0: Vector3,
    pub e0uv0: Vector2,
    pub e0p1: Vector3,
    pub e0uv1: Vector2,
    // Edge1
    pub e1p0: Vector3,
    pub e1uv0: Vector2,
    pub e1p1: Vector3,
    pub e1uv1: Vector2,
}

#[inline]
fn approx_eq_vec3(a: Vector3, b: Vector3) -> bool {
    const EPS: f32 = 0.0001;
    (a - b).length_squared() < EPS * EPS
}

#[inline]
fn approx_eq_vec2(a: Vector2, b: Vector2) -> bool {
    const EPS: f32 = 0.0001;
    (a - b).length_squared() < EPS * EPS
}

pub fn find_seams(
    indices: &[u32],
    positions: &[Vector3],
    normals: &[Vector3],
    uvs: &[Vector2],
) -> Vec<Seam> {
    let mut edges = HashSet::new();

    let is_seam = |a: &Edge, b: &Edge| -> bool {
        let pa0 = positions[a.a as usize];
        let na0 = normals[a.a as usize];
        let uva0 = uvs[a.a as usize];

        let pb0 = positions[b.a as usize];
        let nb0 = normals[b.a as usize];
        let uvb0 = uvs[b.a as usize];

        let positions_equal = approx_eq_vec3(pa0, pb0);
        let normals_equal = approx_eq_vec3(na0, nb0);
        let uvs_equal = approx_eq_vec2(uva0, uvb0);

        if positions_equal && normals_equal && !uvs_equal {
            let pa1 = positions[a.b as usize];
            let na1 = normals[a.b as usize];
            let uva1 = uvs[a.b as usize];
            let pb1 = positions[b.b as usize];
            let nb1 = normals[b.b as usize];
            let uvb1 = uvs[b.b as usize];

            let positions_equal = approx_eq_vec3(pa1, pb1);
            let normals_equal = approx_eq_vec3(na1, nb1);
            let uvs_equal = approx_eq_vec2(uva1, uvb1);

            return positions_equal && normals_equal && !uvs_equal;
        }

        false
    };

    let mut i = 0;
    while i + 2 < indices.len() {
        let i0 = indices[i + 0];
        let i1 = indices[i + 1];
        let i2 = indices[i + 2];

        edges.insert(Edge::new_sorted(i0, i1));
        edges.insert(Edge::new_sorted(i1, i2));
        edges.insert(Edge::new_sorted(i2, i0));

        i += 3;
    }

    let edges: Vec<Edge> = edges.into_iter().collect();

    let mut seams = Vec::new();
    for i in 0..edges.len() {
        for j in (i + 1)..edges.len() {
            let e0 = &edges[i];
            let e1 = &edges[j];

            if is_seam(e0, e1) {
                let e0p0 = positions[e0.a as usize];
                let e0uv0 = uvs[e0.a as usize];
                let e0p1 = positions[e0.b as usize];
                let e0uv1 = uvs[e0.b as usize];

                let e1p0 = positions[e1.a as usize];
                let e1uv0 = uvs[e1.a as usize];
                let e1p1 = positions[e1.b as usize];
                let e1uv1 = uvs[e1.b as usize];

                seams.push(Seam {
                    e0p0,
                    e0uv0,
                    e0p1,
                    e0uv1,
                    e1p0,
                    e1uv0,
                    e1p1,
                    e1uv1,
                });
            }
        }
    }

    println!("found {} seams out of {} edges", seams.len(), edges.len());

    seams
}
