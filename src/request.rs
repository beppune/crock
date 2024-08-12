
#[derive(Debug)]
pub enum HttpRequest {
    GET(String)
}

impl HttpRequest {

    pub fn deserialize(message:&str) -> Option<Self> {
        if message.is_empty() {
            return None;
        }


        Some(Self::GET(message.to_string()))
    }

}

