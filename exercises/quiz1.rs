// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought. No hints this time!
//
// No hints this time ;)

// I AM NOT DONE

// Put your function here!
// fn calculate_price_of_apples {

// Don't modify this function!
fn calculate_price_of_apples(quantity: u32) -> u32 {  
    if quantity <= 40 {  
        quantity * 2  
    } else {  
        40 * 2 + (quantity - 40) * 1  
    }  
}

#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(81, price3);
    assert_eq!(105, price4);
}
