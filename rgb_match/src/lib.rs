#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        let mut components = [self.r, self.g, self.b, self.a];

        let mut first_index = None;
        let mut second_index = None;

        for i in 0..4 {
            if components[i] == first {
                first_index = Some(i);
            }
            if components[i] == second {
                second_index = Some(i);
            }
        }

        if let (Some(first_idx), Some(second_idx)) = (first_index, second_index) {
            components.swap(first_idx, second_idx);
            return Color {
                r: components[0],
                g: components[1],
                b: components[2],
                a: components[3],
            };
        }

        // If swap is not possible, return the original color
        self
    }
}
