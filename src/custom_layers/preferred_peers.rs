use crate::{GossipsBuilder, Id, Layer, NodeProfile, Nodes, ViewBuilder};

#[derive(Clone, Debug)]
pub struct PreferredPeers {
    preferred: Vec<Id>,
}

impl PreferredPeers {
    pub fn with_peers(peers: Vec<Id>) -> Self {
        PreferredPeers { preferred: peers }
    }
}

impl Layer for PreferredPeers {
    fn alias(&self) -> &'static str {
        "preferred_peers"
    }

    fn reset(&mut self) {
        // No-op
    }

    fn populate(&mut self, _: &NodeProfile, _: &Nodes) {
        // No-op
    }

    fn view(&mut self, view: &mut ViewBuilder, all_nodes: &mut Nodes) {
        self.preferred.iter().for_each(|id| {
            if let Some(node) = all_nodes.get_mut(id) {
                view.add(node);
            }
        })
    }

    fn gossips(&mut self, _: &NodeProfile, _: &mut GossipsBuilder, _: &Nodes) {
        // We won't gossip preferred peers. Instead we rely on them to perform their own gossiping.
    }
}
