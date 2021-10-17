use std::fs::File;
use std::io::Write;

pub struct JsonSerializer
{
    string: String,
}

impl JsonSerializer
{
    pub fn new(texture: &str, texture_size: (u8, u8)) -> Self
    {
        let mut new_string: String = String::new();
        new_string.push_str("{\n\t\"texture\": {\"1\": \"");
        new_string.push_str(texture);
        new_string.push_str("\"},\n\t\"texture_size\": [");
        new_string.push_str(&(texture_size.0).to_string());
        new_string.push_str(", ");
        new_string.push_str(&(texture_size.1).to_string());
        new_string.push_str("],\n\t\"elements\": [\n\t\t");
        Self
        {
            string: new_string,
        }
    }

    pub fn add_box(&mut self, start: (u32, u32, u32), end: (u32, u32, u32))
    {
        self.string.push_str("{\"name\": \"element\", \"from\": [");
        self.string.push_str(&(start.0).to_string());
        self.string.push_str(", ");
        self.string.push_str(&(start.2).to_string());
        self.string.push_str(", ");
        self.string.push_str(&(start.1).to_string());
        self.string.push_str("], \"to\": [");
        self.string.push_str(&(end.0).to_string());
        self.string.push_str(", ");
        self.string.push_str(&(end.2).to_string());
        self.string.push_str(", ");
        self.string.push_str(&(end.1).to_string());
        self.string.push_str("], \"faces\": {\"north\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#1\"}, ");
        self.string.push_str("\"south\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#1\"}, ");
        self.string.push_str("\"east\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#1\"}, ");
        self.string.push_str("\"west\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#1\"}, ");
        self.string.push_str("\"up\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#1\"}, ");
        self.string.push_str("\"down\": {\"uv\": [0, 0, 16, 16], \"texture\": \"#1\"}}},\n\t\t");
    }

    pub fn write_file(&mut self, filepath: &str) -> std::io::Result<()>
    {
        self.string.pop();
        self.string.pop();
        self.string.pop();
        self.string.pop();
        self.string.push_str("\n\t]\n}");

        let mut file: File = File::create(filepath)?;
        file.write(&self.string.clone().into_bytes())?;
        Ok(())
    }
}
