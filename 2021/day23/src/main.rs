use std::collections::VecDeque;
use std::fmt::Display;

use petgraph::algo::all_simple_paths;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn move_cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    Hallway,
    Room(Amphipod),
}

#[derive(Debug, Clone, Copy)]
struct Place {
    kind: Kind,
    occupant: Option<Amphipod>,
}

impl Place {
    fn new(kind: Kind, occupant: Option<Amphipod>) -> Self {
        Self { kind, occupant }
    }

    fn free(&self) -> bool {
        self.occupant.is_none()
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Default)]
struct State {
    graph: Graph<Place, (), Undirected>,
    cost: usize,
}

impl State {
    fn room(&self, idx: NodeIndex) -> &Place {
        &self.graph[idx]
    }

    fn rooms(&self) -> impl Iterator<Item = &Place> {
        self.graph
            .node_weights()
            .filter(|node| matches!(node.kind, Kind::Room(_)))
    }

    fn free_hallway_nodes_idx(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph
            .node_indices()
            // FIXME don't hardcode these indexes
            .filter(|idx| ![2, 4, 6, 8].contains(&idx.index()))
            .filter(|idx| {
                let room = self.room(*idx);
                room.kind == Kind::Hallway && room.free()
            })
    }

    /// Returns an iterator over the rooms destined for given amphipod
    fn destination_rooms(
        &self,
        amphipod: Amphipod,
    ) -> impl DoubleEndedIterator<Item = NodeIndex> + '_ {
        self.graph
            .node_indices()
            .filter(move |idx| self.room(*idx).kind == Kind::Room(amphipod))
    }

    fn is_finished(&self) -> bool {
        self.rooms().all(|room| {
            if let Some(occupant) = room.occupant {
                room.kind == Kind::Room(occupant)
            } else {
                // this room is free
                false
            }
        })
    }

    fn finished_moving(&self, idx: NodeIndex) -> bool {
        let room = self.room(idx);
        let this_room_occupant = room.occupant.unwrap();
        match room.kind {
            Kind::Hallway => false,
            Kind::Room(room_kind) => {
                if this_room_occupant == room_kind {
                    self.destination_rooms(room_kind).all(|idx| {
                        let room = self.room(idx);
                        if let Some(occupant) = room.occupant {
                            occupant == room_kind
                        } else {
                            true
                        }
                    })
                } else {
                    false
                }
            }
        }
    }

    // Returns an iterator over all nodes that could be moved
    fn movable(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.graph
            .node_indices()
            .filter(|idx| !self.room(*idx).free())
            .filter(|idx| !self.finished_moving(*idx))
    }

    fn can_traverse(&self, path: &[NodeIndex]) -> bool {
        if let (Some(from), Some(to)) = (path.first(), path.last()) {
            // FIXME don't hardcode these indexes
            if [2, 4, 6, 8].contains(&to.index()) {
                // Amphipod cannot stop at 'crossroads'
                return false;
            }

            let amphipod = self.room(*from).occupant.unwrap();
            if matches!(self.room(*from).kind, Kind::Hallway) {
                // From hallway you can go to your room only
                if self.room(*to).kind != Kind::Room(amphipod) {
                    return false;
                }
            }
            if let Kind::Room(kind) = self.room(*to).kind {
                // Amphipod can only enter its room.
                if kind != amphipod
                    || !self.destination_rooms(amphipod).all(|idx| {
                        let room = self.room(idx);
                        if let Some(occupant) = room.occupant {
                            occupant == amphipod
                        } else {
                            true
                        }
                    })
                {
                    return false;
                }
            }
        }
        path.iter()
            .skip(1)
            .all(|idx| self.graph[*idx].occupant.is_none())
    }

    fn path_to(&self, from: NodeIndex, to: NodeIndex) -> Option<Vec<NodeIndex>> {
        let mut paths = all_simple_paths::<Vec<_>, _>(&self.graph, from, to, 1, None);
        if let Some(path) = paths.next() {
            if self.can_traverse(&path) {
                return Some(path);
            }
        }

        None
    }

    fn try_enter_own_room(&self, from: NodeIndex) -> Option<Vec<NodeIndex>> {
        if let Some(kind) = self.graph[from].occupant {
            for idx in self.destination_rooms(kind).rev() {
                if let Some(occupant) = self.room(idx).occupant {
                    // occupied
                    if occupant != kind {
                        return None;
                    }
                } else {
                    // free
                    if let Some(path) = self.path_to(from, idx) {
                        return Some(path);
                    }
                }
            }
        }

        None
    }

    fn traverse(&mut self, path: Vec<NodeIndex>) {
        if let (Some(from), Some(to)) = (path.first(), path.last()) {
            let amphipod = self.room(*from).occupant.unwrap();
            let cost = (path.len() - 1) * amphipod.move_cost();
            self.go(*from, *to, cost);
        }
    }

    fn go(&mut self, from: NodeIndex, to: NodeIndex, cost: usize) {
        assert!(self.room(to).free());
        self.graph[to].occupant = self.graph[from].occupant;
        self.graph[from].occupant = None;
        self.cost += cost;
    }
}

fn solve(graph: Graph<Place, (), Undirected>) -> usize {
    let mut queue = VecDeque::<State>::from([State { graph, cost: 0 }]);
    let mut min_cost = usize::MAX;

    'outer: while let Some(state) = queue.pop_front() {
        if state.is_finished() {
            if state.cost < min_cost {
                println!("New min cost: {}", state.cost);
                min_cost = state.cost;
            }
            continue;
        }

        for idx in state.movable() {
            if let Some(path) = state.try_enter_own_room(idx) {
                let mut new_state = state.clone();
                new_state.traverse(path);
                if new_state.cost < min_cost {
                    queue.push_front(new_state);
                }
                continue 'outer;
            } else {
                if state.room(idx).kind == Kind::Hallway {
                    continue;
                }
                for target_idx in state.free_hallway_nodes_idx() {
                    if let Some(path) = state.path_to(idx, target_idx) {
                        let mut new_state = state.clone();
                        new_state.traverse(path);
                        if new_state.cost < min_cost {
                            queue.push_front(new_state);
                        }
                    }
                }
            }
        }
    }
    min_cost
}

fn build_hallway(graph: &mut Graph<Place, (), Undirected>) -> Vec<NodeIndex> {
    let hallway = (0..=10)
        .map(|_| graph.add_node(Place::new(Kind::Hallway, None)))
        .collect::<Vec<_>>();

    for pair in hallway.as_slice().windows(2) {
        if let [l, r] = pair {
            graph.add_edge(*l, *r, ());
        }
    }

    hallway
}

fn graph_for_part1() -> Graph<Place, (), Undirected> {
    let mut graph = Graph::<Place, (), Undirected>::new_undirected();
    let hallway = build_hallway(&mut graph);

    let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::D)));
    let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::B)));
    graph.add_edge(hallway[2], room0, ());
    graph.add_edge(room0, room1, ());

    let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::A)));
    let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::C)));
    graph.add_edge(hallway[4], room0, ());
    graph.add_edge(room0, room1, ());

    let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::C), Some(Amphipod::C)));
    let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::C), Some(Amphipod::B)));
    graph.add_edge(hallway[6], room0, ());
    graph.add_edge(room0, room1, ());

    let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::D), Some(Amphipod::D)));
    let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::D), Some(Amphipod::A)));
    graph.add_edge(hallway[8], room0, ());
    graph.add_edge(room0, room1, ());

    graph
}

fn graph_for_part2() -> Graph<Place, (), Undirected> {
    let mut graph = Graph::<Place, (), Undirected>::new_undirected();
    let hallway = build_hallway(&mut graph);

    let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::D)));
    let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::D)));
    let room2 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::D)));
    let room3 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::B)));
    graph.add_edge(hallway[2], room0, ());
    graph.add_edge(room0, room1, ());
    graph.add_edge(room1, room2, ());
    graph.add_edge(room2, room3, ());

    let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::A)));
    let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::C)));
    let room2 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::B)));
    let room3 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::C)));
    graph.add_edge(hallway[4], room0, ());
    graph.add_edge(room0, room1, ());
    graph.add_edge(room1, room2, ());
    graph.add_edge(room2, room3, ());

    let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::C), Some(Amphipod::C)));
    let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::C), Some(Amphipod::B)));
    let room2 = graph.add_node(Place::new(Kind::Room(Amphipod::C), Some(Amphipod::A)));
    let room3 = graph.add_node(Place::new(Kind::Room(Amphipod::C), Some(Amphipod::B)));
    graph.add_edge(hallway[6], room0, ());
    graph.add_edge(room0, room1, ());
    graph.add_edge(room1, room2, ());
    graph.add_edge(room2, room3, ());

    let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::D), Some(Amphipod::D)));
    let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::D), Some(Amphipod::A)));
    let room2 = graph.add_node(Place::new(Kind::Room(Amphipod::D), Some(Amphipod::C)));
    let room3 = graph.add_node(Place::new(Kind::Room(Amphipod::D), Some(Amphipod::A)));
    graph.add_edge(hallway[8], room0, ());
    graph.add_edge(room0, room1, ());
    graph.add_edge(room1, room2, ());
    graph.add_edge(room2, room3, ());

    graph
}

fn main() {
    let graph = graph_for_part1();
    let cost = solve(graph);
    println!("PART1: Minimum cost: {}", cost);

    let graph = graph_for_part2();
    let cost = solve(graph);
    println!("PART2: Minimum cost: {}", cost);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_entering_own_room() {
        let mut state = State {
            graph: graph_for_part1(),
            ..State::default()
        };

        assert_eq!(None, state.try_enter_own_room(NodeIndex::new(11)));

        state.go(NodeIndex::new(17), NodeIndex::new(9), 0);
        assert_eq!(None, state.try_enter_own_room(NodeIndex::new(10)));

        state.go(NodeIndex::new(18), NodeIndex::new(1), 0);
        let path = state.try_enter_own_room(NodeIndex::new(11));
        assert!(path.is_some());
        assert_eq!(Some(&NodeIndex::new(18)), path.as_ref().unwrap().last());
        state.traverse(path.unwrap());

        let path = state.try_enter_own_room(NodeIndex::new(9));
        assert!(path.is_some());
        assert_eq!(Some(&NodeIndex::new(17)), path.as_ref().unwrap().last());
    }

    #[test]
    fn test_finished() {
        let mut graph = Graph::<Place, (), Undirected>::new_undirected();
        let hallway = build_hallway(&mut graph);

        let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::A)));
        let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::A)));
        graph.add_edge(hallway[2], room0, ());
        graph.add_edge(room0, room1, ());

        let room0 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::B)));
        let room1 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::B)));
        graph.add_edge(hallway[4], room0, ());
        graph.add_edge(room0, room1, ());

        let state = State { graph, cost: 0 };

        assert!(state.is_finished());
    }

    #[test]
    fn test_finished_moving() {
        let mut graph = Graph::<Place, (), Undirected>::new_undirected();
        build_hallway(&mut graph);

        graph.add_node(Place::new(Kind::Room(Amphipod::A), None));
        let rooma1 = graph.add_node(Place::new(Kind::Room(Amphipod::A), Some(Amphipod::A)));

        let roomb0 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::B)));
        let roomb1 = graph.add_node(Place::new(Kind::Room(Amphipod::B), Some(Amphipod::B)));

        let roomc0 = graph.add_node(Place::new(Kind::Room(Amphipod::C), Some(Amphipod::B)));
        let roomc1 = graph.add_node(Place::new(Kind::Room(Amphipod::C), Some(Amphipod::C)));

        graph.add_node(Place::new(Kind::Room(Amphipod::D), None));
        let roomd1 = graph.add_node(Place::new(Kind::Room(Amphipod::D), Some(Amphipod::C)));

        let state = State { graph, cost: 0 };

        assert!(state.finished_moving(rooma1));
        assert!(state.finished_moving(roomb0));
        assert!(state.finished_moving(roomb1));
        assert!(!state.finished_moving(roomc0));
        assert!(!state.finished_moving(roomc1));
        assert!(!state.finished_moving(roomd1));
        assert!(!state.is_finished());
    }
}
