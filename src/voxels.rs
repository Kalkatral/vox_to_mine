extern crate dot_vox;

pub struct Voxels
{
    voxel_array: Vec<u8>,
    pub size: (u32, u32, u32),
}

impl Voxels
{
    pub fn new(size: (u32, u32, u32)) -> Self
    {
        Self
        {
            voxel_array: vec![0; (size.0 * size.1 * size.2) as usize],
            size: size
        }
    }

    pub fn get(&mut self, pos : (u32, u32, u32)) -> u8
    {
        self.voxel_array[(pos.0 + pos.1 * self.size.0 + pos.2 * self.size.0 * self.size.1) as usize]
    }

    pub fn set(&mut self, pos: (u32, u32, u32), value: u8)
    {
        self.voxel_array[(pos.0 + pos.1 * self.size.0 + pos.2 * self.size.0 * self.size.1) as usize] = value;
    }

    pub fn from_vox(vox: dot_vox::DotVoxData) -> Self
    {
        let mut voxels: Self = Self::new((vox.models[0].size.x, vox.models[0].size.y, vox.models[0].size.z));
        for voxel in vox.models[0].voxels.iter()
        {
            voxels.set((voxel.x as u32, voxel.y as u32, voxel.z as u32), 1);
        }
        voxels
    }
}