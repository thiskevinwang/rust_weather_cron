#! /bin/sh

set -e

echo "<<< Deploying >>>"
echo ""

ZIP_FILE='rust.zip'

echo "<<< Buil binary>>>"
cargo build --release --target x86_64-unknown-linux-musl
echo "  ✅"

echo "<<< Zip up binary for aws-cdk >>>"
zip -j $ZIP_FILE ./target/x86_64-unknown-linux-musl/release/bootstrap
echo "  ✅"

echo "<<< Deploy via aws-cdk >>>"
cdk deploy
echo "  ✅"

echo "<<< Remove zip file >>>"
rm $ZIP_FILE
echo "  ✅"