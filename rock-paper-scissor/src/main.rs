/* Rock Paper Scissor Game */
/* User will play against the computer */
/* User will be asked to enter a number between 1 and 3 */
/* 1 = Rock, 2 = Paper, 3 = Scissor */
/* The computer will randomly generate a number between 1 and 3 */
/* The computer will then compare the user's number to its own */
/* The computer will then determine the winner */
/* The computer will then print the winner */
use rand::Rng;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    // Choice is a number between 1 and 3
    choice: i32,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    println!("1 = Rock, 2 = Paper, 3 = Scissor");
    // Extract some useful info from the request
    let choice = event.payload.choice;
    // Convert choice to number
    let user_choice = match choice {
        1 => 1,
        2 => 2,
        3 => 3,
        _ => 0,
    };
    // Generate a random number between 1 and 3
    let mut rng = rand::thread_rng();
    let computer_choice: i32 = rng.gen_range(1..4);
    // Compare the user's choice to the computer's choice
    // If the user's choice is equal to the computer's choice, it's a tie
    let result = match (user_choice, computer_choice) {
        (1, 1) => "It's a tie!",
        (2, 2) => "It's a tie!",
        (3, 3) => "It's a tie!",
        (1, 2) => "The computer wins!",
        (1, 3) => "You win!",
        (2, 1) => "You win!",
        (2, 3) => "The computer wins!",
        (3, 1) => "The computer wins!",
        (3, 2) => "You win!",
        _ => "Invalid input!",
    };

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("You chose {user_choice}, the computer chose {computer_choice}. {result}",),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
