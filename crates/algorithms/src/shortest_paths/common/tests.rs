use alloc::vec::Vec;
use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

use error_stack::Result;
use hashbrown::HashMap;
use petgraph_core::{Graph, GraphStorage, Node};
use petgraph_dino::NodeId;

use crate::shortest_paths::{DirectRoute, Route, ShortestDistance, ShortestPath};

/// Helper Macro to create a map of expected results
///
/// Technically this macro is not necessarily needed, but it makes the test code much more
/// readable
macro_rules! expected {
    (@rule $nodes:ident; $source:ident; ($($path:ident),*) ;$target:ident; $cost:literal) => {
        Expect {
            source: $nodes.$source,
            target: $nodes.$target,
            transit: alloc::vec![$($nodes.$path),*],
            cost: $cost,
        }
    };
    ($nodes:ident; [$($source:ident -( $($path:ident),* )> $target:ident : $cost:literal),* $(,)?]) => {
        alloc::vec![
            $(expected!(@rule $nodes; $source; ($($path),*) ;$target; $cost)),*
        ]
    };
}

pub(in crate::shortest_paths) use expected;

pub(in crate::shortest_paths) struct Expect<G, T>
where
    G: GraphExt,
{
    pub(in crate::shortest_paths) source: <G::Storage as GraphStorage>::NodeId,
    pub(in crate::shortest_paths) target: <G::Storage as GraphStorage>::NodeId,

    pub(in crate::shortest_paths) transit: Vec<<G::Storage as GraphStorage>::NodeId>,

    pub(in crate::shortest_paths) cost: T,
}

pub(in crate::shortest_paths) struct TestCase<'a, G, A, T>
where
    G: GraphExt,
{
    graph: &'a Graph<G::Storage>,
    algorithm: &'a A,
    expected: &'a [Expect<G, T>],
}

impl<'a, G, A, T> TestCase<'a, G, A, T>
where
    G: GraphExt,
{
    pub(crate) const fn new(
        graph: &'a Graph<G::Storage>,
        algorithm: &'a A,
        expected: &'a [Expect<G, T>],
    ) -> Self {
        Self {
            graph,
            algorithm,
            expected,
        }
    }
}

impl<'a, G, A, T> TestCase<'a, G, A, T>
where
    G: GraphExt,
    <G::Storage as GraphStorage>::NodeId: Eq + Hash + Debug + Display,
    A: ShortestPath<G::Storage, Cost = T>,
    T: PartialEq + Debug,
{
    fn assert_path_routes(
        &self,
        routes: Result<impl Iterator<Item = Route<'a, G::Storage, T>>, A::Error>,
    ) {
        let mut routes: HashMap<_, _> = routes
            .unwrap()
            .map(|route| {
                (
                    (route.path().source().id(), route.path().target().id()),
                    route,
                )
            })
            .collect();

        for expect in self.expected {
            let source = &expect.source;
            let target = &expect.target;

            let route = routes.remove(&(source, target)).expect("route not found");

            let (path, cost) = route.into_parts();

            assert_eq!(path.source().id(), source, "source of {source} -> {target}");
            assert_eq!(path.target().id(), target, "target of {source} -> {target}");
            assert_eq!(
                path.transit().iter().map(Node::id).collect::<Vec<_>>(),
                expect.transit.iter().collect::<Vec<_>>(),
                "transit of {source} -> {target}"
            );
            assert_eq!(*cost.value(), expect.cost, "cost of {source} -> {target}");
        }

        assert!(routes.is_empty());
    }

    pub(in crate::shortest_paths) fn assert_every_path(&self) {
        self.assert_path_routes(self.algorithm.every_path(self.graph));
    }

    pub(in crate::shortest_paths) fn assert_path_from(
        &self,
        source: &<G::Storage as GraphStorage>::NodeId,
    ) {
        self.assert_path_routes(self.algorithm.path_from(self.graph, source));
    }
}

impl<'a, G, A, T> TestCase<'a, G, A, T>
where
    G: GraphExt,
    <G::Storage as GraphStorage>::NodeId: Eq + Hash + Debug + Display,
    A: ShortestDistance<G::Storage, Cost = T>,
    T: PartialEq + Debug,
{
    fn assert_distance_routes(
        &self,
        routes: Result<impl Iterator<Item = DirectRoute<'a, G::Storage, T>>, A::Error>,
    ) {
        let mut routes: HashMap<_, _> = routes
            .unwrap()
            .map(|route| ((route.source().id(), route.target().id()), route))
            .collect();

        for expect in self.expected {
            let source = &expect.source;
            let target = &expect.target;

            let route = routes.remove(&(source, target)).expect("route not found");

            assert_eq!(
                route.source().id(),
                source,
                "source of {source} -> {target}"
            );
            assert_eq!(
                route.target().id(),
                target,
                "target of {source} -> {target}"
            );

            assert_eq!(
                *route.cost().value(),
                expect.cost,
                "cost of {source} -> {target}"
            );
        }

        assert!(routes.is_empty());
    }

    pub(in crate::shortest_paths) fn assert_every_distance(&self) {
        self.assert_distance_routes(self.algorithm.every_distance(self.graph));
    }

    pub(in crate::shortest_paths) fn assert_distance_from(
        &self,
        source: &<G::Storage as GraphStorage>::NodeId,
    ) {
        self.assert_distance_routes(self.algorithm.distance_from(self.graph, source));
    }
}

pub(in crate::shortest_paths) trait GraphExt {
    type Storage: GraphStorage;
}

impl<S> GraphExt for Graph<S>
where
    S: GraphStorage,
{
    type Storage = S;
}

pub(in crate::shortest_paths) fn assert_path<S, T>(
    mut received: HashMap<NodeId, (T, Vec<Node<S>>)>,
    expected: &[(NodeId, T, &[NodeId])],
) where
    S: GraphStorage<NodeId = NodeId>,
    T: PartialEq + Debug,
{
    assert_eq!(received.len(), expected.len());

    for (node, expected_distance, expected_route) in expected {
        let (distance, route) = received.remove(node).unwrap();
        let route: Vec<_> = route.into_iter().map(|node| *node.id()).collect();

        assert_eq!(distance, *expected_distance);
        assert_eq!(&route, expected_route);
    }
}

pub(in crate::shortest_paths) fn assert_distance<T>(
    mut received: HashMap<NodeId, T>,
    expected: &[(NodeId, T)],
) where
    T: PartialEq + Debug,
{
    assert_eq!(received.len(), expected.len());

    for (node, expected_distance) in expected {
        let distance = received.remove(node).unwrap();

        assert_eq!(distance, *expected_distance);
    }
}

pub(in crate::shortest_paths) fn path_from<'a, S, P>(
    graph: &'a Graph<S>,
    source: &'a S::NodeId,
    algorithm: &P,
) -> HashMap<S::NodeId, (P::Cost, Vec<Node<'a, S>>)>
where
    P: ShortestPath<S>,
    S: GraphStorage,
    S::NodeId: Copy + Eq + Hash,
{
    algorithm
        .path_from(graph, source)
        .unwrap()
        .map(|route| {
            let (path, cost) = route.into_parts();

            (*path.target().id(), (cost.into_value(), path.to_vec()))
        })
        .collect()
}

pub(in crate::shortest_paths) fn distance_from<'a, S, P>(
    graph: &'a Graph<S>,
    source: &'a S::NodeId,
    algorithm: &P,
) -> HashMap<S::NodeId, P::Cost>
where
    P: ShortestDistance<S>,
    S: GraphStorage,
    S::NodeId: Copy + Eq + Hash,
{
    algorithm
        .distance_from(graph, source)
        .unwrap()
        .map(|route| {
            let (_, target, cost) = route.into_parts();

            (*target.id(), cost.into_value())
        })
        .collect()
}
