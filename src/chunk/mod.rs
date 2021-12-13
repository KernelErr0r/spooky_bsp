mod material;
mod mesh;
mod model_part;
mod occlusion;
mod sector_octree;
mod texture;
mod world;

pub use material::*;
pub use mesh::*;
pub use model_part::*;
pub use occlusion::*;
pub use sector_octree::*;
pub use texture::*;
pub use world::*;

use crate::Decode;
use std::io::Read;

use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(i32)]
pub(crate) enum ChunkType {
    Textures = 20002,
    Materials = 1010,
    MaterialObj = 5,
    World = 1012,
    AnimLib = 1017,
    Entities = 20000,
    Entity = 20001,
    SpLights = 1029,
    Zones = 1023,
    NavigationMesh = 1021,
    WpPoints = 1020,
    SectorOctree = 1011,
    Occlusion = 1019,
    Area = 1024,
    SkinObj = 1005,
    BoneObj = 1001,
    OcclusionMesh = 1018,
    ModelGroup = 1000,
    SPMesh = 1002,
    Collision = 1003,
    AtomicMesh = 1004,
    GLCamera = 1006,
    GLProject = 1,
    LightObj = 1007,
    LinkEmm = 1026,
    LevelObj = 1009,
}

#[derive(Debug)]
pub(crate) struct ChunkHeader {
    chunk_type: ChunkType,
    size: i32,
    version: i32,
}

impl ChunkHeader {
    pub(crate) fn get_chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub(crate) fn get_size(&self) -> i32 {
        self.size
    }

    pub(crate) fn get_version(&self) -> i32 {
        self.version
    }
}

impl Decode for ChunkHeader {
    fn decode(reader: &mut impl Read) -> eyre::Result<Self> {
        Ok(ChunkHeader {
            chunk_type: ChunkType::try_from(i32::decode(reader)?)?,
            size: i32::decode(reader)?,
            version: i32::decode(reader)?,
        })
    }
}
