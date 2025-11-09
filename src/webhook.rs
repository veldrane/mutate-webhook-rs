use base64::{engine::general_purpose, Engine as _};
use k8s_openapi::api::core::v1::Pod;
//use kube::api::core::v1::Pod;
use crate::prelude::*;


use poem::{handler, web::Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct AdmissionReviewRequest {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
    pub request: AdmissionRequest,
}

#[derive(Debug,Deserialize)]
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

/// Najde Envoy container v Podu a připraví JSONPatch pro přidání metrics portu.
///
/// Vrací `None`, pokud:
/// - Pod nemá spec/containers
/// - nebo se nenašel Envoy container
/// - nebo už ten port existuje
/// 

pub fn build_patch(pod: &Pod) -> Option<Vec<Value>> {
    
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

    let desired_port = 9100;
    let desired_name = "metrics";

    // když už port existuje, nic nepatchujeme
    if let Some(ports) = &container.ports {
        if ports.iter().any(|p| p.container_port == desired_port) {
            return None;
        }

        // ports existují → přidáme nový záznam na konec
        let path = format!("/spec/containers/{}/ports/-", idx);
        let op = json!({
            "op": "add",
            "path": path,
            "value": {
                "name": desired_name,
                "containerPort": desired_port,
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
                    "name": desired_name,
                    "containerPort": desired_port,
                    "protocol": "TCP"
                }
            ]
        });

        Some(vec![op])
    }
}

#[handler]
pub async fn mutate(body: Body) -> Json<AdmissionReviewResponse> {
    let data = match body.into_bytes().await {
        Ok(data) => data,
        Err(_) => {
            return Json(AdmissionReviewResponse {
                api_version: "admission.k8s.io/v1".to_string(),
                kind: "AdmissionReview".to_string(),
                response: AdmissionResponse {
                    uid: "".to_string(),
                    allowed: false,
                    patch: None,
                    patch_type: None,
                },
            });
        }
    };

    let review: AdmissionReviewRequest = match serde_json::from_slice(&data) {
        Ok(review) => review,
        Err(_) => {
            return Json(AdmissionReviewResponse {
                api_version: "admission.k8s.io/v1".to_string(),
                kind: "AdmissionReview".to_string(),
                response: AdmissionResponse {
                    uid: "".to_string(),
                    allowed: false,
                    patch: None,
                    patch_type: None,
                },
            });
        }
    };

    //println!("Received AdmissionReview: {:?}", review);
    //println!("{}", serde_json::to_string(&review.request.object.spec).unwrap_or_else(|_| "Failed to serialize object".to_string()));



    let uid = review.request.uid.clone();
    let pod = review.request.object;

    let patch_ops = build_patch(&pod);

    println!("{}", serde_json::to_string_pretty(&patch_ops).unwrap_or_else(|_| "Failed to serialize patch ops".to_string()));

    let (patch_b64, patch_type) = if let Some(ops) = patch_ops {
        let bytes = serde_json::to_vec(&ops)
            .expect("failed to serialize JSONPatch ops");
        let patch_b64 = general_purpose::STANDARD.encode(bytes);
        (Some(patch_b64), Some("JSONPatch".to_string()))
    } else {
        (None, None)
    };

    let response = AdmissionResponse {
        uid,
        allowed: true,
        patch: patch_b64,
        patch_type,
    };

    let review_response = AdmissionReviewResponse {
        api_version: "admission.k8s.io/v1".to_string(),
        kind: "AdmissionReview".to_string(),
        response,
    };

    Json(review_response)
}