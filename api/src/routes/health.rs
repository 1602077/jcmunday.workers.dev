use crate::startup::Application;

pub fn health(
    _req: worker::Request,
    _ctx: worker::RouteContext<Application>,
) -> Result<worker::Response, worker::Error> {
    worker::Response::ok("hello jack")
}
