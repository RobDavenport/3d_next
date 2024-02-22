use glam::Vec3;

use crate::shaders::{ColorBlend, PixelShaderInput, Textured};

#[derive(Default)]
pub struct GraphicsDb {
    vertices: Vec<VertexList>, // Collection of collections of Vertices
    indices: Vec<IndexList>,   // Collection of collections of Index Lists
    parameters: ParameterDb,   // Collection of collections of Parameter data

    pub color_blend: ColorBlend,
    pub textured: Textured,
}

// A mesh which is ready to be stored into the DB
pub struct Mesh<const P: usize> {
    pub vertices: VertexList,
    pub indices: IndexList,
    pub parameters: ParameterData<P>,
}

// Keys used to access the GraphicsDb
#[derive(Clone, Copy)]
pub struct MeshIndex<const P: usize> {
    geometry_index: usize,
    parameter_index: MeshParameterIndex<P>,
}

#[derive(Clone, Copy)]
pub struct MeshParameterIndex<const P: usize> {
    index: usize,
}

pub struct MeshReference<'a, const P: usize> {
    pub vertices: &'a [Vec3],
    pub indices: &'a [TriangleIndices],
    pub parameters: &'a [PixelShaderInput<P>],
}

impl GraphicsDb {
    pub fn push_mesh<const P: usize>(&mut self, mesh: Mesh<P>) -> MeshIndex<P>
    where
        ParameterDb: ParameterDataBuffer<P>,
    {
        let geometry_index = self.vertices.len();
        self.vertices.push(mesh.vertices);
        self.indices.push(mesh.indices);

        let parameter_index = self.parameters.push(mesh.parameters);

        MeshIndex {
            geometry_index,
            parameter_index,
        }
    }

    pub fn get<const P: usize>(&self, index: MeshIndex<P>) -> MeshReference<'_, P>
    where
        ParameterDb: ParameterDataBuffer<P>,
    {
        let vertices = &self.vertices[index.geometry_index].0;
        let indices = &self.indices[index.geometry_index].0;
        let parameters = &self.parameters.get(index.parameter_index).0;
        MeshReference {
            vertices,
            indices,
            parameters,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TriangleIndices(pub usize, pub usize, pub usize);

pub struct IndexList(pub Box<[TriangleIndices]>);

pub struct VertexList(pub Box<[Vec3]>);

#[derive(Default)]
pub struct ParameterDb {
    vec2s: Vec<ParameterData<2>>,
    vec3s: Vec<ParameterData<3>>,
}

pub struct ParameterData<const P: usize>(pub Box<[PixelShaderInput<P>]>);

pub trait ParameterDataBuffer<const P: usize> {
    fn buffer(&self) -> &Vec<ParameterData<P>>;
    fn buffer_mut(&mut self) -> &mut Vec<ParameterData<P>>;

    fn get(&self, index: MeshParameterIndex<P>) -> &ParameterData<P> {
        self.buffer().get(index.index).unwrap()
    }
    fn push(&mut self, parameters: ParameterData<P>) -> MeshParameterIndex<P> {
        let index = self.buffer().len();
        self.buffer_mut().push(parameters);
        MeshParameterIndex { index }
    }
}

impl ParameterDataBuffer<2> for ParameterDb {
    fn buffer(&self) -> &Vec<ParameterData<2>> {
        &self.vec2s
    }

    fn buffer_mut(&mut self) -> &mut Vec<ParameterData<2>> {
        &mut self.vec2s
    }
}

impl ParameterDataBuffer<3> for ParameterDb {
    fn buffer(&self) -> &Vec<ParameterData<3>> {
        &self.vec3s
    }

    fn buffer_mut(&mut self) -> &mut Vec<ParameterData<3>> {
        &mut self.vec3s
    }
}
