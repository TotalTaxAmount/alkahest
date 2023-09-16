use binrw::BinRead;
use destiny_pkg::TagHash;
use std::io::SeekFrom;

use crate::entity::{ELodCategory, EPrimitiveType};
use crate::types::Vector2;
use crate::{
    structure::TablePointer,
    types::{Vector3, Vector4},
};

#[derive(BinRead, Debug)]
pub struct Unk80807194 {
    pub file_size: u64,
    pub mesh_groups: TablePointer<Unk8080719b>,
    pub parts: TablePointer<Unk8080719a>,
    pub buffers: TablePointer<(TagHash, TagHash, TagHash, u32)>,

    #[br(seek_before(SeekFrom::Start(0x40)))]
    pub mesh_offset: Vector3,
    pub mesh_scale: f32,
    pub texture_coordinate_scale: f32,
    pub texture_coordinate_offset: Vector2,
}

#[derive(BinRead, Debug, Clone)]
pub struct Unk8080719a {
    pub index_start: u32,
    pub index_count: u32,
    pub buffer_index: u8,
    pub unk9: u8,
    pub lod_category: ELodCategory,
    pub primitive_type: EPrimitiveType,
}

#[derive(BinRead, Debug, Clone)]
pub struct Unk8080719b {
    pub part_index: u16,
    pub unk2: u8,
    pub unk3: u8,
    pub unk5: u16,
}

#[derive(BinRead, Debug, Clone)]
pub struct Unk8080966d {
    #[br(seek_before(SeekFrom::Current(0x40)))]
    pub transforms: TablePointer<Unk808071a3>,
    pub unk50: u64,
    pub unk58: [u64; 4],
    pub statics: TablePointer<TagHash>,
    pub instances: TablePointer<Unk80807190>,
}

#[derive(BinRead, Debug, Clone)]
pub struct Unk80807190 {
    pub instance_count: u16,
    pub instance_start: u16,
    pub static_index: u16,
    pub unk6: u16,
}

#[derive(BinRead, Debug, Clone)]
pub struct Unk808071a3 {
    pub rotation: Vector4, // TODO(cohae): Quat type? (alias?)
    pub translation: Vector3,
    pub scale: Vector3,
    pub unk28: u32,
    pub unk2c: u32,
    pub unk30: [u32; 4],
}

#[derive(BinRead, Debug, Clone)]
pub struct Unk808071a7 {
    pub file_size: u64,
    pub unk8: TagHash,
    pub unkc: u32,
    pub materials: TablePointer<TagHash>,
    // pub unk20: TablePointer<Unk80807193>, // Overlay/transparent meshes
    pub unk20: [u64; 2],
    pub unk30: [u32; 2],
    pub unk38: [f32; 6],
    pub unk50: Vector3, // ? Similar to model_offset, but not quite right...
    pub unk5c: f32,
}

#[derive(BinRead, Debug, Clone)]
pub struct Unk80807193 {
    pub unk0: u16,
    pub unk2: u16,
    pub unk4: u32,
    pub index_buffer: TagHash,
    pub vertex_buffer: TagHash,
    pub vertex_buffer2: TagHash,
    pub index_start: u32,
    pub index_count: u32,
    pub material: TagHash,
}
