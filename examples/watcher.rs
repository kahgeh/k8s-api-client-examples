use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use kube::{Client, Error};
use k8s_openapi::api::core::v1::{Endpoints};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::try_default().await?;
    let namespace = std::env::var("NAMESPACE").unwrap_or_else(|_| "default".into());
    let mut args = std::env::args();
    args.next();
    let service_name = args.next().expect("missing service name");

    let api: Api<Endpoints> = Api::namespaced(client, &namespace);
    let lp = ListParams::default()
        .fields(&format!("metadata.name={}", service_name))
        .timeout(10);

    let mut stream= api.watch(&lp, "0").await?.boxed();
    while let Some(status) = stream.try_next().await? {
        match status {
            WatchEvent::Added(o) => println!("Added {}", o.name()),
            WatchEvent::Modified(o) => {
                println!("Modified: {} ", o.name());
            }
            WatchEvent::Deleted(o) => println!("Deleted {}", o.name()),
            WatchEvent::Error(e) => println!("Error {}", e.message),
            _ => {}
        }
    }

    Ok(())
}