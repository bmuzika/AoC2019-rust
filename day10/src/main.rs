use std::fs::{File};
use std::io::{Read};

fn main() {
    // Read in map
    let mut file_contents = read_file("../day10/input.txt");

    let mut map_lines = file_contents.clone().trim().split('\n').map(|s| s.to_string()).collect::<Vec<String>>();

    // Convert map into set of points.
    let mut y_counter = 0;
    let mut points = Vec::new();
    for line in map_lines {
        let mut x_counter = 0;
        for char in line.chars() {
            if char == '#' {
                points.push(Point{x: x_counter, y: y_counter, numGoodPairs: 0, eqns: Vec::new() });
            }
            x_counter += 1;
        }
        y_counter += 1;
    }

    // Iterate over all points. Find the number of visible asteroids for each point
    let points_to_check = points.clone();
    for mut point in points {
        for point_to_check in points_to_check.clone() {
            'eqn_check:for  eqn in (&point).eqns {
                if point_to_check.y as f64 == eqn.check(point_to_check.x) {
                    let distance = point.distance_to_point(point_to_check.x, point_to_check.y);
                    let direction = point.direction_to_point(point_to_check.x, point_to_check.y);
                    let eqn_result = eqn.check_dist_dir(distance, direction);

                    match eqn_result {
                        EqnOptions::New => point.add_eqn(point_to_check.x, point_to_check.y),
                        EqnOptions::Skip => (),
                        EqnOptions::Modify => panic!("Shouldn't use this"),
                    }
                    break 'eqn_check;
                }
            }
        }
    }
    // Report max number of visible points
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
struct Point {
    x: i64,
    y: i64,
    numGoodPairs: i64,
    eqns: Vec::<EqnInfo>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

        self.eqns.push(EqnInfo{slope, intercept, distance, direction});
        self.numGoodPairs += 1;
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