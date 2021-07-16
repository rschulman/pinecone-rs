use std::{collections::BTreeMap, sync::Mutex};

use crate::router::Router;

pub trait DhtEntry {
    fn public_key(&self) -> PublicKey;
    fn coordinates(&self) -> SwitchPorts;
    fn seen_recently(&self) -> bool;
}

pub struct Dht {
    r: Router,
    finger: Mutex<BTreeMap<PublicKey, Box<dyn DhtEntry>>>,
    sorted: Vec<Box<dyn DhtEntry>>,
    requests: Mutex<BTreeMap<Vec<u8>, DhtRequestContext>>,
}

impl Dht {
    pub fn new(r: Router) -> Dht {
        Dht {
            r,
            finger: Mutex::new(BTreeMap::<PublicKey, Box<dyn DhtEntry>>::new()),
            sorted: Vec::new(),
            requests: Mutex::new(BTreeMap::<Vec<u8>, DhtRequestContext>::new()),
        }
    }
}

struct DhtRequestContext {
  id: [u8; 8],
  ctx: 
}

type PublicKey = [u8; 32];
type PrivateKey = [u8; 32];
