use k8s_openapi::api::core::v1::Endpoints;
use kube::{
    Error,
    api::{Api},
    Client,
};
use kube::api::ListParams;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::try_default().await?;
    let namespace = std::env::var("NAMESPACE").unwrap_or_else(|_| "default".into());
    let mut args = std::env::args();
    args.next();
    let service_name = args.next().expect("missing service name");

    let api: Api<Endpoints> = Api::namespaced(client, &namespace);
    if let Ok(endpoints)= api.get(&service_name).await {
        for addr in endpoints.subsets {
            println!("{}:{}", addr.addresses[0].ip, addr.ports[0].port)
        }
    }

    Ok(())
}