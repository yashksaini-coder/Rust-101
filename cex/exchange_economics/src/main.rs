// order_economics: Computes the four key metrics for a trade.
//
// INPUT:
//   best_bid: u64  → highest resting buy price (e.g., 100)
//   best_ask: u64  → lowest resting sell price (e.g., 105)
//   price: u64     → the order's execution price (e.g., 100)
//   qty: u64       → quantity to trade (e.g., 10)
//   fee_bps: u64   → fee in basis points (e.g., 30 means 0.30%)
//                     1 bps = 1/10,000 of the value
//
// OUTPUT:
//   (u64, u64, u64, u64) → (spread, midprice, notional, fee)
//
// FORMULAS:
//   spread   = best_ask - best_bid
//   midprice = (best_bid + best_ask) / 2      (integer division)
//   notional = price * qty
//   fee      = notional * fee_bps / 10000     (integer division)
//
// EXAMPLE:
//   order_economics(100, 105, 100, 10, 30) → (5, 102, 1000, 3)
//
fn order_economics(best_bid: u64, best_ask: u64, price: u64, qty: u64, fee_bps: u64) -> (u64, u64, u64, u64) {
    let spread = best_ask - best_bid;

    // TODO: calculate midprice — average of best_bid and best_ask (integer division)
    let midprice = (best_ask + best_bid) / 2;

    // TODO: calculate notional — total value of the trade
    let notional = price * qty;

    // TODO: calculate fee — exchange's cut in basis points
    let fee = notional * fee_bps / 10000;

    (spread, midprice, notional, fee)
}

fn main() {
    let order = order_economics(110, 120, 100, 50, 30);
    println!("{:?}", order);
}