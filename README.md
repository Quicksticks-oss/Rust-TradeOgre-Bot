# TradeOgre Bot

![trade ogre logo](https://raw.githubusercontent.com/Quicksticks-oss/Rust-TradeOgre-Bot/c1623b631e94bc6f1a7a6a4c9276a329b9e6244f/img/logo.svg)

A crypto trading bot designed for the TradeOgre exchange.

## Introduction
This trading bot is designed to automate trading on the TradeOgre exchange. It utilizes a sophisticated strategy combining a SSL (Support and Stop Loss) hybrid indicator with a slope indicator for efficient and effective trading decisions. Built in Rust, this bot offers exceptional performance and speed. Additionally, it leverages a vector store database for rapid data retrieval and management, ensuring quick response to market changes.

## Features
 * SSL Hybrid Indicator: Combines support and resistance levels with stop-loss functionality for informed trading decisions.
 * Slope Indicator: Helps in determining the trend's direction and strength, enhancing the decision-making process.
 * TradeOgre Exchange Integration: Directly interfaces with TradeOgre's API for seamless trading.
 * Vector Store Database: Utilizes a high-performance database for fast data access and storage.
 * High-Speed Performance: Written in Rust, known for its speed and memory efficiency.
 * Automatic Trading: Automates the trading process, from analysis to execution.

## Getting Started
#### Prerequisites
 * Rust (latest stable version)
 * API keys from TradeOgre
 * Basic understanding of trading and cryptocurrency markets
 * Install the `cargo` package and its dependencies.

Ubuntu:
   ```bash
   sudo apt install cargo
   ```

### Installation
1. Clone the repository: 
   ```bash
   git clone [repository URL]
   ```
2. Navigate to the project directory:
   ```bash
   cd [project directory]
   ```
3. Run install.sh:
    ```bash
    ./install.sh
    ```
4. Usage
    To run the trading bot, run start.sh
    ```bash
    ./start.sh
    ```

### Strategy
This bot uses a combination of SSL hybrid and slope indicators. The SSL hybrid indicator identifies support and resistance levels, while the slope indicator determines the trend direction. The bot enters a trade when both indicators align, ensuring a high probability of success.

### Safety and Security
Please note that trading cryptocurrencies involves significant risk. This bot does not guarantee profits and should be used with caution. Always test trading strategies in a simulated environment before deploying real capital.

### Contribution
Contributions are welcome! Please fork the repository and submit a pull request with your proposed changes.

### License
This project is licensed under MIT - see the LICENSE file for details.







