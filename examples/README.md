# examples

To validate the order book streams, the modules are named using the format `<exchange>_<derivative>_<orderbook level>_streams.rs`. For instance, `bit_order_books_l2_streams.rs` is the module for validating the order book streams for BitMEX.

Use the following command to run the examples:

```bash
cargo run --package barter-data --example <example_name>
```

Perpetual Futures:

- `aevo_perps_ob_l2_streams`
- `bit_perps_ob_l2_streams`
- `deribit_perps_ob_l2_streams`
- `dydx_perps_ob_l2_streams`
- `hyperliquid_perps_ob_l2_streams`
- `powertrade_perps_ob_l3_streams`

Options:

- `aevo_options_ob_l2_streams`
- `bit_options_ob_l2_streams`
- `deribit_options_ob_l2_streams`
- `dydx_options_ob_l2_streams`
- `powertrade_options_ob_l3_streams`
