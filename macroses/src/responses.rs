#[derive(Debug)]
pub struct Response(usize);

pub fn register_handler(method: &str, path: &str, handler: & dyn Fn() -> Response) {

    println!("Register {:?} with method {} on path {}", handler(), method, path);
}

macro_rules! web {
    (GET $path:literal => $b:block) => {
     register_handler("GET", $path, &|| $b) };
    (POST $path:literal => $b:block) => {
     register_handler("POST", $path, &|| $b) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web() {
        web!(GET "/" => { Response(200) });
        web!(POST "/" => { Response(403) });
    }
}

