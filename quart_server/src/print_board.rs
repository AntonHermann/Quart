use std::{
	future::{Future},//, Ready, ok},
	pin::Pin,
	task::{Context, Poll},
};
use actix_web::{
	Error,
	dev::{Service, Transform, ServiceRequest, ServiceResponse},
};
use futures::future::{ok, Ready};

pub struct PrintBoard;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for PrintBoard
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PrintBoardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(PrintBoardMiddleware { service })
    }
}


pub struct PrintBoardMiddleware<S> {
	service: S,
}

impl<S,B> Service for PrintBoardMiddleware<S>
	where S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
	type Request = ServiceRequest;
	type Response = ServiceResponse<B>;
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(cx)
	}
	fn call(&mut self, req: ServiceRequest) -> Self::Future {
		println!("Hi from start. You requested: {}", req.path());

		let fut = self.service.call(req);

		Box::pin(async move {
			let res = fut.await?;

			println!("Hi from response");
			Ok(res)
        })
	}
}
