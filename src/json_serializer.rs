

pub struct JsonSerializer
{
    string: String,
}

impl JsonSerializer
{
    pub fn new(texture: &str, texture_size: (u8, u8)) -> Self
    {
        let mut new_string: String = String::new();
        new_string.push_str("{\"texture\": {\"1\": \"");
        new_string.push_str(texture);
        new_string.push_str("\"},\"texture_size\": [");
        new_string.push_str(&(texture_size.0).to_string());
        new_string.push_str(",");
        new_string.push_str(&(texture_size.1).to_string());
        new_string.push_str("]}");
        Self
        {
            string: new_string,
        }
    }

    pub fn add_box(&mut self, string: &str)
    {
        self.string += string;
    }
}
