#!/bin/bash
set -e
rm -rf tmp && mkdir tmp && cd tmp

echo Installing Java libraries
curl -SL --progress-bar --fail https://github.com/iotaledger/iota-sdk/releases/download/iota-wallet-java-1.0.0-rc.3/jniLibs.zip > iota-wallet-java.zip
unzip iota-wallet-java.zip             
rm -rf ../android/src/main/jniLibs
cp -rv jniLibs ../android/src/main
curl -SL --progress-bar --fail https://github.com/iotaledger/iota-sdk/releases/download/iota-wallet-java-1.0.0-rc.3/iota-wallet-1.0.0-rc.3.jar > iota-wallet.jar
rm -rf ../android/libs && mkdir -p ../android/libs             
cp -rv iota-wallet.jar ../android/libs

# 🛑 temporarily using host https://files.iota.org/firefly/bindings until Swift mobile CI will be done.
echo Installing Swift libraries
curl -SL --progress-bar --fail https://files.iota.org/firefly/bindings/WalletFramework-1.0.0-alpha.0.zip > iota-wallet-swift.zip
unzip iota-wallet-swift.zip             
rm -f ../ios/WalletFramework.xcframework/ios-x86/WalletFramework.framework/WalletFramework
rm -f ../ios/WalletFramework.xcframework/ios-arm64/WalletFramework.framework/WalletFramework
cp -v x86_64-apple-ios/libiota_wallet.a ../ios/WalletFramework.xcframework/ios-x86/WalletFramework.framework/WalletFramework
cp -v aarch64-apple-ios/libiota_wallet.a ../ios/WalletFramework.xcframework/ios-arm64/WalletFramework.framework/WalletFramework
cd .. && rm -rf tmp