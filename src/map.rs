use std::fmt::Debug;

use alkahest_data::{
    occlusion::AABB, statics::SStaticMeshInstances, ExtendedHash, ExtendedTag, Tag,
};
use destiny_pkg::{TagHash, TagHash64};
use glam::Vec4;
use tiger_parse::{tiger_tag, NullString, Pointer, ResourcePointer};

use crate::{ecs::Scene, types::ResourceHash};

pub struct MapData {
    pub hash: TagHash,
    pub name: String,
    pub scene: Scene,
    pub command_buffer: hecs::CommandBuffer,
}

#[derive(Clone)]
pub struct SimpleLight {
    pub pos: Vec4,
    pub attenuation: Vec4,
}

pub struct MapDataList {
    pub current_map: usize, // TODO(cohae): Shouldn't be here
    pub maps: Vec<(TagHash, Option<TagHash64>, MapData)>,
}

impl MapDataList {
    pub fn current_map(&self) -> Option<&(TagHash, Option<TagHash64>, MapData)> {
        if self.maps.is_empty() {
            None
        } else {
            self.maps.get(self.current_map % self.maps.len())
        }
    }

    pub fn current_map_mut(&mut self) -> Option<&mut MapData> {
        if self.maps.is_empty() {
            None
        } else {
            let map_index = self.current_map % self.maps.len();
            self.maps.get_mut(map_index).map(|v| &mut v.2)
        }
    }

    pub fn map_mut(&mut self, i: usize) -> Option<&mut MapData> {
        self.maps.get_mut(i).map(|v| &mut v.2)
    }
}

#[derive(Debug)]
#[tiger_tag(id = 0xffffffff, size = 0x50)]
pub struct SBubbleParent {
    pub file_size: u64,
    // 808091e0
    pub child_map: alkahest_data::Tag<SBubbleDefinition>,
    pub unkc: u32,

    pub unk10: u64,
    pub map_name: ResourceHash,

    #[tag(offset = 0x40)]
    pub unk40: Vec<Unk80809644>,
}

#[derive(Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80809644 {
    pub unk0: u32,
    pub unk4: u32,
    pub unk8: u32,
    pub unkc: u32, // 8080964e
}

// D2Class_01878080
#[derive(Debug)]
#[tiger_tag(id = 0xffffffff, size = 0x18)]
pub struct SBubbleDefinition {
    pub file_size: u64,
    pub map_resources: Vec<ExtendedTag<SMapContainer>>,
}

// D2Class_07878080
#[derive(Debug)]
#[tiger_tag(id = 0xffffffff, size = 0x38)]
pub struct SMapContainer {
    pub file_size: u64,
    #[tag(offset = 0x28)]
    pub data_tables: Vec<Tag<SMapDataTable>>,
}

// D2Class_83988080
#[derive(Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct SMapDataTable {
    pub file_size: u64,
    pub data_entries: Vec<Unk808099d8>,
}

// D2Class_85988080
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk808099d8 {
    pub rotation: glam::Quat,    // 0x0
    pub translation: glam::Vec4, // 0x10
    pub entity_old: TagHash,     // 0x20
    pub unk24: u32,
    pub entity: ExtendedHash,
    pub unk38: [u32; 9], //
    pub unk5c: f32,
    pub unk60: f32,
    pub unk64: TagHash,
    pub unk68: ResourceHash,
    pub unk6c: u32,
    pub world_id: u64,
    pub data_resource: ResourcePointer,
    pub unk80: [u32; 4],
}

#[derive(Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806ef4 {
    pub unk0: u64,
    pub instances: alkahest_data::Tag<SStaticMeshInstances>,
    pub unkc: [u32; 7],
}

/// Terrain
#[derive(Debug)]
#[tiger_tag(id = 0xffffffff, size = 0x88)]
pub struct STerrain {
    pub file_size: u64,
    pub unk8: u64,

    pub unk10: glam::Vec4,
    pub unk20: glam::Vec4,
    pub unk30: glam::Vec4,

    #[tag(offset = 0x50)]
    pub mesh_groups: Vec<Unk80807154>,

    pub vertex_buffer: TagHash,
    pub vertex_buffer2: TagHash,
    pub indices: TagHash,
    pub material1: TagHash,
    pub material2: TagHash,

    #[tag(offset = 0x78)]
    pub mesh_parts: Vec<Unk80807152>,
}

#[derive(Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80807154 {
    pub unk0: glam::Vec4,
    pub unk10: f32,
    pub unk14: f32,
    pub unk18: f32,
    pub unk1c: u32,
    pub unk20: glam::Vec4,
    pub unk30: u32,
    pub unk34: u32,
    pub unk38: u32,
    pub unk3c: u32,
    pub unk40: u32,
    pub unk44: u32,
    pub unk48: u32,
    pub unk4c: u32,
    pub dyemap: TagHash,
    pub unk54: u32,
    pub unk58: u32,
    pub unk5c: u32,
}

#[derive(Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80807152 {
    pub material: TagHash,
    pub index_start: u32,
    pub index_count: u16,
    pub group_index: u8,
    pub detail_level: u8,
}

/// Terrain resource
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff, size = 0x20)]
pub struct Unk8080714b {
    #[tag(offset = 0x10)]
    pub unk10: u16,
    pub unk12: u16,
    pub unk14: ResourceHash,
    pub terrain: TagHash,
    pub terrain_bounds: TagHash,
}

/// Cubemap volume resource
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff, size = 0x1e0)]
pub struct Unk80806b7f {
    #[tag(offset = 0x20)]
    pub cubemap_extents: glam::Vec4,
    /// Represents the visual center of the cubemap
    pub cubemap_center: glam::Vec4,
    pub unk40: f32,
    pub unk44: [u32; 3],
    pub unk50: glam::Vec4,
    pub unk60: glam::Vec4,

    pub unk70: [u32; 20],

    // Transform matrices?
    pub unkc0: [glam::Vec4; 4],
    pub unk100: [glam::Vec4; 4],

    pub unk140: [u32; 28],

    pub cubemap_name: Pointer<NullString>,
    pub cubemap_texture: TagHash,
    pub unk1bc: u32,
    pub unk1c0: TagHash,
    pub unk1c4: [u32; 7],
}

/// Decal collection resource
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806e68 {
    pub file_size: u64,
    pub instances: Vec<Unk80806e6c>,
    pub transforms: Vec<glam::Vec4>, // 80806e6d
    pub instance_points: TagHash,
    pub unk_vertex_colors: TagHash,

    pub unk30: [u32; 2],
    pub occlusion_bounds: alkahest_data::Tag<SOcclusionBounds>,
    _pad3c: u32,
    pub bounds: AABB,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806e6c {
    pub material: TagHash,
    pub start: u16,
    pub count: u16,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806df3 {
    pub file_size: u64,
    pub unk8: Vec<Unk80806dec>,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806dec {
    pub material: TagHash,
    pub index_buffer: TagHash,
    pub vertex_buffer: TagHash,
    pub unkc: u32,
    pub unk10: [u32; 4],

    pub translation: glam::Vec4,

    pub unk30: glam::Vec4,
    pub unk40: glam::Vec4,
    pub unk50: glam::Vec4,
}

// Unknown resource (some kind of octree?)
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80807268 {
    pub file_size: u64,
    /// Vertex buffer
    pub unk8: TagHash,
    pub unkc: u32,
    pub unk10: Vec<Unk8080726a>,
    pub unk20: [u32; 6],
    /// Vertex buffer
    pub unk38: TagHash,
    pub unk3c: u32,
    pub unk40: Vec<Unk8080726a>,
    pub unk50: Vec<Unk8080726d>,
    pub unk60: Vec<u16>,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk8080726a {
    pub unk0: [u32; 4],
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk8080726d {
    pub unk0: glam::Vec4,
    pub unk10: glam::Vec4,
    pub unk20: glam::Vec4,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80809162 {
    pub file_size: u64,
    pub unk8: Vec<Unk80809164>,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80809164 {
    pub unk0: glam::Vec4,
    pub unk10: glam::Vec4,
    pub unk20: [u32; 4],
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80809802 {
    pub file_size: u64,
    pub unk8: TagHash,
    pub unkc: TagHash,
    pub unk10: u32,
    pub unk14: TagHash,
    pub unk18: TagHash,
    pub unk1c: u32,
    pub streams: Vec<TagHash>,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806aa7 {
    pub file_size: u64,
    pub unk8: Vec<Unk80806aa9>,
    pub unk18: Vec<Unk808093b3>,
    pub unk28: Vec<u32>,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806aa9 {
    /// Transformation matrix
    pub transform: [f32; 16],

    /// Same as the bounding box from the Unk808093b3 array
    pub bounds: AABB,

    pub unk60: alkahest_data::Tag<Unk80806aae>,
    pub unk64: f32,
    pub unk68: f32,
    pub unk6c: i16,
    pub unk6e: u16,

    pub unk70: f32,
    pub unk74: u32,
    pub unk78: TagHash,
    pub unk7c: u32,

    pub unk80: u64,
    pub unk88: u32,
    pub unk8c: u32,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806aae {
    pub file_size: u64,
    pub entity_model: TagHash,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk808093b3 {
    pub bb: AABB,
    pub unk20: [u32; 4],
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct SLightCollection {
    pub file_size: u64,
    pub unk8: u64,
    pub bounds: AABB,
    pub unk30: Vec<SLight>,
    pub unk40: Vec<Unk80809f4f>,
    pub light_count: u32,
    pub unk54: u32,
    pub occlusion_bounds: alkahest_data::Tag<SOcclusionBounds>,
}

// 706C8080
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct SLight {
    pub unk0: glam::Vec4,
    pub unk10: glam::Vec4,
    pub unk20: glam::Vec4,
    pub unk30: glam::Vec4,
    pub unk40: [u32; 4],
    pub unk50: glam::Vec4,
    pub unk60: glam::Mat4,
    pub unka0: u32,
    pub unka4: u32,
    pub unka8: u32,
    pub unkac: f32,
    pub unkb0: f32,
    pub unkb4: f32,
    pub unkb8: f32,
    pub unkbc: f32,

    pub technique_shading: TagHash,
    pub technique_volumetrics: TagHash,
    pub technique_compute_lightprobe: TagHash,
    pub unkcc: TagHash, // Unk80806da1
    pub unkd0: TagHash, // Unk80806da1
    pub unkd4: [u32; 7],
}

// 716C8080
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct SShadowingLight {
    pub unk0: glam::Vec4,
    pub unk10: glam::Vec4,
    pub unk20: glam::Vec4,
    pub unk30: glam::Vec4,
    pub unk40: [u32; 4],
    pub unk50: glam::Vec4,
    pub unk60: glam::Mat4,
    pub unka0: u32,
    pub unka4: u32,
    pub unka8: u32,
    pub unkac: f32,
    pub unkb0: f32,
    pub unkb4: f32,
    pub unkb8: f32,

    // Might be FoV?
    pub unkbc: f32,

    pub unkc0: f32,
    pub unkc4: f32,
    pub unkc8: f32,
    pub unkcc: f32,

    pub technique_shading: TagHash,
    pub technique_shading_shadow: TagHash,
    pub technique_volumetrics: TagHash,
    pub technique_volumetrics_shadow: TagHash,
    pub technique_compute_lightprobe: TagHash,
    pub technique_compute_lightprobe_shadow: TagHash,

    pub unke8: TagHash, // Unk80806da1
    pub unkec: TagHash, // Unk80806da1

    pub unkd0: [u32; 8],
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80809f4f {
    pub rotation: glam::Vec4,
    pub translation: glam::Vec4,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80808cb7 {
    pub file_size: u64,
    pub unk8: Vec<Unk80808cb9>,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80808cb9 {
    pub rotation: glam::Vec4,
    pub translation: glam::Vec4,
    pub unk20: u32,
    // cohae: Probably padding
    pub unk24: [u32; 3],
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk808085c2 {
    pub file_size: u64,
    pub unk8: Vec<Unk808085c4>,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk808085c4 {
    pub unk0: [u32; 4],
    pub unk10: [u32; 4],
    pub translation: glam::Vec4,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806d19 {
    pub file_size: u64,
    pub unk8: TagHash,
    pub unkc: u32, // Padding
    pub unk10: Vec<()>,
    pub unk20: TagHash,
    pub unk24: u32, // Padding
    pub unk28: Vec<()>,
    pub unk38: TagHash,
    pub unk3c: u32, // Padding
    pub unk40: Vec<()>,
    pub unk50: Vec<Unk80806d4f>,
    pub unk60: Vec<()>,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806d4f {
    pub translation: glam::Vec4,
    pub unk10: [u32; 4],
    pub unk20: [u32; 4],
}

//#[derive(Clone, Debug)]
// #[tiger_tag(id = 0xffffffff)]
// pub struct Unk808066a2 {
//     pub file_size: u64,
//     pub unk8: Vec<()>,
//     pub unk18: Vec<()>,
//     /// Havok file
//     pub unk28: TagHash,
// }
#[derive(Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806c98 {
    pub file_size: u64,
    pub unk8: Vec<TagHash>,
    pub unk18: Vec<u32>,
    pub unk28: Vec<u32>,
    pub unk38: Vec<u32>,
    pub unk48: TagHash,
    pub unk4c: alkahest_data::Tag<SOcclusionBounds>,
    pub unk50: Vec<u32>,
    pub unk60: [u32; 4],
    pub bounds: AABB,
}

/// B1938080
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct SOcclusionBounds {
    pub file_size: u64,
    pub bounds: Vec<SMeshInstanceOcclusionBounds>,
}

// B3938080
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct SMeshInstanceOcclusionBounds {
    pub bb: AABB,
    pub unk20: [u32; 4],
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80809178 {
    // Points to havok pre-tag
    pub unk0: Pointer<SSlipSurfaceVolume>,

    pub unk8: u32,
    pub unkc: u32,
    pub area_name: ResourceHash,
    pub unk14: ResourceHash,
    pub unk18: ResourceHash,

    // Absolute offset to havok pre-tag??
    pub unk1c: u64,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk8080917b {
    // Points to havok pre-tag
    pub unk0: Pointer<SSlipSurfaceVolume>,
    pub unk8: u32,
    pub unkc: u32,
    pub kind: u8,
    pub unk11: u8,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct SSlipSurfaceVolume {
    pub unk0: [u32; 4],
    pub havok_file: TagHash,
    pub unk14: u32,
    pub shape_index: u32,
}

#[derive(Clone)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk808068d4 {
    pub unk0: u32,
    pub unk4: u32,
    pub unk8: u32,
    pub unkc: u32,
    pub entity_model: TagHash,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80808604 {
    pub unk0: [u32; 4],
    pub unk10: alkahest_data::Tag<Unk80808724>,
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80808606 {
    pub rotation: glam::Quat,
    pub translation: glam::Vec4,
    pub unk20: glam::Vec4,
    pub unk30: glam::Vec4,
    pub unk40: u32,
    pub shape_index: u32,
    pub unk48: u32,
    pub unk4c: u32,
    pub unk50: [u32; 4],
}

#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80808724 {
    pub file_size: u64,
    pub unk8: Vec<Unk80808606>,
    pub havok_file: TagHash,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk8080824c {
    pub rotation: glam::Quat,
    pub translation: glam::Vec4,
    pub unk20: glam::Vec4,
    pub unk30: glam::Vec4,
    pub unk40: glam::Vec4,
    pub unk50: glam::Vec4,
    pub unk60: Vec<glam::Vec4>,
    pub unk70: Vec<()>,
    pub unk80: f32,
    pub unk84: [u32; 3],
    pub unk90: [u32; 4],
    pub unka0: [u32; 3],
    pub shape_index: u32,
    pub unkb0: [u32; 4],
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80808248 {
    pub file_size: u64,
    pub havok_file: TagHash,
    _pad: u32,
    pub unk10: Vec<Unk8080824c>,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80808246 {
    pub unk0: [u32; 4],
    pub unk10: alkahest_data::Tag<Unk80808248>,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806ac2 {
    pub unk0: [u32; 4],
    pub unk10: alkahest_data::Tag<Unk80806ac4>,
    pub array_index: u32,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806ac4 {
    pub file_size: u64,
    pub havok_file: TagHash,
    _pad: u32,
    pub unk10: Vec<Unk80806ed8>,
}
#[derive(Clone, Debug)]
#[tiger_tag(id = 0xffffffff)]
pub struct Unk80806ed8 {
    pub rotation: glam::Quat,
    pub translation: glam::Vec4,
    pub unk20: glam::Vec4,
    pub unk30: glam::Vec4,
    pub unk40: glam::Vec4,
    pub unk50: glam::Vec4,

    pub unk60: Vec<glam::Vec4>,
    pub unk70: Vec<()>,

    pub unk80: f32,
    pub unk84: [u32; 3],
    pub unk90: [u32; 4],
    pub unka0: [u32; 3],
    pub shape_index: u32,
    pub unkb0: Vec<()>,
    pub unkc0: [u32; 4],
    pub unkd0: [u32; 4],
    pub unke0: [u32; 4],
}
