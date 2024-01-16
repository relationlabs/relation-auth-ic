import { ethers } from 'ethers'
import { ethereumAuth } from './EthereumAuth'

const checkMetamaskExist = () => {
  if (typeof window.ethereum !== 'undefined') {
    return
  }
  throw new Error('Metamask is not installed!')
}

export const metamaskAuth = async () => {
  checkMetamaskExist()
  const provider = new ethers.providers.Web3Provider(window.ethereum)
  return ethereumAuth(provider)
}
