

pub struct Boxes
{
    content: Vec<(u8, u8, u8)>,
    best_volume: u32,
    best_content_id: usize,
}

impl Boxes
{
    pub fn new() -> Self
    {
        Self
        {
            content: Vec::new(),
            best_volume: 0,
            best_content_id: 0,
        }
    }

    pub fn add_box(&mut self, start: (u32, u32, u32), end: (u32, u32, u32))
    {
        self.content.push((start.0 as u8, start.1 as u8, start.2 as u8));
        self.content.push((end.0 as u8, end.1 as u8, end.2 as u8));
        let volume: u32 = (end.0 - start.0) * (end.1 - start.1) * (end.2 - start.2);
        if volume > self.best_volume
        {
            self.best_volume = volume;
            self.best_content_id = self.content.len() - 2;
        }
    }

    pub fn get_best(&mut self) -> ((u32, u32, u32), (u32, u32, u32))
    {
        let start: (u8, u8, u8) = self.content[self.best_content_id];
        let end: (u8, u8, u8) = self.content[self.best_content_id + 1];
        ((start.0 as u32, start.1 as u32, start.2 as u32), (end.0 as u32, end.1 as u32, end.2 as u32))
    }

    pub fn clear(&mut self)
    {
        self.content.clear();
        self.best_volume = 0;
        self.best_content_id = 0;
    }
}