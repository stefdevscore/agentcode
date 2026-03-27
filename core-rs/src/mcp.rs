use crate::budgeter::Budget;
use crate::map::{build_context_map, serialize_map};
use anyhow::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct GetMapParams {
    #[serde(default = "default_path")]
    path: PathBuf,
    #[serde(default = "default_budget")]
    budget: BudgetParam,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum BudgetParam {
    String(String),
    Integer(usize),
}

impl BudgetParam {
    fn parse(&self) -> Result<Budget, String> {
        match self {
            Self::String(value) => Budget::parse(value),
            Self::Integer(value) => Budget::parse(&value.to_string()),
        }
    }
}

fn default_path() -> PathBuf {
    PathBuf::from(".")
}

fn default_budget() -> BudgetParam {
    BudgetParam::Integer(32000)
}

pub fn run_server() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();

    for line in stdin.lock().lines() {
        let line = line?;
        if let Some(response) = handle_request_line(&line) {
            writeln!(stdout, "{response}")?;
            stdout.flush()?;
        }
    }

    Ok(())
}

pub fn handle_request_line(line: &str) -> Option<String> {
    match serde_json::from_str::<JsonRpcRequest>(line) {
        Ok(request) => handle_request(request),
        Err(error) => Some(error_response(
            None,
            -32700,
            format!("invalid JSON: {error}"),
        )),
    }
}

fn handle_request(request: JsonRpcRequest) -> Option<String> {
    let id = request.id.clone();
    let is_notification = id.is_none();
    match request.method.as_str() {
        "get_map" => {
            let params_value = request.params.unwrap_or_else(|| json!({}));
            let params = match serde_json::from_value::<GetMapParams>(params_value) {
                Ok(params) => params,
                Err(error) => {
                    return if is_notification {
                        None
                    } else {
                        Some(error_response(
                            id,
                            -32602,
                            format!("invalid params: {error}"),
                        ))
                    };
                }
            };

            let budget = match params.budget.parse() {
                Ok(budget) => budget,
                Err(error) => {
                    return if is_notification {
                        None
                    } else {
                        Some(error_response(id, -32602, error))
                    };
                }
            };

            match build_context_map(&params.path, budget)
                .and_then(|output| serialize_map(&output.pruned_map))
            {
                Ok(map) => {
                    id.map(|response_id| success_response(Some(response_id), json!({ "map": map })))
                }
                Err(error) => {
                    if is_notification {
                        None
                    } else {
                        Some(error_response(id, -32000, error.to_string()))
                    }
                }
            }
        }
        _ => {
            if is_notification {
                None
            } else {
                Some(error_response(
                    id,
                    -32601,
                    format!("unknown method: {}", request.method),
                ))
            }
        }
    }
}

fn success_response(id: Option<Value>, result: Value) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    })
    .to_string()
}

fn error_response(id: Option<Value>, code: i64, message: String) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message,
        }
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::handle_request_line;
    use serde_json::Value;

    #[test]
    fn returns_error_for_unknown_method() {
        let response: Value = serde_json::from_str(
            &handle_request_line(r#"{"jsonrpc":"2.0","id":1,"method":"nope"}"#)
                .as_deref()
                .unwrap(),
        )
        .unwrap();
        assert_eq!(response["error"]["code"], -32601);
    }

    #[test]
    fn returns_error_for_invalid_json() {
        let response: Value =
            serde_json::from_str(handle_request_line("{not-json").as_deref().unwrap()).unwrap();
        assert_eq!(response["error"]["code"], -32700);
    }

    #[test]
    fn returns_error_for_invalid_budget_param() {
        let response: Value = serde_json::from_str(
            &handle_request_line(
                r#"{"jsonrpc":"2.0","id":1,"method":"get_map","params":{"budget":"bad"}}"#,
            )
            .as_deref()
            .unwrap(),
        )
        .unwrap();
        assert_eq!(response["error"]["code"], -32602);
    }

    #[test]
    fn notification_request_produces_no_response() {
        assert!(handle_request_line(
            r#"{"jsonrpc":"2.0","method":"get_map","params":{"path":"."}}"#
        )
        .is_none());
    }
}
