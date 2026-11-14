# Manifold

A from-scratch voxel engine built with **Rust** and **Vulkan**.

![Terrain preview](docs/images/preview.png)
![Terrain preview](docs/images/preview2.png)
![Terrain preview](docs/images/preview3.png)

## Features

**Rendering**

**Terrain generation**

**World**

## Architecture

```
src/
  client/
    app/              Client side application
    render/
      graphical_core/ Vulkan renderer, pipelines, GPU pools, command recording
      shaders/        GLSL mesh/task/vertex/fragment/compute shaders (SPIR-V)
    session/  
  server/             Server architecture
    storage/          Region files, world metadata, disk caching
  shared/             Player collision, server protocol, blocks, chunks

```

## Build

Requires the [Vulkan SDK](https://vulkan.lunarg.com/) and `glslc` on PATH.

```
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| WASD | Move |
| Mouse | Look |
| Space | Jump (walk mode) |
| Shift | Fast fly |
| E / Q | Fly up / down |
| F | Toggle fly/walk |
| Left click | Break block |
| Right click | Place block |
| Esc | Release cursor |
