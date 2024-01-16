import { Actor, HttpAgent, HttpAgentOptions } from '@dfinity/agent'
import dfxCfg from './constant/config'

// @ts-ignore
import { idlFactory as authIdlFactory } from './constant/relation_auth.did.js'

const { AuthCanister, host } = dfxCfg

export const createAuthActor = async ({
  agentOptions = {},
  actorOptions = {},
  local = false,
}: {
  agentOptions?: HttpAgentOptions
  actorOptions?: any
  local?: boolean
} = {}) => {
  const agent = new HttpAgent({ host, ...agentOptions })
  if (local) {
    await agent.fetchRootKey().catch((err) => {
      console.warn(
        'Unable to fetch root key. Check to ensure that your local replica is running'
      )
      console.error(err)
    })
  }
  return Actor.createActor(authIdlFactory, {
    agent,
    canisterId: AuthCanister,
    ...actorOptions,
  })
}
