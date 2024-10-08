use super::*;
use k8s_openapi::api::core::v1::Pod;
use kube::api::DeleteParams;
use tracing::info;

impl<T> KubeHandler<T>
where
    T: Repo,
{
    pub async fn inner_stop(&mut self) -> anyhow::Result<()> {
        let pods_api: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);

        let pods = pods_api.list(&ListParams::default()).await?;
        for pod in pods {
            let pod_name = pod.metadata.name.as_ref().unwrap();
            let delete_options = DeleteParams::default();
            pods_api.delete(pod_name, &delete_options).await?;

            info!("Pod {} is deleted.", pod_name);
        }

        Ok(())
    }
}
