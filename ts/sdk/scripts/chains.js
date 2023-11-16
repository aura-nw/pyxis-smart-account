'use strict';

const local = {
  rpcEndpoint: 'http://localhost:26657',
  prefix: 'aura',
  denom: 'uaura',
  chainId: 'local-aura',
  broadcastTimeoutMs: 2000,
  broadcastPollIntervalMs: 500,
  // TODO: should calculate from onchain data
  averageBlockTimeMs: 1000,
};

const localDocker = {
  rpcEndpoint: 'http://dev-aurad:26657',
  prefix: 'aura',
  denom: 'uaura',
  chainId: 'local-aura',
  broadcastTimeoutMs: 2000,
  broadcastPollIntervalMs: 500,
  averageBlockTimeMs: 1000,
};

const serenity = {
  rpcEndpoint: 'https://rpc.serenity.aura.network',
  prefix: 'aura',
  denom: 'uaura',
  chainId: 'serenity-testnet-001',
  broadcastTimeoutMs: 5000,
  broadcastPollIntervalMs: 1000,
  averageBlockTimeMs: 1000,
};

const auraTestnet = {
  rpcEndpoint: 'https://rpc.dev.aura.network',
  lcdEndpoint: 'https://lcd.dev.aura.network',
  prefix: 'aura',
  denom: 'utaura',
  chainId: 'aura-testnet-2',
  broadcastTimeoutMs: 5000,
  broadcastPollIntervalMs: 1000,
  averageBlockTimeMs: 1000,
};

const euphoria = {
  rpcEndpoint: 'https://rpc.euphoria.aura.network',
  prefix: 'aura',
  denom: 'ueaura',
  chainId: 'euphoria-2',
  broadcastTimeoutMs: 5000,
  broadcastPollIntervalMs: 1000,
  averageBlockTimeMs: 5000,
};

const xstaxy = {
  rpcEndpoint: process.env.INTERNAL_RPC || 'https://rpc.aura.network',
  prefix: 'aura',
  denom: 'uaura',
  chainId: 'xstaxy-1',
  broadcastTimeoutMs: 10000,
  broadcastPollIntervalMs: 1000,
  averageBlockTimeMs: 5000,
};

let defaultChain = null;
switch (process.env.CHAIN_ID) {
  case 'euphoria':
    defaultChain = euphoria;
    break;
  case 'serenity':
    defaultChain = serenity;
    break;
  case 'local-docker':
    defaultChain = localDocker;
    break;
  case 'aura-testnet-2':
    defaultChain = auraTestnet;
    break;
  case 'xstaxy-1':
    defaultChain = xstaxy;
    break;
  default:
    defaultChain = local;
    break;
}

defaultChain.mnemonic = process.env.MNEMONIC
  || 'notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius';

module.exports = {
  local,
  serenity,
  euphoria,
  auraTestnet,
  defaultChain,
};
