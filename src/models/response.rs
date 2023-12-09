#[derive(Debug, Clone)]
pub struct Response {
    encoded: String,
    decoded: String
}

impl Response {
    pub fn build() -> Self {
        Self {
            encoded: String::new(),
            decoded: String::new()
        }
    }

    pub fn set_encoded(&mut self, encoded: String) {
        self.encoded = encoded;
    }

    pub fn set_decoded(&mut self, decoded: String) {
        self.decoded = decoded;
    }
}