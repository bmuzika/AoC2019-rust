extern crate num;

use crate::num::Integer;
use std::fs::{File};
use std::io::{Read};

fn main() {
    // Read in map
    let file_contents = read_file("../day10/input2.txt");

    let map_lines = file_contents.clone().trim().split('\n').map(|s| s.to_string()).collect::<Vec<String>>();

    // Convert map into set of points.
    let mut y_counter = 0;
    let mut points = Vec::new();
    for line in map_lines {
        let mut x_counter = 0;
        for char in line.chars() {
            if char == '#' {
                points.push(Point{x: x_counter, y: y_counter, numGoodPairs: 0, eqns: Vec::new(), relPoints: Vec::new() });
            }
            x_counter += 1;
        }
        y_counter += 1;
    }

    // Iterate over all points. Find the number of visible asteroids for each point
    let points_to_check = points.clone();
    let mut output_points = Vec::new();
    for mut point in points {
        for point_to_check in points_to_check.clone() {
            point.add_eqn(point_to_check.x, point_to_check.y);
        }
        output_points.push(point);
    }

    let mut final_points = Vec::new();
    for mut point in output_points {
        point.dedup_eqns();
        final_points.push(point);
    }

    let mut final_points2 = Vec::new();
    for mut point in final_points {
        point.dedup_relpoints();
        final_points2.push(point);
    }
    // Report max number of visible points
    let mut max_points = 0;

    let mut best_point= Point{
        x: 0,
        y: 0,
        numGoodPairs: 0,
        eqns: vec![],
        relPoints: vec![]
    };

    for point in final_points2 {
        if point.relPoints.len() > max_points {
            max_points = point.relPoints.len();
            best_point = point.clone();
        }
    }
    print!("Max points = {}\n", max_points);
    print!("Best point is {:#?}", best_point);
}

fn read_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");

    file_contents
}

#[derive(Debug, PartialEq, Clone)]
struct RelPoints {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: i64,
    y: i64,
    numGoodPairs: i64,
    eqns: Vec::<EqnInfo>,
    relPoints: Vec<RelPoints>,
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
struct EqnInfo {
    slope: f64,
    intercept: f64,
    distance: f64,
    direction: f64,
}

impl Point {
    fn add_eqn(&mut self, x: i64, y: i64) {
        let f_x = x as f64;
        let f_y = y as f64;

        let slope: f64 = (f_y-self.y as f64)/(f_x-self.x as f64);
        let intercept: f64 = self.y as f64 - slope*self.x as f64;
        let distance: f64 = ((f_x - self.x as f64).powi(2) + (f_y - self.y as f64).powi(2)).sqrt();
        let direction: f64 = (f_y - self.y as f64)/distance;

        if (self.x == x && self.y == y) {
            return;
        } else if (slope.is_normal() && intercept.is_normal()) {
            self.eqns.push(EqnInfo{slope, intercept, distance: 0 as f64, direction});
            self.numGoodPairs += 1;
        } else if self.x == x {
            self.eqns.push(EqnInfo{slope: 10000000.0, intercept: 0.0, distance: 0 as f64, direction});
        }
        self.relPoints.push(RelPoints{x: x-self.x, y: y-self.y});
    }

    fn remove_eqn(&mut self, eqn: EqnInfo) {
        let index = self.eqns.iter().position(|e| *e == eqn);

        match index {
            Some(v) => self.eqns.remove(v),
            None => panic!("Ahh"),
        };
        self.numGoodPairs -= 1;
    }

    fn modify_eqn_distance(&mut self, eqn_to_modify: EqnInfo, new_distance: f64) {
        let new_eqn = EqnInfo{distance: new_distance, ..eqn_to_modify};

        self.remove_eqn(eqn_to_modify);
        self.eqns.push(new_eqn);
        self.numGoodPairs += 1;
    }

    fn distance_to_point(&self, x: i64, y: i64) -> f64 {
        let f_x = x as f64;
        let f_y = y as f64;
        let distance: f64 = ((f_x - self.x as f64).powi(2) + (f_y - self.y as f64).powi(2)).sqrt();

        distance
    }

    fn direction_to_point(&self, x:i64, y:i64) -> f64 {
        let f_x = x as f64;
        let f_y = y as f64;
        let distance: f64 = ((f_x - self.x as f64).powi(2) + (f_y - self.y as f64).powi(2)).sqrt();
        let direction: f64 = (f_y - self.y as f64)/distance;
        direction
    }

    fn dedup_eqns(&mut self) {
        self.eqns.sort_by(|a, b| a.slope.partial_cmp(&b.slope).unwrap());
        self.eqns.dedup();
    }

    fn dedup_relpoints(&mut self) {
        let mut new_vec = Vec::new();

        let mut x_flag = 0;
        let mut y_flag = 0;

        for rel_point in self.relPoints.clone() { // for every point in the list
            if rel_point.x.gcd(&rel_point.y) != 1 { // check if co-prime
                let check_list = self.relPoints.clone();
                let mut incomplete = 0;
                'inner: for point in check_list {
                    if (point.x == 0 && rel_point.x == 0) {
                        if x_flag != 0 {
                            new_vec.push(RelPoints{x: rel_point.x, y: rel_point.y});
                            x_flag = 1;
                            break 'inner;
                        }
                    }

                    if (point.y == 0 && rel_point.y == 0) {
                        if y_flag != 0 {
                            new_vec.push(RelPoints{x: rel_point.x, y: rel_point.y});
                            y_flag = 1;
                            break 'inner;
                        }
                    }

                    if point.x == 0 || point.y == 0 || rel_point.x == 0 || rel_point.y == 0 {
                        continue;
                    }

                    let is_x_a_mult = rel_point.x.is_multiple_of(&point.x);
                    let is_y_a_mult = rel_point.y.is_multiple_of(&point.y);
                // TODO: Fix a divide by zero error here. I don't know where it is coming from.
                    if (is_x_a_mult && is_y_a_mult) {
                        let div_x = rel_point.x/point.x;
                        let div_y = rel_point.y/point.y;

                        if div_x == div_y {
                            incomplete = 1;
                            break 'inner;
                        }
                    }
                }
                if incomplete == 0 {
                    new_vec.push(RelPoints{x: rel_point.x, y: rel_point.y});
                }
            } else {
                new_vec.push(RelPoints{x: rel_point.x, y: rel_point.y});
            }
        }
        self.relPoints = new_vec;
    }
}

impl EqnInfo {
    fn check(&self, x_coord: i64) -> f64 {
        let y = self.slope*x_coord as f64 + self.intercept;
        y
    }

    fn check_dist_dir(&mut self, direction: f64, distance: f64) -> EqnOptions {
        if direction == self.direction {
            if distance < self.distance {
               self.distance = distance;
            }
            return EqnOptions::Skip;
        }

        return EqnOptions::New;
    }
}


enum EqnOptions {
    Modify,
    Skip,
    New,
}