fn sort_buy_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();

    indices.sort_by(|&a, &b| {
        // Buy: Highest Price First (b.cmp(a)), then Earliest Time (a.cmp(b))
        orders[b].0.cmp(&orders[a].0)
            .then(orders[a].1.cmp(&orders[b].1))
    });

    indices
}

fn sort_sell_orders(orders: &[(u64, u64)]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..orders.len()).collect();

    indices.sort_by(|&a, &b| {
        // Sell: Lowest Price First (a.cmp(b)), then Earliest Time (a.cmp(b))
        orders[a].0.cmp(&orders[b].0) // FIXED: Changed b.cmp(a) to a.cmp(b)
            .then(orders[a].1.cmp(&orders[b].1))
    });

    indices
}

fn main() {
    // Data format: (Price, Timestamp)
    let market_orders = vec![
        (100, 10), // Index 0
        (105, 12), // Index 1
        (100, 8),  // Index 2 (Same price as 0, but earlier)
        (110, 15), // Index 3
    ];

    println!("--- Market Data ---");
    for (i, (p, t)) in market_orders.iter().enumerate() {
        println!("Index {}: Price={}, Time={}", i, p, t);
    }

    // 1. Sort Buys (Priority: Highest Price)
    let buy_priority = sort_buy_orders(&market_orders);
    println!("\nBuy Side Priority (Indices): {:?}", buy_priority);
    // Expected: [3, 1, 2, 0] 
    // (Price 110, then 105, then price 100 with earlier time 8, then time 10)

    // 2. Sort Sells (Priority: Lowest Price)
    let sell_priority = sort_sell_orders(&market_orders);
    println!("Sell Side Priority (Indices): {:?}", sell_priority);
    // Expected: [2, 0, 1, 3] 
    // (Price 100/Time 8, then Price 100/Time 10, then 105, then 110)
}