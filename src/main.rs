mod graph;
mod levenshtein;
mod dijkstra;

use graph::Node;
use std::collections::HashSet;
use typed_arena::Arena;

fn init<'a>(arena: &'a Arena<Node<'a>>) -> &'a Node<'a> {
    let root = Node::new("A", arena);

    let b = Node::new("B", arena);
    let c = Node::new("C", arena);
    let d = Node::new("D", arena);
    let e = Node::new("E", arena);
    let f = Node::new("F", arena);
    let g = Node::new("G", arena);

    unsafe {
        (*root.edges.get()).push((b, 4));
        (*root.edges.get()).push((c, 7));
        (*root.edges.get()).push((d, 2));

        (*b.edges.get()).push((e, 2));

        (*c.edges.get()).push((e, 2));
        (*c.edges.get()).push((f, 3));
        (*c.edges.get()).push((root, 1));

        (*d.edges.get()).push((f, 3));
        (*e.edges.get()).push((g, 2));
        (*f.edges.get()).push((g, 3));
        (*f.edges.get()).push((c, 1));
    }

    root
}

fn foo<'a>(node: &'a Node<'a>) {
    println!("foo: {}", node.datum);
}


fn main() {
    let arena = Arena::new();
    let g = init(&arena);
    g.traverse(&|d| println!("{}", d), &mut HashSet::new());
    foo(g.first());

    for (node, _) in g.children() {
        println!("{}", node);
    }

    let target = "C";
    if dijkstra::dijkstra(g, target).is_err() {
        println!("Target {} could not be found.", target)
    }

    let str1 = "test";
    let str2 = "fest";
    println!(
        "Levenshtein distance between {} and {}: {}",
        str1,
        str2,
        levenshtein::levenshtein(str1, str2)
    );
}
