pub struct Coin {
    pub value: u32
}

impl Coin {
    pub fn new(value: u32) -> Coin {
        Coin{value}
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn set_value(&mut self, value: u32) {
        self.value = value;
    } 
}