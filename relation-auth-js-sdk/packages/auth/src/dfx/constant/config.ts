import * as dev from './config.dev'
import * as sim from './config.sim'
import * as prod from './config.prod'

const config: any = {
  development: dev,
  simulation: sim,
  production: prod,
}

const output = config[/* import.meta.env.VITE_NODE_ENV */ 'simulation'] || prod

export default output
