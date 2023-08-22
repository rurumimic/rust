use kube::{Client, Api, api::ListParams};
use k8s_openapi::api::core::v1::Pod;
use tracing::info;
use tracing_subscriber;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::try_default().await?;

    let pods: Api<Pod> = Api::default_namespaced(client);
    for p in pods.list(&ListParams::default()).await? {
        info!("Found pod {}", p.metadata.name.unwrap());
    }

    Ok(())
}

/*


kubectl create deployment hello-node --image=registry.k8s.io/e2e-test-images/agnhost:2.39 -- /agnhost netexec --http-port=8080

kubectl get deployments
kubectl get pods

kubectl delete deployment hello-node

*/
