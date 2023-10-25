cargo_component_bindings::generate!();

use bindings::wasi::http::types::{
    Headers, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct HelloWorld;

impl bindings::exports::wasi::http::incoming_handler::Guest for HelloWorld {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        let hdrs = Headers::new(&[]);
        let resp = OutgoingResponse::new(200, &hdrs);
        let body = resp.write().expect("outgoing response");

        ResponseOutparam::set(outparam, Ok(resp));

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(b"Hello, wasi:http/proxy world!")
            .expect("writing response");

        drop(out);
        OutgoingBody::finish(body, None);
    }
}
