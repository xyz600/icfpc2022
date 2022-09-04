use std::path::Path;

use crate::problem::{Color8, Rectangle};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RawBlockConfig {
    blockId: String,
    bottomLeft: Vec<usize>,
    topRight: Vec<usize>,
    color: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RawTwinImageConfig {
    width: usize,
    height: usize,
    blocks: Vec<RawBlockConfig>,
}

pub struct BlockConfig {
    pub id: usize,
    pub rect: Rectangle,
    pub color: Color8,
}

pub struct TwinImageConfig {
    pub width: usize,
    pub height: usize,
    blocks: Vec<BlockConfig>,
}

impl TwinImageConfig {
    pub fn load(filepath: &Path) -> TwinImageConfig {
        let file = std::fs::File::open(&filepath).unwrap();
        let reader = std::io::BufReader::new(file);
        let deserialized: RawTwinImageConfig = serde_json::from_reader(reader).unwrap();

        let mut config = TwinImageConfig {
            width: deserialized.width,
            height: deserialized.height,
            blocks: vec![],
        };

        for block in deserialized.blocks.iter() {
            let bottom = block.bottomLeft[0];
            let left = block.bottomLeft[1];
            let height = block.topRight[0] - block.bottomLeft[0];
            let width = block.topRight[1] - block.bottomLeft[1];
            let rect = Rectangle::new(bottom, left, height, width);
            let color = Color8::new(block.color[0], block.color[1], block.color[2], block.color[3]);
            let block_config = BlockConfig {
                id: block.blockId.parse().unwrap(),
                rect,
                color,
            };
            config.blocks.push(block_config);
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::TwinImageConfig;

    #[test]
    fn test_config() {
        let path = Path::new("dataset/26.initial.json");
        let config = TwinImageConfig::load(path);
        assert_eq!(config.height, 400);
    }
}
