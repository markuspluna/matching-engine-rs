#![cfg(test)]
mod test {
    use crate::lob_tests::utils::get_level_capacity;
    use alloy::primitives::U256;
    /// Tests for the same book and multiple levels
    /// It tests all the use cases for the same level and book
    /// like add, cancel, remove, and execute orders.
    use optimized_lob::order::OrderId;
    use optimized_lob::orderbook_manager::OrderBookManager;
    use optimized_lob::quantity::Qty;
    use optimized_lob::utils::BookId;

    #[test]
    fn test_for_same_book_with_multiple_levels() {
        let mut orderbook_manager = OrderBookManager::new();

        orderbook_manager.add_order(
            OrderId(0),
            BookId(1),
            Qty(U256::from(800)),
            U256::from(500),
            true,
        );
        orderbook_manager.add_order(
            OrderId(1),
            BookId(1),
            Qty(U256::from(50)),
            U256::from(600),
            true,
        );
        orderbook_manager.add_order(
            OrderId(2),
            BookId(1),
            Qty(U256::from(26)),
            U256::from(600),
            true,
        );

        assert_eq!(
            Qty(U256::from(800)),
            get_level_capacity(&orderbook_manager, 1, 0)
        );

        orderbook_manager.remove_order(OrderId(2));
        assert_eq!(
            Qty(U256::from(50)),
            get_level_capacity(&orderbook_manager, 1, 1)
        );

        orderbook_manager.cancel_order(OrderId(0), Qty(U256::from(100)));
        assert_eq!(
            Qty(U256::from(700)),
            get_level_capacity(&orderbook_manager, 1, 0)
        );

        orderbook_manager.remove_order(OrderId(1));
        assert_eq!(
            Qty(U256::from(0)),
            get_level_capacity(&orderbook_manager, 1, 1)
        );

        orderbook_manager.add_order(
            OrderId(3),
            BookId(1),
            Qty(U256::from(50)),
            U256::from(800),
            true,
        );
        orderbook_manager.add_order(
            OrderId(4),
            BookId(1),
            Qty(U256::from(26)),
            U256::from(600),
            true,
        );
        assert_eq!(
            Qty(U256::from(50)),
            get_level_capacity(&orderbook_manager, 1, 1)
        );
        assert_eq!(
            Qty(U256::from(26)),
            get_level_capacity(&orderbook_manager, 1, 2)
        );
        orderbook_manager.remove_order(OrderId(3));
        assert_eq!(
            Qty(U256::from(0)),
            get_level_capacity(&orderbook_manager, 1, 1)
        );
        orderbook_manager.remove_order(OrderId(4));
        assert_eq!(
            Qty(U256::from(0)),
            get_level_capacity(&orderbook_manager, 1, 2)
        );
        orderbook_manager.remove_order(OrderId(0));
        assert_eq!(
            Qty(U256::from(0)),
            get_level_capacity(&orderbook_manager, 1, 0)
        );
        orderbook_manager.add_order(
            OrderId(5),
            BookId(1),
            Qty(U256::from(50)),
            U256::from(1500),
            true,
        );
        orderbook_manager.add_order(
            OrderId(6),
            BookId(1),
            Qty(U256::from(26)),
            U256::from(500),
            true,
        );
        assert_eq!(
            Qty(U256::from(50)),
            get_level_capacity(&orderbook_manager, 1, 0)
        );
        assert_eq!(
            Qty(U256::from(26)),
            get_level_capacity(&orderbook_manager, 1, 2)
        );
        assert_eq!(
            Qty(U256::from(0)),
            get_level_capacity(&orderbook_manager, 1, 1)
        );
        orderbook_manager.add_order(
            OrderId(6),
            BookId(1),
            Qty(U256::from(86)),
            U256::from(1400),
            true,
        );
        assert_eq!(
            Qty(U256::from(86)),
            get_level_capacity(&orderbook_manager, 1, 1)
        );
        orderbook_manager.add_order(
            OrderId(6),
            BookId(1),
            Qty(U256::from(96)),
            U256::from(1300),
            true,
        );
        assert_eq!(
            Qty(U256::from(96)),
            get_level_capacity(&orderbook_manager, 1, 3)
        );
    }
}
