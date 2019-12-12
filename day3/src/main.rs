use std::ops;
use std::cmp;
use std::fs::{File, read_to_string};
use std::io::Read;

fn main() {
    let mut file = match File::open("input.txt") {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");

    let strings = file_contents.trim().split("\n").collect::<Vec<_>>();


    let (distance, fastest) = find_min_crossing_distance(strings[0], strings[1]);
    println!("Minimum distance is {}, Fastest intersection is {} steps", distance, fastest);

}

fn find_fastest_crossing_distance(wire1: &str, wire2: &str) -> i64 {
    // split each string into Vec<String>, extract each element into a {direction, distance} pair
    let wire1_vec = wire1.split(",").map(|s| s.to_string()).map(|s| parse_to_wiremove(s)).collect::<Vec<WireMove>>();
    let wire2_vec = wire2.split(",").map(|s| s.to_string()).map(|s| parse_to_wiremove(s)).collect::<Vec<WireMove>>();

    // for each move in the list, iterate over every point from start to end of the move, add to a list of points crossed
    let mut current_point = Point{x:0, y:0};
    let mut wire1_points_hit: Vec<Point> = Vec::new();
    let mut wire2_points_hit: Vec<Point> = Vec::new();

    for wire_move in wire1_vec {
        let (new_points, new_position) = find_points_hit(wire_move, current_point);
        wire1_points_hit.extend(new_points);
        current_point = new_position;
    }
    wire1_points_hit.push(current_point);

    current_point = Point{x: 0, y: 0};

    for wire_move in wire2_vec {
        let (new_points, new_position) = find_points_hit(wire_move, current_point);
        wire2_points_hit.extend(new_points);
        current_point = new_position;
    }
    wire2_points_hit.push(current_point);
    println!("Hello!");

    // search for all intersections between the two lists
    let mut wire1a = 0;
    let mut wire1b = 0;
    let mut wire2a = 0;
    let mut wire2b = 0;
    let zero_point = Point{x: 0, y:0};
    'outera: for wire1_point in wire1_points_hit.clone() {
        if wire1_point == zero_point {
            continue;
        }

        wire1a += 1;
        wire2a = 0;
        for wire2_point in &wire2_points_hit {
            if *wire2_point == zero_point {
                continue;
            }
            wire2a += 1;
            if wire1_point == *wire2_point {
                break 'outera;
            }
        }
    }

    'outerb: for wire2_point in wire2_points_hit {
        if wire2_point == zero_point {
            continue;
        }
        wire2b += 1;
        wire1b = 0;
        for wire1_point in &wire1_points_hit {
            if *wire1_point == zero_point {
                continue;
            }
            wire1b += 1;
            if wire2_point == *wire1_point {
                break 'outerb;
            }
        }
    }

    let steps_a = wire1a + wire2a;
    let steps_b = wire1b + wire2b;
    let distance = steps_a.min(steps_b);

    distance
}

fn find_min_crossing_distance(wire1: &str, wire2: &str) -> (i64, i64) {
    // split each string into Vec<String>, extract each element into a {direction, distance} pair
    let wire1_vec = wire1.split(",").map(|s| s.to_string()).map(|s| parse_to_wiremove(s)).collect::<Vec<WireMove>>();
    let wire2_vec = wire2.split(",").map(|s| s.to_string()).map(|s| parse_to_wiremove(s)).collect::<Vec<WireMove>>();

    // for each move in the list, iterate over every point from start to end of the move, add to a list of points crossed
    let mut current_point = Point{x:0, y:0};
    let mut wire1_points_hit: Vec<Point> = Vec::new();
    let mut wire2_points_hit: Vec<Point> = Vec::new();

    for wire_move in wire1_vec {
        let (new_points, new_position) = find_points_hit(wire_move, current_point);
        wire1_points_hit.extend(new_points);
        current_point = new_position;
    }
    wire1_points_hit.push(current_point);

    current_point = Point{x: 0, y: 0};

    for wire_move in wire2_vec {
        let (new_points, new_position) = find_points_hit(wire_move, current_point);
        wire2_points_hit.extend(new_points);
        current_point = new_position;
    }
    wire2_points_hit.push(current_point);
    println!("Hello!");

    let mut wire1_len = 0;
    let mut wire2_len = 0;
    // search for all intersections between the two lists
    let mut intersections: Vec<(i64, Point)> = Vec::new();

    let zero_point = Point{x: 0, y:0};
    for wire1_point in wire1_points_hit {
        if wire1_point == zero_point {
            continue;
        }
        wire1_len += 1;
        wire2_len = 0;
        for wire2_point in &wire2_points_hit {
            if *wire2_point == zero_point {
                continue;
            }
            wire2_len += 1;
            if wire1_point == *wire2_point {
                intersections.push((wire1_len+wire2_len, Point{ x: wire1_point.x, y: wire1_point.y }));
            }
        }
    }

    // compute manhattan distance as abs(x)+abs(y)
    let distance = intersections.clone().into_iter().map(|p| p.1.x.abs() + p.1.y.abs()).fold(i64::max_value(), |acc, d| i64::min(acc, d));
    let fastest = intersections.clone().into_iter().map(|t| t.0).fold(i64::max_value(), |acc, d| i64::min(acc, d));
    (distance, fastest)
}

fn find_points_hit(wire_move: WireMove, current_point: Point) -> (Vec<Point>, Point) {
    let mut points_hit= Vec::new();

    let new_point = match wire_move.direction {
        Direction::Up => {
            for coord in 0..wire_move.distance {
                points_hit.push(Point { x: current_point.x, y: (current_point.y + coord) });
            }

            current_point + Point{x: 0, y: wire_move.distance }
        },
        Direction::Down => {
            for coord in 0..wire_move.distance {
                points_hit.push(Point { x: current_point.x, y: (current_point.y - coord) });
            }

            current_point - Point{x: 0, y: wire_move.distance }
        },
        Direction::Right => {
            for coord in 0..wire_move.distance {
                points_hit.push(Point { x: (current_point.x + coord), y: current_point.y });
            }

            current_point + Point{x: wire_move.distance, y: 0 }
        },
        Direction::Left => {
            for coord in 0..wire_move.distance {
                points_hit.push(Point { x: (current_point.x - coord), y: current_point.y });
            }

            current_point - Point{x: wire_move.distance, y: 0 }
        },
    };

    (points_hit, new_point)
}

fn parse_to_wiremove(string: String) -> WireMove {
    let mut wire_chars = string.chars();
    let direction_to_move = match wire_chars.next() {
        Some('U') => Direction::Up,
        Some('D') => Direction::Down,
        Some('R') => Direction::Right,
        Some('L') => Direction::Left,
        Some(_) => panic!(),
        None => panic!(),
    };
    let distance_to_move = wire_chars.collect::<String>().parse::<i64>().unwrap();

    WireMove{direction: direction_to_move, distance: distance_to_move}
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct WireMove {
    direction: Direction,
    distance: i64,
}

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        let out = Point { x: (self.x + rhs.x), y: (self.y + rhs.y) };
        out
    }
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        if (self.x == other.x && self.y == other.y) {
            true
        }
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_tests() {
        assert_eq!(find_min_crossing_distance("R8,U5,L5,D3", "U7,R6,D4,L4"), (6, 30));
        assert_eq!(find_min_crossing_distance("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"), (159, 610));
        assert_eq!(find_min_crossing_distance("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), (135,410));
    }

    /*#[test]
    fn speed_tests() {
        assert_eq!(find_min_crossing_distance("R8,U5,L5,D3", "U7,R6,D4,L4"), 30);
        assert_eq!(find_min_crossing_distance("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"), 610);
        assert_eq!(find_min_crossing_distance("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
    }*/
}
