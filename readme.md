app (Axum + Leptos)
├─ src/
│  ├─ main.rs          ← ★Axumサーバの入口（Routerを書く場所）
│  ├─ routes/
│  │   ├─ mod.rs
│  │   └─ bio.rs       ← ★foxp2_analyze を書く場所
│  ├─ components/
│  │   └─ foxp2.rs     ← ★Leptosコンポーネントを書く場所
│  └─ lib.rs or app.rs（構成により）
