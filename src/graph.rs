use bevy::{
    math,
    prelude::{system_adapter::new, *},
    render::render_graph::Edge,
    ui::node,
    utils::HashMap,
};
use petgraph::{
    algo::astar::astar,
    data::Build,
    graph,
    graphmap::{AllEdges, GraphMap, Nodes},
    stable_graph::{
        EdgeIndex, EdgeIndices, EdgeReference, NodeIndex, NodeIndices, StableGraph, StableUnGraph,
    },
    visit::IntoEdgeReferences,
    Graph, Undirected,
};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Component)]
pub struct TrackNode {
    pub id: NodeIndex,
}

#[derive(Component)]
pub struct TrackEdge {
    pub id: EdgeIndex,
}

pub struct TrackWeight {
    pub length: f32,
}

#[derive(Resource)]
pub struct TrackGraph {
    graph: StableGraph<(), TrackWeight>,
}
impl TrackGraph {}
// pub fn new() -> Self {
//     let mut graph = StableGraph::<(), TrackWeight>::new();
//     TrackGraph { graph: graph }
// }

// pub fn get_all_nodes(&self) -> NodeIndices<()> {
//     self.graph.node_indices()
// }
// pub fn get_all_edges(&self) -> EdgeIndices<TrackWeight> {
//     self.graph.edge_indices()
// }
// pub fn contains_node(&self, node: NodeIndex) -> bool {
//     self.graph.contains_node(node)
// }
// pub fn contains_edge(&self, a: NodeIndex, b: NodeIndex) -> bool {
//     self.graph.contains_edge(a, b)
// }

// pub fn add_node(&mut self) -> NodeIndex {
//     self.graph.add_node(())
// }

// pub fn add_unconnected_track(
//     &mut self,
//     track_weight: TrackWeight,
// ) -> (NodeIndex, NodeIndex, EdgeIndex) {
//     let a = self.graph.add_node(());
//     let b = self.graph.add_node(());
//     (a, b, self.add_track(a, b, track_weight))
// }

// pub fn add_track(
//     &mut self,
//     a: NodeIndex,
//     b: NodeIndex,
//     track_weight: TrackWeight,
// ) -> EdgeIndex {
//     self.graph.add_edge(a, b, track_weight)
// }

// pub fn extend_track(
//     &mut self,
//     node: NodeIndex,
//     track_weight: TrackWeight,
// ) -> (NodeIndex, EdgeIndex) {
//     let end = self.graph.add_node(());
//     (end, self.graph.add_edge(node, end, track_weight))
// }

// pub fn split_track(
//     &mut self,
//     a: NodeIndex,
//     b: NodeIndex,
//     e: EdgeIndex,
//     weight_left: TrackWeight,
//     weight_right: TrackWeight,
// ) -> (NodeIndex, EdgeIndex, EdgeIndex) {
//     self.graph.remove_edge(e);

//     let c = self.graph.add_node(());
//     let f = self.graph.add_edge(a, c, weight_left);
//     let g = self.graph.add_edge(b, c, weight_right);

//     (c, f, g)
// }

// pub fn remove_track(&mut self, a: NodeIndex, b: NodeIndex, e: EdgeIndex) {
//     self.graph.remove_edge(e);
//     for node in [a, b] {
//         if self.graph.edges(node).next().is_none() {
//             self.graph.remove_node(node);
//         }
//     }
// }

// pub fn get_path(
//     &self,
//     from: NodeIndex,
//     to: NodeIndex,
//     velocity: f32,
// ) -> Option<(f32, Vec<NodeIndex>)> {
//     astar(
//         &self.graph,
//         from,
//         |finish| finish == to,
//         |(edge_ref)| edge_ref.weight().length * velocity,
//         |node| 0.,
//     )
// }

// #[test]
// fn test_graph() {
//     let mut track_graph = TrackGraph::new();

//     let pos1 = Vec2 { x: 0.0, y: 0.0 };
//     let pos2 = Vec2 { x: 0.0, y: 10.0 };
//     let pos3 = Vec2 { x: 0.0, y: 15.0 };
//     let pos4 = Vec2 { x: 10.0, y: 15.0 };
//     let (n1, n2) = track_graph.add_unconnected_track(pos1, pos2);
//     assert_eq!(n1.position, pos1);
//     assert_eq!(n2.position, pos2);

//     let n3 = track_graph.extend_track(n2, pos3);
//     assert_eq!(n3.position, pos3);
//     let n4 = track_graph.extend_track(n3, pos4);
//     assert_eq!(n4.position, pos4);

//     let path = track_graph.get_path(n1, n4);
//     assert_eq!(path, Some((25.0, vec![n1, n2, n3, n4])));

//     track_graph.add_track(n2, n4);
//     let path = track_graph.get_path(n1, n4);
//     assert_eq!(path, Some((10.0 + pos2.distance(pos4), vec![n1, n2, n4])));

//     track_graph.remove_track(n1, n2);
// }
