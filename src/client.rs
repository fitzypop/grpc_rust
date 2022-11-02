// use todo::todo_client::TodoClient;
// use todo::{CreateTodoRequest, TodoResponse};

// pub mod todo {
//     include!("todo.rs");
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = TodoClient::connect("http://[::1]:50051").await?;

//     let request = tonic::Request::new(BtcPaymentRequest {
//         from_addr: "12345".to_owned(),
//         to_addr: "654321".to_owned(),
//         amount: 22,
//     });

//     let response = client.send_payment(request).await?;

//     println!("REPONSE={:?}", response);
//     Ok(())
// }

fn main() {
    todo!()
}
