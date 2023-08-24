#[cfg(test)]
mod tests {
  use std::io::{Error, ErrorKind};

  async fn my_async(url: &str) -> Result<serde_json::Value, Error> {
    let response = reqwest::get(url)
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "could not retrieve error"))?;

    let json_response = response
    .json::<serde_json::Value>()
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "could not decode json"))?;

    Ok(json_response)
  }


  #[tokio::test]
  async fn test_call_async_fn() {
    let api_url = "https://cat-fact.herokuapp.com/facts/";
    let result: Result<serde_json::Value, std::io::Error> = my_async(api_url).await;
    match result {
      Ok(r) => {
        dbg!(r)
      },
      Err(_) => {
        panic!("there was a problem");
      }
    };
  }
}