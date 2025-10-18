import {
    address,
    appendTransactionMessageInstructions,
    assertIsTransactionWithinSizeLimit,
    createKeyPairSignerFromBytes,
    createSolanaRpc,
    createSolanaRpcSubscriptions,
    createTransactionMessage,
    devnet,
    getSignatureFromTransaction,
    pipe,
    sendAndConfirmTransactionFactory,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
    addSignersToTransactionMessage,
    getProgramDerivedAddress,
    generateKeyPairSigner,
    getAddressEncoder
} from "@solana/kit";
import { getInitializeInstruction, getSubmitTsInstruction } from "./clients/js/src/generated/index"
import wallet from "./Turbin3-wallet.json";

const rpc = createSolanaRpc(devnet("https://api.devnet.solana.com"));
const rpcSubscriptions = createSolanaRpcSubscriptions(devnet('ws://api.devnet.solana.com'));

const MPL_CORE_PROGRAM = address("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const PROGRAM_ADDRESS = address("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");
const COLLECTION = address("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2"); //Mint Collection
const SYSTEM_PROGRAM = address("11111111111111111111111111111111");

const keypair = await createKeyPairSignerFromBytes(new Uint8Array(wallet));
// console.log(${keypair.address});

const addressEncoder = getAddressEncoder();
// Create the PDA for enrollment account
const accountSeeds = [Buffer.from("prereqs"), addressEncoder.encode(keypair.address)];
//derived using ascii to text conversion
const authoritySeeds = [Buffer.from("collection"), addressEncoder.encode(COLLECTION)];

const [account, _bump] = await getProgramDerivedAddress({
    programAddress: PROGRAM_ADDRESS,
    seeds: accountSeeds
});

const [authority, bump_] = await getProgramDerivedAddress({
    programAddress: PROGRAM_ADDRESS,
    seeds: authoritySeeds
});


const mintKeyPair = await generateKeyPairSigner();

// Execute the initialize transaction
// const initializeIx = getInitializeInstruction({
//     github: "praise03",
//     user: keypair,
//     account,
//     systemProgram: SYSTEM_PROGRAM
// });
// // Fetch latest blockhash
const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
// const transactionMessageInit = pipe(
//     createTransactionMessage({ version: 0 }),
//     tx => setTransactionMessageFeePayerSigner(keypair, tx),
//     tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
//     tx => appendTransactionMessageInstructions([initializeIx], tx)
// );
// const signedTxInit = await signTransactionMessageWithSigners(transactionMessageInit);
// assertIsTransactionWithinSizeLimit(signedTxInit);
const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
// try {
//     const result = await sendAndConfirmTransaction(signedTxInit,
//         { commitment: 'confirmed', skipPreflight: false }
//     );
//     console.log(result);
//     const signatureInit = getSignatureFromTransaction(signedTxInit);
//     console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signatureInit}?cluster=devnet`);
// } catch (e) {
//     console.error(`Oops, something went wrong: ${e}`);
// }

// Execute the submitTs transaction
const submitIx = getSubmitTsInstruction({
    user: keypair,
    account,
    mint: mintKeyPair,
    collection: COLLECTION,
    authority,
    mplCoreProgram: MPL_CORE_PROGRAM,
    systemProgram: SYSTEM_PROGRAM
});
const transactionMessageSubmit = pipe(
    createTransactionMessage({ version: 0 }),
    tx => setTransactionMessageFeePayerSigner(keypair, tx),
    tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
    tx => appendTransactionMessageInstructions([submitIx], tx),
    tx => addSignersToTransactionMessage([mintKeyPair], tx) // Add mint as additional signer after appending instructions
);
const signedTxSubmit = await signTransactionMessageWithSigners(transactionMessageSubmit);
assertIsTransactionWithinSizeLimit(signedTxSubmit);
try {
    await sendAndConfirmTransaction(signedTxSubmit,
        { commitment: 'confirmed', skipPreflight: false }
    );
    const signatureSubmit = getSignatureFromTransaction(signedTxSubmit);
    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signatureSubmit}?cluster=devnet`);
} catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
}