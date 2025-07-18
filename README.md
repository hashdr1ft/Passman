
---
# PASSMAN

> 🔐 *The last password manager you'll ever need. Secure. Fast. Beautiful.*  
> Built with Rust ⚙️ and Electron ⚡ — because your secrets deserve both **speed** and **style**.

---

### ⚡ Tech Stack

| Layer       | Tech                            |
|-------------|---------------------------------|
| Backend     | 🦀 Rust (blazingly fast, memory-safe) |
| Frontend    | ⚡ Electron (cross-platform desktop) + Tailwind CSS |
| UI Engine   | HTML, CSS, JS                   |
| Auth        | AES Encryption (Client-side Vaults) |
| Platform    | macOS, Linux, Windows (universal `.app`, `.exe`, `.AppImage`) |

---

### 📦 Project Structure

```

passman/  
├── password_manager/ # Rust backend (vault, encryption, storage)  
│ ├── src/  
│ └── Cargo.toml  
│  
├── password-manager-electron/ # Electron frontend (UI, logic)  
│ ├── index.html  
│ ├── main.js  
│ ├── renderer.js  
│ ├── tailwind.config.js  
│ └── package.json

````

---

### Getting Started

#### 1. 🔧 Install Dependencies

```bash
# Backend (Rust)
cd password_manager
cargo build

# Frontend (Electron)
cd ../password-manager-electron
npm install
````

#### 2. 🚀 Run the App

```bash
# Start backend (in one terminal)
cd password_manager
cargo run

# Start frontend (in another terminal)
cd password-manager-electron
npm start
```

---

### Features

- ✅ End-to-end encrypted password vault
    
- 🧊 AES-based encryption — secrets never leave your machine
    
- 🌙 Clean Tailwind-based dark UI
    
- 🪪 No account, no cloud — local-first privacy
    
- 🛠 Offline-only storage (no telemetry, no spyware)
    
- 💥 Blazing fast Rust core for cryptographic operations
    
- 🖥 Desktop ready: `.dmg`, `.AppImage`, `.exe`
    

---

### 📦 Build macOS `.dmg` with electron-builder

We use [`electron-builder`](https://www.electron.build) for creating native installers.

#### 🧪 Build `.dmg` on macOS


`cd password-manager-electron npm run dist`

The output will be:

`dist/Pass_M4N-1.0.0.dmg`

#### ✅ package.json Example Setup

```
"scripts": {
  "start": "electron .",
  "dist": "electron-builder --mac"
},
"build": {
  "appId": "com.hashdr1ft.passman",
  "productName": "Pass_M4N",
  "mac": {
    "target": "dmg",
    "arch": ["x64", "arm64"],         // Optional: universal build
    "category": "public.app-category.utilities"
  },
  "dmg": {
    "title": "${productName} ${version}",
    "backgroundColor": "#ffffff",
    "contents": [
      { "x": 130, "y": 150, "type": "file" },
      { "x": 410, "y": 150, "type": "link", "path": "/Applications" }
    ]
  },
  "files": [
    "**/*",
    "!node_modules/*/{test,__tests__}/**",
    "!**/*.map"
  ]
}


```

#### Want a Universal Binary?

`npm run dist -- --universal`

---

This project is built for people who care about **security**, **speed**, and **sovereignty**.  
No more giving away your secrets. Take control.

---

