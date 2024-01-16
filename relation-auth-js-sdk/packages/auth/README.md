# Relation auth js sdk

> JS SDK for relation auth.

## Usage

### `authByPersonal`

```typescript
import {authByPersonal} from '@relationlabs/auth'

const {token, error} = authByPersonal(options)
```

#### options

- `message` - `string`: Current timestamp string (valid within 15 minutes)
- `signature` - `string`: Current timestamp string signed by your wallet
- `walletName` - `string`: Your Wallet Name (`metamask`, `binance`, `polkadot`, or other)
- `chainName` - `string`: Your Chain Name (eth, substrate, or other)

### `authByMetamask`

```typescript
import {authByMetamask} from '@relationlabs/auth'

const {token, error} = authByMetamask()
```

### `authByEthereum`

```typescript
import {authByEthereum} from '@relationlabs/auth'
import {ethers} from 'ethers'

const provider = new ethers.providers.Web3Provider(window.ethereum)
const {token, error} = authByEthereum(provider)
```

#### options

- `provider` - `Provider`: (`ethers.providers.Web3Provider`, `ethers.providers.JsonRpcProvider`, or other)

### Result `AuthResult`

```typescript
interface AuthResult {
    token: string
    error?: string
}
```

## Example

### Metamask

```typescript
import {authByMetamask} from '@relationlabs/auth'

const {token, error} = authByMetamask()
```

### Other ethereum wallets

```typescript
import {authByEthereum} from '@relationlabs/auth'
import {ethers} from 'ethers'

const provider = new ethers.providers.Web3Provider(window.BinanceChain)
const {token, error} = authByEthereum(provider, {
    walletName: 'binance',
    chainName: 'eth',
})
```

### Other wallets

#### Wallet Connect

```typescript
import QRCodeModal from '@walletconnect/qrcode-modal'
import WalletConnect from '@walletconnect/client'
import {authByPersonal} from '@relationlabs/auth';

const connector = new WalletConnect({
    bridge: 'https://bridge.walletconnect.org',
    qrcodeModal: QRCodeModal,
})

if (connector.connected) {
    await connector.killSession()
}
connector.createSession()
const message = `${new Date().getTime()}`
const accounts = connector.accounts

return connector.signPersonalMessage([message, accounts[0]])
const signature = await signPersonalMessage(connector, accounts, message)
const {token, error} = await authByPersonal({
    message,
    signature,
    walletName: 'metamask',
    chainName: 'eth',
})
```

#### Polkadot

```typescript
import {web3Accounts, web3FromSource} from '@polkadot/extension-dapp'

const allAccounts = await web3Accounts()
if (allAccounts.length <= 0) throw new Error('There is no polkadot account')
const account = allAccounts[0]
const injector = await web3FromSource(account.meta.source)
const signRaw = injector?.signer?.signRaw
if (!signRaw) throw new Error('There is no polkadot signer')
const message = `${new Date().getTime()}`
const {signature} = await signRaw({
    address: account.address,
    data: stringToHex(message),
    type: 'bytes',
})
const {token, error} = await authByPersonal({
    message,
    signature,
    walletName: 'polkadot',
    chainName: 'substrate',
})
```
