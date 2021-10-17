//extern crate dot_vox;
mod json_serializer;
use json_serializer::JsonSerializer;

//const LOAD_FILE_PATH: &str = "D:\\Docs\\Desktop\\Mess\\cup.vox";
const SAVE_FILE_PATH: &str = "D:\\Docs\\Desktop\\Mess\\cup.json";

fn main()
{
    //let data: dot_vox::DotVoxData = dot_vox::load(LOAD_FILE_PATH).expect("Failed to load .vox");

    let mut json: JsonSerializer = JsonSerializer::new("item/test", (16, 16));
    json.add_box((0, 0, 0), (6, 7, 1));
    match json.write_file(SAVE_FILE_PATH)
    {
        Ok(()) =>
        {
            println!("Successfully saved json to {}", SAVE_FILE_PATH);
        }
        Err(e) =>
        {
            println!("Could not save json :\n{}", e);
        }
    }
}
