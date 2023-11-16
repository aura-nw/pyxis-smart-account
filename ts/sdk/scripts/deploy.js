const fs = require('fs');
const crypto = require('crypto');
const pako = require('pako');

const { MsgStoreCode } = require('cosmjs-types/cosmwasm/wasm/v1/tx');
const { makeCosmoshubPath } = require('@cosmjs/amino');
const { GasPrice } = require('@cosmjs/stargate');
const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
const { toUtf8 } = require("@cosmjs/encoding");
const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const { calculateFee, SigningStargateClient } = require('@cosmjs/stargate');

const { PubKey } = require("cosmjs-types/cosmos/crypto/secp256k1/keys.js");
const { TxRaw } = require("cosmjs-types/cosmos/tx/v1beta1/tx.js");

const { QueryGenerateAccountRequest, SmartAccount, MsgActivateAccount } = require('@aura-nw/aurajs').aura.smartaccount.v1beta1;
const { ClientFactory } = require('@aura-nw/aurajs').aura;

const _ = require('lodash');

const chainConfig = require('./chains').defaultChain;
const account = { client: null, wallet: null };


const contractInfos = [
  {
    dir: `${process.cwd()}/../../artifacts/sample_plugin_manager.wasm`,
    name: 'plugin-manager',
    codeId: 723,
    contractAddress: 'aura1mjq9u2pteesx4wr4u3ddnxhxcspyz2yk7rt4snq820la0cwpruvs0qkhk8'
  },
  { dir: `${process.cwd()}/../../artifacts/sample_plugin.wasm`, name: 'sample-plugin', codeId: 724 },
  { dir: `${process.cwd()}/../../artifacts/simple_recovery_plugin.wasm`, name: 'simple-recovery-plugin', codeId: 725 },
  { dir: `${process.cwd()}/../../artifacts/pyxis_sm_base.wasm`, name: 'pyxis-sm-base', codeId: 726 },
];

async function setupBlockchainClient(chainConfig, nUsers = 0) {
  const hdPaths = [];
  for (let i = 0; i <= nUsers; i += 1) {
    hdPaths.push(makeCosmoshubPath(i));
  }
  account.wallet = await DirectSecp256k1HdWallet.fromMnemonic(chainConfig.mnemonic, {
    prefix: chainConfig.prefix,
    hdPaths,
  });

  const gasPrice = GasPrice.fromString(`0.025${chainConfig.denom}`);
  account.cosmwasmClient = await SigningCosmWasmClient.connectWithSigner(chainConfig.rpcEndpoint, account.wallet, {
    gasPrice,
    broadcastTimeoutMs: 10000,
    broadcastPollIntervalMs: 500,
  });
  account.client = await SigningStargateClient.connectWithSigner(chainConfig.rpcEndpoint, account.wallet, {
    gasPrice,
    broadcastTimeoutMs: 10000,
    broadcastPollIntervalMs: 500,
  });
}

async function storeCode(client, deployerAddress) {
  const uploadMsgs = contractInfos.map((info) => {
    const wasmFile = fs.readFileSync(info.dir);
    info.checksum = crypto.createHash('sha256').update(wasmFile).digest('hex');
    const compressed = pako.gzip(wasmFile, { level: 9 });
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgStoreCode',
      value: MsgStoreCode.fromPartial({
        sender: deployerAddress,
        wasmByteCode: compressed,
      }),
    };
  });
  const response = await client.signAndBroadcast(deployerAddress, uploadMsgs, 'auto', 'Upload pyxis contracts');
  const storeCodeEvents = response.events.filter((event) => event.type === 'store_code');

  // console.log(`Contracts uploaded to blockchain. StoreCode: ${JSON.stringify(storeCodeEvents, null, 2)}`);
  // map code_id to contractInfos using checksum
  storeCodeEvents.forEach((event) => {
    const codeId = event.attributes.find((attr) => attr.key === 'code_id').value;
    const checksum = event.attributes.find((attr) => attr.key === 'code_checksum').value;
    const info = contractInfos.find((info) => info.checksum === checksum);
    info.codeId = Number.parseInt(codeId, 10);
  });

  console.log('Contract Info: ', JSON.stringify(contractInfos, null, 2));

}

// the same as seeds/00_standard_contracts.js
async function uploadContract() {
  console.log('=========================');
  console.log('Upload contracts');
  console.log('=========================');

  const deployerAddress = (await account.wallet.getAccounts())[0].address;

  await storeCode(account.cosmwasmClient, deployerAddress);
};

async function setupPlugin() {
  console.log('=========================');
  console.log('Setup plugin manager');
  console.log('=========================');

  const deployerAddress = (await account.wallet.getAccounts())[0].address;

  const pluginManagerInfo = contractInfos.find((info) => info.name === 'plugin-manager');
  const funds = 'auto';
  const options = { admin: deployerAddress };
  const initMarketResponse = await account.client.instantiate(
    deployerAddress,
    pluginManagerInfo.codeId,
    {},
    `pyxis plugin manager`,
    funds,
    options,
  );
  console.log('address', initMarketResponse.contractAddress);
  pluginManagerInfo.contractAddress = initMarketResponse.contractAddress;
}

async function getSignData(sm_address) {
  const queryClient = account.client.getQueryClient()
  const accountRaw = await queryClient.auth.account(sm_address)
  const smAccount = SmartAccount.decode(accountRaw.value)

  const nextSignData = {
    chainId: await account.client.getChainId(),
    accountNumber: parseInt(smAccount.accountNumber),
    sequence: parseInt(smAccount.sequence),
  };
  console.log(`Sign data set to: ${JSON.stringify(nextSignData)}`)

  return nextSignData
}

async function setupSmartAccount() {
  const pluginManagerInfo = contractInfos.find((info) => info.name === 'plugin-manager');
  const user = (await account.wallet.getAccounts())[0];

  console.log('Pubkey', user.pubkey);

  const pk = Uint8Array.from(
    PubKey.encode(
      PubKey.fromPartial({
        key: user.pubkey
      }),
    ).finish(),
  )

  const req = QueryGenerateAccountRequest.fromPartial({
    codeId: 726,
    salt: toUtf8('1234'),
    initMsg: toUtf8(JSON.stringify({ plugin_manager_addr: pluginManagerInfo.contractAddress })),
    publicKey: {
      typeUrl: '/cosmos.crypto.secp256k1.PubKey',
      value: pk
    }
  });
  console.log('Req', req);

  // generate smart account address
  const reqBytes = QueryGenerateAccountRequest.encode(req).finish();

  const queryClient = await ClientFactory.createRPCQueryClient({ rpcEndpoint: chainConfig.rpcEndpoint });

  const response = await queryClient.aura.smartaccount.v1beta1.generateAccount(req);
  console.log(response);

  const sm_address = response.address;

  console.log(user.address, sm_address)

  // send some fund to smart account
  await account.client.sendTokens(
    user.address,
    sm_address,
    [{ denom: chainConfig.denom, amount: '1000000' }],
    'auto',
    'Send fund smart account'
  );

  // activate smart account
  const activateMsg = {
    typeUrl: "/aura.smartaccount.v1beta1.MsgActivateAccount",
    value: {
      accountAddress: sm_address,
      codeId: 726,
      salt: toUtf8('1234'),
      initMsg: toUtf8(JSON.stringify({ plugin_manager_addr: pluginManagerInfo.contractAddress })),
      publicKey: {
        typeUrl: '/cosmos.crypto.secp256k1.PubKey',
        value: pk
      }
    }
  };

  const signerData = await getSignData(sm_address);
  console.log('Signer data', signerData);


  account.client.registry.register("/aura.smartaccount.v1beta1.MsgActivateAccount", MsgActivateAccount);
  const signedTx = await account.client.sign(
    user.address,
    [activateMsg],
    calculateFee(400000, '0.025utaura'),
    'activate smart account',
    signerData,
  );

  console.log('Signed tx', signedTx);

  const tx = Uint8Array.from(TxRaw.encode(signedTx).finish());
  const res = await account.client.broadcastTx(tx);
  console.log(res);
}


setupBlockchainClient(chainConfig)
  // .then(() => uploadContract())
  // .then(() => setupPlugin())
  .then(() => setupSmartAccount());
