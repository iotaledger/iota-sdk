#!/bin/bash
set -e
rm -rf tmp && mkdir tmp && cd tmp

# retrieve the latest version of org.iota.iota-wallet from Maven Central
latest_version=$(wget -qO- "https://search.maven.org/solrsearch/select?q=g:%22org.iota%22+AND+a:%22iota-wallet%22&core=gav&rows=1&wt=json" | jq ".response.docs[0].v" -r)

echo Installing Java libraries v$latest_version
curl -SL --progress-bar --fail https://github.com/iotaledger/iota-sdk/releases/download/iota-wallet-java-$latest_version/jniLibs.zip > iota-wallet-java.zip
unzip iota-wallet-java.zip             
rm -rf ../android/src/main/jniLibs
cp -rv jniLibs ../android/src/main
curl -SL --progress-bar --fail https://github.com/iotaledger/iota-sdk/releases/download/iota-wallet-java-$latest_version/iota-wallet-$latest_version.jar > iota-wallet.jar
rm -rf ../android/libs && mkdir -p ../android/libs             
cp -rv iota-wallet.jar ../android/libs

echo Installing Swift libraries
# retrieve the latest version and tag from latest the release made
latest_tag=$(git tag | sort -rV | grep swift | head -n1)
latest_version=$(echo $latest_tag | grep -o [0-9\.] | tr -d '\n')
curl -SL --progress-bar --fail https://github.com/iotaledger/iota-sdk/releases/download/$latest_tag/WalletFramework-$latest_version.zip > iota-wallet-swift.zip
unzip iota-wallet-swift.zip             
rm -f ../ios/WalletFramework.xcframework/ios-x86/WalletFramework.framework/WalletFramework
rm -f ../ios/WalletFramework.xcframework/ios-arm64/WalletFramework.framework/WalletFramework
cp -v x86_64-apple-ios/libiota_wallet.a ../ios/WalletFramework.xcframework/ios-x86/WalletFramework.framework/WalletFramework
cp -v aarch64-apple-ios/libiota_wallet.a ../ios/WalletFramework.xcframework/ios-arm64/WalletFramework.framework/WalletFramework
cd .. && rm -rf tmp