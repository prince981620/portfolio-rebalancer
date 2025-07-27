# Solana DeFi Portfolio Rebalancer

A sophisticated Solana-based smart contract system for automated portfolio rebalancing across multiple DeFi protocols. This program intelligently manages capital allocation based on performance metrics, risk scores, and yield optimization.

## 🌟 Features

- **Automated Portfolio Management**: Intelligent rebalancing based on performance thresholds
- **Multi-Protocol Support**: Integrate with various DeFi protocols (Lending, Liquidity Pools, Staking, etc.)
- **Performance Tracking**: Real-time monitoring of yield rates and volatility scores
- **Risk Management**: Built-in emergency pause and minimum rebalancing intervals
- **Capital Efficiency**: Optimized capital extraction and redistribution algorithms
- **Transparent Ranking**: Performance-based strategy ranking system

## 🏗️ Architecture

The project consists of several key components:

### Core Accounts
- **Portfolio**: Main account managing overall portfolio state and configuration
- **Strategy**: Individual strategy accounts tracking protocol-specific investments

### Key Instructions
- `initialize_portfolio`: Set up a new portfolio with management parameters
- `register_strategy`: Add new investment strategies to the portfolio
- `update_performance`: Update strategy performance metrics
- `extract_capital`: Remove capital from underperforming strategies
- `redistribute_capital`: Reallocate capital to top-performing strategies
- `execute_ranking_cycle`: Run the complete rebalancing algorithm

## 🛠️ Prerequisites

Before setting up the project, ensure you have the following installed:

- **Rust**: Latest stable version
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source ~/.cargo/env
  ```

- **Solana CLI**: Version 1.14.0 or higher
  ```bash
  sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
  ```

- **Anchor CLI**: Version 0.31.1 or higher
  ```bash
  npm install -g @coral-xyz/anchor-cli
  ```

- **Node.js & Yarn**: Latest LTS versions
  ```bash
  # Install Node.js (using nvm recommended)
  curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
  nvm install --lts
  nvm use --lts
  
  # Install Yarn
  npm install -g yarn
  ```

## 🚀 Setup & Installation

### 1. Clone the Repository
```bash
git clone https://github.com/prince981620/portfolio-rebalancer.git
cd portfolio-rebalancer
```

### 2. Install Dependencies
```bash
# Install JavaScript/TypeScript dependencies
yarn install

# Build the Rust program
anchor build
```

### 3. Configure Solana Environment
```bash
# Set to localnet for development
solana config set --url localhost

# Create a new keypair (if you don't have one)
solana-keygen new --outfile ~/.config/solana/id.json

# Start local validator (in a separate terminal)
solana-test-validator
```

### 4. Fund Your Wallet
```bash
# Airdrop SOL for testing
solana airdrop 10
```

### 5. Deploy the Program
```bash
# Deploy to localnet
anchor deploy
```

## 🧪 Testing

The project includes comprehensive tests to ensure functionality:

### Run All Tests
```bash
# Run the test suite
anchor test

# Or run specific test file
yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/simple-working-tests.ts
```

### Test Coverage

The test suite covers:
- ✅ Portfolio initialization
- ✅ Strategy registration
- ✅ Performance updates
- ✅ Capital extraction
- ✅ Capital redistribution
- ✅ Complete ranking cycles
- ✅ Error handling and edge cases

### Example Test Output
```
portfolio-rebalancer-working
  ✅ Portfolio initialization works
  ✅ Strategy registration works
  ✅ Performance update works
  ✅ Capital extraction works
  ✅ Capital redistribution works
  ✅ Complete ranking cycle works
```

## 🎯 Usage Examples

### Initialize a Portfolio
```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PortfolioRebalancer } from "./target/types/portfolio_rebalancer";

const program = anchor.workspace.PortfolioRebalancer as Program<PortfolioRebalancer>;

await program.methods
  .initializePortfolio(
    manager.publicKey,    // Portfolio manager
    25,                   // 25% rebalance threshold
    new anchor.BN(3600)   // 1 hour minimum interval
  )
  .accounts({
    portfolio: portfolioPda,
    manager: manager.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .signers([manager.payer])
  .rpc();
```

### Register a Strategy
```typescript
await program.methods
  .registerStrategy(
    strategyId,
    { lending: {} },              // Protocol type
    new anchor.BN(1000000000)     // Initial balance (1 SOL)
  )
  .accounts({
    portfolio: portfolioPda,
    strategy: strategyPda,
    manager: manager.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .signers([manager.payer])
  .rpc();
```

### Update Strategy Performance
```typescript
await program.methods
  .updatePerformance(
    strategyId,
    new anchor.BN(1200),  // 12% yield rate
    800,                  // 8% volatility score
    new anchor.BN(1100000000)  // Updated balance
  )
  .accounts({
    strategy: strategyPda,
    manager: manager.publicKey,
  })
  .signers([manager.payer])
  .rpc();
```

## 📁 Project Structure

```
portfolio-rebalancer/
├── Anchor.toml                 # Anchor configuration
├── Cargo.toml                  # Rust workspace configuration
├── package.json                # Node.js dependencies
├── tsconfig.json              # TypeScript configuration
│
├── programs/rebalancer/        # Main Solana program
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs             # Program entry point
│       ├── state.rs           # Account structures
│       ├── error.rs           # Custom error types
│       └── instructions/      # Instruction handlers
│           ├── initialize.rs
│           ├── register_strategy.rs
│           ├── update_performance.rs
│           ├── extract_capital.rs
│           ├── redistribute_capital.rs
│           └── execute_ranking.rs
│
├── tests/                     # Test files
│   ├── simple-working-tests.ts
│   └── rebalancer.ts
│
├── target/                    # Build artifacts
│   ├── deploy/               # Deployed program files
│   ├── idl/                  # Interface Definition Language
│   └── types/                # Generated TypeScript types
│
└── migrations/               # Deployment scripts
    └── deploy.ts
```

## 🔧 Configuration

### Anchor.toml Settings
- **Cluster**: Set to `localnet` for development, `devnet` or `mainnet-beta` for production
- **Wallet**: Path to your Solana keypair
- **Test Script**: Configured to run specific test files

### Environment Variables
```bash
# Optional: Set custom RPC endpoint
export ANCHOR_PROVIDER_URL="https://api.devnet.solana.com"

# Optional: Set custom wallet path
export ANCHOR_WALLET="~/.config/solana/id.json"
```

## 🔍 Monitoring & Debugging

### View Program Logs
```bash
# Watch program logs in real-time
solana logs --url localhost
```

### Check Account Data
```bash
# View portfolio account data
solana account <PORTFOLIO_PDA_ADDRESS> --url localhost

# View strategy account data
solana account <STRATEGY_PDA_ADDRESS> --url localhost
```

### Debug Tests
```bash
# Run tests with verbose output
RUST_LOG=debug anchor test
```

## 🚨 Security Considerations

- **Manager Authority**: Only the portfolio manager can execute rebalancing operations
- **Emergency Pause**: Built-in circuit breaker for emergency situations
- **Minimum Intervals**: Prevents excessive rebalancing that could drain resources
- **Threshold Validation**: Rebalancing only occurs when performance deviation exceeds thresholds

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Follow Rust best practices and Anchor patterns
- Write comprehensive tests for new features
- Update documentation for any API changes
- Ensure all tests pass before submitting PRs

## 📜 License

This project is licensed under the ISC License - see the LICENSE file for details.

## 🆘 Troubleshooting

### Common Issues

**1. Program Build Failures**
```bash
# Clean and rebuild
anchor clean
anchor build
```

**2. Test Failures**
```bash
# Ensure local validator is running
solana-test-validator --reset

# Check wallet balance
solana balance
```

**3. Deployment Issues**
```bash
# Verify program ID matches in lib.rs and Anchor.toml
anchor keys list
```

**4. RPC Connection Errors**
```bash
# Check if local validator is running
solana cluster-version --url localhost
```

## 📞 Support

For support and questions:
- Open an issue on GitHub
- Contact: [prince981620@gmail.com]

## 🎉 Acknowledgments

- Built with [Anchor Framework](https://github.com/coral-xyz/anchor)
- Powered by [Solana](https://solana.com)
- Inspired by modern DeFi portfolio management strategies