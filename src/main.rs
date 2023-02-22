mod test {
    tonic::include_proto!("test");
}

use test::{
    test_service_server::{TestService, TestServiceServer},
    HelloRequest, HelloResponse, T1, T2,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct MyTestService {}

type TonicResponse<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl TestService for MyTestService {
    async fn call(&self, request: Request<T1>) -> Result<Response<T2>, Status> {
        let rep = request.into_inner().a;
        let replay = T2 { a: rep as i64 };
        Ok(Response::new(replay))
    }
    async fn greet(
        &self,
        request: Request<test::HelloRequest>,
    ) -> TonicResponse<test::HelloResponse> {
        let name = request.into_inner().name;
        Ok(Response::new(test::HelloResponse {
            message: format!("hello {}", name),
        }))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user = MyTestService::default();
    Server::builder()
        .add_service(TestServiceServer::new(user))
        .serve(addr)
        .await?;
    Ok(())
}
