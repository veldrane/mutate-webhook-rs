use base64::{engine::general_purpose, Engine as _};
use k8s_openapi::api::core::v1::Pod;
//use kube::api::core::v1::Pod;
use crate::{app::ContainerPorts, prelude::*};


use poem::{handler, web::Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct AdmissionReviewRequest {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub request: AdmissionRequest,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct AdmissionRequest {
    pub uid: String,
    #[serde(rename = "object")]
    pub object: Pod,
    // můžeš si sem přidat i další pole (operation, userInfo, atd.)
}

#[derive(Serialize)]
pub struct AdmissionReviewResponse {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub response: AdmissionResponse,
}

pub trait ToReview {
    fn to_review(self) -> AdmissionReviewResponse;
}


impl AdmissionReviewResponse {
    pub fn empty() -> Self {
        AdmissionReviewResponse {
            api_version: "admission.k8s.io/v1".to_string(),
            kind: "AdmissionReview".to_string(),
            response: AdmissionResponse::empty(),
        }
    }
}

#[derive(Serialize)]
pub struct AdmissionResponse {
    pub uid: String,
    pub allowed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "patchType")]
    pub patch_type: Option<String>,
    // volitelné: status, warnings, auditAnnotations...
}

impl AdmissionResponse {
    pub fn empty() -> Self {
        AdmissionResponse {
            uid: "".to_string(),
            allowed: false,
            patch: None,
            patch_type: None,
        }
    }

    pub fn with_allowed(self, uid: String, patch: &Vec<Value>) -> Self {

        let b64 = match serde_json::to_vec(&patch) {
            Ok(b) => general_purpose::STANDARD.encode(b),
            Err(_) => return Self::empty()
        };

        AdmissionResponse {
            uid,
            allowed: true,
            patch: Some(b64),
            patch_type: Some("JSONPatch".to_string()),
        }
    }
}

impl ToReview for AdmissionResponse {
    fn to_review(self) -> AdmissionReviewResponse {
        AdmissionReviewResponse {
            api_version: "admission.k8s.io/v1".to_string(),
            kind: "AdmissionReview".to_string(),
            response: self,
        }
    }
}


/// Najde Envoy container v Podu a připraví JSONPatch pro přidání metrics portu.
///
/// Vrací `None`, pokud:
/// - Pod nemá spec/containers
/// - nebo se nenašel Envoy container
/// - nebo už ten port existuje
/// 

pub fn build_patch(container_ports: &ContainerPorts, pod: &Pod) -> Option<Vec<Value>> {
    
    let spec = pod.spec.as_ref()?;

    // heuristika: name nebo image obsahuje envoy / dataplane -> zmenit
    let idx = spec.containers.iter().position(|c| {
        c.name == "simple-api"
            || c.name == "simple-api"
            || c
                .image
                .as_deref()
                .unwrap_or("")
                .contains("simple-api")
            || c
                .image
                .as_deref()
                .unwrap_or("")
                .contains("simple-api")
    })?;

    let container = &spec.containers[idx];

    //let desired_port = 9100;
    //let desired_name = "metrics";

    // když už port existuje, nic nepatchujeme
    if let Some(ports) = &container.ports {
        if ports.iter().any(|p| p.container_port == container_ports.port) {
            return None;
        }

        // ports existují → přidáme nový záznam na konec
        let path = format!("/spec/containers/{}/ports/-", idx);
        let op = json!({
            "op": "add",
            "path": path,
            "value": {
                "name": container_ports.name,
                "containerPort": container_ports.port,
                "protocol": "TCP"
            }
        });

        Some(vec![op])
    } else {
        // žádné ports → přidáme celé pole
        let path = format!("/spec/containers/{}/ports", idx);
        let op = json!({
            "op": "add",
            "path": path,
            "value": [
                {
                    "name": container_ports.name,
                    "containerPort": container_ports.port,
                    "protocol": "TCP"
                }
            ]
        });

        Some(vec![op])
    }
}

#[handler]
pub async fn mutate(state: Data<&AppState>, body: Body) -> Json<AdmissionReviewResponse> {

    let AppState { log, container_ports, .. } = *state;

    
    let data = match body.into_bytes().await {
        Ok(data) => data,
        Err(_) => {
            log.error("Failed to read request body".to_string()).await;
            return Json(AdmissionReviewResponse::empty());
        }
    };

    let review: AdmissionReviewRequest = match serde_json::from_slice(&data) {
        Ok(review) => review,
        Err(_) => {
            log.error("Failed to parse AdmissionReviewRequest".to_string()).await;
            return Json(AdmissionReviewResponse::empty());
        }
    };

    let uid = &review.request.uid;
    let pod = &review.request.object;
    let patch_ops = build_patch(container_ports, &pod);

    log.info(format!("Mutate request for pod {}", review.request.object.spec.unwrap_or_default().containers[0].name)).await;

    let patch = match patch_ops {
        Some(ref ops) => ops,
        None => {
            log.info("No patch needed".to_string()).await;
            return Json(AdmissionReviewResponse::empty());
        }
    };

    let response = AdmissionResponse::empty()
        .with_allowed(uid.clone(), patch).to_review();

    Json(response)
}