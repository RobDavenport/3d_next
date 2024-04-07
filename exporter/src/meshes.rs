use bytemuck::{cast_slice, from_bytes};
use glam::Vec3A;
use rkyv::AlignedVec;
use shared::{
    mesh::Mesh, vertex_parameters::VertexParametersList, IndexList, TriangleIndices, VertexList,
    VERTEX_MAX_PARAMETERS,
};

use crate::{
    animations::generate_animation,
    skeleton::generate_skeleton,
    skin::{SkinEntryVec, SkinOutput},
    textures::handle_glb_images,
    *,
};

pub struct MeshOutput {
    pub name: String,
    pub vertices: Vec<Vec3A>,
    pub indices: Vec<TriangleIndices>,
    pub parameters: Vec<f32>,
    pub attribute_count: usize,
}

impl MeshOutput {
    pub fn to_archive(&self) -> AlignedVec {
        match self.attribute_count {
            // This could be a seq! macro, but compile times are already quite long
            p if p > VERTEX_MAX_PARAMETERS => {
                panic!("Unhandled Attribute Count: {p}, max is {VERTEX_MAX_PARAMETERS}")
            }
            0 => self.extract_params::<0>(),
            1 => self.extract_params::<1>(),
            2 => self.extract_params::<2>(),
            3 => self.extract_params::<3>(),
            4 => self.extract_params::<4>(),
            5 => self.extract_params::<5>(),
            6 => self.extract_params::<6>(),
            7 => self.extract_params::<7>(),
            8 => self.extract_params::<8>(),
            9 => self.extract_params::<9>(),
            10 => self.extract_params::<10>(),
            11 => self.extract_params::<11>(),
            12 => self.extract_params::<12>(),
            13 => self.extract_params::<13>(),
            14 => self.extract_params::<14>(),
            15 => self.extract_params::<15>(),
            16 => self.extract_params::<16>(),
            _ => unreachable!(),
        }
    }

    fn extract_params<const P: usize>(&self) -> AlignedVec {
        let mesh: Mesh<P> = Mesh {
            vertices: VertexList(self.vertices.clone().into_boxed_slice()),
            indices: IndexList(self.indices.clone().into_boxed_slice()),
            parameters: VertexParametersList::from_flat_slice(&self.parameters),
        };

        rkyv::to_bytes::<_, 256>(&mesh).unwrap()
    }

    pub fn to_output(&self, config: &AssetList) -> String {
        let filename = format!("{}_{MESH_EXTENSION}", self.name);

        let archive = self.to_archive();
        write_file(config, &filename, &archive);

        let name = self.name.to_uppercase();
        let p = self.attribute_count;

        format!("pub const {name}: &MeshBytes<{p}> = &MeshBytes(include_bytes!(\"{filename}\"));\n")
    }
}

pub fn generate_meshes(config: &AssetList) -> String {
    let input_dir = &config.in_dir;
    let mut out = String::from(
        "pub mod meshes {
    use super::*;\n",
    );

    [
        proc_meshes::triangle(),
        proc_meshes::plane(),
        proc_meshes::cube(),
    ]
    .iter()
    .for_each(|mesh| {
        out.push_str(&mesh.to_output(config));
    });

    config.meshes.iter().for_each(|filename| {
        // Read in the glb file
        let read_path = format!("{input_dir}/{filename}.glb");

        println!("### Importing {filename}... ###");

        let (document, buffers, images) = gltf::import(read_path).unwrap();

        let blob = &buffers[0].0;

        let mut indices = Vec::new();
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut uvs = Vec::new();
        let mut normals = Vec::new();
        let mut tangents = Vec::new();
        let mut bones = Vec::new();
        let mut weights = Vec::<f32>::new();

        let mut weights_length = 0;
        let mut joints_length = 0;

        let mut vertex_count: usize = 0;

        let material_count = document.materials().count();

        if material_count > 1 {
            println!("Material count is greater than 1 ({material_count}), meshes may not be rendered correctly");
        }

        for mesh in document.meshes() {
            let primitive = mesh.primitives().next().unwrap();
            let primitive_count = mesh.primitives().count();

            if primitive_count > 1 {
                println!(
                    "Primitive count > 1 ({primitive_count}), mesh may not be exported correctly..."
                )
            }

            for (kind, attribute) in primitive.attributes() {
                if attribute.view().unwrap().buffer().index() != 0 {
                    panic!("wrong buffer index");
                }
                println!(
                    "Found {kind:?}: {:?} x {:?}",
                    attribute.data_type(),
                    attribute.dimensions()
                );
                let view = attribute.view().unwrap();
                let start = attribute.offset() + view.offset();
                let end = start + (attribute.count() * attribute.size());
                let view = &blob[start..end];

                match kind {
                    gltf::Semantic::Positions => {
                        let view: &[f32] = cast_slice(view);

                        for p in view.chunks_exact(3) {
                            positions.push(Vec3A::from_slice(p));
                        }
                    }
                    gltf::Semantic::Normals => {
                        let view: &[f32] = cast_slice(view);

                        for n in view.chunks_exact(3) {
                            normals.push(Vec3A::from_slice(n))
                        }
                    }
                    gltf::Semantic::TexCoords(_) => {
                        let view: &[f32] = cast_slice(view);

                        for uv in view.chunks_exact(2) {
                            uvs.push(Vec2::from_slice(uv));
                        }
                    }
                    gltf::Semantic::Colors(_) => {
                        let view: &[f32] = cast_slice(view);

                        for c in view.chunks_exact(3) {
                            colors.push(Vec3A::from_slice(c));
                        }
                    }
                    gltf::Semantic::Tangents => {
                        let view: &[f32] = cast_slice(view);

                        for t in view.chunks_exact(3) {
                            tangents.push(Vec3A::from_slice(t))
                        }
                    }
                    gltf::Semantic::Weights(w) => {
                        if w != 0 {
                            panic!("Unhandled weight {w}");
                        }
                        let view: &[f32] = cast_slice(view);
                        weights_length = attribute.dimensions().multiplicity();

                        for w in view.chunks_exact(weights_length) {
                            weights.extend(w)
                        }
                    }
                    gltf::Semantic::Joints(j) => {
                        if j != 0 {
                            panic!("Unhandled joint {j}");
                        }
                        let size = attribute.data_type().size();
                        joints_length = attribute.dimensions().multiplicity();
                        for index in view.chunks_exact(size) {
                            match size {
                                1 => {
                                    bones.push(index[0] as u32);
                                }
                                2 => {
                                    bones.push(*from_bytes::<u16>(&index[0..size]) as u32);
                                }
                                4 => {
                                    bones.push(*from_bytes::<u32>(&index[0..size]) as u32);
                                }
                                e => panic!("unhandled bone size: {e}"),
                            }
                        }
                    }
                }
            }

            if let Some(indices_accessor) = primitive.indices() {
                let size = indices_accessor.size();
                let start = indices_accessor.offset() + indices_accessor.view().unwrap().offset();
                let count = indices_accessor.count();
                let end = start + (count * size);

                for index in blob[start..end].chunks_exact(size * 3) {
                    let [a, b, c] = if size == 2 {
                        let a = *from_bytes::<u16>(&index[0..2]) as u16;
                        let b = *from_bytes::<u16>(&index[2..4]) as u16;
                        let c = *from_bytes::<u16>(&index[4..6]) as u16;
                        [a, b, c]
                    } else if size == 4 {
                        let a = *from_bytes::<u32>(&index[0..4]) as u16;
                        let b = *from_bytes::<u32>(&index[4..8]) as u16;
                        let c = *from_bytes::<u32>(&index[8..12]) as u16;
                        [a, b, c]
                    } else {
                        panic!("Unhandled byte size for mesh: {filename}");
                    };
                    indices.push(TriangleIndices(a + vertex_count as u16, b + vertex_count as u16, c + vertex_count as u16))
                }
                println!("Triangles found: {}", indices.len());
            } else {
                indices.clear();
                for n in (0..positions.len()).step_by(3) {
                    let n = n as u16;
                    indices.push(TriangleIndices(n, n + 1, n + 2))
                }
                println!("Autogenerated {} triangles.", indices.len());
            }

            // Generate Normals
            if normals.is_empty() {
                println!("Normals not found, manually generating them...");
                normals.extend((0..positions.len()).map(|_| Vec3A::default()));

                for indices in indices.iter() {
                    let TriangleIndices(a, b, c) = indices;

                    let a = *a as usize + vertex_count;
                    let b = *b as usize + vertex_count;
                    let c = *c as usize + vertex_count;

                    let a_pos = positions[a];
                    let b_pos = positions[b];
                    let c_pos = positions[c];

                    let normal = calculate_triangle_normal(a_pos, b_pos, c_pos);

                    normals[a] = normal;
                    normals[b] = normal;
                    normals[c] = normal;
                }
            };

            // Generate Tangents
            // TODO

            vertex_count = positions.len();
        }
        let mut parameters = Vec::<Vec<f32>>::new();

        positions.iter().enumerate().for_each(|(index, _)| {
            let mut this_vertex_parameters = Vec::new();
            // Colors, UVs, Normals
            if let Some(color) = colors.get(index) {
                this_vertex_parameters.push(color.x);
                this_vertex_parameters.push(color.y);
                this_vertex_parameters.push(color.z);
            }

            if let Some(uv) = uvs.get(index) {
                this_vertex_parameters.push(uv.x);
                this_vertex_parameters.push(uv.y);
            }

            if let Some(normal) = normals.get(index) {
                this_vertex_parameters.push(normal.x);
                this_vertex_parameters.push(normal.y);
                this_vertex_parameters.push(normal.z);
            }

            if let Some(tangent) = tangents.get(index) {
                this_vertex_parameters.push(tangent.x);
                this_vertex_parameters.push(tangent.y);
                this_vertex_parameters.push(tangent.z);
            }

            parameters.push(this_vertex_parameters);
        });

        let parameters = parameters.into_iter().flatten().collect::<Vec<_>>();

        let mut attribute_count = 0;

        if !colors.is_empty() {
            attribute_count += 3;
        }
        if !uvs.is_empty() {
            attribute_count += 2;
        }
        if !normals.is_empty() {
            attribute_count += 3;
        }
        if !tangents.is_empty() {
            attribute_count += 3;
        }

        let static_mesh = MeshOutput {
            name: filename.to_string(),
            vertices: positions,
            indices,
            parameters,
            attribute_count,
        };

        out.push_str(&static_mesh.to_output(config));

        // ###################################
        // # Handle Skeleton/Animation Stuff #
        // ###################################
        let mut total_bone_count = 0;

        if document.animations().next().is_some() || document.skins().next().is_some() {
            println!("## Skeleton ##");
            let skeleton_result = generate_skeleton(config, filename, &document, blob);
            let skeleton = if let Some((metadata, text)) = skeleton_result {
                out.push_str(&text);
                total_bone_count = metadata.bone_count;
                Some(metadata)
            } else {
                None
            };

            println!("## End Skeleton ##");
            println!("## Animations ##");

            if let Some(metadata) = skeleton {
                for animation in document.animations() {
                    out.push_str(&generate_animation(
                        config, &animation, blob, &metadata, filename,
                    ));
                }
            }
        }

        println!("## End Animations ##");

        if total_bone_count > 0 {
            let mut entries = Vec::new();

            for vertex in 0..vertex_count {
                let start_weights = vertex * weights_length;
                let end_weights = start_weights + weights_length;
                let weights = weights[start_weights..end_weights].to_vec();

                let bone_start = vertex * joints_length;
                let bone_end = bone_start + joints_length;
                let bone_indices = bones[bone_start..bone_end]
                    .iter()
                    .map(|x| *x as i8)
                    .collect::<Vec<_>>();

                let entry = SkinEntryVec {
                    bones_indices: bone_indices,
                    weights,
                };

                entries.push(entry)
            }

            let skin = SkinOutput {
                name: filename.to_string(),
                entries,
            };
            out.push_str(&skin.to_output(config))
        };

        // Handle output Images (if they exist)
        handle_glb_images(images, &mut out, config, filename);

        println!("### Finished Importing {filename} ###");
    });

    out.push_str("}\n");

    out
}

// Function to calculate the normal of a triangle given its vertices
fn calculate_triangle_normal(v0: Vec3A, v1: Vec3A, v2: Vec3A) -> Vec3A {
    // Calculate the vectors representing two edges of the triangle
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;

    // Calculate the cross product of the two edges to get the normal
    edge1.cross(edge2).normalize()
}
