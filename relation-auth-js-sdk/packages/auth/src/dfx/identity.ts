import { ActorSubclass } from '@dfinity/agent'

export type WalletType =
  | 'internet identity'
  | 'metamask'
  | 'polkadot'
  | 'plug'
  | 'wallet connect'
  | 'binance'
type SignAlgorithm = 'secp256k1' | 'sr25519' | 'ed25519' | 'none'

type GetIdentityParams = {
  walletName: WalletType
  publicKey: string
  algorithm: SignAlgorithm
  signature?: string
  message?: string
  chainName?: string
}

export const getIdentity = async (
  authAcotr: ActorSubclass | null,
  params: GetIdentityParams
) => {
  if (authAcotr) {
    const {
      walletName,
      publicKey,
      signature = '',
      message = '',
      algorithm,
      chainName,
    } = params
    const tokenInput = {
      pk: publicKey,
      algorithm: { [algorithm]: null },
      message,
      wallet_name: walletName,
      decoded_signature: signature,
      chain_name: chainName,
    }
    const tokenRes: any = await authAcotr.auth(tokenInput)
    const identityRes: {
      token?: string
      delegation?: any
    } = {}
    if (tokenRes && tokenRes.Ok) {
      // const token = tokenRes.Ok
      // http.config(token)
      // localStorage.setItem('Identity_Authorization', token)
      identityRes.token = tokenRes.Ok
    }
    if (tokenRes && tokenRes.Err) throw new Error(tokenRes.Err)
    return identityRes
  }
  throw new Error('must create auth actor first!')
}
