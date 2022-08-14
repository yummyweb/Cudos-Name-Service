## Steps to Deploy

1. ```RES=$(cudos-noded tx wasm store artifacts/cudos_name_service_app-aarch64.wasm --from <account-name> --node http://sentry1.gcp-uscentral1.cudos.org:26657 --chain-id cudos-testnet-public-3 --gas auto --gas-prices 5000000000000acudos --gas-adjustment 1.8 --keyring-backend os -y)```

2. ```CODE_ID=$( echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value' | tee /dev/tty )```

3. ```
INIT = '{}'
cudos-noded tx wasm instantiate $CODE_ID "$INIT" --from <account-name> --label "cudos name service app" --node http://sentry1.gcp-uscentral1.cudos.org:26657 --chain-id cudos-testnet-public-3 --gas auto --gas-prices 5000000000000acudos --gas-adjustment 1.8 --keyring-backend os -y
```

4. ```CONTRACT=$(cudos-noded query wasm list-contract-by-code $CODE_ID --node http://sentry1.gcp-uscentral1.cudos.org:26657 --output json | jq -r '.contracts[-1]' | tee /dev/tty | tail -1 | tr -d '\r')```

5. ```cudos-noded tx wasm execute $CONTRACT "$EXEC" --from <account-name> --node http://sentry1.gcp-uscentral1.cudos.org:26657 --chain-id cudos-testnet-public-3 --gas auto --gas-prices 5000000000000acudos --gas-adjustment 1.8 --keyring-backend os -y```