export const idlFactory = ({ IDL }) => {
  const Algorithm = IDL.Variant({
    secp256k1: IDL.Null,
    sr25519: IDL.Null,
    none: IDL.Null,
  })
  const AuthReq = IDL.Record({
    pk: IDL.Text,
    algorithm: Algorithm,
    message: IDL.Text,
    wallet_name: IDL.Text,
    decoded_signature: IDL.Text,
    chain_name: IDL.Text,
  })
  const Result = IDL.Variant({ Ok: IDL.Text, Err: IDL.Text })
  const AuthV2Req = IDL.Record({
    pk: IDL.Text,
    session_key: IDL.Vec(IDL.Nat8),
  })
  const Delegation = IDL.Record({
    pubkey: IDL.Vec(IDL.Nat8),
    targets: IDL.Opt(IDL.Vec(IDL.Principal)),
    expiration: IDL.Nat64,
  })
  const SignedDelegation = IDL.Record({
    signature: IDL.Vec(IDL.Nat8),
    delegation: Delegation,
  })
  const GetDelegationResponse = IDL.Variant({
    no_such_delegation: IDL.Null,
    signed_delegation: SignedDelegation,
  })
  return IDL.Service({
    auth: IDL.Func([AuthReq], [Result], ['query']),
    auth_v2: IDL.Func([AuthV2Req], [GetDelegationResponse], ['query']),
  })
}
export const init = ({ IDL }) => {
  return []
}
