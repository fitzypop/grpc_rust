use tonic::{transport::Server, Request, Response, Status};

use todo::todo_server::{Todo, TodoServer};
use todo::{CreateTodoRequest, TodoResponse};

mod todo {
    include!("todo.rs");
}

#[derive(Default, Debug)]
struct TodoImpl {}

#[tonic::async_trait]
impl Todo for TodoImpl {
    async fn create_item(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<TodoResponse>, Status> {
        let todo_req = request.into_inner();
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let todo = TodoImpl::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .build()
        .unwrap();

    Server::builder()
        .add_service(TodoServer::new(todo))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
