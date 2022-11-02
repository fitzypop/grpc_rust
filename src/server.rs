use tonic::{transport::Server, Request, Response, Status};

use todo::todo_server::{Todo, TodoServer};
use todo::{CreateTodoRequest, TodoResponse};

mod todo {
    include!("todo.rs");
}

#[derive(Debug, Default)]
struct TodoService {}

#[tonic::async_trait]
impl Todo for TodoService {
    async fn create_item(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<TodoResponse>, Status> {
        let _todo_req = request.into_inner();
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let todo_service = TodoService::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .build()
        .unwrap();

    Server::builder()
        .add_service(TodoServer::new(todo_service))
        .add_service(reflection_service)
        .serve(addr)
        .await?;

    Ok(())
}
