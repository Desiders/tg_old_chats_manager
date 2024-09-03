use std::{
    error::Error,
    io::{self, Write as _},
};

use grammers_client::{Client, Config, InitParams, SignInError};
use grammers_session::Session;
use tracing::{debug, info, instrument, trace};

use crate::configs::ClientConfig;

#[instrument(skip_all)]
pub async fn init(config: &ClientConfig) -> Client {
    trace!("Load or create session file");

    let session = Session::load_file_or_create(config.session_file_path())
        .expect("Error while creating or writing to session file");

    debug!("Connect client");

    Client::connect(Config {
        session,
        api_id: config.api_id,
        api_hash: config.api_hash.clone(),
        params: InitParams {
            catch_up: true,
            ..Default::default()
        },
    })
    .await
    .expect("Error during client authorization")
}

async fn sign_out_disconnect_on_err<T, E: Error>(client: &Client, val: Result<T, E>) -> T {
    match val {
        Ok(val) => val,
        Err(err) => {
            debug!("Sign out and disconnect");

            drop(client.sign_out_disconnect().await);

            panic!("Sign out and disconnect: {err}");
        }
    }
}

#[instrument(skip_all)]
pub async fn authorize(client: &Client, config: &ClientConfig) {
    debug!("Check authorization info");

    if client
        .is_authorized()
        .await
        .expect("Error while checking authorization info")
    {
        return;
    }

    info!("Request login code");

    let token = client
        .request_login_code(config.phone_number())
        .await
        .expect("Error while request login code");

    print!("Enter the code you received on your Telegram account: ");
    io::stdout().flush().unwrap();

    let code = {
        let mut code = String::new();
        io::stdin().read_line(&mut code).unwrap();
        code.trim().to_owned().into_boxed_str()
    };

    info!("Client sign in");

    match client.sign_in(&token, &code).await {
        Ok(_) => {}
        Err(SignInError::PasswordRequired(password)) => {
            let input_password = if let Some(password) = config.password() {
                password.to_owned().into_boxed_str()
            } else {
                print!("Enter the password of your 2FA: ");
                io::stdout().flush().unwrap();

                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                password.trim().to_owned().into_boxed_str()
            };

            info!("Client check password");

            sign_out_disconnect_on_err(
                client,
                client.check_password(password, &*input_password).await,
            )
            .await;
        }
        Err(err) => panic!("Error while sign in: {err}"),
    };

    sign_out_disconnect_on_err(
        client,
        client.session().save_to_file(config.session_file_path()),
    )
    .await;
}
