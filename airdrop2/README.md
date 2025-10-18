### Airdrop-RS Program
**lib.rs** - Contains instructions which perform the following actions
- Generate a PubKey and store its private key (in JSON byte array format) in a file
- Airdrops 2 SOL to the wallet created above by sending a **request_airdrop** instruction to the solana devnet client
- Transfers the generated wallet's entire balance (- transfer fee) to a user supplied address
- Prompts a user to input the BS58 encoded private key for a wallet they control and then converts it to a JSON byte array format
- Imports the user controlled wallet through its JSON byte array formatted private key
- Mints a completion NFT to the user's wallet by using the wallet to sign an instruction call to the submit_rs function of the Turbin3 Program
