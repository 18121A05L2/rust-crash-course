use std::time::Duration;
use tokio::time::sleep;
use tokio::{join, select};

async fn get_eth_price_from_exchange_1() -> u32 {
    sleep(Duration::from_millis(1000)).await;
    1000
}

async fn get_eth_price_from_exchange_2() -> u32 {
    sleep(Duration::from_millis(1000)).await;
    1010
}

async fn get_btc_price_from_exchange_1() -> u32 {
    sleep(Duration::from_millis(500)).await;
    20000
}

#[tokio::main]
async fn main() {
    // Exercise 1
    let eth_price = get_eth_price_from_exchange_1().await;
    let btc_price = get_btc_price_from_exchange_1().await;

    let (eth_price_using_join, btc_price_using_join) = join!(
        get_eth_price_from_exchange_1(),
        get_btc_price_from_exchange_1()
    );

    println!("join: ETH price: {}", eth_price);
    println!("join: BTC price: {}", btc_price);
    println!("join: ETH price using join: {}", eth_price_using_join);
    println!("join: BTC price using join: {}", btc_price_using_join);

    // Exercise 2
    let eth_price = 0;
    println!("select: ETH price: {}", eth_price);

    let result = select! {
        price1 = get_eth_price_from_exchange_1() => {
            price1
        }
        price2 = get_eth_price_from_exchange_2() => {
            price2
        }
    };

    println!("faster price from two exchanges : {}", result)
}
