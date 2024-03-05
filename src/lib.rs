mod bindings;

pub use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

bindings::export!(Component with_types_in bindings);

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(outparam, Ok(resp));

        let out = body.write().expect("outgoing stream");
        out.blocking_write_and_flush(b"Hello, wasi:http/proxy world!\n")
            .expect("writing response");

        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}
