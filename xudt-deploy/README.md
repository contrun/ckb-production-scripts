# xudt-deploy

See [Deploy contracts · nervosnetwork/ckb-cli Wiki](https://github.com/nervosnetwork/ckb-cli/wiki/Deploy-contracts)

``` sh
ckb-cli deploy gen-txs --deployment-config ./deployment.toml --migration-dir ./migrations --from-address ckt1qyqfw5xpwmjjjyw70uzckwvrarxhxc5qd8dsdww9cd --sign-now --info-file info.json
ckb-cli deploy apply-txs --migration-dir ./migrations --info-file info.json
```
