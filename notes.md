Todo:
- Ways to benchmark performance
- Trivially depth reject tiles
- Consider Deferred Rendering
- Use Mesh Shader instead of Vertex Shader, option to include Triangle Components
- Consider reworking Shader Input Parameters (support Vec, Mat) instead of [f32; N]
- Reconsider tiled rendering (last attempt was too slow)
- Add support for multiple primitives/meshes
- Add support for multiple skeletons
  - Separate exporting of skeleton, animations

---
Helmet Model Textures
0 - Diffuse Texture
1 - Metallic Roughness
2 - Emissive
3 - Occlusion Map
4 - Normals

---

chatGPT Optimization Suggestions:

Raster/Rendering Level
SIMD: Done
BB Clipping: Done
Back-Face Culling: Done
Early Depth Testing: Done
Scanline Algorithms: TODO, see https://oa.upm.es/9184/1/INVE_MEM_2010_84947.pdf

Application Level:
Spatial Partitioning
LUTs

General:
Cache-Aware Optimization: TODO
Profile and Benchmark: TODO
Data-Oriented Design: TODO
Compiler Optimization Flags: Done
