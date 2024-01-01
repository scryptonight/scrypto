use scrypto::prelude::*;

#[blueprint]
mod test_metadata {
    struct TestMetadata {
    }
    
    impl TestMetadata {
        pub fn test_metadata(resource: ResourceAddress, expected_nfgid: NonFungibleGlobalId) {
            info!("We will be looking in resource metadata for this nfgid: {:?}", expected_nfgid);

            // We will first try, and fail, to retrieve user_nfgid
            // from resource metadata.
            let resmgr = ResourceManager::from(resource);
            let user_nfgid = resmgr.get_metadata::<String, NonFungibleGlobalId>("user_nfgid".to_string());
            info!("user_nfgid from metadata is {:?}", user_nfgid);
            if let Ok(user_nfgid) = user_nfgid {
                if let Some(user_nfgid) = user_nfgid {
                    // Getting to here would be a success but we never
                    // do.
                    info!("Success: Hooray we have our test_nfgid: {:?}", user_nfgid);
                } else {
                    // We could get to here by sending in a resource
                    // without the "test_nfgid" metadata field set.
                    info!("Failed: Could not unwrap Some(user_nfgid)");
                }
            } else {
                // We end up here because for some reason get_metadata
                // thinks I'm asking for a PublicKey when I want a
                // NonFungibleGlobalId.
                info!("Failed: Could not unwrap Result(_)");
            }

            
            // The following code demonstrates a workaround to pass nf
            // global id by putting resource_address and local_id in
            // metadata separately instead of the global_id as such.
            let nflid = resmgr.get_metadata::<String, NonFungibleLocalId>("user_nflid".to_string());
            info!("(workaround) user_nflid result is {:?}", nflid);
            let addr = resmgr.get_metadata::<String, GlobalAddress>("user_res".to_string());
            info!("(workaround) user_res result is {:?}", addr);
            if let Ok(nflid) = nflid {
                if let Some(nflid) = nflid {
                    if let Ok(addr) = addr {
                        if let Some(addr) = addr {
                            let nfgid = NonFungibleGlobalId::new(ResourceAddress::try_from(addr).unwrap(), nflid);
                            info!("(workaround) the user_nfgid reconstructed from component parts: {:?}", nfgid);
                        }
                    }
                }
            }
        }
    }
}
