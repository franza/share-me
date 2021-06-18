use std::sync::Mutex;
use actix_web::rt::time::Instant;

#[derive(Debug, Clone)]
struct StoredLink {
    value: String,
    created_on: Instant,
}

#[derive(Debug)]
pub struct Storage {
    links: Mutex<Vec<StoredLink>>
}

impl Storage {
    pub fn new() -> Self {
        Storage { links: Mutex::new(Vec::new()) }
    }

    pub fn links(&self) -> Vec<String> {
        let mut links = self.links.lock().unwrap();
        links.sort_by(|l1, l2| l1.created_on.cmp(&l2.created_on));
        links.iter().map(|l| l.value.clone()).collect()
    }

    pub fn add_link(&self, new_link: String) {
        let mut links = self.links.lock().unwrap();
        links.push(StoredLink { value: new_link, created_on: Instant::now() });
    }
}
