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
