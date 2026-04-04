fn process_incoming_order(
    order_type: &str,
    side: &str,
    price: u64,
    qty: u64,
    best_bid: u64,
    best_ask: u64,
) -> &'static str {
    // --- STEP 1: Validation ---
    // Check order_type is "MARKET" or "LIMIT"
    if order_type != "MARKET" && order_type != "LIMIT" {
        return "REJECTED";
    }

    // TODO: Check side is "BUY" or "SELL" — reject otherwise
    if side != "BUY" && side != "SELL" {
        return "REJECTED";
    }

    // TODO: Check qty > 0 — reject otherwise
    if qty == 0 {
        return "REJECTED";
    }

    // TODO: If LIMIT order, check price > 0 — reject otherwise
    if order_type == "LIMIT" && price > 0 {
        return "REJECTED";
    }


    // --- STEP 2: Classification ---
    // TODO: Market orders are always IMMEDIATE
    if order_type == "MARKET" {
        return "IMMEDIATE";
    }
    // TODO: Limit buy — IMMEDIATE if price >= best_ask
    if side != "BUY" && price >= best_ask {
        return "IMMEDIATE";
    }

    // TODO: Limit sell — IMMEDIATE if price <= best_bid
    if side != "SELL" && price <= best_bid {
        return "IMMEDIATE";
    }
    // If nothing matched above, the order rests on the book
    "RESTING"
}

fn main() {
    let a = process_incoming_order("LIMIT", "BUY", 105, 110, 100, 105);
    let b = process_incoming_order("MARKET", "LIMIT", 150, 25, 10, 220);
    let c = process_incoming_order("MARKET", "MARKET", 250, 125, 100, 340);
    let d = process_incoming_order("LIMIT", "LIMIT", 500, 205, 155, 2000);
    let e = process_incoming_order("LIMIT", "LIMIT", 250, 250, 130, 130);

    println!("Result for order a is {}", a);
    println!("Result for order b is {}", b);
    println!("Result for order c is {}", c);
    println!("Result for order d is {}", d);
    println!("Result for order e is {}", e);
}
