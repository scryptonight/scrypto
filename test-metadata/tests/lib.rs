use radix_engine_interface::prelude::*;
use scrypto::this_package;
use scrypto_test::prelude::*;
use scrypto_unit::*;

#[test]
fn test_metadata_retrieval() {
    let mut test_runner = TestRunnerBuilder::new().build();
    let (public_key, _private_key, account) = test_runner.new_allocated_account();
    let user_nfgid = NonFungibleGlobalId::from_public_key(&public_key);
    let package_address = test_runner.compile_and_publish(this_package!());

    // Create an NF resource with metadata
    // "user_nfgid" = <NonFungibleGlobalId> user_nfgid
    //
    // We will later try to retrieve this metadata in our Scrypto
    // code, where we will fail to do so.
    //
    // We also add a "user_res" and a "user_nflid" field just to
    // demonstrate a workaround for the problem.
    let manifest = ManifestBuilder::new()
        .create_non_fungible_resource::<Vec<_>, EmptyNonFungibleData>(
            OwnerRole::None,
            NonFungibleIdType::Integer,
            true,
            NonFungibleResourceRoles::single_locked_rule(rule!(allow_all)),
            metadata! {
                init {
                    "user_nfgid" => user_nfgid.clone(), locked;
                    "user_res" => user_nfgid.resource_address(), locked;
                    "user_nflid" => user_nfgid.local_id().clone(), locked;
                }
            },
            None)
        .deposit_batch(account)
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid.clone()],
    );
    receipt.expect_commit_success();
    let nft_resaddr =
        receipt
        .expect_commit(true)
        .new_resource_addresses()[0];

    // Now pass that new NF resource to our Scrypto test method
    let manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "TestMetadata",
            "test_metadata",
            manifest_args!(
                nft_resaddr,
                user_nfgid.clone()),
        )
        .build();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![user_nfgid],
    );

    // I use this command line to be able to see the output:
    // scrypto test -- --nocapture
    println!("{:?}\n", receipt);
}
