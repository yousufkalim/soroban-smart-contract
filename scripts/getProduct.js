require("dotenv").config()

const { Contract, SorobanRpc, TransactionBuilder, Networks,
    BASE_FEE,
    Keypair, Address,  nativeToScVal,
    xdr} = require("stellar-sdk");

let rpcUrl = process.env.RPC_URL;

let contractAddress = process.env.SMART_CONTRACT;

let params = {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET
}

async function contractInt(functName, values) {
    const kp = Keypair.fromSecret(process.env.ADMIN_SECRET_KEY);
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
                // console.log((result._value[0]._attributes.key._value.toString()))
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
let result =  contractInt('get_product', [nativeToScVal(1, {type: "u32"})])

// console.log(result)
