#[derive(Clone)]
pub struct ArrayMap<T> {
    offset: usize,
    elements: Vec<Option<T>>
}

impl <T: std::clone::Clone> ArrayMap<T> {
    pub fn new(start_key: usize, end_key: usize) -> ArrayMap<T> {
        ArrayMap {
            offset: start_key,
            elements: vec![None; end_key - start_key]
        }
    }

    pub fn insert(&mut self, key: usize, value: T) {
        self.elements[key - self.offset] = Some(value);
    }

    pub fn get(&self, key: usize) -> T {
        self.elements[key - self.offset].clone().unwrap()
    }
}

#[derive(Clone)]
pub struct UsizeArrayMap {
    offset: usize,
    elements: Vec<usize>
}

impl UsizeArrayMap {
    pub fn new(start_key: usize, end_key: usize) -> UsizeArrayMap {
        UsizeArrayMap {
            offset: start_key,
            elements: vec![0; end_key - start_key]
        }
    }

    pub fn swap(&mut self, key: usize, other_key: usize) {
        self.elements.swap(key, other_key);
    }

    pub fn insert(&mut self, key: usize, value: usize) {
        self.elements[key - self.offset] = value;
    }

    pub fn get(&self, key: usize) -> usize {
        self.elements[key - self.offset]
    }

    pub fn decrement(&mut self, key: usize) {
        self.elements[key - self.offset] -= 1;
    }

    pub fn increment(&mut self, key: usize) {
        self.elements[key - self.offset] += 1;
    }
}
