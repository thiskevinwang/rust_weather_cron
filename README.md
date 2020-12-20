# Deployment

Currently, the deploy process is manual, but does leverage infrastructure-as-code

```bash
# build the binary
cargo build --release --target x86_64-unknown-linux-musl

# zip it up, so that aws-cdk TS code can point to it
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap

# deploy with aws-cdk
cdk deploy
```

# Errors

## TS error

```
❯ cdk synth
TypeError [ERR_UNKNOWN_FILE_EXTENSION]: Unknown file extension ".ts" for /Users/kevin/repos/rust_weather_cron/index.ts
    at Loader.defaultGetFormat [as _getFormat] (internal/modules/esm/get_format.js:65:15)
    at Loader.getFormat (internal/modules/esm/loader.js:116:42)
    at Loader.getModuleJob (internal/modules/esm/loader.js:247:31)
    at Loader.import (internal/modules/esm/loader.js:181:17)
    at Object.loadESM (internal/process/esm_loader.js:84:5)
```

### Fix:

> [Remove `"type": "module"` from package.json](https://github.com/TypeStrong/ts-node/issues/1062#issuecomment-650746948)
