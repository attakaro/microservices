use std::env;

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};
use tokio::time::{sleep, Duration};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::authentication::{SignUpResponse, SignInResponse, SignOutResponse};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or("[::0]".to_owned());

    let mut client = AuthClient::connect(format!("http://{}:50051", auth_hostname)).await?;

    loop {
        let username: String = Uuid::new_v4().to_string(); 
        let password: String = Uuid::new_v4().to_string(); 

        // sign up

        let request: Request<SignUpRequest> = 
            Request::new(SignUpRequest {
                    username: username.clone(), 
                    password: password.clone()
                }
            );

        let response: Response<SignUpResponse> = client.sign_up(request).await?; 

        println!(
            "SIGN UP RESPONSE STATUS: {:?}",
            response.into_inner().status_code
        );

        // sign in

        let request: Request<SignInRequest> = 
            Request::new(SignInRequest {
                    username: username.clone(), 
                    password: password.clone()
                }
            ); 

        let response: SignInResponse = client.sign_in(request).await?.into_inner();

        println!(
            "SIGN IN RESPONSE STATUS: {:?}",
            response.status_code 
        );

        // sign out

        let request: Request<SignOutRequest> = 
            Request::new(SignOutRequest {
                    session_token: response.session_token
                }
            ); 

        let response: Response<SignOutResponse> = client.sign_out(request).await?; 

        println!(
            "SIGN OUT RESPONSE STATUS: {:?}",
            response.into_inner().status_code 
        );

        println!("--------------------------------------",);

        sleep(Duration::from_secs(3)).await;
    }
}