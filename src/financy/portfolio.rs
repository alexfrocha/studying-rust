pub enum Asset {
    Stocks,
    Bond,
    Funds,
    Cash
}

impl Asset {
    pub fn price(&self) -> f64 {
        match self {
            Asset::Stocks => 10.0,
            Asset::Bond => 20.0,
            Asset::Funds => 30.0,
            Asset::Cash => 40.0
        }
    }
}