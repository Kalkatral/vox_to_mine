extern crate dot_vox;
mod json_serializer;
use json_serializer::JsonSerializer;
mod voxels;
use voxels::Voxels;

const LOAD_FILE_PATH: &str = "D:\\Docs\\Desktop\\Mess\\cup.vox";
const SAVE_FILE_PATH: &str = "D:\\Docs\\Desktop\\Mess\\cup.json";

fn main()
{
    let data: dot_vox::DotVoxData = dot_vox::load(LOAD_FILE_PATH).expect("Failed to load .vox");

    let mut voxels: Voxels = Voxels::from_vox(data);

    let mut json: JsonSerializer = JsonSerializer::new("item/test", (16, 16));

    for z in 0..voxels.size.2
    {
        for y in 0..voxels.size.1
        {
            for x in 0..voxels.size.0
            {
                if voxels.get((x, y, z)) == 1
                {
                    json.add_box((x, y, z), (x+1, y+1, z+1));
                }
            }
        }
    }

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
