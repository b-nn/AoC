use crate::REPEAT;
use itertools::Itertools;
use petgraph::csr::IndexType;
use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use petgraph::visit::NodeRef;
use std::fs;
use std::time::Instant;

fn bron_kerbosch(
    r: Vec<NodeIndex>,
    mut p: Vec<NodeIndex>,
    mut x: Vec<NodeIndex>,
    graph: &UnGraph<&str, &str>,
) -> Vec<Vec<NodeIndex>> {
    let t = p.len();
    let mut cliques = vec![];
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return cliques;
    }
    for index in (0..t).rev() {
        let vertex = p[index];
        let neighbors = graph.neighbors(vertex).collect::<Vec<_>>();
        let mut rtemp = r.clone();
        rtemp.push(vertex);
        let mut ptemp = p.clone();
        for i in (0..ptemp.len()).rev() {
            if !neighbors.contains(&ptemp[i]) {
                ptemp.remove(i);
            }
        }
        let mut xtemp = x.clone();
        for i in (0..xtemp.len()).rev() {
            if !neighbors.contains(&xtemp[i]) {
                xtemp.remove(i);
            }
        }
        cliques.append(&mut bron_kerbosch(rtemp, ptemp, xtemp, graph));
        x.push(vertex);
        p.remove(index);
    }

    cliques
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day23.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());
        let mut nodes = UnGraph::<&str, &str>::new_undirected();
        let mut n: Vec<&str> = vec![];
        for i in content.lines() {
            let mut t = i.split('-').into_iter();
            let temp = t.next().unwrap();
            let n1 = if let Some(idx) = nodes.raw_nodes().iter().position(|x| x.weight == temp) {
                NodeIndex::new(idx)
            } else {
                nodes.add_node(temp)
            };
            let temp = t.next().unwrap();
            let n2 = if let Some(idx) = nodes.raw_nodes().iter().position(|x| x.weight == temp) {
                NodeIndex::new(idx)
            } else {
                nodes.add_node(temp)
            };
            nodes.extend_with_edges(&[(n1, n2)]);
        }
        let t = nodes.node_indices();
        for i in nodes.neighbors(NodeIndex::new(0)) {
            println!("{:?}", nodes.node_weight(i));
        }
        let mut startswitht = vec![];
        for i in nodes.node_indices() {
            if nodes[i].chars().nth(0) == Some('t') {
                startswitht.push(i);
                println!("{} {:?}", nodes[i], i);
            }
        }
        let mut groupings = vec![];
        for i in startswitht {
            let neighbors = nodes.neighbors(i).collect::<Vec<_>>();
            for degree2 in nodes.neighbors(i) {
                for degree2neighbor in nodes.neighbors(degree2) {
                    if neighbors.contains(&degree2neighbor) {
                        let mut sort = vec![nodes[i], nodes[degree2], nodes[degree2neighbor]];
                        sort.sort();
                        if !groupings.contains(&sort) {
                            groupings.push(sort);
                        }
                    }
                }
            }
        }
        let cliques = bron_kerbosch(
            vec![],
            nodes.node_indices().collect::<Vec<_>>(),
            vec![],
            &nodes,
        );
        let mut pass = "".to_owned();
        let mut len = 0;
        for mut i in cliques {
            if i.len() > len {
                pass = "".to_owned();
                len = i.len();
                let mut weights = i.iter().map(|x| nodes[*x]).collect::<Vec<_>>();
                weights.sort();
                for j in weights {
                    pass.push_str(j);
                    pass.push(',');
                }
            }
        }
        println!("password is {}", pass);
        // for node in nodes.neighbors(NodeIndex::new(5)) {
        //     println!("neighbor {}", nodes[node]);
        // }
        // for i in &groupings {
        //     println!("group {:?}", i);
        // }
        println!("{}", groupings.len());
        println!("RAN");
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
