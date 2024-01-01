This project demonstrates a problem I have in reading resource
metadata of type `NonFungibleGlobalId` in Scrypto. I believe the
problem is caused by a bug in Scrypto which thinks I am asking for
data of type `PublicKey` (`type_id` 9) but I _am_ asking for
`NonFungibleGlobalId` (`type_id` 10) which is also what is in fact in
the metadata.

The project also demonstrates a workaround for the problem, in which
instead of storing the `NonFungibleGlobalId` as such in metadata I
instead store the `ResourceAddress` and the `NonFungibleLocalId`
separately and then re-assemble them manually later.

To see the problem, run the project with
```
scrypto test -- --nocapture
```

(Ignore the three compiler warnings, they are because Rust doesn't
recognize that variables only used inside `info!()` are in use.)

You will get output like this (among everything else):
```
Logs: 6
├─ [INFO ] We will be looking in resource metadata for this nfgid: ResourceAddress(9a4c6318c6318c686701820c6318c6318cf7d75139d5aad5e6318c6318c6):[d8ae76c7ce94a60f254465161b81f39b68aadea7141f45990b083cfb0f]
├─ [INFO ] user_nfgid from metadata is Err(UnexpectedType { expected_type_id: 9, actual_type_id: 10 })
├─ [INFO ] Failed: Could not unwrap Result(_)
├─ [INFO ] (workaround) user_nflid result is Ok(Some([d8ae76c7ce94a60f254465161b81f39b68aadea7141f45990b083cfb0f]))
├─ [INFO ] (workaround) user_res result is Ok(Some(Address(9a4c6318c6318c686701820c6318c6318cf7d75139d5aad5e6318c6318c6)))
└─ [INFO ] (workaround) reconstructed user_nfgid from component parts: ResourceAddress(9a4c6318c6318c686701820c6318c6318cf7d75139d5aad5e6318c6318c6):[d8ae76c7ce94a60f254465161b81f39b68aadea7141f45990b083cfb0f]
```

The first three lines is me trying and failing to retrieve a
`NonFungibleGlobalId`, and the last three lines is the workaround
doing its thing.

The line of code that doesn't work as I would expect it to is this one in `src/lib.rs`:
```
let user_nfgid = resmgr.get_metadata::<String, NonFungibleGlobalId>("user_nfgid".to_string());
```

Instead of returning an `Ok` wrapping the requested `NonFungibleGlobalId`, `get_metadata` returns the `Err` that is displayed in the output above.

_Scryptonight_
