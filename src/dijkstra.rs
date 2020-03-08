use crate::graph::Node;

pub enum DijkstraError {
    TargetNotFound,
}

fn min_node_index<'a>(nodes: &Vec<NodeInfo<'a>>) -> Result<usize, DijkstraError> {
    let mut min_index = 0;
    let mut min_weight = i32::max_value();
    for (i, n) in nodes.iter().enumerate() {
        if n.weight < min_weight {
            min_weight = n.weight;
            min_index = i;
        }
    }

    Ok(min_index)
}

#[derive(Clone)]
struct NodeInfo<'a> {
    pub node: &'a Node<'a>,
    pub predecessor: Option<&'a Node<'a>>,
    pub weight: i32,
}

pub fn dijkstra<'a>(start: &'a Node<'a>, goal_label: &'static str) -> Result<(), DijkstraError> {
    let mut to_visit = vec![NodeInfo {
        node: start,
        predecessor: None,
        weight: 0,
    }];
    let mut chosen: Vec<NodeInfo<'a>> = Vec::new();

    while !to_visit.is_empty() {
        // get node with lowest weight from nodes to visit
        let cur_node_index = min_node_index(&to_visit)?;
        let cur_node_info = to_visit[cur_node_index].clone();
        let cur_node = cur_node_info.node;

        println!("CurNode: {}\nChosen:", cur_node);
        for node_info in &chosen {
            println!("{} with weight {}", node_info.node, node_info.weight);
        }

        to_visit.remove(cur_node_index);

        // add all children of the current node that were not chosen already
        let children = cur_node.children();
        let raw_nodes: Vec<&'a Node<'a>> = chosen.iter().map(|x| x.node).collect();
        for (n, w) in children {
            if !raw_nodes.contains(n) && *n != cur_node {
                to_visit.push(NodeInfo {
                    node: n,
                    predecessor: Some(cur_node),
                    weight: cur_node_info.weight + w,
                });
            }
        }

        chosen.push(cur_node_info);

        if cur_node.datum == goal_label {
            break;
        }
    }

    let target_nodes: Vec<&NodeInfo<'a>> = chosen
        .iter()
        .filter(|x| x.node.datum == goal_label)
        .collect::<Vec<_>>();

    if target_nodes.len() > 1 || target_nodes.is_empty() {
        return Err(DijkstraError::TargetNotFound);
    }

    let mut target_node = target_nodes[0];

    let mut target_route = Vec::new();

    while target_node.node != start {
        target_route.push((target_node.node, target_node.weight));

        target_node = chosen
            .iter()
            .filter(|x| x.node == target_node.predecessor.unwrap())
            .collect::<Vec<_>>()[0];
    }

    println!("Target Route:");
    for t in target_route {
        println!("{} with weight of {}", t.0, t.1);
    }

    Ok(())
}