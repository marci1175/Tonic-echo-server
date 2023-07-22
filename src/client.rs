use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use std::io;
use std::os::windows::thread;
use std::time::Duration;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    loop {
        println!("Enter something");
        let mut input = Default::default();
        io::stdin().read_line(&mut input).expect("failed to readline");
        let request = tonic::Request::new(HelloRequest {
            name: input.trim().into(),
        });
        let response = match client.say_hello(request).await{
            Ok(ok) => {ok},
            Err(_) => {break;}
        };

        println!("{:?}", response);
    }
    println!("Error occured ,bye bye!");
    std::thread::sleep(Duration::from_secs(10));
    Ok(())
}