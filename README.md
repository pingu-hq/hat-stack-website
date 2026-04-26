# 🎩 HAT-Stack Website  
*HTMX • Axum • TailwindCSS*

> *"A humble learning diary, one phase at a time..."*

---

## 🌸 Purpose

Welcome, kind visitor! ✨  
This repository is my personal **learning footprint**—a gentle, phased journey to grow comfortable with:

| Stack | Role | My Current Comfort Level |
|-------|------|--------------------------|
| **Axum** (Rust) | SSR backend, routing, handlers | 🌱 Beginner, eager to learn |
| **HTMX** | Interactive, hypermedia-driven frontend | 📚 "I know it... but don't use it" |
| **TailwindCSS** | Utility-first styling | 🎨 Familiar concept, seeking fluency |

This is **not** a production-ready project (yet!).  
It is a *diary*: each **Phase** documents what I built, what I struggled with, and what I learned.  
Think of it as leaving little footprints in the sand of my Rust/HTMX/Tailwind journey. 🐾

---

## 🗂️ Structure: Phased Learning

Progress is tracked in numbered **Phases** (Phase 1 → Phase 10 → ... → Phase ∞?).  
Each phase focuses on one small, achievable goal:


```progress
phases/
├── phase-01-navbar/
│ ├── what-i-did.md
│ ├── challenges.md
│ └── code-diff/
├── phase-02-routing/
├── phase-03-htmx-partials/
├── phase-04-tailwind-components/
├── ...
└── phase-N-whatever-comes-next/
```


### Example: `phase-01-navbar/what-i-did.md`
```markdown
## Phase 1: Building a Simple Navbar

✅ What I accomplished:
- Set up Axum route for `/` returning a basic HTML template
- Used Askama (or tera) for server-side template rendering
- Styled a responsive navbar with TailwindCSS utility classes
- Added HTMX `hx-get` for a dropdown menu (no JS!)

🧠 What I learned:
- How Axum's `Router` and `Handler` traits connect
- Tailwind's `@layer` and `@apply` in `styles/input.css`
- HTMX attributes feel magical but need practice to trust

🚧 Struggles:
- Forgot to run `cargo run` after changing `Cargo.toml` 😅
- Tailwind config took 3 tries to pick up my `templates/**/*.html`
