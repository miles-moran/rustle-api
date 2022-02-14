use lambda_http::{handler, lambda, Context, IntoResponse, Request, Body};
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
mod solver;
mod reader;

#[derive(Serialize, Deserialize)]
struct Req {
    solution: String
}

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(solve)).await?;
    Ok(())
}

async fn solve(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    let body = request.body();
    let results;
    if let Body::Text(text) = body {
        let parsed: Req = serde_json::from_str(text)?;
        println!("{:?}", parsed.solution);
        let solutions = reader::get_words("/assets/solution-lexicon.json");
        let guesses = reader::get_words("/assets/guess-lexicon.json");
        let solution:&str = &parsed.solution[..];
        results = solver::solve(solution, solutions.clone(), guesses.clone());
        for r in results {
            println!("{:?}", r.word);
        }
    }
    
    Ok(json!({
        "message": "Go Serverless v1.0! Your function executed successfully!"
    }))
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn solve_handles() {
//         let request = Request::default();
//         let expected = json!({
//             "message": "Go Serverless v1.0! Your function executed successfully!"
//         })
//         .into_response();
//         let response = solve(request, Context::default())
//             .await
//             .expect("expected Ok(_) value")
//             .into_response();
//         assert_eq!(response.body(), expected.body())
//     }
// }
