pub trait Price {
    fn calculate(&self, price: f32) -> f32;
}

pub trait PriceDecorator: Price {}

pub struct BasePrice;

impl Price for BasePrice {
    fn calculate(&self, price: f32) -> f32 {
        price
    }
}

pub struct TaxtDecorator<T: Price> {
    decorator: T,
}

impl<T: Price> TaxtDecorator<T> {
    pub fn new(decorator: T) -> Self {
        Self { decorator }
    }
}

impl<T: Price> Price for TaxtDecorator<T> {
    fn calculate(&self, price: f32) -> f32 {
        self.decorator.calculate(price) * 1.15
    }
}

impl<T: Price> PriceDecorator for TaxtDecorator<T> {}

pub struct DiscountDecorator<T: Price> {
    decorator: T,
}

impl<T: Price> DiscountDecorator<T> {
    pub fn new(decorator: T) -> Self {
        Self { decorator }
    }
}

impl<T: Price> Price for DiscountDecorator<T> {
    fn calculate(&self, price: f32) -> f32 {
        self.decorator.calculate(price) * 0.75
    }
}

impl<T: Price> PriceDecorator for DiscountDecorator<T> {}
