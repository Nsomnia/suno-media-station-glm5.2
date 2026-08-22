**TODO**

> Updated at the start/end of every work session — see
> `docs/03-agent-constitution.md` §10 and `docs/19-git-workflow-and-repo-
> governance.md` §7.
>
> Current phase: **Phase 0 — Foundation** (`docs/04-phase-roadmap.md`)

**In Progress**
- [ ] *(empty — project not yet started)*

**Up Next (this phase)**
- [ ] Initialize git repo, `.gitignore`, root `Cargo.toml` workspace skeleton
      (docs/02, docs/19 §9)
- [ ] Scaffold every crate stub per docs/02 (empty `lib.rs` + `README.md`
      per docs/templates/crate-stub-template)
- [ ] Implement `structured-logging-and-tracing`, `app-configuration-
      loader`, `design-tokens-theme-definitions` (Catppuccin + Monokai)
- [ ] `Suno Station-app` binary: themed empty window + nav shell + working
      theme-switcher
- [ ] egui-vs-iced spike + projectM texture-compositing feasibility spike
      (docs/01 §4, docs/09 §4-5) → record decision as ADR
- [ ] `xtask check-layering` command (docs/18 §2.1)
- [ ] End-of-phase Senior Architect Pass + Phase Audit Summary (docs/18 §3)

**Blocked / Needs Human Input**
- [ ] Need first sanitized Burp Suite capture(s) of Suno auth + library-
      list + track-detail flows to unblock meaningful Phase 1 work on
      `suno-http-client-core` (docs/06)

**Recently Completed**
- [x] Full planning doc set (docs/00-18, 99) authored and approved by human
