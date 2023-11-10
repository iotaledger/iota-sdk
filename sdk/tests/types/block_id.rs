// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use iota_sdk::{
    crypto::signatures::ed25519::PublicKey,
    types::{
        block::{
            output::RentStructure, protocol::ProtocolParameters, rand::bytes::rand_bytes_array, slot::SlotIndex,
            BlockHash, BlockId, SignedBlock, SignedBlockDto,
        },
        TryFromDto,
    },
};
use packable::PackableExt;
use pretty_assertions::assert_eq;

const BLOCK_ID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000";

#[test]
fn debug_impl() {
    assert_eq!(
        format!("{:?}", BlockId::from_str(BLOCK_ID).unwrap()),
        r#"BlockId { id: "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000", slot_index: SlotIndex(0) }"#
    );
}

#[test]
fn from_str_valid() {
    BlockId::from_str(BLOCK_ID).unwrap();
}

#[test]
fn from_to_str() {
    assert_eq!(BLOCK_ID, BlockId::from_str(BLOCK_ID).unwrap().to_string());
}

// Validate that the length of a packed `BlockId` matches the declared `packed_len()`.
#[test]
fn packed_len() {
    let block_id = BlockId::from_str(BLOCK_ID).unwrap();

    assert_eq!(block_id.packed_len(), 36);
    assert_eq!(block_id.pack_to_vec().len(), 36);
}

// Validate that a `unpack` ∘ `pack` round-trip results in the original block id.
#[test]
fn pack_unpack_valid() {
    let block_id = BlockId::from_str(BLOCK_ID).unwrap();
    let packed_block_id = block_id.pack_to_vec();

    assert_eq!(packed_block_id.len(), block_id.packed_len());
    assert_eq!(
        block_id,
        PackableExt::unpack_verified(packed_block_id.as_slice(), &()).unwrap()
    );
}

#[test]
fn memory_layout() {
    let block_hash = BlockHash::new(rand_bytes_array());
    let slot_index = SlotIndex(12345);
    let block_id = block_hash.into_block_id(slot_index);
    assert_eq!(slot_index, block_id.slot_index());
    let memory_layout =
        <[u8; BlockId::LENGTH]>::try_from([block_hash.as_ref(), &slot_index.to_le_bytes()].concat()).unwrap();
    assert_eq!(block_id.as_ref(), memory_layout);
}

// TODO: re-enable below tests when source is updated
fn protocol_parameters() -> ProtocolParameters {
    ProtocolParameters::new(3, "test", "rms", RentStructure::default(), 0, 1695275822, 10, 0).unwrap()
}

// TODO: include this test with fixed test vector in TIP-46
#[test]
#[ignore = "invalid public key in test vector"]
fn basic_block_id_tagged_data_payload() {
    // Test vector from https://github.com/iotaledger/tips/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-tagged-data-payload
    let block_string = std::fs::read_to_string("./tests/types/fixtures/basic_block_tagged_data_payload.json").unwrap();
    let block_json = serde_json::from_str(&block_string).unwrap();

    let block_dto = serde_json::from_value::<SignedBlockDto>(block_json).unwrap();
    let block = SignedBlock::try_from_dto(block_dto).unwrap();
    let block_bytes_hex = prefix_hex::encode(block.pack_to_vec());

    let expected_bytes_hex = "0x03000000000000000000c4df9963d486178633b2eb1845fdecf12ee6c5e789c3cf1f0d0bbb3cee65cb5fb2757e995b5cd700000000000000007d534a464b76373157466e412a643733787626623a3a556f4333606b6471034b000114385d6b66073665247c4e0f17075e5a28015f61454e5b725e11686c2d6d6071382e31730000900000000003746167870000006f354e0077473c7c730974575172383d17721a013103477159793576283c31027e38703a2e1a1e3d50703c1a4e60405f6d7d5c564727012d2b1e585c083d721a533a20774a36417a6d1563291a714c4a66100a712e793428315324705a45673f276f62263b0848264a2409646c182b67565f2c6a6c543d40645108461d3e630f1a611c5a2e64336400000000000000001d7237456c3508712f7c5c4a471b544b43285e571b2b23631a3331164e2f14794f216c783a290e3e5f5e6a13226d603046376c083d0f4a4737644e311d0f234338537555582f3151361060566d371d30083d25503d03620c316a6e42205f3321";
    assert_eq!(block_bytes_hex, expected_bytes_hex);

    let block_id = block.id(&protocol_parameters()).to_string();
    assert_eq!(
        block_id,
        "0x90854936ca5fda332065d882ebb38580d14cd8429ada34c92401051d291a180702000000"
    );
}

// TODO: include this test with fixed test vector in TIP-46
#[test]
#[ignore = "invalid public key in test vector"]
fn basic_block_id_transaction_payload() {
    // Test vector from https://github.com/iotaledger/tips/blob/tip46/tips/TIP-0046/tip-0046.md#basic-block-id-transaction-payload
    let block_string = std::fs::read_to_string("./tests/types/fixtures/basic_block_transaction_payload.json").unwrap();
    let block_json = serde_json::from_str(&block_string).unwrap();

    let block_dto = serde_json::from_value::<SignedBlockDto>(block_json).unwrap();
    let block = SignedBlock::try_from_dto(block_dto).unwrap();
    let block_bytes_hex = prefix_hex::encode(block.pack_to_vec());

    let expected_bytes_hex = "0x03000000000000000000c4df9963d486178633b2eb1845fdecf12ee6c5e789c3cf1f0d0bbb3cee65cb5fb2757e995b5cd70000000000000000554813020e6324372e4b2018284c1403321457351a620a227a2e68201d3254710003580b62530a1a4f2f6a25442448043a0d324c7579291a5c144742314c3a481e667035040e5a29685d615d331e38684c0d1c7a2b7875243c77322048215c5c6740562c1461557474797c602d30626b1056351c464b03672f260c50730f344b1152491451445e18653f3a5155670000c001000001490443ee9f5955c40000000000000200003b27570e163e3c283e744d305a57473c51624e3e4c093865592576054025411920390b7c3900001c0c0c3e6740486b175a333c4a414b295036795b1e67045f6b405b790968750d6352770a72000100470b7b276e6c644b7b3577213c7b3874183d677831596e667702386952432e689720000000000000000000000002000004170000000000000000000000000000010000286a580f29774c5b7b7d0e6a7610303a277b124e6427363f1635161c753f3e5300002b2300000000000000000000000000000100007a0d271a7b347e7e614c5463683b405b357a2b0c594a22340b6a1d1a2d667a2600020000000d2128337477155d5e48225e6c500b2538124611456c7739447031592f1d4d1d55414109320a16182c162c1022713126396c4330505352550608266d3c5a364274525914514475730030295e0e0b1a7e6b367a236348382a6e625147512b381300004459093a134a3b285d3c720c4d5c415204700d0c5c09525b340905233e1a7d61446e174632766b145020080225044d545423625d6a12440e58791f122f3b150e266c4a1429774857602b7c5d3e015313330020025833610e541d74770a047a4d64000000000000000020306836691b10254f3e3e5e6c315b4963433152054d452d1b582f785b3b38645263165b422d662427447346404c7d634c49301c0505231e624b481331544f271c4f7e6c33044a06432b655c607a6e056f2e0d1a122004334258020e7d2a3d42";
    assert_eq!(block_bytes_hex, expected_bytes_hex);

    let block_id = block.id(&protocol_parameters()).to_string();
    assert_eq!(
        block_id,
        "0x209b69c1c332fc3649d941847b549ed97e0d2eae6a5fa45237854affbb2c8c9302000000"
    );
}

#[test]
fn validation_block_id() {
    // Test vector from https://github.com/iotaledger/tips/blob/tip46/tips/TIP-0046/tip-0046.md#validation-block-id
    let block_string = std::fs::read_to_string("./tests/types/fixtures/validation_block.json").unwrap();
    let block_json = serde_json::from_str(&block_string).unwrap();

    let block_dto = serde_json::from_value::<SignedBlockDto>(block_json).unwrap();
    let block = SignedBlock::try_from_dto(block_dto).unwrap();
    let block_bytes_hex = prefix_hex::encode(block.pack_to_vec());

    let expected_bytes_hex = "0x03000000000000000000c4df9963d486178633b2eb1845fdecf12ee6c5e789c3cf1f0d0bbb3cee65cb5fb2757e995b5cd70000000000000000154a1767047449742d2063001f3105661562281d523476623c505f142b111455011c06532d24754f512011512f6b3d522e451c1f65457036383f5f7c3f297342403557411d73085a4f1c5028673403193c2f6c630b18340e1a3a191612011418780b211c250c79204a200d44571c59600f553755491e2d71325e2f1c252772517b4b6d3d78452e1f7e0f663d2603104b6d354f00073677382e553d1c78230820095a2702700b6f466a59520445176d0b1f2b141c2d2c4e1c7d6b3c621c7852331c7d005d541c596d0c4d7c163621471d234a4a6d507514645b184b485f60393f2a216309490a411d17454f604d31570d777250307160194301521801471a315c75100068344b4f277c0f6776766d16140a6908470c4323396152663633191a66261f7323385f326d1b47780a74561473161e496f672b233161200c112d59767458461b1a687a4b40184f7d26234441672d11084646452b603f767b43757d032f4f6a327217211b6e2a68476a303f2814222b604b4c253f0c005817215f1c5f7021263948756f4e37066c230752313b2e5c296972110e78457c6117274a1d2f475f5e1932111115546930017c1f1b2b03440a5e016c4d6b41060f6d3f101f423b2c256b3969755d395e2a7d266c1d7a607b542b2b3929396c42657b1e2a6f7925367937674567645b5318731a5b162f153a655a12542c2c1b4023067c182533573c4f102d0230683d384b4c382c4f4b530974215065122a5c6360300b01792b715a44625c17451c296a36165f331f282b40573f71146116632c5201410a68304e482a4029224275364039686d70180c1a084474354c5845432663502d2e4f355c6f6a375c75465c0e50343c483c4205504457305f65066e6c741b1b347d374b342770151f67453f19694a000f3247651a7b3c0c05101b5e5b3c3768581d4a0e0e20212e017a6640123950425c6b69220e5e0f365b7d43694e7129254f223442670c771b677e2723475f0e5f154e114a276f4f39592c2f255c0631146e0445684c6e732d6d774045071e2d44787c497d3d5d494b20275230617d5377705d64671a7b6a63114f2a54106f1d016d5353734112394a465604592e4e781b41466d556e526a5f654200351e782763452c732d2f4a7336783e27274b5e4e607e06657a433d2e0417461c78185732567c78531b5b670226781b26526813364b72675b62202d1d2d66522e07703d3d767a6b3a330829542b6d454b690c5a4364516c302d43562562797960364210310a0f01223b504a1436553c44765570503514051034062e48310d6a2670094d783979401a4b4f117372101f2f27196649382e113f4671194c27080c2b3e712d287371635333410a7500102e1240131a7a3b1b3a5b7131195706056907214a334f573e69067419161176184c42345c2722236b131d6940061373243b533e5f10712a473a2748195c15000004a19d3fef401f40efc739569dbef84627db1b8af59ee3ef82f9028d00c4d77413007026712f3170262410010e037e6f454243567c7d3a4827060e617b6b0f757064092d0c2e2374337e060819320c3a5b39686334564a560c0b007b2c041f76455677576e310e16683a367c690c2a425c3f1f3d525b201f4c7c41675b443804383c";
    assert_eq!(block_bytes_hex, expected_bytes_hex);

    let block_id = block.id(&protocol_parameters()).to_string();
    assert_eq!(
        block_id,
        "0x566efb97c267bee5195b03c2e42bfb665f82e0535b72005f3cdbb50f3ad0da2702000000"
    );
}
