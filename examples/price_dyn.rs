use patterns::decorator_dyn::{BasePrice, DiscauntDecorator, Price, TaxtDecorator};
fn main() {
    let with_tax = TaxtDecorator::new(Box::new(BasePrice));
    let with_tax_and_discaunt = DiscauntDecorator::new(Box::new(with_tax));

    let start_price = 100.0;
    let finish_prise = with_tax_and_discaunt.calculate(start_price);
    println!(
        "Start price: {}, finish price: {}",
        start_price, finish_prise
    );
}
