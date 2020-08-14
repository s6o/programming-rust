extern crate iron;
extern crate params;
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
  let mut router = Router::new();

  router.get("/", get_form, "root");
  router.post("/gcd", post_gcd, "gcd");

  println!("Serving on http://localhost:3000 ...");
  Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(request: &mut Request) -> IronResult<Response> {
  use params::{Params, Value};
  let map = request.get_ref::<Params>().unwrap();
  println!("{:?}", map);

  let mut response = Response::new();

  let title = r#"
    <title>GCD Calculator</title>
  "#;

  let prev_gcd = match map.find(&["d"]) {
    None => "",
    Some(&Value::String(ref name)) => name,
    _ => panic!("Missing previous gcd result"),
  };
  let prev_input = match map.find(&["i"]) {
    None => "",
    Some(&Value::String(ref name)) => name,
    _ => panic!("Missing previous gcd result"),
  };
  let mut prev_result = String::new();
  if prev_gcd.len() > 0 && prev_input.len() > 0 {
    prev_result = format!(
      "<p>Previous GCD result for {} was {}</p>\n",
      prev_input, prev_gcd
    );
  };

  let input_form = r#"
    <form action="/gcd" method="post">
        <em>Enter a number into each field</em>
        <p><input type ="text" name="n" autofocus=true/><p/>
        <p><input type ="text" name="n"/><p/>
        <button type="submit">Compute GCD</button>
    </form>
  "#;

  response.set_mut(status::Ok);
  response
    .headers
    .set_raw("content-type", vec![b"text/html; charset=utf8".to_vec()]);
  response.set_mut(format!("{} {} {}", title, prev_result, input_form));

  Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
  let mut response = Response::new();

  let form_data = match request.get_ref::<UrlEncodedBody>() {
    Err(e) => {
      response.set_mut(status::BadRequest);
      response.set_mut(format!("Error parsing form data: {:?}\n", e));
      return Ok(response);
    }
    Ok(map) => map,
  };

  let unparsed_numbers = match form_data.get("n") {
    None => {
      response.set_mut(status::BadRequest);
      response.set_mut(format!("Form data has no 'n' parameter.\n"));
      return Ok(response);
    }
    Some(nums) => nums,
  };

  let mut numbers = Vec::new();
  for unparsed in unparsed_numbers {
    match u64::from_str(&unparsed) {
      Err(_) => {
        response.set_mut(status::BadRequest);
        response.set_mut(format!(
          "Value for 'n' parameter not a number: {:?}\n",
          unparsed
        ));
        return Ok(response);
      }
      Ok(n) => {
        numbers.push(n);
      }
    }
  }

  let mut d = numbers[0];
  for m in &numbers[1..] {
    d = gcd(d, *m);
  }

  response.set_mut(status::MovedPermanently);
  response.set_mut(iron::modifiers::RedirectRaw(format!(
    "/?i={:?}&d={}",
    numbers, d
  )));
  Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n {
      let t = m;
      m = n;
      n = t;
    }
    m = m % n;
  }
  n
}

#[test]
fn test_gcd() {
  assert_eq!(gcd(14, 15), 1);

  assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13), 3 * 11);
}
