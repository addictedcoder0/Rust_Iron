/// simple prog to run the web based GCD program.
/// there is no URL based mapping currently.
extern crate iron;
#[macro_use] extern crate mime;
use iron::prelude::*;
use iron::status;
fn main() {
println!("Serving on http://localhost:3000...");
Iron::new(get_form).http("localhost:3000").unwrap();
}
/*
Iron::new creates a server , and then sets it to 
listening on TCP port 3000 on local machine.

Iron::new(get_form) => server should use the get_form()
to handle every request. 
*/

#[allow(unused_variables)]
fn get_form(request: &mut Request) -> IronResult<Response> {
let mut response = Response::new();
response.set_mut(status::Ok);
response.set_mut(mime!(Text/Html; Charset=Utf8));
//Rust's raw string : syntax 'r'
response.set_mut(r#"
<title>GCD Calculator</title>
<form action="/gcd" method="post">
<input type="text" name="n"/>
<input type="text" name="n"/>
<button type="submit">Compute GCD</button>
</form>
"#);
Ok(response)
}