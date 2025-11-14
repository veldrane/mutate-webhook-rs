use base64::{engine::general_purpose, Engine as _};
use k8s_openapi::api::core::v1::Pod;
//use kube::api::core::v1::Pod;
use crate::{app::Container, prelude::*};


use poem::{handler, web::Json, Result, http::StatusCode};
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
    pub fn empty(uid: &str) -> Self {
        AdmissionResponse {
            uid: uid.to_string(),
            allowed: true,
            patch: None,
            patch_type: None,
        }
    }

    pub fn with_patch(self, patch: &Vec<Value>) -> Self {

        let b64 = match serde_json::to_vec(&patch) {
            Ok(b) => general_purpose::STANDARD.encode(b),
            Err(_) => return Self::empty(&self.uid)
        };

        AdmissionResponse {
            uid: self.uid,
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


pub async fn is_annotated(pod: &Pod, log: Arc<Logger>) -> bool {

    match pod.metadata.annotations.as_ref() {
        Some(annotations) => {
            if let Some(value) = annotations.get("syscallx86.com/container-port-injector") {
                if value == "true" {
                    log.info("Pod is annotated to inject to ports".to_string()).await;
                    return true;
                }
            }
            log.info("Pod is annotated to skip injection.".to_string()).await;
            false
        },
        None => {
            log.info("Pod has no annotation!".to_string()).await;
            false
        }
    }
}

pub async fn build_patch(c: &Container, pod: &Pod, log: Arc<Logger>) -> Option<Vec<Value>> {
    
    log.info("Building patch...".to_string()).await;

    let spec = pod.spec.as_ref()?;

    // heuristika: name nebo image obsahuje envoy / dataplane -> zmenit
    let idx = spec.containers.iter().position(|c| {
        c.name == "simple-api"
    })?;

    let container = &spec.containers[idx];


    // když už port existuje, nic nepatchujeme
    if let Some(ports) = &container.ports {
        if ports.iter().any(|p| p.container_port as u16 == c.port_number) {
            log.info(format!("Port {} already exists in container {}", c.port_name, c.port_number)).await;
            return None;
        }

        // ports existují → přidáme nový záznam na konec
        let path = format!("/spec/containers/{}/ports/-", idx);
        let op = json!({
            "op": "add",
            "path": path,
            "value": {
                "name": c.port_name,
                "containerPort": c.port_number,
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
                    "name": c.port_name,
                    "containerPort": c.port_number,
                    "protocol": "TCP"
                }
            ]
        });

        Some(vec![op])
    }
}

#[handler]
pub async fn mutate(state: Data<&AppState>, body: Body) -> Result<Json<AdmissionReviewResponse>> {

    let AppState { log, container_properties, .. } = *state;

    
    let data = match body.into_bytes().await {
        Ok(data) => data,
        Err(_) => {
            log.error("Failed to read request body".to_string()).await;
            return Err(StatusCode::BAD_REQUEST.into());
        }
    };

    let review: AdmissionReviewRequest = match serde_json::from_slice(&data) {
        Ok(review) => review,
        Err(_) => {
            log.error("Failed to parse AdmissionReviewRequest".to_string()).await;
            return Err(StatusCode::BAD_REQUEST.into());
        }
    };

    let uid = &review.request.uid;
    let pod = &review.request.object;

    if is_annotated(pod, log.clone()).await == false {
        return Ok(Json(AdmissionResponse::empty(uid).to_review()));
    }

    let patch_ops = build_patch(container_properties, &pod, log.clone()).await;
    let patch = match patch_ops {
        Some(ref ops) => ops,
        None => {
            log.info("No patch needed".to_string()).await;
            return Ok(Json(AdmissionResponse::empty(uid).to_review()));
        }
    };

    let response = AdmissionResponse::empty(&uid)
        .with_patch(patch).to_review();

    Ok(Json(response))
}