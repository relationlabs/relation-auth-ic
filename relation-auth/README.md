# Relation Auth

## Config

### Generate JWK

```shell
java -jar json-web-key-generator.jar -t EC -c P-256 -u sig -a ES256 -g sha256 -x -p
```

### config jwt

config file: `src/config.rs`

```rust
// jwt exp, default: 7day
pub const TOKEN_EXPIRATION_SECS_DEFAULT: u64 = 1000 * 60 * 24 * 7;
// wallet sign exp, default: 15min
pub const WALLET_VERIFY_EXPIRATION_EPOCH_SECS_DEFAULT: u64 = 1000 * 60 * 15;
```

## deploy

### local

```shell
dfx start --background

dfx deploy --network local
```

### IC

```shell
dfx deploy --network ic
```

## HTTP

HTTP API support(query & update) for IC. dfx canister --network ic --wallet "$(dfx identity --network ic get-wallet)"
update-settings relation_auth --add-controller "$(dfx identity get-principal)"

### Query

#### publicKey

```shell
# query jwk public
http://127.0.0.1:8000/pem?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai
# query pem public
http://127.0.0.1:8000/jwk?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai
```

#### JWT Generate Token

```bash
curl -X POST http://localhost:8000/auth?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai \
 -H 'Content-Type: application/json' \
     -d '{"pk":"04183d8ebb840e82a0aa79b3c9c7d265acec52260877a9778ff3f372ae899e92615c35a66333188294414bea0d70dbafcad723ffcf1ed7cfc6feaeb41fcf4c4519",
     "message":"1705373619921", 
     "decoded_signature":"917da06c782710ddadf44e8874e59fd4fc29bf769ffc76d4308a52babbaadbf77e6d2bea186d3fbb174acf6168620a4bdf2bccf64c0a7111d6d809fbdd311df31b",
      "algorithm":"secp256k1", "wallet_name":"metamask", "chain_name":"eth" }'
```

#### JWT Verify Token

```bash
curl -X POST http://localhost:8000/jwt/verify?canisterId=rrkah-fqaaa-aaaaa-aaaaq-cai \
     -H 'Content-Type: application/json' \
     -d '{"token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJleHAiOjE3MTU0NTM3MzAsImlhdCI6MTcwNTM3MzczMCwic3ViIjoie1wiYWNjb3VudFNvdXJjZVwiOlwiZXRoXCIsXCJhZGRyZXNzXCI6XCIweDg3N2YyNmQ1OTdjMmFlMGVkZWM0NDExZjdjM2M1OWI3MTg2NzZlZDFcIixcInVzZXJQcmluY2lwYWxcIjpcIlwifSJ9.cvg7wUfZ_hbzzdAsHrMF8hZBIZLPJ1xZfiG9uOLeM4BwiuiYJ3wMCwmDzFgPHnwbyOG76rnwkx8qD2sf8v80Zw"}'
```
