use topcoat::{Result, context::Cx, router::route};

/// For scripts that wait for the server to start.
#[route(GET)]
async fn healthz(cx: &Cx) -> Result<&'static str> {
    Ok("ok\n")
}
