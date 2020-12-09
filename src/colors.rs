use palette::Srgb;

type BasicColor = Srgb<u8>;
pub struct DiscretePalette {
    color_count: usize,
    colors: Vec<BasicColor>,
}

impl DiscretePalette {
    pub fn from_vec(colors: Vec<BasicColor>) -> DiscretePalette {
        DiscretePalette {
            color_count: colors.len(),
            colors,
        }
    }

    pub fn nth<'a>(&'a self, i: usize) -> &'a BasicColor {
        &self.colors[i % self.color_count]
    }
}

pub mod discrete {
    use super::*;

    pub fn dark2() -> DiscretePalette {
        let colors = vec![
            BasicColor::new(27, 158, 119),
            BasicColor::new(217, 95, 2),
            BasicColor::new(117, 112, 179),
            BasicColor::new(231, 41, 138),
            BasicColor::new(102, 166, 30),
            BasicColor::new(230, 171, 2),
            BasicColor::new(166, 118, 29),
            BasicColor::new(102, 102, 102),
        ];

        DiscretePalette::from_vec(colors)
    }
}
