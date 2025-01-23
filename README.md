Decentralized Betting Smart Contract (Solana)

Overview
This repository contains the backend implementation of a decentralized betting application on the Solana blockchain. The smart contract is written in Rust using the Anchor framework. The application enables users to:

Create bets with specific parameters.
Join bets with cryptocurrency contributions.
Close bets after a specified duration.
Randomly select and announce a winner among participants.
Features
Create Bet:
A unique Bet ID is generated.
Allows specification of bet amount, cryptocurrency type, and duration.
Join Bet:
Validates participant eligibility.
Prevents duplicate participants in the same bet.
End Bet:
Locks the bet after its duration.
Disables further participation.
Announce Winner:
Bet creator can randomly select and announce the winner.
Winner receives the total bet amount minus platform fees.
Price Oracle Integration:
Fetches real-time prices using a reliable price oracle for cross-currency support.
Project Structure
programs/betting-contract/src/: Contains the smart contract implementation in Rust.
tests/: Includes test cases for the contract to ensure functionality.
migrations/: Handles deployment scripts for Solana.
Anchor.toml: Anchor configuration file.
Cargo.toml: Dependencies and build information for Rust.
Setup Instructions
Clone the Repository

bash
Copy
Edit
git clone https://github.com/ItzDhruv/betting-smartcontract-solana-.git
cd solana-betting-smartcontract
Install Dependencies Ensure you have the following installed:

Rust
Solana CLI
Anchor
Install project dependencies:

bash
Copy
Edit
anchor build
Deploy to Devnet Update the Anchor.toml file with your Solana Devnet wallet:

toml
Copy
Edit
[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"
Deploy the program:

bash
Copy
Edit
anchor deploy
Run Tests Ensure the smart contract behaves as expected by running test cases:

bash
Copy
Edit
anchor test
Test Cases
Test cases verify:

Bet creation with valid parameters.
Proper handling of participant entries.
Random selection of a winner and payout distribution.
Integration with a price oracle for cross-currency support.
Images of Test Cases


Test Case : Bet Creation Validation and join bet 
![img alt](https://github.com/ItzDhruv/betting-smartcontract-solana-/blob/548653e476579259e27dcd7ce49c5f821de2dbfc/Screenshot%202025-01-23%20172538.png)


Test Case 3: Announcing the Winner

Future Enhancements
Integrate a frontend for user interaction.
Extend support for additional cryptocurrencies.
Add more complex randomization mechanisms for winner selection.
