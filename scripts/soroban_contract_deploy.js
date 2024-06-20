const StellarSdk = require('stellar-sdk');
const fs = require('fs');
const  child_process = require('child_process');

function exe(command) {
    console.log(command);
    child_process.execSync(command, { stdio: 'inherit' });
  }
// Configuration
const server = new StellarSdk.SorobanRpc.Server('https://soroban-testnet.stellar.org:443');
const networkPassphrase = StellarSdk.Networks.TESTNET;
const contractWasmPath = '/home/yousuf-hexaa/workspace/soroban-smart-contract/target/wasm32-unknown-unknown/release/marketplace.wasm'; // Update with your WASM file path
const sourceSecretKey = 'SAY6LXJJL6S2AUFYVFIWH7LMPNFIYU5FUVAPYH7IU5QZ74KQN6M722JG'; // Replace with your source account secret key
const sourceKeypair = StellarSdk.Keypair.fromSecret(sourceSecretKey);
const contractWasm = fs.readFileSync(contractWasmPath);


function deploy() {
    exe(`(soroban contract deploy --source-account SAY6LXJJL6S2AUFYVFIWH7LMPNFIYU5FUVAPYH7IU5QZ74KQN6M722JG --network testnet  --wasm ${contractWasmPath} --ignore-checks) > marketplace.wasm.txt`);
}

deploy()
