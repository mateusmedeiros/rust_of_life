#[derive(Clone, Default)]
pub struct Cell {
    _alive: bool
}

impl Cell {
    pub fn arise(&mut self) {
        self._alive = true;
    }

    pub fn die(&mut self) {
        self._alive = false;
    }

    pub fn set_alive(&mut self, value: bool) {
        self._alive = value;
    }

    pub fn is_alive(&self) -> bool {
        self._alive
    }

    pub fn alive(mut self, value: bool) -> Self {
        self._alive = value;
        self
    }
}
