extern crate dot_vox;
mod json_serializer;
mod voxels;
mod box_vector;
use json_serializer::JsonSerializer;
use voxels::Voxels;
use box_vector::Boxes;

const LOAD_FILE_PATH: &str = "D:\\Docs\\Desktop\\Mess\\cup.vox";
const SAVE_FILE_PATH: &str = "D:\\Docs\\Desktop\\Mess\\cup.json";

fn main()
{
    let data: dot_vox::DotVoxData = dot_vox::load(LOAD_FILE_PATH).expect("Failed to load .vox");
    let mut voxels: Voxels = Voxels::from_vox(data);
    let mut boxes: Boxes = Boxes::new();
    let mut json: JsonSerializer = JsonSerializer::new("item/test", (16, 16));

    let mut empty: bool = false;

    while !empty
    {
        for z in 0..voxels.size.2
        {
            for y in 0..voxels.size.1
            {
                for x in 0..voxels.size.0
                {
                    if voxels.get((x, y, z)) == 1
                    {
                        let mut max_x: u32 = voxels.size.0 - 1;
                        let mut max_y: u32 = voxels.size.1 - 1;
                        let mut max_z: u32 = 0;
                        let mut cur_x: u32;
                        let mut cur_y: u32;

                        while z + max_z < voxels.size.2 && voxels.get((x, y, z + max_z)) != 0
                        {
                            cur_y = 0;
                            while y + cur_y < voxels.size.1 && voxels.get((x, y + cur_y, z + max_z)) != 0
                            {
                                cur_x = 0;
                                while x + cur_x < voxels.size.0 && voxels.get((x + cur_x, y + cur_y, z + max_z)) != 0
                                {
                                    cur_x += 1;
                                }
                                cur_x -= 1;
                                if max_x > cur_x
                                {
                                    max_x = cur_x;
                                }
                                cur_y += 1;
                            }
                            cur_y -= 1;
                            if max_y > cur_y
                            {
                                max_y = cur_y;
                            }
                            max_z += 1;
                        }
                        max_z -= 1;

                        boxes.add_box((x, y, z), (x + max_x + 1, y + max_y + 1, z + max_z + 1));
                    }
                }
            }
        }

        let best_box: ((u32, u32, u32), (u32, u32, u32)) = boxes.get_best();
        json.add_box(best_box.0, best_box.1);
        voxels.set_area(best_box.0, best_box.1, 0);
        boxes.clear();
        empty = voxels.is_empty();
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
