/// simple prog to run the web based GCD program.

extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use std::str::FromStr;

fn main() {
	let mut router = Router::new();
	router.get("/",get_form,"index");//index won't be used
	router.post("/gcd",post_gcd,"submit");//submit won't be used
println!("Serving on http://localhost:3000...");
Iron::new(router).http("localhost:3000").unwrap();
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

fn post_gcd(request: &mut Request)-> IronResult<Response>{
	let mut response = Response::new();
	let hashmap;

	match request.get_ref::<UrlEncodedBody>(){
		Err(e) => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("Error while parsing the form data {:?} \n",e));
			return Ok(response);
		}
		Ok(map) => {
			hashmap = map;
		}
	}

	let unparsed_numbers;
	match hashmap.get("n"){
		None => {
			response.set_mut(status::BadRequest);
			response.set_mut(format!("form data has no 'n' parameter \n"));
			return Ok(response);
		}
		Some(nums) => {
			unparsed_numbers = nums;
		}
	}

	let mut numbers = Vec::new();
	for unparsed in unparsed_numbers{
		match u32::from_str(&unparsed){
			Err(_)=>{
				response.set_mut(status::BadRequest);
				response.set_mut(mime!(Text/Html; Charset=Utf8));
				response.set_mut(format!("<h3>provided data: {:?} is not a number</h3>",unparsed));
				return Ok(response);
			}
			Ok(n )=>{
				numbers.push(n);
			}
		}	
	}
	let mut d = numbers[0];
	for num in &numbers[1..]{
		d = gcd(d,*num);
	}

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(format!("Gcd of numbers : {:?} is <b>{:?}</b>",numbers,d));
	Ok(response)
}
fn gcd(mut x:u32,mut y:u32) -> u32{
	assert!(x!=0 && y!=0);
	while x!=0{
		if x<y {
			let temp:u32 = x;
			x=y;
			y=temp;
		}
		x = x%y;
	}
	y
}