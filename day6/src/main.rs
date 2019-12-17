use std::fs::{File, read_to_string};
use std::io::{Read, Write};
use std::io;
use trees::{tr, fr};
use treeline::Tree;
use indextree::*;

use petgraph::graph::{Graph, UnGraph};
use petgraph::visit::{IntoNodeIdentifiers, IntoEdgeReferences, NodeCompactIndexable, EdgeRef, IntoNodeReferences, Dfs, Bfs};
use petgraph::dot::{Dot, Config};
use petgraph::Direction::Incoming;
use petgraph::algo::astar;


fn main() {
    let mut file = match File::open("input.txt") {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");

    let object_pairs = file_contents.trim().split("\n").map(|s| s.to_string()).collect::<Vec<String>>();

    let objects = object_pairs.into_iter().map(|s| s.as_str().split(")").map(|internalString| internalString.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

    let orbiting_objects = objects.into_iter().map(|sp| OrbitingObject{parent:sp[0].clone(), object: sp[1].clone(), weight: 0}).collect::<Vec<OrbitingObject>>();

    let mut g = UnGraph::<OrbitingObject, f32>::new_undirected();
    let root_node = g.add_node(OrbitingObject{parent: "".to_string(), object: "COM".to_string(), weight: 0});

    for object in orbiting_objects {
        let v0 = g.add_node(object.clone());
        for (node_idx, node)  in g.clone().node_references() {
            if object.parent == node.object {
                let new_edge = g.add_edge(node_idx, v0, 1.0);
                g[v0].weight = node.weight + 1;
            }
        }
    }

    for (second_pass_idx, second_pass_node) in g.clone().node_references() {
        for (node_idx, node) in g.clone().node_references() {
            if second_pass_node.parent == node.object {
                if g.find_edge(node_idx, second_pass_idx) == None {
                    g.add_edge(node_idx, second_pass_idx, 1.0);
                }
            }
        }

    }
    let mut level = 0;
    for (node, _) in g.node_references() {
        let mut dfs = Dfs::new(&g, node);

        while let Some(node) = dfs.next(&g) {
            if g[node].parent == "".to_string() {
                break;
            }
            level += 1;

        }
    }

    let mut santa_distance: f32 = 0.0;
    for (node_idx, node) in g.node_references() {
        if node.object != "YOU".to_string() {
            continue;
        }

        for (sn, sno) in g.node_references() {
            if sno.object == "SAN".to_string() {
                let (cost, _) = astar(
                   &g,
                    node_idx,
                    |n| n == sn,
                    |e| *e.weight(),
                    |_|0.0,
                ).unwrap();

                santa_distance = cost;
                break;
            }
        }
    }

    println!("Edges = {}", level);
    println!("Santa Distance = {}", santa_distance);

    //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    let mut f = File::create("example1.dot").unwrap();
    let output = format!("{}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    f.write_all(&output.as_bytes());
    //println!("{}", g);
}

#[derive(Debug, Clone)]
struct OrbitingObject {
    object: String,
    parent: String,
    weight: u32,
}

impl std::fmt::Display for OrbitingObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.object)
    }
}

/*
impl std::cmp::PartialEq for OrbitingObject {
    fn eq(&self, other: &Self) -> bool {
        self.parent
    }
} */