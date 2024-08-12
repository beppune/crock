use std::str;

mod crock {

    pub enum Request {
        GET(String)
    }

    pub impl Request {

        pub parse_raw(buffer:&[u8]) -> Self {
            
            let s = str::from_utf8( &buffer ).unwrap();

            Self::GET("hello")
        }

    }

}
