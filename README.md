# Kumo Engine — starter

A modular, lightweight editor shell in Rust. This is the first UI foundation, not yet a complete game engine.

## Run

```bash
cargo run
```

## Current foundation

- Modular editor areas: toolbar, Explorer, Inspector, viewport, Console, Asset Browser, theme.
- Compact Select / Move / Rotate / Scale tools.
- Play / Pause / Stop controls.
- Resizable side and bottom panels, with close/toggle controls.
- Neutral scene viewport, intentionally separate from the phosphor-blue editor theme.
- Optional JetBrains Mono font loading.

## Architecture

```text
src/
  main.rs
  app/
    mod.rs
  editor/
    mod.rs
    toolbar.rs
    explorer.rs
    inspector.rs
    viewport.rs
    assets.rs
  theme.rs
assets/
  fonts/
```

Each editor area is isolated in its own module. Keep engine systems (scene graph, renderer, asset pipeline, physics, audio) separate from editor UI as they are introduced.

## Graphics backend note

This starter uses egui's Glow renderer (OpenGL) to keep the first editor shell relatively accessible. The viewport is currently a UI-drawn placeholder, not a real 3D renderer. A dedicated renderer module should replace it without coupling rendering logic to the editor panels.

## Next milestones

1. Add a real project model and scene serialization.
2. Add an actual 3D viewport renderer and camera navigation.
3. Implement selection and transform gizmos.
4. Add asset import and a real Inspector backed by scene properties.
5. Add undo/redo and play-mode isolation.

## Insert Part (prototype)

The toolbar now has an **Insert Part** action that creates a cube immediately, plus an adjacent dropdown for Cube, Sphere, Cylinder, Cone, Plane, and Triangle. Inserted primitives are added to the Explorer, selected automatically, and their primitive type appears in the Inspector. The viewport draws a simple 2D editor preview of the selected primitive; this is still placeholder UI geometry, not a true 3D mesh or renderer.

## Lighting status

There is not yet a scene-lighting implementation. The dark phosphor-blue palette belongs to the editor UI only. The viewport uses neutral grey placeholder colours and a drawn grid; lights, materials, shadows, and physically based rendering will be implemented with the future scene renderer.
