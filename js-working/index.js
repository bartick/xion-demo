import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice, coin } from "@cosmjs/stargate";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import fs from "fs/promises"; // Use Promises-based fs module for better handling.

const rpc = "https://rpc.xion-testnet-1.burnt.com:443";
const mnemonic =
  "void require tower educate mimic resource fence parrot shrug gesture pig tank tell phone kind voice abandon thumb olympic addict lobster earth evolve stomach";

// Create wallet and get the account
const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
  prefix: "xion",
});
const [account] = await wallet.getAccounts();
console.log("Account:", account);

const gasPrice = GasPrice.fromString("0.001uxion"); // Set gas price
const client = await SigningCosmWasmClient.connectWithSigner(rpc, wallet, {
  gasPrice,
});

async function uploadCode() {
  try {
    const wasmFileAdmin = await fs.readFile("./xion_contract.wasm"); // Read the wasm file
    const wasmBytesAdmin = new Uint8Array(wasmFileAdmin);

    const uploadAdmin = await client.upload(
      account.address,
      wasmBytesAdmin,
      "auto"
    );

    const codeId = uploadAdmin.codeId;
    console.log("Code ID:", codeId);
  } catch (error) {
    console.error("Error uploading contract:", error.message);
  }
}

// Call the uploadCode function
await uploadCode();
