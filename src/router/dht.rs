pub struct Dht {
  r: Router,
  finger: Mutex<BTreeMap<PublicKey, D>>,
  sorted: Vec<D>,
  requests: Mutex<BTreeMap<u8[8], DhtRequestContext>>
}

impl Dht {
  pub new(r: Router) {
    Dht {
      r,
      finger: Mutex::new(BTreeMap::new()),

