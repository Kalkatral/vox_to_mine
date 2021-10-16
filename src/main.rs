//extern crate dot_vox;
mod json_serializer;
use json_serializer::JsonSerializer;

//const FILE_PATH: &str = "D:\\Docs\\Desktop\\Mess\\cup.vox";

fn main()
{
    //let data: dot_vox::DotVoxData = dot_vox::load(FILE_PATH).expect("Failed to load .vox");

    let mut json: JsonSerializer = JsonSerializer::new("item/test", (16, 16));
}
