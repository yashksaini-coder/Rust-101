use std::collections::BTreeMap;

// ─── types ───────────────────────────────────────────────────────────────────

type Price = u64;
type Qty   = u64;
type Level = (Price, Qty);

struct OrderBook {
    bid_levels: Vec<Level>,
    ask_levels: Vec<Level>,
    best_bid:   Price,
    best_ask:   Price,
}

// ─── core logic ──────────────────────────────────────────────────────────────

fn aggregate_and_best(orders: &[Level], is_buy: bool) -> (Vec<Level>, Price) {
    let mut map: BTreeMap<Price, Qty> = BTreeMap::new();

    for &(price, qty) in orders {
        *map.entry(price).or_insert(0) += qty;
    }

    let levels: Vec<Level> = if is_buy {
        map.into_iter().rev().collect()   // descending: highest bid first
    } else {
        map.into_iter().collect()         // ascending:  lowest ask first
    };

    let best_price = levels[0].0;
    (levels, best_price)
}

fn build_book(bids: &[Level], asks: &[Level]) -> OrderBook {
    let (bid_levels, best_bid) = aggregate_and_best(bids, true);
    let (ask_levels, best_ask) = aggregate_and_best(asks, false);
    OrderBook { bid_levels, ask_levels, best_bid, best_ask }
}

// ─── display ─────────────────────────────────────────────────────────────────

fn print_level(price: Price, qty: Qty, side: &str, is_best: bool) {
    let tag = if is_best { " <-- best" } else { "" };
    println!("  {:>6}  |  {:>8}  |  {}{}", price, qty, side, tag);
}

fn print_book(book: &OrderBook) {
    println!("┌────────┬────────────┬────────────────┐");
    println!("│ price  │    qty     │   side         │");
    println!("├────────┼────────────┼────────────────┤");

    for &(price, qty) in book.ask_levels.iter().rev() {
        print_level(price, qty, "ASK", price == book.best_ask);
    }

    println!(
        "  ············ spread: {} ············",
        book.best_ask - book.best_bid
    );

    for &(price, qty) in &book.bid_levels {
        print_level(price, qty, "BID", price == book.best_bid);
    }

    println!("└────────┴────────────┴────────────────┘");
    println!("  best bid: {}   best ask: {}   spread: {}",
             book.best_bid, book.best_ask, book.best_ask - book.best_bid);
}

fn print_aggregation_steps(orders: &[Level], is_buy: bool) {
    println!("  raw input:  {:?}", orders);

    let mut map: BTreeMap<Price, Qty> = BTreeMap::new();
    for &(price, qty) in orders {
        *map.entry(price).or_insert(0) += qty;
    }
    println!("  after agg:  {:?}", map.iter().collect::<Vec<_>>());

    let (levels, best) = aggregate_and_best(orders, is_buy);
    println!("  sorted:     {:?}", levels);
    println!("  best price: {}", best);
}

// ─── tests ───────────────────────────────────────────────────────────────────

fn test_spec_example() {
    println!("════════════════════════════════════════");
    println!("  test 1 — spec example (buy side)");
    println!("════════════════════════════════════════");

    let orders: &[Level] = &[(100, 10), (100, 20), (101, 5)];
    print_aggregation_steps(orders, true);
}

fn test_full_book() {
    println!("\n════════════════════════════════════════");
    println!("  test 2 — full order book");
    println!("════════════════════════════════════════");

    let bids: &[Level] = &[
        (100, 10),
        (100, 20),  // merges with above → 100: 30
        (101,  5),
        ( 99, 15),
        ( 98, 40),
    ];
    let asks: &[Level] = &[
        (102,  8),
        (103, 12),
        (103,  3),  // merges with above → 103: 15
        (104, 20),
    ];

    println!("bid aggregation:");
    print_aggregation_steps(bids, true);
    println!("ask aggregation:");
    print_aggregation_steps(asks, false);

    println!();
    let book = build_book(bids, asks);
    print_book(&book);
}

fn test_single_order() {
    println!("\n════════════════════════════════════════");
    println!("  test 3 — single order per side");
    println!("════════════════════════════════════════");

    let bids: &[Level] = &[(500, 1)];
    let asks: &[Level] = &[(501, 1)];
    let book = build_book(bids, asks);
    print_book(&book);
}

fn test_deep_book() {
    println!("\n════════════════════════════════════════");
    println!("  test 4 — deep book, many merges");
    println!("════════════════════════════════════════");

    let bids: &[Level] = &[
        (200,  5), (200, 10), (200, 15),  // all merge → 200: 30
        (199,  8), (199,  2),             // merge     → 199: 10
        (198, 50),
        (195, 100),
    ];
    let asks: &[Level] = &[
        (201, 20),
        (202,  5), (202,  5),             // merge     → 202: 10
        (203, 30), (203, 30), (203, 30),  // merge     → 203: 90
        (205, 75),
    ];

    println!("raw bids: {:?}", bids);
    println!("raw asks: {:?}", asks);
    println!();

    let book = build_book(bids, asks);
    print_book(&book);
}

// ─── main ────────────────────────────────────────────────────────────────────

fn main() {
    test_spec_example();
    test_full_book();
    test_single_order();
    test_deep_book();
}