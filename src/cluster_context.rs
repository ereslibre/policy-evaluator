use kube::api::{ListParams, Resource};
use kube::Client;
use std::{collections::HashMap, sync::RwLock};

use k8s_openapi::api::core::v1::{Namespace, Service};
use k8s_openapi::api::networking::v1::Ingress;

use lazy_static::lazy_static;

lazy_static! {
    static ref CLUSTER_CONTEXT: ClusterContext = ClusterContext::default();
}

// ClusterContext represents a structure that can be used to retrieve
// information about a running Kubernetes cluster.
#[derive(Default)]
pub struct ClusterContext {
    context: RwLock<HashMap<String, String>>,
}

impl ClusterContext {
    pub fn get<'a>() -> &'a ClusterContext {
        &CLUSTER_CONTEXT
    }

    pub async fn refresh(&self, kubernetes_client: &Client) -> kube::Result<()> {
        // TODO: ereslibre
        Ok(())
    }
}
