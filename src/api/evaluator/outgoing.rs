use codes::*;
use serde::{Deserialize, Serialize};

mod codes {
    pub const CREATE_EVALUATOR: u64 = 0x20;
    pub const EVALUATE_REQUEST: u64 = 0x23;
    pub const CLOSE: u64 = 0x22;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEvaluator {
    pub request_id: u64,
    pub allowed_modules: Vec<String>,
    pub client_module_readers: Vec<ClientModuleReader>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientModuleReader {
    pub scheme: String,
    pub has_hierarchical_uris: bool,
    pub is_globbable: bool,
    pub is_local: bool,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum OutgoingMessage {
    CreateEvaluator(CreateEvaluator),
    EvaluateRequest(EvaluateRequest),
    CloseEvaluator(CloseEvaluator),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseEvaluator {
    pub evaluator_id: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateRequest {
    pub request_id: u64,
    pub evaluator_id: i64,
    pub module_uri: String,
}

pub fn get_messagepack_header(msg: &OutgoingMessage) -> u64 {
    match msg {
        OutgoingMessage::CreateEvaluator(_) => CREATE_EVALUATOR,
        OutgoingMessage::EvaluateRequest(_) => EVALUATE_REQUEST,
        OutgoingMessage::CloseEvaluator(_) => CLOSE,
    }
}

pub fn pack_msg(msg: OutgoingMessage) -> Vec<u8> {
    let header = get_messagepack_header(&msg);
    let mut serialized_request = Vec::new();
    let _ = &(header, msg)
        .serialize(
            &mut rmp_serde::Serializer::new(&mut serialized_request)
                .with_struct_map()
                .with_binary(),
        )
        .unwrap();
    serialized_request
}
