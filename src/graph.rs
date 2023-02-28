use bevy::{math, prelude::*, ui::node};
use petgraph::{algo::astar::astar, data::Build, graph, graphmap::GraphMap, Undirected};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Resource)]
pub struct TrackGraph {
    graph: GraphMap<Node, (), Undirected>,
    next_id: u32,
}

impl TrackGraph {
    pub fn new() -> Self {
        let mut graph = GraphMap::<Node, (), Undirected>::new();
        TrackGraph {
            graph: graph,
            next_id: 0,
        }
    }

    fn get_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id + 1;
        id
    }

    pub fn add_unconnected_track(&mut self, pos1: Vec2, pos2: Vec2) -> (Node, Node) {
        let n1 = Node {
            id: self.get_id(),
            position: pos1,
        };
        let n2 = Node {
            id: self.get_id(),
            position: pos2,
        };

        self.graph.add_node(n1);
        self.graph.add_node(n2);
        self.add_track(n1, n2);

        (n1, n2)
    }

    pub fn add_track(&mut self, a: Node, b: Node) {
        self.graph.add_edge(a, b, ());
    }

    pub fn extend_track(&mut self, node: Node, pos: Vec2) -> Node {
        let end = Node {
            id: self.get_id(),
            position: pos,
        };
        let track = Track;

        self.graph.add_node(end);
        self.graph.add_edge(node, end, ());

        end
    }

    pub fn split_track(&mut self, a: Node, b: Node, pos: Vec2) -> Node {
        self.graph.remove_edge(a, b);

        let node = Node {
            id: self.get_id(),
            position: pos,
        };
        self.graph.add_node(node);
        self.graph.add_edge(a, node, ());
        self.graph.add_edge(b, node, ());

        node
    }

    pub fn remove_track(&mut self, a: Node, b: Node) {
        self.graph.remove_edge(a, b);
        for node in [a, b] {
            if self.graph.edges(node).next().is_none() {
                self.graph.remove_node(node);
            }
        }
    }

    pub fn get_path(&self, from: Node, to: Node) -> Option<(f32, Vec<Node>)> {
        astar(
            &self.graph,
            from,
            |finish| finish == to,
            |bundle| bundle.0.position.distance(bundle.1.position),
            |node| node.position.distance(to.position),
        )
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq)]
pub struct Node {
    id: u32,
    position: Vec2,
}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.id as u8)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Component)]
pub struct Track;

#[test]
fn test_graph() {
    let mut track_graph = TrackGraph::new();

    let pos1 = Vec2 { x: 0.0, y: 0.0 };
    let pos2 = Vec2 { x: 0.0, y: 10.0 };
    let pos3 = Vec2 { x: 0.0, y: 15.0 };
    let pos4 = Vec2 { x: 10.0, y: 15.0 };
    let (n1, n2) = track_graph.add_unconnected_track(pos1, pos2);
    assert_eq!(n1.position, pos1);
    assert_eq!(n2.position, pos2);

    let n3 = track_graph.extend_track(n2, pos3);
    assert_eq!(n3.position, pos3);
    let n4 = track_graph.extend_track(n3, pos4);
    assert_eq!(n4.position, pos4);

    let path = track_graph.get_path(n1, n4);
    assert_eq!(path, Some((25.0, vec![n1, n2, n3, n4])));

    track_graph.add_track(n2, n4);
    let path = track_graph.get_path(n1, n4);
    assert_eq!(path, Some((10.0 + pos2.distance(pos4), vec![n1, n2, n4])));

    track_graph.remove_track(n1, n2);
}
