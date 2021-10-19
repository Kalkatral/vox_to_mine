extern crate dot_vox;
mod json_serializer;
mod voxels;
mod box_vector;
use json_serializer::JsonSerializer;
use voxels::Voxels;
use box_vector::Boxes;
use std::env;
use std::io::{stdout, Write};

fn main()
{
    let args: Vec<String> = env::args().collect();
    let mut stdout = stdout();

    if args.len() != 3
    {
        println!("Syntax : vox_to_mine <source.vox> <target.json>");
        return;
    }

    let data: dot_vox::DotVoxData = dot_vox::load(&args[1]).expect("Failed to load .vox");
    let mut voxels: Voxels = Voxels::from_vox(data);
    let mut boxes: Boxes = Boxes::new();
    let mut json: JsonSerializer = JsonSerializer::new("item/test", (16, 16));

    let mut empty: bool = false;

    print!("0.00%");

    while !empty
    {
        for z in 0..voxels.size.2
        {
            for y in 0..voxels.size.1
            {
                for x in 0..voxels.size.0
                {
                    if voxels.get((x, y, z)) != 0
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
        print!("\r{:.2}%", voxels.completion());
        stdout.flush().unwrap();
    }

    println!("");

    match json.write_file(&args[2])
    {
        Ok(()) =>
        {
            println!("Successfully saved json to {}", &args[2]);
        }
        Err(e) =>
        {
            println!("Could not save json :\n\n{}", e);
        }
    }
}
