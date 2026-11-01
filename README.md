# Kumo Engine — Scene Editor Foundation

A lightweight Rust/egui scene editor foundation. It is an independent project; it does not use Roblox APIs, Roblox runtime systems, or Lua.

## Build

```bash
cargo run
```

The project currently uses `eframe` / `egui` 0.29.1. A committed `Cargo.lock` is recommended for reproducible builds. Do not use `cargo build --locked` until a lockfile has been generated and committed.

## Viewport controls

- **Right mouse + drag:** look around.
- **W / A / S / D:** fly forward, left, backward, right.
- **Arrow keys:** same movement as WASD.
- **Q / E:** move down / up.
- **Mouse wheel:** dolly forward / backward.
- **Shift:** move faster.
- **F:** focus the selected object.

## Editing

- **1:** Select tool.
- **2:** Move tool.
- **3:** Rotate tool.
- **4:** Resize / Scale tool.
- Left-click an object to select it; its edges turn white.
- Drag a selected object with Select or Move to move it along the ground plane.
- Move gizmo: drag the red, green, or blue axis handle to constrain movement.
- Rotate gizmo: drag a ring, or drag the selected object, to rotate it.
- Scale gizmo: drag an axis handle, or drag the selected object, to scale it.
- **Delete:** remove the selected object or folder and its contents.
- **Ctrl+D:** duplicate a selected object.
- **Ctrl+N:** create an empty scene.
- **Ctrl+S:** save the scene as `scene.kumo` in the current working directory.

## Scene organization

The Explorer starts empty. Use its **+** button or right-click the empty area to create folders. Right-click a folder to add a subfolder, rename it, or delete it. Right-click an object to duplicate it, move it into a folder, or delete it. The Inspector exposes object transforms and folder information.

Use **Insert** to add a cube, sphere, cylinder, cone, plane, or triangle. There are no default SpawnPoints, Workspace objects, scripts, or game-specific services.

## Current scope and limitations

The viewport is a lightweight CPU-side projected scene preview drawn with egui. It supports perspective camera movement, basic primitive geometry, selection, and transform editing. It is **not yet a full 3D renderer**: there is no mesh import pipeline, material system, lighting/shadows, physics, game runtime, scripting, or GPU scene renderer. Play/Game controls have intentionally been left out rather than showing buttons that do nothing. Those systems can be added as separate engine modules later.
