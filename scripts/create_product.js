const { promisify } = require("util");
// const {  } = require("@stellar/stellar-sdk");
const { exec } = require("child_process");

const { Contract, SorobanRpc, TransactionBuilder, Networks,
    BASE_FEE, 
    Keypair, Address,  nativeToScVal,
    xdr} = require("stellar-sdk");
const execute = promisify(exec);

async function exe(command) {
    let { stdout } = await execute(command, { stdio: "inherit" });
    return stdout;
};

let rpcUrl = "https://soroban-testnet.stellar.org:443"

let contractAddress = 'CAEAXVLXUNPPWWICCCUZEU7CELSUBE6GXDHAE6UBJUGTQ6UQPCOLDEW6'
//let contractAddress = 'CCYZ6YOAPTK4LDC45TXAGPJTFKHY6M6RKY4AXK3NHNTQB7W6FNNVKPHZ'

let params = {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET
}

async function contractInt(functName, values) {
    const kp = Keypair.fromSecret("SAY6LXJJL6S2AUFYVFIWH7LMPNFIYU5FUVAPYH7IU5QZ74KQN6M722JG");
    const caller = kp.publicKey();
    const provider = new SorobanRpc.Server(rpcUrl, { allowHttp: true });
    const sourceAccount = await provider.getAccount(caller);
    const contract = new Contract(contractAddress);
    let buildTx = new TransactionBuilder(sourceAccount, params)
        .addOperation(contract.call(functName, ...values))
        .setTimeout(30)
        .build();
    let prepareTx = await provider.prepareTransaction(buildTx);
    prepareTx.sign(kp);
    try {
        let sendTx = await provider.sendTransaction(prepareTx).catch(function (err) {
            return err;
        });
        if (sendTx.errorResult) {
            throw new Error("Unable to submit transaction");
        }
        if (sendTx.status === "PENDING") {
            let txResponse = await provider.getTransaction(sendTx.hash);
            while (txResponse.status === "NOT_FOUND") {
                txResponse = await provider.getTransaction(sendTx.hash);
                await new Promise((resolve) => setTimeout(resolve, 100));
            }
            if (txResponse.status === "SUCCESS") {
                let result = txResponse.returnValue;
                for (let element of result._value) {
                    console.log(element._attributes.key._value.toString() + ": "+ element._attributes.val._value.toString())
                }
                return result;
            }
        }
    } catch (err) {
        return err;
    }
}

// product_title: String,
//         product_description: String,
//         product_category: String,
//         product_expiry: u64,
//         product_image: String,
//         product_price: i128,
//         product_target: i128,
contractInt('create_product', [nativeToScVal("Product 1", {type: "string"}),nativeToScVal("Description 1", {type: "string"}), nativeToScVal("Category 1", {type: "string"}), nativeToScVal(Math.floor(new Date().getTime()/1000.0)+86400, {type: "u64"}), nativeToScVal("image.png", {type: "string"}), nativeToScVal(1000, {type: "i128"}), nativeToScVal(10, {type: "i128"})])