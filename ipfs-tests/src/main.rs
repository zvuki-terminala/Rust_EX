use ipfs_embed::{Config, DefaultParams, Ipfs};
use libipld::DagCbor;
use libipld::store::Store;

#[derive(Clone, DagCbor, Debug, Eq, PartialEq)]
struct Identity {
    id: u64,
    name: String,
    age: u8,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache_size = 10;
    let ipfs = Ipfs::<DefaultParams>::new(Config::new(None, cache_size)).await?;
    ipfs.listen_on("/ip4/0.0.0.0/tcp/0".parse()?).await?;

    let identity = Identity {
        id: 0,
        name: "David Craven".into(),
        age: 26,
    };
    let cid = ipfs.insert(&identity)?;
    let identity2 = ipfs.get(&cid)?;
    assert_eq!(identity, identity2);
    println!("identity cid is {}", cid);

    Ok(())
}
