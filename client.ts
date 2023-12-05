import {
    clusterApiUrl,
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction
} from "@solana/web3.js";
import {sendTransaction} from "./utils/nft-functions";
import * as borsh from 'borsh';


const connection = new Connection(clusterApiUrl('devnet'));
const keypair = Keypair.fromSecretKey(Uint8Array.from([]))
const programId = new PublicKey('6imjKDztdh6YNmNVqWnj7zafHxV54gim4VRNYSbnEGWn');

const initializeIxSchema = {
    struct: {
        goal_amount: 'u64',
        extra_seed: 'u8'
    }
}
const fundraiserAccountSchema = {
    struct: {
        is_initialized: 'bool', // 1
        initializer_pubkey: {array: {type: 'u8', len: 32}}, // 32
        funds_holder_account_pubkey: {array: {type: 'u8', len: 32}}, // 32
        goal_amount: 'u64' // 8
    }
};

const initializerAddress = keypair.publicKey;
async function initializeAccount() {
    const [stateAccountAddress, _] = PublicKey.findProgramAddressSync([
        Buffer.from("fundraiser"),
        initializerAddress.toBytes()
    ], programId);

    const [balancePdaAddress, __] = PublicKey.findProgramAddressSync([
        Buffer.from("balance"),
        stateAccountAddress.toBytes()
    ], programId);
    // let initializer_account = next_account_info(iter)?;
    // let balance_pda_account = next_account_info(iter)?;
    // let fundraser_state_account = next_account_info(iter)?;
    // let system_program = next_account_info(iter)?;

    let ixData = borsh.serialize(initializeIxSchema, {
        goal_amount: 12345,
        extra_seed: 1,
    });
    const dataArr = Array.from(ixData);
    dataArr.unshift(0);

    console.log(dataArr);

    // const lamports = await connection.getMinimumBalanceForRentExemption(73);

    // const createAccIx = SystemProgram.createAccount({
    //     programId,
    //     newAccountPubkey: stateAccountAddress,
    //     lamports: lamports,
    //     space: 73,
    //     fromPubkey: keypair.publicKey
    // });

    const ix = new TransactionInstruction({
        programId,
        keys: [
            {
                pubkey: initializerAddress,
                isWritable: false,
                isSigner: false,
            },
            {
                pubkey: balancePdaAddress,
                isSigner: false,
                isWritable: false
            },
            {
                pubkey: stateAccountAddress,
                isSigner: false,
                isWritable: true
            },
            {
                pubkey: SystemProgram.programId,
                isWritable: false,
                isSigner: false
            }
        ],
        data: Buffer.from(dataArr)
    });

    const tx = new Transaction();
    tx.add(ix);

    tx.feePayer = initializerAddress;
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

    tx.sign(keypair);

    const txId = await connection.sendRawTransaction(tx.serialize(), {skipPreflight: true})

    console.log('txId: ', txId);
}













async function main(){
    const fundraiserAccount = Keypair.generate();
    const createFundraiserAccIx = SystemProgram.createAccount({
        newAccountPubkey: fundraiserAccount.publicKey,
        space: 73,
        lamports: await connection.getMinimumBalanceForRentExemption(73),
        fromPubkey: keypair.publicKey,
        programId: programId
    });

    const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from("funder"),fundraiserAccount.publicKey.toBuffer()], programId);

    const value = {goal_amount: 1000};

    const data = Array.from(borsh.serialize(initializeIxSchema, value));
    data.unshift(0);

    const initializeFundraiserAccIx = new TransactionInstruction({
        programId,
        keys: [
            {
                pubkey: keypair.publicKey,
                isWritable: true,
                isSigner: true
            },
            {
                pubkey: pda,
                isSigner: false,
                isWritable: true
            },
            {
                pubkey: fundraiserAccount.publicKey,
                isWritable: true,
                isSigner: false
            },
            {
                pubkey: SystemProgram.programId,
                isWritable: false,
                isSigner: false,
            }
        ],
        data: Buffer.from(data)
    })

    const txId = await sendTransaction([createFundraiserAccIx, initializeFundraiserAccIx],connection, [keypair, fundraiserAccount])



    console.log('txId: ', txId);
}

async function closeFundraiser(fundraiserAccount: PublicKey){
    const [pda, bump] = PublicKey.findProgramAddressSync([Buffer.from("funder"),fundraiserAccount.toBuffer()], programId);

    const ix = new TransactionInstruction({
        programId,
        keys: [
            {
                pubkey: keypair.publicKey,
                isWritable: true,
                isSigner: true,
            },
            {
                pubkey: pda,
                isSigner: false,
                isWritable: true,
            },
            {
                pubkey: fundraiserAccount,
                isWritable: true,
                isSigner: false,
            },
            {
                pubkey: SystemProgram.programId,
                isWritable: false,
                isSigner: false
            }
        ],
        data: Buffer.from([2])
    });

    const txId = await sendTransaction([ix], connection, [keypair])
    console.log('txId: ', txId)

}

// main();

async function fetchAcc(){
    const accountInfo = await connection.getParsedAccountInfo(new PublicKey('EPeXqJXz8LAXRcVgnEDbm3paVc6ZJVL4ahZXYDaP4eRm'), {});

    const data = accountInfo.value.data as Buffer;

    console.log(data);

    const res = borsh.deserialize(fundraiserAccountSchema, Uint8Array.from(Array.from(data)));

    console.log(res)

    const p = new PublicKey(Buffer.from([
        105,  60, 219, 239, 127, 170,  61,  50,
        186, 169, 249, 102,  76, 214, 233,  24,
        10, 144,  53, 145,  85, 101,  96,   1,
        255, 152, 159, 120,  89,  57, 236, 113
    ]));
    console.log(p.toBase58());

}

// closeFundraiser(new PublicKey('DZi8goB4B1d8G2k7EArqWRXMvcwip8MuT1h7miRgfKUU'))
// fetchAcc()

// main()

initializeAccount()