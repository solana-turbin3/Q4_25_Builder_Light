import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("8S3RYiYWqsiyThvTzEzfWqfQqWa1x76PF1fprJVphEUc");

// Recipient address
// const to = Keypair.generate();
const to = new PublicKey("EfDWrJMpg3ExKSYQZWnCKQrZMFAa3xmuJGzmkiKNov63");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)

        // Get the token account of the toWallet address, and if it does not exist, create it
        const receiver_ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to)
        console.log("Receiver at is: " + receiver_ata.address)

        // Transfer the new token to the "toTokenAccount" we just created
        await transfer(
            connection,
            keypair,
            ata.address,
            receiver_ata.address,
            keypair,
            5000000
        );
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();