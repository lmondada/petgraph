//! A bi-map between node indices and their position in a topological order.
//!
//! This data structure is an implementation detail and is not exposed in the
//! public API.
use std::fmt;

use crate::{
    algo::{toposort, Cycle},
    visit::{GraphBase, IntoNeighborsDirected, IntoNodeIdentifiers, NodeIndexable, Visitable},
};

/// A bijective map between node indices and their position in a topological order.
#[derive(Clone)]
pub(super) struct OrderMap<N> {
    /// The topological order of the nodes.
    order: Vec<N>,
    /// The inverse of `order`, i.e. for each node index, its position in `order`
    /// (requires `NodeIndexable`).
    order_inv: Vec<usize>,
}

impl<N> Default for OrderMap<N> {
    fn default() -> Self {
        Self {
            order: Default::default(),
            order_inv: Default::default(),
        }
    }
}

impl<N: fmt::Debug> fmt::Debug for OrderMap<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OrderMap")
            .field("order", &self.order)
            .finish()
    }
}

impl<N: Copy> OrderMap<N> {
    pub(super) fn try_from_graph<G>(graph: G) -> Result<Self, Cycle<G::NodeId>>
    where
        G: NodeIndexable<NodeId = N> + IntoNeighborsDirected + IntoNodeIdentifiers + Visitable,
    {
        let order = toposort(graph, None)?;
        let mut order_inv = vec![0; graph.node_bound()];
        for (i, &id) in order.iter().enumerate() {
            order_inv[graph.to_index(id)] = i;
        }
        Ok(Self { order, order_inv })
    }

    pub(super) fn with_capacity(nodes: usize) -> Self {
        Self {
            order: Vec::with_capacity(nodes),
            order_inv: Vec::with_capacity(nodes),
        }
    }

    /// Map a node to its position in the topological order.
    pub(super) fn get_order(&self, id: N, graph: impl NodeIndexable<NodeId = N>) -> usize {
        self.order_inv[graph.to_index(id)]
    }

    /// Map a position in the topological order to a node.
    pub(super) fn get_node(&self, pos: usize) -> N {
        self.order[pos]
    }

    pub(super) fn as_slice(&self) -> &[N] {
        &self.order
    }

    pub(super) fn add_node(&mut self, id: N, graph: impl NodeIndexable<NodeId = N>) {
        self.order.push(id);
        let pos = self.order.len() - 1;
        let idx = graph.to_index(id);
        // Make sure the order_inv is large enough.
        if idx >= self.order_inv.len() {
            self.order_inv.resize(graph.node_bound(), 0);
        }
        self.order_inv[idx] = pos;
    }

    pub(super) fn set_order(&mut self, id: N, pos: usize, graph: impl NodeIndexable<NodeId = N>) {
        self.order[pos] = id;
        self.order_inv[graph.to_index(id)] = pos;
    }
}

impl<G: Visitable> super::Acyclic<G> {
    pub(super) fn get_order(&self, id: G::NodeId) -> usize
    where
        for<'a> &'a G: NodeIndexable + GraphBase<NodeId = G::NodeId>,
    {
        self.order_map.get_order(id, &self.graph)
    }

    pub(super) fn get_node(&self, pos: usize) -> G::NodeId {
        self.order_map.get_node(pos)
    }
}
