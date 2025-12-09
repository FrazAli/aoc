use std::collections::HashMap;
use std::error::Error;

use common::input::Input;

#[derive(Clone, Copy, Debug)]
struct Column {
    x: i64,
    min_y: i64,
    max_y: i64,
}

#[derive(Clone, Copy, Debug)]
struct HLineSegment {
    y: i64,
    x1: i64,
    x2: i64,
}

#[derive(Clone, Copy, Debug)]
struct VLineSegment {
    x: i64,
    y1: i64,
    y2: i64,
}

fn parse_points(input: &str) -> Result<Vec<(i64, i64)>, Box<dyn Error>> {
    let mut points = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut coords = trimmed.split(',');
        let x = coords
            .next()
            .ok_or_else(|| format!("Missing x on line {}", idx + 1))?
            .trim()
            .parse::<i64>()?;
        let y = coords
            .next()
            .ok_or_else(|| format!("Missing y on line {}", idx + 1))?
            .trim()
            .parse::<i64>()?;

        points.push((x, y));
    }

    Ok(points)
}

fn build_columns(points: &[(i64, i64)]) -> Vec<Column> {
    let mut hmap: HashMap<i64, (i64, i64)> = HashMap::new();

    for &(x, y) in points {
        hmap.entry(x)
            .and_modify(|(min_y, max_y)| {
                if y < *min_y {
                    *min_y = y;
                }

                if y > *max_y {
                    *max_y = y;
                }
            })
            .or_insert((y, y));
    }

    let mut cols: Vec<Column> = hmap
        .into_iter()
        .map(|(x, (min_y, max_y))| Column { x, min_y, max_y })
        .collect();

    cols.sort_by_key(|c| c.x);
    cols
}

fn max_rectangle_area(columns: &[Column]) -> i64 {
    let mut best: i64 = columns
        .iter()
        .map(|c| c.max_y - c.min_y + 1)
        .max()
        .unwrap_or(0);
    for i in 0..columns.len() {
        for j in (i + 1)..columns.len() {
            let dx = (columns[j].x - columns[i].x).abs();
            let diff1 = columns[j].max_y - columns[i].min_y;
            let diff2 = columns[i].max_y - columns[j].min_y;
            let dy = diff1.max(diff2);

            let area = (dx + 1) * (dy + 1);
            if area > best {
                best = area;
            }
        }
    }

    best
}

fn build_line_segments(points: &[(i64, i64)]) -> (Vec<HLineSegment>, Vec<VLineSegment>) {
    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();
    if points.is_empty() {
        return (horizontal, vertical);
    }

    for idx in 0..points.len() {
        let a = points[idx];
        let b = points[(idx + 1) % points.len()];
        if a.0 == b.0 {
            let (y1, y2) = if a.1 <= b.1 { (a.1, b.1) } else { (b.1, a.1) };
            vertical.push(VLineSegment { x: a.0, y1, y2 });
        } else if a.1 == b.1 {
            let (x1, x2) = if a.0 <= b.0 { (a.0, b.0) } else { (b.0, a.0) };
            horizontal.push(HLineSegment { y: a.1, x1, x2 });
        } else {
            continue;
        }
    }

    (horizontal, vertical)
}

fn point_on_boundary(
    px: i64,
    py: i64,
    horizontal: &[HLineSegment],
    vertical: &[VLineSegment],
) -> bool {
    for h in horizontal {
        if py == h.y && px >= h.x1 && px <= h.x2 {
            return true;
        }
    }

    for v in vertical {
        if px == v.x && py >= v.y1 && py <= v.y2 {
            return true;
        }
    }

    false
}

// ray casting to see if the point is inside, see: https://en.wikipedia.org/wiki/Point_in_polygon
fn point_inside(px: i64, py: i64, horizontal: &[HLineSegment], vertical: &[VLineSegment]) -> bool {
    if point_on_boundary(px, py, horizontal, vertical) {
        return true;
    }

    let xray = px as f64 + 0.5;
    let yray = py as f64 + 0.5;
    let mut crossings = 0;

    for v in vertical {
        if v.x as f64 <= xray {
            continue;
        }

        let y1 = v.y1 as f64;
        let y2 = v.y2 as f64;
        if y1 <= yray && yray < y2 {
            crossings += 1;
        }
    }

    // if there's an odd number of crossings, the point is inside
    crossings % 2 == 1
}

fn boundary_cuts_rectangle(
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64,
    horizontal: &[HLineSegment],
    vertical: &[VLineSegment],
) -> bool {
    if maxx - minx >= 2 {
        for h in horizontal {
            if h.y <= miny || h.y >= maxy {
                continue;
            }

            // clamp segment to within rectangle and check if any part of it is still inside
            let left = h.x1.max(minx + 1);
            let right = h.x2.min(maxx - 1);
            if left <= right {
                return true;
            }
        }
    }

    if maxy - miny >= 2 {
        for v in vertical {
            if v.x <= minx || v.x >= maxx {
                continue;
            }

            // clamp segment to within rectangle and check if any part of it is still inside
            let top = v.y1.max(miny + 1);
            let bottom = v.y2.min(maxy - 1);
            if top <= bottom {
                return true;
            }
        }
    }

    false
}

fn max_rectangle_area_inside(
    points: &[(i64, i64)],
    horizontal: &[HLineSegment],
    vertical: &[VLineSegment],
) -> i64 {
    let mut best = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let minx = x1.min(x2);
            let maxx = x1.max(x2);
            let miny = y1.min(y2);
            let maxy = y1.max(y2);

            let midx = (minx + maxx) / 2;
            let midy = (miny + maxy) / 2;

            if !point_inside(midx, midy, horizontal, vertical) {
                continue;
            }

            if boundary_cuts_rectangle(minx, maxx, miny, maxy, horizontal, vertical) {
                continue;
            }

            let area = (maxx - minx + 1) * (maxy - miny + 1);
            if area > best {
                best = area;
            }
        }
    }

    best
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_default()?;
    let points = parse_points(&input.raw)?;
    let columns = build_columns(&points);

    let part1 = max_rectangle_area(&columns);
    println!("Part 1: {part1}");

    let (hsegments, vsegments) = build_line_segments(&points);
    let part2 = max_rectangle_area_inside(&points, &hsegments, &vsegments);
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = concat!(
        "7,1\n", "11,1\n", "11,7\n", "9,7\n", "9,5\n", "2,5\n", "2,3\n", "7,3\n",
    );

    #[test]
    fn example_part1() {
        let points = parse_points(EXAMPLE).expect("parse example");
        let columns = build_columns(&points);
        assert_eq!(max_rectangle_area(&columns), 50);
    }

    #[test]
    fn single_row_rectangle_is_counted() {
        let pts = vec![(0, 5), (3, 5)];
        let cols = build_columns(&pts);
        // width 4 tiles (0..=3), height 1 tile (y=5)
        assert_eq!(max_rectangle_area(&cols), 4);
    }

    #[test]
    fn single_column_rectangle_is_counted() {
        let pts = vec![(2, 1), (2, 5)];
        let cols = build_columns(&pts);
        // width 1 tile (x=2), height 5 tiles (1..=5)
        assert_eq!(max_rectangle_area(&cols), 5);
    }

    #[test]
    fn example_part2() {
        let points = parse_points(EXAMPLE).expect("parse example");
        let (horiz, vert) = build_line_segments(&points);
        assert_eq!(max_rectangle_area_inside(&points, &horiz, &vert), 24);
    }
}
