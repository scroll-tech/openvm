pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

// #[derive(Default)]
pub struct Page2Requester {
    path_bus_index: usize,
    val_bus_index: usize,
    key_len: usize,
    val_len: usize,
    pub requests: Vec<Vec<u32>>,
}

impl Page2Requester {
    pub fn new(
        path_bus_index: usize,
        val_bus_index: usize,
        key_len: usize,
        val_len: usize,
        requests: Vec<Vec<u32>>,
    ) -> Self {
        Self {
            path_bus_index,
            val_bus_index,
            key_len,
            val_len,
            requests,
        }
    }

    pub fn path_bus_index(&self) -> usize {
        self.path_bus_index
    }
    pub fn val_bus_index(&self) -> usize {
        self.val_bus_index
    }
    pub fn key_len(&self) -> usize {
        self.key_len
    }
    pub fn val_len(&self) -> usize {
        self.val_len
    }

    pub fn reset_request(&mut self, requests: Vec<Vec<u32>>) {
        self.requests = requests.clone();
    }

    pub fn add_request(&mut self, key: Vec<u32>) {
        assert!(key.len() == self.key_len);
        self.requests.push(key);
    }
}
