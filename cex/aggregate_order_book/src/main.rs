use std::collections::BTreeMap;

// aggregate_and_best: Builds one side of the order book from raw orders.
//
// INPUT:
//   orders: &[(u64, u64)] → list of (price, quantity) tuples
//   is_buy: bool → true for bid side, false for ask side
//
// OUTPUT:
//   (Vec<(u64, u64)>, u64) → (sorted_levels, best_price)
//     - Levels: aggregated (price, total_qty) tuples
//       Sorted DESCENDING if is_buy (highest first)
//       Sorted ASCENDING if !is_buy (lowest first)
//     - Best price: first element's price (the best)
//
// APPROACH:
//   1. Create a BTreeMap<u64, u64> (automatically sorted ascending by key)
//   2. Loop through orders, summing qty at each price:
//      for &(price, qty) in orders { *map.entry(price).or_insert(0) += qty; }
//   3. Convert to Vec:
//      - If is_buy: map.into_iter().rev().collect()  (reverse = descending)
//      - If !is_buy: map.into_iter().collect()        (ascending)
//   4. Best price = levels[0].0
//
// EXAMPLE:
//   aggregate_and_best(&[(100,10),(100,20),(101,5)], true)
//     → (vec![(101,5), (100,30)], 101)
//
fn aggregate_and_best(orders: &[(u64, u64)], is_buy: bool) -> (Vec<(u64, u64)>, u64) {
    let mut map = BTreeMap::new();

    for &(price, qty) in orders {
        *map.entry(price).or_insert(0u64) += qty;
    }

    // TODO: Convert the map to a Vec of (price, qty) tuples
    //   If is_buy: reverse the iterator for descending order
    let items: Vec <(u64, u64)> = if is_buy {
        map.into_iter().rev().collect()
    //   If !is_buy: keep ascending order
    } else {
        map.into_iter().collect()
    };
    // Then extract the best price from levels[0].0
    let _best_price = items[0].0;

    (items, _best_price)
}

fn main() {
    let bids = &[(100,10),(100,20),(101,5),(99,15),(98,40)];
    let asks = &[(102,8),(103,12),(103,3),(104,20)];

    let (bid_levels, best_bid) = aggregate_and_best(bids, true);
    let (ask_levels, best_ask) = aggregate_and_best(asks, false);

    println!(bid_levels, ask_levels, best_bid, best_ask);
}