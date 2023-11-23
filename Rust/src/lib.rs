pub enum CollectionData {
    String(String),
    StaticStr(&'static str),
    Bool(bool),
    Char(char),
    Usize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Isize(isize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
}

pub struct Collection {
    items: Vec<CollectionData>,
    cursor: usize,
}

impl Collection {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            cursor: 0,
        }
    }

    pub fn add_item(&mut self, item: CollectionData) {
        self.items.push(item);
    }

    pub fn get_next(&mut self) -> Option<&CollectionData> {
        if self.cursor < self.items.len() {
            let item = &self.items[self.cursor];
            self.cursor += 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn reset_next(&mut self) {
        self.cursor = 0;
    }

    pub fn has_next(&self) -> bool {
        self.cursor < self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

pub struct TypedCollection<T> {
    items: Vec<T>,
    cursor: usize,
}

impl<T> TypedCollection<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            cursor: 0,
        }
    }

    pub fn add_item(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn get_next(&mut self) -> Option<&T> {
        if self.cursor < self.items.len() {
            let item = &self.items[self.cursor];
            self.cursor += 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn reset_next(&mut self) {
        self.cursor = 0;
    }

    pub fn has_next(&self) -> bool {
        self.cursor < self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}