Todo:
- Add const screen width / height settings
- Ways to benchmark performance
- Exporter improvements
  - Add support for multiple primitives/meshes
  - Add support for multiple skeletons
    - Separate exporting of skeleton, animations
- Convert rendering to lib, separate out game from rendering 
- Trivially depth reject tiles
  - Store lowest depth value in tile
  - Reject triangle if its lowest Z is higher than tiles
- Use Mesh Shader instead of Vertex Shader, option to include Triangle Components
- Consider reworking Shader Input Parameters (support Vec, Mat) instead of [f32; N]
- Separate geometry and fragment stages 
- Reconsider tiled rendering (last attempt was too slow)
- Deferred Rendering?

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
