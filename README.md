my-tauri-app/
│
├── src/ # Frontend code (JS/TS)
├── src-tauri/ # Rust backend
│ ├── Cargo.toml
│ └── src/
│ └── main.rs
│
├── package.json
└── Cargo.toml # Workspace root


invi/
|
|-- crates/
| |-- libs/
|   |-- lib-core/
|   |-- lib-utils/
|   |-- lib-model/
|   |-- tauri-backend # Tauri backend
| |-- services/
|   |-- invi/ 
|     |-- src/ # Frontend
|     |-- Cargo.toml # The one with the workspace configurated
|-- Cargo.toml # Workspace root
|-- Trunk.toml