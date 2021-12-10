use std::io::{Read, self};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{BoundingBox, Rgb, Rgba};

pub struct World {
    flags: u32,
    ambient: Rgba,
    floors: Vec<Floor>,
    zone_count: i32,
    have_occlusion_bsp: bool,
    have_nulls: bool,
    have_waypoints: bool,
    have_mesh: bool,
}

impl World {
    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Self> {
        let flags = reader.read_u32::<LittleEndian>()?;
        let ambient = Rgb::decode_u8(reader)?.into();

        let floor_count = reader.read_i32::<LittleEndian>()?;

        let mut floors = Vec::with_capacity(floor_count as usize);

        for floor_index in 0 .. floor_count {
            let floor = Floor::decode(reader)?;

            floors.push(floor);
        }

        let zone_count = reader.read_i32::<LittleEndian>()?;

        let have_occlusion_bsp = reader.read_i32::<LittleEndian>()? != 0;
        let have_nulls = reader.read_i32::<LittleEndian>()? != 0;
        let have_waypoints = reader.read_i32::<LittleEndian>()? != 0;
        let have_mesh = reader.read_i32::<LittleEndian>()? != 0;
        
        Ok(Self { flags, ambient, floors, zone_count, have_occlusion_bsp, have_nulls, have_waypoints, have_mesh })
    }
}

pub struct Floor {
    occlusion_bsp: u32,
    ghost_camera: BoundingBox,
}

impl Floor {
    pub fn new(occlusion_bsp: u32, ghost_camera: BoundingBox) -> Self {
        Self { occlusion_bsp, ghost_camera }
    }

    pub(crate) fn decode(reader: &mut impl Read) -> io::Result<Self> {
        let occlusion_bsp = reader.read_u32::<LittleEndian>()?;
        let ghost_camera = BoundingBox::decode(reader)?;

        Ok(Self::new(occlusion_bsp, ghost_camera))
    }
}