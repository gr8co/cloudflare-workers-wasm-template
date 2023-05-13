use wasm_bindgen::prelude::*;
use worker::*;

#[wasm_bindgen(module = "/src/lib.js")]
extern "C" {
    fn quote() -> String;
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Response::ok(format!("Hello, {}!", quote()))
}
