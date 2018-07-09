use rmp_rpc::{Service, Value};

// Our server type
#[derive(Clone)]
pub struct Echo;

// The Service trait defines how the server handles incoming requests and notifications.
impl Service for Echo 
{
    // This is the type of future we send back from `handle_request`. Since we have a response
    // available immediately, there's no need to send back a "genuine" future.
    type RequestFuture = Result<Value, Value>;

    // Define how the server handle requests.
    //
    // This server accept requests with the method "echo".
    // It echoes back the first parameter.
    // If the method is not echo, or if the first parameter is not a string, it returns an error.
    fn handle_request(&mut self, method: &str, params: &[Value]) -> Self::RequestFuture 
    {
        // If the method is not "echo", return an error. Note that we "return" by sending back a
        // message on the return channel. If we wanted to, we could spawn some long-running
        // computation on another thread, and have that computation be in charge of sending back
        // the result.
        // if method != "echo" {
        //     return Err(format!("Unknown method {}", method).into());
        // }
      
        

        match method 
        {
            "sum" => Methods::sum(params[0].as_u64().expect("expected u64"), params[1].as_u64().expect("expected u64")),
            "conc" => Methods::concatenate(params[0].as_str().expect("expected str"), params[1].as_str().expect("expected str")),
            "draw" => Methods::draw(params),
            "import" => Methods::get_filepath(params[0].as_str().expect("expected str")),
            _ => Err("invalid argument".into())
        }
        
    }
        // Take the first parameter, which should be a string, and echo it back
    //     if let Value::String(ref string) = params[0] {
    //         if let Some(text) = string.as_str() {
    //             return Ok(text.into());
    //         }
    //     }
        
    //     // If we reach this point, return an error, that means the first parameter is not a String.
    //     Err("Invalid argument".into())
    // }

    // Define how the server handle notifications.
    //
    // This server just prints the method in the console.
    fn handle_notification(&mut self, method: &str, _: &[Value]) {
        println!("{}", method);
    }
}


// Here declare the functions that are going to be executed on the server
struct Methods;

impl Methods
{
    fn sum(a: u64, b: u64) -> Result<Value, Value>
    {
        let c: Value = Value::from(a + b);
        Ok(c)
    }   

    // Import function to accept filepath TESTING ONLY
    fn get_filepath(path: &str) -> Result<Value, Value>
    {
        let v: Value = Value::from(path);
        Ok(v)
    }

    // Improved import function to accept an array of paths
    fn import_paths(path: &[String]) -> Result<Value, Value>
    {
        // let v: Value = Value::from(path);
        Ok(Value::from(vec![1,2,4]))
    }

    fn concatenate(a: &str, b: &str) -> Result<Value, Value>
    {
        let s: Value = Value::from(a.to_owned() + b);
        Ok(s)
    }

    fn draw(a: &[Value]) -> Result<Value, Value>
    {
        let v: Value = Value::from(a[0].as_str().unwrap());
        Ok(v)
    }
}

