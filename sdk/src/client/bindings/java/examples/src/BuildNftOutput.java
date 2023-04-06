import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.exceptions.ClientException;
import org.iota.types.Output;
import org.iota.types.UnlockCondition;
import org.iota.types.exceptions.InitializeClientException;
import org.iota.types.ids.NftId;
import org.iota.types.output_builder.NftOutputBuilderParams;
import org.iota.types.secret.GenerateAddressesOptions;
import org.iota.types.secret.MnemonicSecretManager;
import org.iota.types.secret.Range;
import org.iota.types.Feature;

import com.google.gson.*;
import org.apache.commons.codec.binary.Hex;

public class BuildNftOutput {
  public static void main(String[] args) throws ClientException, InitializeClientException {
    // Build the client.
    Client client = new Client(new ClientConfig().withNodes(new String[] {
      "https://api.testnet.shimmer.network"
    }));

    // Configure an NFT output.
    String hexAddress = client.bech32ToHex("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy");
    // NftId needs to be null the first time
    NftId nftId = new NftId("0x0000000000000000000000000000000000000000000000000000000000000000");
    UnlockCondition[] unlockConditions = new UnlockCondition[] {
      new UnlockCondition("{ type: 0, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }")
    };

    // IOTA NFT Standard - IRC27: https://github.com/iotaledger/tips/blob/main/tips/TIP-0027/tip-0027.md
    JsonObject tip27ImmutableMetadata = new JsonObject();
    tip27ImmutableMetadata.addProperty("standard", "IRC27");
    tip27ImmutableMetadata.addProperty("version", "v1.0");
    tip27ImmutableMetadata.addProperty("type", "image/jpeg");
    tip27ImmutableMetadata.addProperty("uri", "https://mywebsite.com/my-nft-files-1.jpeg");
    tip27ImmutableMetadata.addProperty("name", "My NFT #0001");

    Feature[] immutableFeatures = new Feature[] {
      // issuer feature
      new Feature("{ type: 1, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
        // metadata feature
        new Feature("{ type: 2, data: \"0x" + Hex.encodeHexString(tip27ImmutableMetadata.toString().getBytes()) + "\" }")
    };
    Feature[] features = new Feature[] {
      // sender feature
      new Feature("{ type: 0, address: { type: 0, pubKeyHash: \"" + hexAddress + "\" } }"),
        // metadata feature
        new Feature("{ type: 2, data: \"0x" + Hex.encodeHexString("mutable metadata".getBytes()) + "\" }"),
        // tag feature
        new Feature("{ type: 3, tag: \"0x" + Hex.encodeHexString("my tag".getBytes()) + "\" }")
    };

    NftOutputBuilderParams params = new NftOutputBuilderParams()
      .withNftId(nftId)
      .withUnlockConditions(unlockConditions)
      .withImmutableFeatures(immutableFeatures)
      .withFeatures(features);

    // Build the output.
    Output output = client.buildNftOutput(params);

    // Print the output.
    System.out.println(
        new GsonBuilder().setPrettyPrinting().create().toJson(JsonParser.parseString(output.toString()))
    );
  }
}