extern crate itch_parser;
extern crate optimized_lob;

use alloy::primitives::U256;
use itch_parser::Body::{
    AddOrder, DeleteOrder, OrderCancelled, OrderExecuted, OrderExecutedWithPrice, ReplaceOrder,
};
use itch_parser::MessageStream;
use optimized_lob::{
    order::OrderId, orderbook_manager::OrderBookManager, quantity::Qty, utils::BookId,
};
use std::path::Path;
use std::time::Instant;

pub fn test_lob(file_path: &str) {
    let path_to_market_data = Path::new(file_path);
    let stream = MessageStream::from_file(path_to_market_data).unwrap();

    println!("------------------------------------");
    println!("LOB Processing...\n");

    // Counters
    let mut messages = 0;
    let mut add_order_count = 0;
    let mut execute_orders_count = 0;
    let mut cancel_order_count = 0;
    let mut delete_order_count = 0;
    let mut replace_order_count = 0;

    let start = Instant::now();
    let mut orderbook = OrderBookManager::new(); // Initialize the orderbook

    // Process messages
    for msg in stream {
        let unwrapped_msg = msg.unwrap();
        let stock_locate = unwrapped_msg.stock_locate;

        match unwrapped_msg.body {
            AddOrder {
                order_id,
                is_bid,
                shares,
                stock: _,
                price,
            } => {
                let oid: Option<u32> = order_id.try_into().ok();

                match oid {
                    Some(id) => {
                        orderbook.add_order(
                            OrderId(id),
                            BookId(stock_locate),
                            Qty(U256::from(shares)),
                            U256::from(price),
                            is_bid,
                        );
                    }
                    None => {
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                add_order_count += 1;
            }
            OrderExecuted {
                order_id,
                shares,
                match_number: _,
            } => {
                let oid: Option<u32> = order_id.try_into().ok();
                match oid {
                    Some(id) => {
                        orderbook.execute_order(OrderId(id), Qty(U256::from(shares)));
                    }
                    None => {
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                execute_orders_count += 1;
            }
            OrderExecutedWithPrice {
                order_id,
                shares,
                match_number: _,
                printable: _,
                price: _,
            } => {
                let oid: Option<u32> = order_id.try_into().ok();
                match oid {
                    Some(id) => {
                        orderbook.execute_order(OrderId(id), Qty(U256::from(shares)));
                    }
                    None => {
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                execute_orders_count += 1;
            }
            OrderCancelled { order_id, shares } => {
                let oid: Option<u32> = order_id.try_into().ok();
                match oid {
                    Some(id) => {
                        orderbook.cancel_order(OrderId(id), Qty(U256::from(shares)));
                    }
                    None => {
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                cancel_order_count += 1;
            }
            DeleteOrder { order_id } => {
                let oid: Option<u32> = order_id.try_into().ok();
                match oid {
                    Some(id) => {
                        orderbook.remove_order(OrderId(id));
                    }
                    None => {
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }
                delete_order_count += 1;
            }
            ReplaceOrder {
                old_order_id,
                new_order_id,
                shares,
                price,
            } => {
                let old_oid: Option<u32> = old_order_id.try_into().ok();
                let new_oid: Option<u32> = new_order_id.try_into().ok();

                match (old_oid, new_oid) {
                    (Some(id), Some(new_id)) => {
                        orderbook.replace_order(
                            OrderId(id),
                            OrderId(new_id),
                            Qty(U256::from(shares)),
                            U256::from(price),
                        );
                    }
                    _ => {
                        println!("Failed to convert Order ID u32 due to overflow");
                        break;
                    }
                }

                replace_order_count += 1;
            }
            _ => {}
        }

        messages += 1;
    }

    let duration = Instant::now() - start;
    let speed = messages as f64 / duration.as_secs_f64();
    println!("Success...\n");
    println!("Performance Metrics:");
    println!("Total Messages: {}", messages);
    println!(
        "ITCH Latency: {} ns",
        duration.as_nanos() / messages as u128
    );
    println!("Total Time: {:.3} seconds", duration.as_secs_f64());
    println!("Speed: {} msg/second\n", speed as u32);
    println!("Orderbook Statistics:");
    println!("Total Add Orders: {}", add_order_count);
    println!("Total Execute Orders: {}", execute_orders_count);
    println!("Total Cancel Orders: {}", cancel_order_count);
    println!("Total Delete Orders: {}", delete_order_count);
    println!("Total Replace Orders: {}", replace_order_count);
    println!("------------------------------------");
}
