/*! The implementation of links used by server and clients
*/

use toxcore::crypto_core::*;

use std::collections::HashMap;
use std::slice::Iter;

pub const MAX_LINKS_N: usize = 240;

/// Link status.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LinkStatus {
    /// The link is registered on one side only.
    ///
    /// We received `RouteResponse` packet with connection id but can't use it
    /// until we get `ConnectNotification` packet.
    Registered,
    /// The link is registered on both sides: both clients are linked.
    /// It means that both clients sent RouteRequest and received ConnectNotification
    Online(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Link {
    pub pk: PublicKey,
    pub status: LinkStatus,
}

impl Link {
    fn new(pk: PublicKey) -> Link {
        Link {
            pk,
            status: LinkStatus::Registered,
        }
    }
    fn downgrade(&mut self) {
        self.status = LinkStatus::Registered;
    }
    fn upgrade(&mut self, to: u8) {
        self.status = LinkStatus::Online(to);
    }
}

/**

*/

pub struct Links {
    links: [Option<Link>; MAX_LINKS_N],
    pk_to_id: HashMap<PublicKey, u8>,
}

impl Links {
    pub fn new() -> Links {
        Links {
            links: [None; 240],
            pk_to_id: HashMap::new()
        }
    }
    pub fn insert(&mut self, pk: &PublicKey) -> Option<u8> {
        let possible_index = { self.pk_to_id.get(pk).cloned() };
        match possible_index {
            Some(index) => Some(index), // already inserted
            None => {
                if let Some(index) = self.links.iter().position(|link| link.is_none()) {
                    let link = Link::new(*pk);
                    self.links[index] = Some(link);
                    self.pk_to_id.insert(*pk, index as u8);
                    Some(index as u8)
                } else {
                    // no enough room for a link
                    None
                }
            }
        }
    }
    pub fn by_id(&self, index: u8) -> Option<&Link> {
        let index = index as usize;
        if index < MAX_LINKS_N {
            self.links[index].as_ref()
        } else {
            None
        }
    }
    pub fn id_by_pk(&self, pk: &PublicKey) -> Option<u8> {
        self.pk_to_id.get(pk).cloned()
    }
    pub fn take(&mut self, index: u8) -> Option<Link> {
        let index = index as usize;
        if index < MAX_LINKS_N {
            if let Some(link) = self.links[index].take() {
                self.pk_to_id.remove(&link.pk);
                Some(link)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn downgrade(&mut self, index: u8) {
        let index = index as usize;
        if index < MAX_LINKS_N {
            if let Some(mut link) = self.links[index] {
                link.downgrade();
            }
        }
    }
    pub fn upgrade(&mut self, index: u8, to: u8) {
        let index = index as usize;
        if index < MAX_LINKS_N {
            if let Some(mut link) = self.links[index] {
                link.upgrade(to);
            }
        }
    }

    /** Iter over each link in self.links
    */
    pub fn iter_links(&self) -> Iter<Option<Link>> {
        self.links.iter()
    }
}
