# 🛡️ Midguard API

A robust REST API built with Rust and Axum framework for managing and querying historical data about prices, earnings, swaps, and rune pools. The API features comprehensive filtering, pagination, and efficient batch operations.

## ⭐ Features

- ✨ RESTful endpoints with comprehensive filtering
- 📊 Advanced querying with multiple parameters
- 🔄 Automatic data synchronization with cron jobs
- 🚀 Efficient batch operations
- 📝 Detailed error handling
- 🔍 Robust search functionality
- 📦 PostgreSQL integration with SQLx

## 🔌 API Endpoints

### 1. 📊 Price History (`GET /depth-history`)

Query historical price depth data with various filters.

```http
GET /depth-history?date_range=2024-12-08,2024-12-09&liquidity_units_lt=352937265746680&sort_by=start_time&order=DESC&page=1&limit=10&interval=hour

Eg Response:
{
  "data": [{
    "id": 1,
    "start_time": "2024-01-01T00:00:00Z",
    "end_time": "2024-01-01T23:59:59Z",
    "asset_depth": 79709431099,
    "rune_depth": 991143596835587,
    "asset_price": 12434.45829646652,
    "asset_price_usd": 70195.4236656004,
    "liquidity_units": 352960202587487,
    "members_count": 3496,
    "synth_units": 205123346124197,
    "synth_supply": 58594327901,
    "units": 558083548711684,
    "luvi": 0.015926631217813333
  }]
}
```

#### Supported Query Params

| Category | Parameter | Type | Description |
|----------|-----------|------|-------------|
| **Common** | interval | string | Time interval (5min, hour, day, week, month, quarter, year) |
| | limit | integer | Number of records per page |
| | page | integer | Page number |
| | order | string | Sort order (asc/desc) |
| | sort_by | string | Field to sort by |
| | date_range | string | Date range (YYYY-MM-DD,YYYY-MM-DD) |
| | count | integer | Count filter |
| **Asset Depth** | asset_depth_gt | integer | Greater than filter |
| | asset_depth_lt | integer | Less than filter |
| | asset_depth_eq | integer | Exact match filter |
| **Rune Depth** | rune_depth_gt | integer | Greater than filter |
| | rune_depth_lt | integer | Less than filter |
| | rune_depth_eq | integer | Exact match filter |
| **Price** | asset_price_gt | decimal | Greater than filter |
| | asset_price_lt | decimal | Less than filter |
| | asset_price_eq | decimal | Exact match filter |
| **USD Price** | asset_price_usd_gt | decimal | Greater than filter |
| | asset_price_usd_lt | decimal | Less than filter |
| | asset_price_usd_eq | decimal | Exact match filter |
| **Liquidity** | liquidity_units_gt | integer | Greater than filter |
| | liquidity_units_lt | integer | Less than filter |
| | liquidity_units_eq | integer | Exact match filter |
| **Members** | members_count_gt | integer | Greater than filter |
| | members_count_lt | integer | Less than filter |
| | members_count_eq | integer | Exact match filter |
| **Synth** | synth_units_gt | integer | Greater than filter |
| | synth_units_lt | integer | Less than filter |
| | synth_units_eq | integer | Exact match filter |
| | synth_supply_gt | integer | Greater than filter |
| | synth_supply_lt | integer | Less than filter |
| | synth_supply_eq | integer | Exact match filter |
| **Units** | units_gt | integer | Greater than filter |
| | units_lt | integer | Less than filter |
| | units_eq | integer | Exact match filter |
| **LUVI** | luvi_gt | decimal | Greater than filter |
| | luvi_lt | decimal | Less than filter |
| | luvi_eq | decimal | Exact match filter |

### 2. 💰 Earning History (`GET /earning-history`)

Query historical earning data with various filters.

```http
GET /earning-history?interval=week&limit=5&earnings_gt=1000

Response:
{
  "data": [{
    "id": 1,
    "start_time": "2024-01-01T00:00:00Z",
    "end_time": "2024-01-07T23:59:59Z",
    "liquidity_fees": 50000,
    "block_rewards": 100000,
    "earnings": 150000,
    "bonding_earnings": 75000,
    "liquidity_earnings": 75000,
    "avg_node_count": 100,
    "rune_price_usd": 5.75,
    "pools": [{
      "pool": "BTC.BTC",
      "asset_liquidity_fees": 1000,
      "rune_liquidity_fees": 2000,
      "total_liquidity_fees_rune": 3000,
      "saver_earning": 500,
      "rewards": 1500,
      "earnings": 2000
    }]
  }]
}
```

#### Supported Query Params

| Category | Parameter | Type | Description |
|----------|-----------|------|-------------|
| **Common** | interval | string | Time interval (5min, hour, day, week, month, quarter, year) |
| | limit | integer | Number of records per page |
| | page | integer | Page number |
| | order | string | Sort order (asc/desc) |
| | sort_by | string | Field to sort by |
| | date_range | string | Date range (YYYY-MM-DD,YYYY-MM-DD) |
| | count | integer | Count filter |
| **Liquidity Fees** | liquidity_fees_gt | integer | Greater than filter |
| | liquidity_fees_lt | integer | Less than filter |
| | liquidity_fees_eq | integer | Exact match filter |
| **Block Rewards** | block_rewards_gt | integer | Greater than filter |
| | block_rewards_lt | integer | Less than filter |
| | block_rewards_eq | integer | Exact match filter |
| **Earnings** | earnings_gt | integer | Greater than filter |
| | earnings_lt | integer | Less than filter |
| | earnings_eq | integer | Exact match filter |
| **Bonding** | bonding_earnings_gt | integer | Greater than filter |
| | bonding_earnings_lt | integer | Less than filter |
| | bonding_earnings_eq | integer | Exact match filter |
| **Liquidity** | liquidity_earnings_gt | integer | Greater than filter |
| | liquidity_earnings_lt | integer | Less than filter |
| | liquidity_earnings_eq | integer | Exact match filter |
| **Node Count** | avg_node_count_gt | decimal | Greater than filter |
| | avg_node_count_lt | decimal | Less than filter |
| | avg_node_count_eq | decimal | Exact match filter |
| **Rune Price** | rune_price_usd_gt | decimal | Greater than filter |
| | rune_price_usd_lt | decimal | Less than filter |
| | rune_price_usd_eq | decimal | Exact match filter |
| **Pool** | pool_eq | string | Pool name filter |
| | pool_asset_liquidity_fees_gt | integer | Asset liquidity fees greater than |
| | pool_asset_liquidity_fees_lt | integer | Asset liquidity fees less than |
| | pool_asset_liquidity_fees_eq | integer | Asset liquidity fees exact match |
| | pool_rune_liquidity_fees_gt | integer | Rune liquidity fees greater than |
| | pool_rune_liquidity_fees_lt | integer | Rune liquidity fees less than |
| | pool_rune_liquidity_fees_eq | integer | Rune liquidity fees exact match |
| | pool_total_liquidity_fees_gt | integer | Total liquidity fees greater than |
| | pool_total_liquidity_fees_lt | integer | Total liquidity fees less than |
| | pool_total_liquidity_fees_eq | integer | Total liquidity fees exact match |
| | pool_saver_earning_gt | integer | Saver earning greater than |
| | pool_saver_earning_lt | integer | Saver earning less than |
| | pool_saver_earning_eq | integer | Saver earning exact match |
| | pool_rewards_gt | integer | Pool rewards greater than |
| | pool_rewards_lt | integer | Pool rewards less than |
| | pool_rewards_eq | integer | Pool rewards exact match |
| | pool_earnings_gt | integer | Pool earnings greater than |
| | pool_earnings_lt | integer | Pool earnings less than |
| | pool_earnings_eq | integer | Pool earnings exact match |


### 3. 🔄 Swap History (`GET /swap-history`)

Query historical swap data with various filters.

```http
GET /swap-history?interval=hour&from_trade_volume_gt=10000

Response:
{
  "data": [{
    "id": 1,
    "start_time": "2024-01-01T00:00:00Z",
    "end_time": "2024-01-01T00:59:59Z",
    "average_slip": 0.1,
    "from_trade_count": 100,
    "from_trade_volume": 50000,
    "from_trade_volume_usd": 250000,
    "rune_price_usd": 5.75,
    "total_fees": 1000,
    "total_volume": 100000
  }]
}
```

#### Supported Query Params

| Category | Parameter | Type | Description |
|----------|-----------|------|-------------|
| **Common** | interval | string | Time interval (5min, hour, day, week, month, quarter, year) |
| | limit | integer | Number of records per page |
| | page | integer | Page number |
| | order | string | Sort order (asc/desc) |
| | sort_by | string | Field to sort by |
| | date_range | string | Date range (YYYY-MM-DD,YYYY-MM-DD) |
| | count | integer | Count filter |
| **From Trade** | from_trade_average_slip_gt/lt/eq | decimal | Average slip filters |
| | from_trade_count_gt/lt/eq | integer | Trade count filters |
| | from_trade_fees_gt/lt/eq | integer | Trade fees filters |
| | from_trade_volume_gt/lt/eq | integer | Volume filters |
| | from_trade_volume_usd_gt/lt/eq | decimal | USD volume filters |
| **Synth Mint** | synth_mint_average_slip_gt/lt/eq | decimal | Average slip filters |
| | synth_mint_count_gt/lt/eq | integer | Count filters |
| | synth_mint_fees_gt/lt/eq | integer | Fees filters |
| | synth_mint_volume_gt/lt/eq | integer | Volume filters |
| | synth_mint_volume_usd_gt/lt/eq | decimal | USD volume filters |
| **Synth Redeem** | synth_redeem_average_slip_gt/lt/eq | decimal | Average slip filters |
| | synth_redeem_count_gt/lt/eq | integer | Count filters |
| | synth_redeem_fees_gt/lt/eq | integer | Fees filters |
| | synth_redeem_volume_gt/lt/eq | integer | Volume filters |
| | synth_redeem_volume_usd_gt/lt/eq | decimal | USD volume filters |
| **To Asset** | to_asset_average_slip_gt/lt/eq | decimal | Average slip filters |
| | to_asset_count_gt/lt/eq | integer | Count filters |
| | to_asset_fees_gt/lt/eq | integer | Fees filters |
| | to_asset_volume_gt/lt/eq | integer | Volume filters |
| | to_asset_volume_usd_gt/lt/eq | decimal | USD volume filters |
| **To Rune** | to_rune_average_slip_gt/lt/eq | decimal | Average slip filters |
| | to_rune_count_gt/lt/eq | integer | Count filters |
| | to_rune_fees_gt/lt/eq | integer | Fees filters |
| | to_rune_volume_gt/lt/eq | integer | Volume filters |
| | to_rune_volume_usd_gt/lt/eq | decimal | USD volume filters |
| **To Trade** | to_trade_average_slip_gt/lt/eq | decimal | Average slip filters |
| | to_trade_count_gt/lt/eq | integer | Count filters |
| | to_trade_fees_gt/lt/eq | integer | Fees filters |
| | to_trade_volume_gt/lt/eq | integer | Volume filters |
| | to_trade_volume_usd_gt/lt/eq | decimal | USD volume filters |
| **Total Metrics** | total_count_gt/lt/eq | integer | Count filters |
| | total_fees_gt/lt/eq | integer | Fees filters |
| | total_volume_gt/lt/eq | integer | Volume filters |

### 4. 🏊 Rune Pool History (`GET /runepool-history`)

Query historical rune pool data with various filters.

```http
GET /runepool-history?interval=day&units_gt=1000000

Response:
{
  "data": [{
    "id": 1,
    "start_time": "2024-01-01T00:00:00Z",
    "end_time": "2024-01-01T23:59:59Z",
    "count": 150,
    "units": 2000000
  }]
}
```

#### Supported Query Params

| Category | Parameter | Type | Description |
|----------|-----------|------|-------------|
| **Common** | interval | string | Time interval (5min, hour, day, week, month, quarter, year) |
| | limit | integer | Number of records per page |
| | page | integer | Page number |
| | order | string | Sort order (asc/desc) |
| | sort_by | string | Field to sort by |
| | date_range | string | Date range (YYYY-MM-DD,YYYY-MM-DD) |
| | count | integer | Count filter |
| **Units** | units_gt | integer | Greater than filter |
| | units_lt | integer | Less than filter |
| | units_eq | integer | Exact match filter |
| **Count** | count_gt | integer | Greater than filter |
| | count_lt | integer | Less than filter |
| | count_eq | integer | Exact match filter |

| Parameter | Description |
|-----------|-------------|
| `interval` | Filter by time interval (5min, hour, day, week, month, quarter, year) |
| `limit` | Number of records to return |
| `page` | Page number for pagination |
| `order` | Sort order (asc/desc) |
| `sort_by` | Field to sort by |
| `date_range` | Date range filter (format: YYYY-MM-DD,YYYY-MM-DD) |
| `count` | Integer count filter |

## 🔧 Implementation Details

### ⚠️ Error Handling
- Comprehensive error types with custom `AppError`
- Detailed error messages with appropriate HTTP status codes
- Consistent error response format

### 💾 Database Operations
- Efficient batch processing with COPY operations
- Transaction management for data integrity
- Connection pooling for better performance

### 📁 Code Organization
- Clear separation of concerns (routes, services, models)
- Modular architecture for better maintainability
- Type-safe database operations with SQLx

### ⚡ Performance Optimization
- Chunked processing for large datasets
- Efficient query building with proper indexes
- Optimized batch operations

### 🔒 Security
- Environment-based configuration
- Input validation
- Database connection security


## 🚀 Getting Started

### 📋 Prerequisites

- Rust (latest stable version)
- PostgreSQL 14 or higher
- Docker (Optional)

### 🔨 Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/midguard-api.git
cd midguard-api
```

2. Create a `.env` file in the project root:
```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/midguard
RUST_LOG=info
PORT=3000
```

3. Set up the database:
```bash
# If using Docker
docker-compose up -d postgres

# Create database and run migrations
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

4. Build and run the server:
```bash
cargo build --release
cargo run
```

The API will be available at `http://localhost:3000`

### 💻 Development Commands

```bash
# Run with hot reload
cargo install cargo-watch
cargo watch -x run

# Check code formatting
cargo fmt --all -- --check

# Run linter
cargo clippy -- -D warnings
```

## 📚 Documentation

API documentation is available at:
- Postman Docs: `https://documenter.getpostman.com/view/27529331/2sAYX9kzJb`

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.



