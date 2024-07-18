use std::collections::HashMap;

pub struct Store {
  buckets: HashMap<String, Bucket>,
}

struct Bucket {
  cfg: crate::Listing
  images: Vec<Image>,
}

struct Image {
  path: String,
  // TODO: buffer?
}

impl Store {
  const fn empty() -> Store {
    Store::Empty,
  }
}