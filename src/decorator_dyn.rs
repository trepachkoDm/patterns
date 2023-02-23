pub trait Price {
    fn calculate(&self, price: f32) -> f32;
}

pub struct BasePrice;

impl Price for BasePrice {
    fn calculate(&self, price: f32) -> f32 {
        price
    }
}

pub struct TaxtDecorator {
    decorator: Box<dyn Price>,
}

impl TaxtDecorator {
    pub fn new(decorator: Box<dyn Price>) -> Self {
        Self { decorator }
    }
}

impl Price for TaxtDecorator {
    fn calculate(&self, price: f32) -> f32 {
        self.decorator.calculate(price) * 1.15
    }
}

pub struct DiscauntDecorator {
    decorator: Box<dyn Price>,
}

impl DiscauntDecorator {
    pub fn new(decorator: Box<dyn Price>) -> Self {
        Self { decorator }
    }
}

impl Price for DiscauntDecorator {
    fn calculate(&self, price: f32) -> f32 {
        self.decorator.calculate(price) * 0.75
    }
}
