use glam::Vec3;

use crate::{
    skeleton::{generate_skeleton, SkinEntryOutput, SkinOutput},
    textures::TextureOutput,
    *,
};

pub struct StaticMeshOutput {
    pub name: String,
    pub vertices: Vec<Vec3>,
    pub indices: Vec<[u32; 3]>,
    pub parameters: Vec<f32>,
    pub attribute_count: usize,
}

impl StaticMeshOutput {
    pub fn to_output(&self) -> String {
        // Write out the bytes of the mesh data
        let verts = format!("{}_{VERTICES_EXTENSION}", self.name);
        let indices = format!("{}_{INDICES_EXTENSION}", self.name);
        let parameters = format!("{}_{PARAMETERS_EXTENSION}", self.name);

        write_file(&verts, cast_slice(&self.vertices));
        write_file(&indices, cast_slice(&self.indices));
        write_file(&parameters, cast_slice(&self.parameters));

        let name = self.name.to_uppercase();

        let p = self.attribute_count;

        format!(
            "
    pub static {name}: &StaticMeshData<{p}> = &StaticMeshData {{
        vertices: include_bytes_aligned!(4, \"generated/{verts}\"),
        indices: include_bytes_aligned!(4, \"generated/{indices}\"),
        parameters: include_bytes_aligned!(4, \"generated/{parameters}\"),
    }};\n"
        )
    }
}

pub fn generate_meshes() -> String {
    let mut out = String::from(
        "pub mod meshes {
    use super::*;\n",
    );

    MESHES.iter().for_each(|[filename, extension]| {
        // Read in the image file
        let read_path = format!("{INPUT_DIR}/{filename}.{extension}");

        println!("### Importing {filename}... ###");

        let (document, buffers, images) = gltf::import(read_path).unwrap();

        let blob = &buffers[0].0;
        let mesh = document.meshes().next().unwrap();

        let primitive = mesh.primitives().next().unwrap();

        println!("## Skeleton ##");
        let skeleton_result = generate_skeleton(filename, &blob, &document);
        let total_bone_count = match skeleton_result {
            Some((total_bone_count, text)) => {
                out.push_str(&text);
                total_bone_count
            }
            None => 0,
        };

        println!("## End Skeleton ##");
        println!("## Animations ##");

        for animation in document.animations() {
            let name = animation.name().unwrap_or("Unnamed");
            println!("Found animation: {name}");
        }

        println!("## End Animations ##");

        let mut indices = Vec::new();
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut uvs = Vec::new();
        let mut normals = Vec::new();
        let mut tangents = Vec::new();
        let mut bones = Vec::new();
        let mut weights = Vec::<f32>::new();
        let mut attribute_count = 0;

        for (kind, attribute) in primitive.attributes() {
            if attribute.view().unwrap().buffer().index() != 0 {
                panic!("wrong buffer index");
            }
            println!("Found {kind:?}: {:?}", attribute.data_type());
            let view = attribute.view().unwrap();
            let start = attribute.offset() + view.offset();
            //let count = attribute.count();
            let end = start + (attribute.count() * attribute.size());
            // println!("View: ({start}..{end}), {count} items.");
            let view = &blob[start..end];

            match kind {
                gltf::Semantic::Positions => {
                    let view: &[f32] = cast_slice(view);

                    for p in view.chunks_exact(3) {
                        positions.push(Vec3::from_slice(p));
                    }
                    println!("Positions found: {}", positions.len());
                }
                gltf::Semantic::Normals => {
                    let view: &[f32] = cast_slice(view);

                    for n in view.chunks_exact(3) {
                        normals.push(Vec3::from_slice(n))
                    }
                    attribute_count += 3;
                    println!("Normals found: {}", normals.len());
                }
                gltf::Semantic::TexCoords(_) => {
                    let view: &[f32] = cast_slice(view);

                    for uv in view.chunks_exact(2) {
                        uvs.push(Vec2::from_slice(uv));
                    }
                    attribute_count += 2;
                    println!("UVs found: {}", uvs.len());
                }
                gltf::Semantic::Colors(_) => {
                    let view: &[f32] = cast_slice(view);

                    for c in view.chunks_exact(3) {
                        colors.push(Vec3::from_slice(c));
                    }
                    attribute_count += 3;
                    println!("Colors found: {}", colors.len());
                }
                gltf::Semantic::Tangents => {
                    let view: &[f32] = cast_slice(view);

                    for t in view.chunks_exact(3) {
                        tangents.push(Vec3::from_slice(t))
                    }
                    attribute_count += 3;
                    println!("Tangents found: {}", tangents.len());
                }
                gltf::Semantic::Weights(_) => {
                    let view: &[f32] = cast_slice(view);

                    for w in view.chunks_exact(total_bone_count) {
                        weights.extend(w)
                    }
                    println!("Weights found: {}", weights.len());
                }
                gltf::Semantic::Joints(_) => {
                    let size = attribute.data_type().size();
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
                    println!("Joint influences found: {}", bones.len());
                }
            }
        }

        if let Some(indices_accessor) = primitive.indices() {
            let size = indices_accessor.size();
            let start = indices_accessor.offset() + indices_accessor.view().unwrap().offset();
            let count = indices_accessor.count();
            let end = start + (count * size);

            for index in blob[start..end].chunks_exact(size * 3) {
                let tri = if size == 2 {
                    let a = *from_bytes::<u16>(&index[0..2]) as u32;
                    let b = *from_bytes::<u16>(&index[2..4]) as u32;
                    let c = *from_bytes::<u16>(&index[4..6]) as u32;
                    [a, b, c]
                } else if size == 4 {
                    let a = *from_bytes::<u32>(&index[0..4]) as u32;
                    let b = *from_bytes::<u32>(&index[4..8]) as u32;
                    let c = *from_bytes::<u32>(&index[8..12]) as u32;
                    [a, b, c]
                } else {
                    panic!("Unhandled byte size for mesh: {filename}");
                };
                indices.push(tri)
            }
            println!("Triangles found: {}", indices.len());
        } else {
            for n in (0..positions.len()).step_by(3) {
                let n = n as u32;
                indices.push([n, n + 1, n + 2])
            }
            println!("Autogenerated {} triangles.", indices.len());
        }

        let vertex_count = positions.len();

        // Generate Normals
        if normals.is_empty() {
            println!("Normals not found, manually generating them...");
            normals.extend((0..positions.len()).map(|_| Vec3::default()));

            for indices in indices.iter() {
                let [a, b, c] = indices;

                let a = *a as usize;
                let b = *b as usize;
                let c = *c as usize;

                let a_pos = positions[a];
                let b_pos = positions[b];
                let c_pos = positions[c];

                let normal = calculate_triangle_normal(a_pos, b_pos, c_pos);

                normals[a] = normal;
                normals[b] = normal;
                normals[c] = normal;
            }
            attribute_count += 3;
        };

        // Generate Tangents
        // TODO

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

        let static_mesh = StaticMeshOutput {
            name: filename.to_string(),
            vertices: positions,
            indices,
            parameters,
            attribute_count,
        };
        let mut append = static_mesh.to_output();

        if total_bone_count > 0 {
            let mut entries = Vec::new();

            let max_bone_influences = weights.len() / vertex_count;

            for vertex in 0..vertex_count {
                let start = vertex * max_bone_influences;
                let end = start + max_bone_influences;
                let weights = weights[start..end].iter().cloned().collect();
                let bone_indices = bones[start..end].iter().cloned().collect();

                entries.push(SkinEntryOutput {
                    bone_indices,
                    weights,
                })
            }

            let skin = SkinOutput {
                name: filename.to_string(),
                entries,
            };
            append.push_str(&skin.to_output())
        };

        // Append the output String
        out.push_str(&append);

        // Import Images from the .glb
        for (index, image) in images.iter().enumerate() {
            let (size, alpha) = match image.format {
                gltf::image::Format::R8G8B8 => (1, false),
                gltf::image::Format::R8G8B8A8 => (1, true),
                gltf::image::Format::R16G16B16 => (2, false),
                gltf::image::Format::R16G16B16A16 => (2, true),
                gltf::image::Format::R32G32B32FLOAT => (4, false),
                gltf::image::Format::R32G32B32A32FLOAT => (4, true),
                // gltf::image::Format::R8 => todo!(),
                // gltf::image::Format::R8G8 => todo!(),
                // gltf::image::Format::R16 => todo!(),
                // gltf::image::Format::R16G16 => todo!(),
                _ => {
                    println!("Unsupported texture format: {:?}", image.format);
                    continue;
                }
            };

            let mut image_data = Vec::with_capacity((image.width * image.height) as usize);

            let chunks = (3 * size) + if alpha { size } else { 0 };

            for pixel in image.pixels.chunks_exact(chunks) {
                let (r, g, b) = match size {
                    1 => (pixel[0], pixel[1], pixel[2]),
                    2 => {
                        let r = *from_bytes::<u16>(&pixel[0..2]) as f32 / u16::MAX as f32;
                        let g = *from_bytes::<u16>(&pixel[2..4]) as f32 / u16::MAX as f32;
                        let b = *from_bytes::<u16>(&pixel[4..6]) as f32 / u16::MAX as f32;

                        let r = (r * u8::MAX as f32) / u8::MAX as f32;
                        let g = (g * u8::MAX as f32) / u8::MAX as f32;
                        let b = (b * u8::MAX as f32) / u8::MAX as f32;

                        (r as u8, g as u8, b as u8)
                    }
                    4 => {
                        let r = *from_bytes::<f32>(&pixel[0..4]) * u8::MAX as f32;
                        let g = *from_bytes::<f32>(&pixel[4..8]) * u8::MAX as f32;
                        let b = *from_bytes::<f32>(&pixel[8..12]) * u8::MAX as f32;

                        (r as u8, g as u8, b as u8)
                    }
                    _ => unreachable!(),
                };

                image_data.push(r);
                image_data.push(g);
                image_data.push(b);
            }

            let texture = TextureOutput {
                name: format!("{filename}_{index}"),
                width: image.width,
                height: image.height,
                image_data,
            };

            out.push_str(&texture.to_output());
        }

        println!("### Finished Importing {filename} ###");
    });

    out.push_str("}\n");

    out
}

// Function to calculate the normal of a triangle given its vertices
fn calculate_triangle_normal(v0: Vec3, v1: Vec3, v2: Vec3) -> Vec3 {
    // Calculate the vectors representing two edges of the triangle
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;

    // Calculate the cross product of the two edges to get the normal
    edge1.cross(edge2).normalize()
}
