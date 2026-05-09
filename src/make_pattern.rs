use std::collections::HashMap;

use crate::{dir::Dir, point::Point, point_storage::PointStorage};

type Points = HashMap<Point, PointStorage>;
pub type Lines = Vec<[Point; 2]>;

pub fn make_pattern(levels: usize, finish_ends: bool, exhaust_edges: bool, exhaust_vertices: bool) -> Lines {
    let mut lines = Vec::new();

    let mut points: Points = HashMap::new();

    // when two points create a vertex, a spur is added to a vertex queue
    let mut vertices: Vec<Point> = Vec::new();
    let mut next_vertices: Vec<Point> = Vec::new();

    // edges tracks points that have zero or one line connected to it
    let mut edges: Vec<Point> = Vec::new();
    let mut next_edges: Vec<Point> = Vec::new();

    // seed the pattern
    seed_pattern(&mut lines, &mut vertices, &mut points);

    // main loop
    for level in 1..levels {
        if level == levels - 1 {
            println!("Entering level {level}, the last one");
        }

        // vertices
        loop {
            add_vertices(
                &mut points,
                &mut edges,
                &mut vertices,
                &mut next_vertices,
                &mut lines,
            );
            if !exhaust_vertices || vertices.is_empty() {
                break;
            }
        }

        // edges
        loop {
            add_edges(
                &mut points,
                &mut edges,
                &mut next_edges,
                &mut vertices,
                &mut lines,
            );

            if finish_ends && level == levels - 1 && !edges.is_empty() {
                continue;
            }
            if !exhaust_edges || edges.is_empty() {
                break;
            }
        }
    }

    lines
}

fn seed_pattern(
    lines: &mut Lines,
    vertices: &mut Vec<Point>,
    points: &mut HashMap<Point, PointStorage>,
) {
    (0..8).fold((Point::default(), Dir::NorthEast), |(pos, dir), _| {
        let next_dir = dir.right();
        let next_pos = pos.go(next_dir);

        // add line
        lines.push([pos, next_pos]);

        // bump points and add vertices
        bump_points(points, vertices, pos, dir);
        bump_points(points, vertices, next_pos, next_dir);

        (next_pos, next_dir)
    });
}

fn add_vertices(
    points: &mut Points,
    edges: &mut Vec<Point>,
    vertices: &mut Vec<Point>,
    next_vertices: &mut Vec<Point>,
    lines: &mut Lines,
) {
    for vertex in vertices.drain(..) {
        // get around no reborrow rule by tracking if a vertex was actually created
        let mut vertex_created: Option<(Point, Dir)> = None;

        points.entry(vertex).and_modify(|p| {
            match p {
                PointStorage::One(_) => {
                    unreachable!("a point added to vertices queue should never have just one point")
                }
                PointStorage::Two(dir) => {
                    let next_dir = dir.left();
                    let next_pos = vertex.go(next_dir);
                    lines.push([vertex, next_pos]);
                    *p = PointStorage::Three;

                    vertex_created = Some((next_pos, next_dir));
                }
                PointStorage::Three => {} // take no action
            }
        });

        // update the edge of the vertex just created
        if let Some((next_pos, next_dir)) = vertex_created {
            bump_points(points, next_vertices, next_pos, next_dir);
            if let Some(PointStorage::One(_)) = points.get(&next_pos) {
                edges.push(next_pos);
            }
        }
    }
    std::mem::swap(vertices, next_vertices);
}

fn add_edges(
    points: &mut Points,
    edges: &mut Vec<Point>,
    next_edges: &mut Vec<Point>,
    vertices: &mut Vec<Point>,
    lines: &mut Lines,
) {
    for edge in edges.drain(..) {
        // get around no reborrow rule by tracking if a edge was actually created
        let mut edge_created: Option<(Dir, Point, Dir)> = None;

        if let Some(PointStorage::One(dir)) = points.get(&edge) {
            let next_dir = dir.right();
            let next_pos = edge.go(next_dir);

            // create line
            lines.push([edge, next_pos]);

            // give info for bumping points below
            edge_created = Some((*dir, next_pos, next_dir));
        }

        if let Some((dir, next_pos, next_dir)) = edge_created {
            bump_points(points, vertices, edge, dir);
            bump_points(points, vertices, next_pos, next_dir);
            if let Some(PointStorage::One(_)) = points.get(&next_pos) {
                next_edges.push(next_pos);
            }
        }
    }
    std::mem::swap(edges, next_edges);
}

fn bump_points(points: &mut Points, vertices: &mut Vec<Point>, pos: Point, dir: Dir) {
    points
        .entry(pos)
        .and_modify(|p| {
            *p = match p {
                PointStorage::One(dir) => {
                    vertices.push(pos);
                    PointStorage::Two(*dir)
                }
                _ => PointStorage::Three,
            };
        })
        .or_insert(PointStorage::One(dir));
}
