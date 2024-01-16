import { WalletType } from '../dfx/identity'
import { metamaskAuth } from './metamask'
import { parsePersonalSign } from './signature'
import { ethers } from 'ethers'
import { ethereumAuth } from './EthereumAuth'


export type AuthResult = {
    error?: string
    token?: string
}

export const authByMetamask = async (): Promise<AuthResult> => {
    const { identity } = await metamaskAuth()
    const { token } = identity || {}
    return { token }
}

export const authByEthereum = async (
    provider: ethers.providers.JsonRpcProvider
): Promise<AuthResult> => {
    const { identity } = await ethereumAuth(provider)
    const { token } = identity || {}
    return { token }
}

export const authByPersonal = async ({
    message,
    signature,
    walletName,
    chainName,
}: {
    message: string
    signature: string
    walletName: WalletType
    chainName: string
}): Promise<AuthResult> => {
    const { identity } = await parsePersonalSign({
        message,
        signature,
        walletName,
        chainName,
    })
    const { token } = identity || {}
    return { token }
}
