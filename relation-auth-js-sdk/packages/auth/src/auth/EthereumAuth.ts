import { ethers } from 'ethers'
import { parsePersonalSign } from './signature'
import type { ParsePersonalSignParams } from './signature'

export const ethereumAuth = async (
  provider: ethers.providers.JsonRpcProvider,
  options?: Partial<ParsePersonalSignParams>
) => {
  const accounts = await provider.send('eth_requestAccounts', [])
  if (!accounts || !accounts[0]) throw new Error('There is no account')
  const signer = provider.getSigner()
  if (!signer) throw new Error('getSigner error')
  const message = `${new Date().getTime()}`
  const signature = await signer.signMessage(message)
  const formatted = await parsePersonalSign({
    message,
    signature,
    walletName: 'metamask',
    chainName: 'eth',
    ...options,
  })
  return {
    ...formatted,
    address: accounts[0],
  }
}
