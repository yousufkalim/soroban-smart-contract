const { promisify } = require("util");
const { exec } = require("child_process");

const { Contract, SorobanRpc, TransactionBuilder, Networks,
    BASE_FEE, 
    Keypair, Address } = require("stellar-sdk");
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


contractInt('initialize', [Address.fromString("GC2EXGMDHRBAOY6NN2K4PCEJSRABSRIMTQQ7Z57IT3MNGXLULANTBK4O").toScVal(), Address.fromString("GC2EXGMDHRBAOY6NN2K4PCEJSRABSRIMTQQ7Z57IT3MNGXLULANTBK4O").toScVal(), Address.fromString("GC2EXGMDHRBAOY6NN2K4PCEJSRABSRIMTQQ7Z57IT3MNGXLULANTBK4O").toScVal(), Address.fromString("GC2EXGMDHRBAOY6NN2K4PCEJSRABSRIMTQQ7Z57IT3MNGXLULANTBK4O").toScVal()])