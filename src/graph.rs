use std::cell::UnsafeCell;
use std::collections::HashSet;
use typed_arena::Arena;

pub struct Node<'a> {
    pub datum: &'static str,
    pub edges: UnsafeCell<Vec<(&'a Node<'a>, i32)>>,
}

impl<'a> Node<'a> {
    pub fn new<'b>(datum: &'static str, arena: &'b Arena<Node<'b>>) -> &'b Node<'b> {
        arena.alloc(Node {
            datum: datum,
            edges: UnsafeCell::new(Vec::new()),
        })
    }

    pub fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
    where
        F: Fn(&'static str),
    {
        if seen.contains(&self.datum) {
            return;
        }
        f(self.datum);
        seen.insert(self.datum);
        unsafe {
            for n in &(*self.edges.get()) {
                n.0.traverse(f, seen);
            }
        }
    }

    pub fn first(&'a self) -> &'a Node<'a> {
        unsafe { (*self.edges.get())[0].0 }
    }

    pub fn children(&'a self) -> &'a Vec<(&'a Node<'a>, i32)> {
        unsafe { &(*self.edges.get()) }
    }
}

impl std::fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {}", self.datum)
    }
}

impl std::cmp::PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.datum == other.datum
    }
}
