Todo:
- Use Mesh Shader instead of Vertex Shader, option to include Triangle Components
- Consider reworking Shader Input Parameters (support Vec, Mat) instead of [f32; N]
- Add Mesh Importer
  - Skinned Animation


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
