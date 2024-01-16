import { ethers } from 'ethers'
import { recoverPublicKey } from '@ethersproject/signing-key'
import { arrayify } from '@ethersproject/bytes'
import type { Bytes, SignatureLike } from '@ethersproject/bytes'
import { hashMessage } from 'ethers/lib/utils'
import { createAuthActor } from '../dfx/createActor'
import { getIdentity } from '../dfx/identity'
import type { WalletType } from '../dfx/identity'

export interface ParsePersonalSignParams {
  message: string
  signature: string
  walletName: WalletType
  chainName: string
}

export const parsePersonalSign = async ({
  message,
  signature,
  walletName,
  chainName,
}: ParsePersonalSignParams) => {
  const digest = hashMessage(message)
  const publicKey = recoverPublicKey(arrayify(digest), signature).slice(2)
  signature = signature.slice(2)
  const authActor = await createAuthActor()
  const identity = await getIdentity(authActor, {
    walletName,
    message: message,
    signature,
    algorithm: 'secp256k1',
    publicKey,
    chainName,
  })
  return {
    signature,
    message,
    identity,
  }
}

export const verifyPersonalSignMessage = (
  message: Bytes | string,
  signature: SignatureLike
) => {
  return ethers.utils.verifyMessage(message, signature)
}
