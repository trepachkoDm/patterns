use patterns::decorator_st::{BasePrice, DiscountDecorator, Price, TaxtDecorator};
fn main() {
    let base_price = BasePrice {};
    let with_tax = TaxtDecorator::new(base_price);
    let with_tax_and_discaunt = DiscountDecorator::new(with_tax);

    let start_price = 100.0;
    let finish_prise = with_tax_and_discaunt.calculate(start_price);
    println!(
        "Start price: {}, finish price: {}",
        start_price, finish_prise
    );
}
