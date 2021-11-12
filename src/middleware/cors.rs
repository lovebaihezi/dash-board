// impl <Service, Req> Transform<Service, Req> for Cors {
//     type Response;

//     type Error;

//     type Transform;

//     type InitError;

//     type Future;

//     fn new_transform(&self, service: Service) -> Self::Future {
//         todo!()
//     }
// }

// impl <S, Body> Service<S> for Cors {
//     type Response;

//     type Error;

//     type Future;

//     fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
//         todo!()
//     }

//     fn call(&self, req: S) -> Self::Future {
//         todo!()
//     }
// }
