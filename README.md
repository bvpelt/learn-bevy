# Bevy tutorial

This document shows a tutorial to use bevy. Based on: https://www.youtube.com/watch?v=TQt-v_bFdao&list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd. 

This code is build with
- rust 1.99.1
- bevy 0.19.0

It is devided in several steps
- Setup environment [EP1](docs/EP1.md)
- Bundle and resources [EP2](docs/EP2.md)
- Moving player inside window, add enemies [EP3](docs/EP3.md)
- Enemy movement, collision detection, sound [EP4](docs/EP4.md)
- Add user resources and score [EP5](docs/EP5.md)
- Add events game over, add high score [EP6](docs/EP6.md)
- Refactoring [EP7](docs/EP7.md)[github](https://github.com/bvpelt/learn-bevy/tree/simple-refactor)
- System ordering and systemsets [EP8](docs/EP8.md) [github](https://github.com/bvpelt/learn-bevy/tree/systemset)
- Statemanagement and conditional run on state [EP9](docs/EP9.md) [github](https://github.com/bvpelt/learn-bevy/tree/state)
- UI [EP10](docs/EP10.md)


[UI https://www.youtube.com/watch?v=iW19V3a96tY](https://youtu.be/iW19V3a96tY?si=PkhTOFocHQQrhgC2&t=1574)

## Building

To build for production use [```cargo build --release``` or ```cargo build-rel```](#build-for-production) to build for development [```cargo build```](#build-for-development) or to run for development type ```cargo run --features dev``` or ```cargo dev```

For development dynamic_linking is enabled, which means the executable dependes on shared libraries.

For a production release dynamic_linking is not enabled.

### Build for production

#### Build for production 
```bash
$ cargo build-rel
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling serde_core v1.0.229
   Compiling cfg-if v1.0.5
   Compiling libc v0.2.190
   Compiling log v0.4.34
   Compiling libm v0.2.16
   Compiling smallvec v1.16.3
   Compiling cfg_aliases v0.2.2
   Compiling pin-project-lite v0.2.17
   Compiling serde v1.0.229
   Compiling foldhash v0.2.0
   Compiling autocfg v1.5.1
   Compiling winnow v1.0.4
   Compiling hashbrown v0.17.1
   Compiling equivalent v1.0.2
   Compiling toml_datetime v1.1.2+spec-1.1.0
   Compiling thiserror v2.0.21
   Compiling parking v2.2.1
   Compiling arrayvec v0.7.8
   Compiling allocator-api2 v0.2.21
   Compiling crossbeam-utils v0.8.23
   Compiling once_cell v1.21.4
   Compiling getrandom v0.4.3
   Compiling futures-core v0.3.34
   Compiling rand_core v0.10.1
   Compiling num-traits v0.2.19
   Compiling fastrand v2.5.0
   Compiling futures-io v0.3.34
   Compiling event-listener v5.4.2
   Compiling raw-window-handle v0.6.2
   Compiling futures-lite v2.6.1
   Compiling indexmap v2.14.2
   Compiling foldhash v0.1.5
   Compiling slab v0.4.12
   Compiling version_check v0.9.5
   Compiling event-listener-strategy v0.5.4
   Compiling syn v3.0.7
   Compiling syn v2.0.119
   Compiling concurrent-queue v2.5.0
   Compiling hashbrown v0.15.5
   Compiling cpufeatures v0.3.1
   Compiling typewit v1.15.2
   Compiling rustix v1.1.5
   Compiling chacha20 v0.10.1
   Compiling find-msvc-tools v0.1.14
   Compiling async-channel v2.5.0
   Compiling toml_parser v1.1.5+spec-1.1.0
   Compiling linux-raw-sys v0.12.1
   Compiling const_panic v0.2.17
   Compiling shlex v2.0.1
   Compiling unicode-segmentation v1.13.3
   Compiling typeid v1.0.3
   Compiling spin v0.10.1
   Compiling cc v1.7.0
   Compiling unicode-xid v0.2.6
   Compiling thread_local v1.1.10
   Compiling toml_edit v0.25.17+spec-1.1.0
   Compiling convert_case v0.10.0
   Compiling erased-serde v0.4.10
   Compiling nix v0.31.3
   Compiling fixedbitset v0.5.7
   Compiling async-task v4.7.1
   Compiling atomic-waker v1.1.2
   Compiling disqualified v1.1.0
   Compiling slotmap v1.1.1
   Compiling bevy_ptr v0.19.1
   Compiling inventory v0.3.25
   Compiling downcast-rs v2.0.2
   Compiling memchr v2.8.3
   Compiling async-executor v1.14.0
   Compiling crossbeam-queue v0.3.14
   Compiling nonmax v0.5.5
   Compiling bumpalo v3.20.3
   Compiling tracing-core v0.1.36
   Compiling zerocopy v0.8.62
   Compiling rand v0.10.3
   Compiling either v1.19.0
   Compiling crossbeam-channel v0.5.17
   Compiling pkg-config v0.3.34
   Compiling itertools v0.14.0
   Compiling piper v0.2.5
   Compiling async-io v2.6.0
   Compiling libloading v0.8.9
   Compiling blocking v1.7.0
   Compiling sysinfo v0.38.4
   Compiling hashbrown v0.16.1
   Compiling uuid v1.28.0
   Compiling async-lock v3.4.2
   Compiling constant_time_eq v0.4.2
   Compiling const-fnv1a-hash v1.1.0
   Compiling futures-task v0.3.34
   Compiling blake3 v1.8.7
   Compiling rand_distr v0.6.0
   Compiling stackfuture v0.3.1
   Compiling async-broadcast v0.7.2
   Compiling atomicow v1.2.0
   Compiling simd-adler32 v0.3.10
   Compiling crc32fast v1.5.2
   Compiling adler2 v2.0.1
   Compiling aho-corasick v1.1.5
   Compiling regex-syntax v0.8.11
   Compiling miniz_oxide v0.9.1
   Compiling miniz_oxide v0.8.9
   Compiling async-fs v2.2.0
   Compiling fdeflate v0.3.7
   Compiling const_soft_float v0.1.4
   Compiling pxfm v0.1.30
   Compiling flate2 v1.1.10
   Compiling constgebra v0.1.4
   Compiling euclid v0.22.14
   Compiling twox-hash v2.1.5
   Compiling svg_fmt v0.4.5
   Compiling bevy_macro_utils v0.19.1
   Compiling encase_derive_impl v0.12.1
   Compiling byteorder-lite v0.1.0
   Compiling bevy_ecs_macro_logic v0.19.1
   Compiling synstructure v0.14.0
   Compiling ruzstd v0.8.3
   Compiling rectangle-pack v0.4.2
   Compiling lazy_static v1.5.1
   Compiling bevy_mikktspace v1.0.0
   Compiling stable_deref_trait v1.2.1
   Compiling sharded-slab v0.1.7
   Compiling tracing-log v0.2.0
   Compiling guillotiere v0.6.2
   Compiling nu-ansi-term v0.50.3
   Compiling unicode-width v0.2.2
   Compiling termcolor v1.4.1
   Compiling naga v29.0.4
   Compiling bit-vec v0.9.1
   Compiling hexf-parse v0.2.1
   Compiling rustc-hash v1.1.0
   Compiling bit-set v0.9.1
   Compiling codespan-reporting v0.13.1
   Compiling writeable v0.6.4
   Compiling wayland-sys v0.31.11
   Compiling dlib v0.5.3
   Compiling litemap v0.8.3
   Compiling ash v0.38.0+1.3.281
   Compiling parking_lot_core v0.9.12
   Compiling regex-automata v0.4.18
   Compiling wayland-backend v0.3.17
   Compiling memmap2 v0.9.11
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.14
   Compiling wgpu-hal v29.0.4
   Compiling utf8_iter v1.0.4
   Compiling downcast-rs v1.2.1
   Compiling scoped-tls v1.0.1
   Compiling presser v0.3.1
   Compiling parking_lot v0.12.5
   Compiling codespan-reporting v0.12.0
   Compiling ordered-float v5.5.0
   Compiling profiling v1.0.18
   Compiling icu_locale_fallback_data v2.3.0
   Compiling encase_derive v0.12.1
   Compiling bytemuck_derive v1.12.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling derive_more-impl v2.1.1
   Compiling bevy_reflect_derive v0.19.1
   Compiling variadics_please v1.1.0
   Compiling assert_type_match v0.1.1
   Compiling bevy_ecs_macros v0.19.1
   Compiling bevy_derive v0.19.1
   Compiling tracing-attributes v0.1.31
   Compiling zerocopy-derive v0.8.62
   Compiling futures-macro v0.3.34
   Compiling bytemuck v1.25.2
   Compiling encase v0.12.1
   Compiling bevy_asset_macros v0.19.1
   Compiling bitflags v2.13.2
   Compiling glam v0.32.1
   Compiling futures-util v0.3.34
   Compiling tracing v0.1.44
   Compiling png v0.18.1
   Compiling moxcms v0.8.1
   Compiling ktx2 v0.5.0
   Compiling zerofrom-derive v0.1.8
   Compiling ctrlc v3.5.2
   Compiling bevy_encase_derive v0.19.1
   Compiling derive_more v2.1.1
   Compiling yoke-derive v0.8.4
   Compiling matchers v0.2.0
   Compiling zerovec-derive v0.11.6
   Compiling spirv v0.4.0+sdk-1.4.341.0
   Compiling tracing-subscriber v0.3.23
   Compiling displaydoc v0.2.7
   Compiling zerofrom v0.1.8
   Compiling gpu-descriptor-types v0.2.0
   Compiling wgpu-types v29.0.4
   Compiling smol_str v0.2.2
   Compiling bevy_platform v0.19.1
   Compiling petgraph v0.8.3
   Compiling ron v0.12.2
   Compiling yoke v0.8.3
   Compiling bevy_utils v0.19.1
   Compiling bevy_tasks v0.19.1
   Compiling zerovec v0.11.8
   Compiling zerotrie v0.2.5
   Compiling gpu-descriptor v0.3.2
   Compiling regex v1.13.1
   Compiling font-types v0.11.3
   Compiling quick-xml v0.41.0
   Compiling tinystr v0.8.4
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling image v0.25.10
   Compiling polling v3.11.0
   Compiling half v2.7.1
   Compiling icu_collections v2.3.0
   Compiling data-encoding v2.11.1
   Compiling icu_provider v2.3.1
   Compiling wayland-client v0.31.15
   Compiling renderdoc-sys v1.1.0
   Compiling icu_properties_data v2.3.0
   Compiling wayland-scanner v0.31.11
   Compiling read-fonts v0.39.2
   Compiling wgpu-core v29.0.4
   Compiling icu_normalizer_data v2.3.0
   Compiling icu_segmenter_data v2.3.0
   Compiling litrs v1.0.0
   Compiling font-types v0.12.6
   Compiling bevy_gizmos_macros v0.19.1
   Compiling document-features v0.2.12
   Compiling bevy_material_macros v0.19.1
   Compiling wgpu v29.0.4
   Compiling icu_properties v2.3.0
   Compiling read-fonts v0.41.0
   Compiling icu_locale_fallback v2.3.0
   Compiling core_maths v0.1.1
   Compiling static_assertions v1.1.0
   Compiling thiserror v1.0.69
   Compiling rustix v0.38.44
   Compiling linebender_resource_handle v0.1.1
   Compiling parlance v0.1.1
   Compiling bevy_reflect v0.19.1
   Compiling hexasphere v18.0.0
   Compiling icu_segmenter v2.3.0
   Compiling parley_data v0.9.0
   Compiling icu_normalizer v2.3.0
   Compiling bevy_render_macros v0.19.1
   Compiling thiserror-impl v1.0.69
   Compiling offset-allocator v0.2.0
   Compiling zmij v1.0.23
   Compiling zeno v0.3.3
   Compiling byteorder v1.5.0
   Compiling yazi v0.2.1
   Compiling linux-raw-sys v0.4.15
   Compiling weak-table v0.3.2
   Compiling sys-locale v0.3.2
   Compiling serde_json v1.0.152
   Compiling accesskit v0.24.1
   Compiling alsa-sys v0.4.0
   Compiling getrandom v0.3.4
   Compiling wayland-protocols v0.32.13
   Compiling xkeysym v0.2.1
   Compiling inotify-sys v0.1.8
   Compiling arrayref v0.3.9
   Compiling itoa v1.0.18
   Compiling xcursor v0.3.11
   Compiling cursor-icon v1.2.0
   Compiling percent-encoding v2.3.2
   Compiling radsort v0.1.1
   Compiling smithay-client-toolkit v0.19.2
   Compiling gpu-allocator v0.28.0
   Compiling strict-num v0.1.1
   Compiling inflections v1.1.1
   Compiling ttf-parser v0.25.1
   Compiling tiny-skia-path v0.11.4
   Compiling gltf-derive v1.4.1
   Compiling wayland-csd-frame v0.3.0
   Compiling wayland-cursor v0.31.14
   Compiling libudev-sys v0.1.4
   Compiling x11-dl v2.21.0
   Compiling ahash v0.8.12
   Compiling ab_glyph_rasterizer v0.1.10
   Compiling tiny-skia v0.11.4
   Compiling gethostname v1.1.0
   Compiling bevy_animation_macros v0.19.1
   Compiling num-integer v0.1.47
   Compiling winit v0.30.13
   Compiling cpal v0.17.3
   Compiling grid v1.0.1
   Compiling x11rb-protocol v0.13.2
   Compiling gltf-json v1.4.1
   Compiling owned_ttf_parser v0.25.1
   Compiling ab_glyph v0.2.32
   Compiling harfrust v0.6.2
   Compiling fontique v0.9.0
   Compiling skrifa v0.42.1
   Compiling calloop v0.13.0
   Compiling as-raw-xcb-connection v1.0.1
   Compiling taffy v0.10.1
   Compiling skrifa v0.44.0
   Compiling calloop-wayland-source v0.3.0
   Compiling gltf v1.4.1
   Compiling alsa v0.11.0
   Compiling bevy_ecs v0.19.1
   Compiling bevy_math v0.19.1
   Compiling wgpu-naga-bridge v29.0.4
   Compiling naga_oil v0.22.0
   Compiling wgpu-core-deps-windows-linux-android v29.0.4
   Compiling swash v0.2.10
   Compiling parley v0.9.0
   Compiling bevy_color v0.19.1
   Compiling wayland-protocols-wlr v0.3.12
   Compiling x11rb v0.13.2
   Compiling wayland-protocols-plasma v0.3.12
   Compiling num-bigint v0.4.8
   Compiling sctk-adwaita v0.10.1
   Compiling inotify v0.11.5
   Compiling xkbcommon-dl v0.4.2
   Compiling ogg v0.8.0
   Compiling dpi v0.1.2
   Compiling dasp_sample v0.11.0
   Compiling gilrs v0.11.2
   Compiling vec_map v0.8.2
   Compiling base64 v0.22.1
   Compiling tinyvec v1.13.3
   Compiling gilrs-core v0.6.8
   Compiling num-rational v0.4.2
   Compiling lewton v0.10.2
   Compiling hound v3.5.1
   Compiling fnv v1.0.7
   Compiling rodio v0.22.2
   Compiling bevy_app v0.19.1
   Compiling bevy_scene_macros v0.19.1
   Compiling bevy_state_macros v0.19.1
   Compiling approx v0.5.1
   Compiling bevy_time v0.19.1
   Compiling bevy_transform v0.19.1
   Compiling bevy_input v0.19.1
   Compiling bevy_log v0.19.1
   Compiling bevy_a11y v0.19.1
   Compiling bevy_clipboard v0.19.1
   Compiling bevy_state v0.19.1
   Compiling bevy_diagnostic v0.19.1
   Compiling bevy_asset v0.19.1
   Compiling accesskit_winit v0.32.2
   Compiling bevy_gilrs v0.19.1
   Compiling bevy_mesh v0.19.1
   Compiling bevy_image v0.19.1
   Compiling bevy_shader v0.19.1
   Compiling bevy_audio v0.19.1
   Compiling bevy_scene v0.19.1
   Compiling bevy_window v0.19.1
   Compiling bevy_text v0.19.1
   Compiling bevy_material v0.19.1
   Compiling bevy_animation v0.19.1
   Compiling bevy_camera v0.19.1
   Compiling bevy_gizmos v0.19.1
   Compiling bevy_picking v0.19.1
   Compiling bevy_render v0.19.1
   Compiling bevy_world_serialization v0.19.1
   Compiling bevy_light v0.19.1
   Compiling bevy_sprite v0.19.1
   Compiling bevy_input_focus v0.19.1
   Compiling bevy_gltf v0.19.1
   Compiling bevy_ui v0.19.1
   Compiling bevy_winit v0.19.1
   Compiling bevy_ui_widgets v0.19.1
   Compiling bevy_core_pipeline v0.19.1
   Compiling bevy_sprite_render v0.19.1
   Compiling bevy_pbr v0.19.1
   Compiling bevy_post_process v0.19.1
   Compiling bevy_anti_alias v0.19.1
   Compiling bevy_ui_render v0.19.1
   Compiling bevy_gizmos_render v0.19.1
   Compiling bevy_internal v0.19.1
   Compiling bevy v0.19.1
   Compiling learn-bevy v0.1.0 (/home/bvpelt/Develop/learn-bevy)
    Finished `release` profile [optimized] target(s) in 6m 46s

$ ldd target/release/learn-bevy
	linux-vdso.so.1 (0x000077d9a9132000)
	libwayland-client.so.0 => /lib/x86_64-linux-gnu/libwayland-client.so.0 (0x000077d9a9105000)
	libudev.so.1 => /lib/x86_64-linux-gnu/libudev.so.1 (0x000077d9a47cd000)
	libasound.so.2 => /lib/x86_64-linux-gnu/libasound.so.2 (0x000077d9a46c3000)
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x000077d9a4695000)
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x000077d9a45ac000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x000077d9a4200000)
	/lib64/ld-linux-x86-64.so.2 (0x000077d9a9134000)
	libffi.so.8 => /lib/x86_64-linux-gnu/libffi.so.8 (0x000077d9a45a0000)
	libcap.so.2 => /lib/x86_64-linux-gnu/libcap.so.2 (0x000077d9a4593000)

$ ls target/release -altr
total 74808
-rw-rw-r--   1 bvpelt bvpelt        0 Oct 24 21:08 .cargo-lock
-rw-rw-r--   1 bvpelt bvpelt        0 Oct 24 21:08 .cargo-build-lock
-rw-rw-r--   1 bvpelt bvpelt        0 Oct 24 21:08 .cargo-artifact-lock
drwxrwxr-x   5 bvpelt bvpelt     4096 Oct 24 21:08 ..
drwxrwxr-x   2 bvpelt bvpelt     4096 Oct 24 21:08 incremental
drwxrwxr-x   2 bvpelt bvpelt     4096 Oct 24 21:08 examples
drwxrwxr-x 473 bvpelt bvpelt    28672 Oct 24 21:08 .fingerprint
drwxrwxr-x  94 bvpelt bvpelt     4096 Oct 24 21:08 build
-rwxrwxr-x   2 bvpelt bvpelt 76497240 Oct 24 21:15 learn-bevy
drwxrwxr-x   2 bvpelt bvpelt   118784 Oct 24 21:15 deps
-rw-rw-r--   1 bvpelt bvpelt     3603 Oct 24 21:15 learn-bevy.d
drwxrwxr-x   7 bvpelt bvpelt     4096 Oct 24 21:15 .

```

#### Release the executable

Run ./release-package.bash after building the production release to get the executable and the assets in the same directory.

```bash
$ ./release-package.bash 
    Finished `release` profile [optimized] target(s) in 0.22s
Release is in ./dist
```

### Building for development

#### Build for development
The default build command builds for development.

```bash
$ cargo build
   Compiling bevy_diagnostic v0.19.1
   Compiling bevy_asset v0.19.1
   Compiling bevy_image v0.19.1
   Compiling bevy_mesh v0.19.1
   Compiling bevy_shader v0.19.1
   Compiling bevy_audio v0.19.1
   Compiling bevy_scene v0.19.1
   Compiling bevy_window v0.19.1
   Compiling bevy_text v0.19.1
   Compiling bevy_camera v0.19.1
   Compiling bevy_material v0.19.1
   Compiling bevy_gizmos v0.19.1
   Compiling bevy_picking v0.19.1
   Compiling bevy_render v0.19.1
   Compiling bevy_world_serialization v0.19.1
   Compiling bevy_light v0.19.1
   Compiling bevy_sprite v0.19.1
   Compiling bevy_input_focus v0.19.1
   Compiling bevy_animation v0.19.1
   Compiling bevy_ui v0.19.1
   Compiling bevy_winit v0.19.1
   Compiling bevy_gltf v0.19.1
   Compiling bevy_ui_widgets v0.19.1
   Compiling bevy_core_pipeline v0.19.1
   Compiling bevy_sprite_render v0.19.1
   Compiling bevy_pbr v0.19.1
   Compiling bevy_post_process v0.19.1
   Compiling bevy_anti_alias v0.19.1
   Compiling bevy_ui_render v0.19.1
   Compiling bevy_gizmos_render v0.19.1
   Compiling bevy_internal v0.19.1
   Compiling bevy v0.19.1
   Compiling learn-bevy v0.1.0 (/home/bvpelt/Develop/learn-bevy)
    Finished `dev` profile [optimized + debuginfo] target(s) in 5m 20s

$ ldd target/debug/learn-bevy
	linux-vdso.so.1 (0x000079a439204000)
	libwayland-client.so.0 => /lib/x86_64-linux-gnu/libwayland-client.so.0 (0x000079a4391d7000)
	libudev.so.1 => /lib/x86_64-linux-gnu/libudev.so.1 (0x000079a431fcd000)
	libasound.so.2 => /lib/x86_64-linux-gnu/libasound.so.2 (0x000079a431ec3000)
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x000079a431e95000)
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x000079a431dac000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x000079a431a00000)
	/lib64/ld-linux-x86-64.so.2 (0x000079a439206000)
	libffi.so.8 => /lib/x86_64-linux-gnu/libffi.so.8 (0x000079a431da0000)
	libcap.so.2 => /lib/x86_64-linux-gnu/libcap.so.2 (0x000079a431d93000)

```

#### Build and run
```bash
$ cargo dev
   Compiling unicode-ident v1.0.26
   Compiling cfg-if v1.0.5
   Compiling log v0.4.34
   Compiling equivalent v1.0.2
   Compiling smallvec v1.16.3
   Compiling pin-project-lite v0.2.17
   Compiling foldhash v0.2.0
   Compiling serde_core v1.0.229
   Compiling libc v0.2.190
   Compiling libm v0.2.16
   Compiling parking v2.2.1
   Compiling allocator-api2 v0.2.21
   Compiling arrayvec v0.7.8
   Compiling indexmap v2.14.2
   Compiling once_cell v1.21.4
   Compiling rand_core v0.10.1
   Compiling proc-macro2 v1.0.107
   Compiling futures-core v0.3.34
   Compiling hashbrown v0.17.1
   Compiling memchr v2.8.3
   Compiling crossbeam-utils v0.8.23
   Compiling fastrand v2.5.0
   Compiling futures-io v0.3.34
   Compiling event-listener v5.4.2
   Compiling raw-window-handle v0.6.2
   Compiling foldhash v0.1.5
   Compiling futures-lite v2.6.1
   Compiling slab v0.4.12
   Compiling event-listener-strategy v0.5.4
   Compiling quote v1.0.47
   Compiling concurrent-queue v2.5.0
   Compiling hashbrown v0.15.5
   Compiling cpufeatures v0.3.1
   Compiling typewit v1.15.2
   Compiling syn v3.0.7
   Compiling syn v2.0.119
   Compiling const_panic v0.2.17
   Compiling async-channel v2.5.0
   Compiling toml_edit v0.25.17+spec-1.1.0
   Compiling chacha20 v0.10.1
   Compiling num-traits v0.2.19
   Compiling linux-raw-sys v0.12.1
   Compiling spin v0.10.1
   Compiling typeid v1.0.3
   Compiling thread_local v1.1.10
   Compiling async-task v4.7.1
   Compiling atomic-waker v1.1.2
   Compiling disqualified v1.1.0
   Compiling getrandom v0.4.3
   Compiling fixedbitset v0.5.7
   Compiling inventory v0.3.25
   Compiling rand v0.10.3
   Compiling downcast-rs v2.0.2
   Compiling bevy_ptr v0.19.1
   Compiling slotmap v1.1.1
   Compiling async-executor v1.14.0
   Compiling crossbeam-queue v0.3.14
   Compiling hashbrown v0.16.1
   Compiling uuid v1.28.0
   Compiling erased-serde v0.4.10
   Compiling nonmax v0.5.5
   Compiling bumpalo v3.20.3
   Compiling tracing-core v0.1.36
   Compiling either v1.19.0
   Compiling crossbeam-channel v0.5.17
   Compiling itertools v0.14.0
   Compiling rand_distr v0.6.0
   Compiling piper v0.2.5
   Compiling libloading v0.8.9
   Compiling sysinfo v0.38.4
   Compiling async-lock v3.4.2
   Compiling const-fnv1a-hash v1.1.0
   Compiling blocking v1.7.0
   Compiling constant_time_eq v0.4.2
   Compiling futures-task v0.3.34
   Compiling blake3 v1.8.7
   Compiling stackfuture v0.3.1
   Compiling async-broadcast v0.7.2
   Compiling atomicow v1.2.0
   Compiling simd-adler32 v0.3.10
   Compiling adler2 v2.0.1
   Compiling aho-corasick v1.1.5
   Compiling async-fs v2.2.0
   Compiling regex-syntax v0.8.11
   Compiling miniz_oxide v0.9.1
   Compiling crc32fast v1.5.2
   Compiling fdeflate v0.3.7
   Compiling miniz_oxide v0.8.9
   Compiling pxfm v0.1.30
   Compiling flate2 v1.1.10
   Compiling const_soft_float v0.1.4
   Compiling synstructure v0.14.0
   Compiling bevy_macro_utils v0.19.1
   Compiling encase_derive_impl v0.12.1
   Compiling constgebra v0.1.4
   Compiling euclid v0.22.14
   Compiling bevy_ecs_macro_logic v0.19.1
   Compiling regex-automata v0.4.18
   Compiling twox-hash v2.1.5
   Compiling svg_fmt v0.4.5
   Compiling byteorder-lite v0.1.0
   Compiling guillotiere v0.6.2
   Compiling ruzstd v0.8.3
   Compiling moxcms v0.8.1
   Compiling lazy_static v1.5.1
   Compiling bevy_mikktspace v1.0.0
   Compiling rectangle-pack v0.4.2
   Compiling stable_deref_trait v1.2.1
   Compiling matchers v0.2.0
   Compiling sharded-slab v0.1.7
   Compiling tracing-log v0.2.0
   Compiling termcolor v1.4.1
   Compiling nu-ansi-term v0.50.3
   Compiling unicode-width v0.2.2
   Compiling bit-vec v0.9.1
   Compiling codespan-reporting v0.13.1
   Compiling hexf-parse v0.2.1
   Compiling bit-set v0.9.1
   Compiling rustc-hash v1.1.0
   Compiling writeable v0.6.4
   Compiling dlib v0.5.3
   Compiling litemap v0.8.3
   Compiling memmap2 v0.9.11
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.14
   Compiling wayland-sys v0.31.11
   Compiling parking_lot_core v0.9.12
   Compiling ash v0.38.0+1.3.281
   Compiling presser v0.3.1
   Compiling utf8_iter v1.0.4
   Compiling downcast-rs v1.2.1
   Compiling scoped-tls v1.0.1
   Compiling parking_lot v0.12.5
   Compiling codespan-reporting v0.12.0
   Compiling regex v1.13.1
   Compiling ordered-float v5.5.0
   Compiling quick-xml v0.41.0
   Compiling profiling v1.0.18
   Compiling data-encoding v2.11.1
   Compiling renderdoc-sys v1.1.0
   Compiling icu_locale_fallback_data v2.3.0
   Compiling icu_properties_data v2.3.0
   Compiling icu_normalizer_data v2.3.0
   Compiling icu_segmenter_data v2.3.0
   Compiling core_maths v0.1.1
   Compiling linebender_resource_handle v0.1.1
   Compiling parlance v0.1.1
   Compiling static_assertions v1.1.0
   Compiling offset-allocator v0.2.0
   Compiling byteorder v1.5.0
   Compiling yazi v0.2.1
   Compiling linux-raw-sys v0.4.15
   Compiling weak-table v0.3.2
   Compiling zeno v0.3.3
   Compiling sys-locale v0.3.2
   Compiling zmij v1.0.23
   Compiling accesskit v0.24.1
   Compiling itoa v1.0.18
   Compiling percent-encoding v2.3.2
   Compiling wayland-scanner v0.31.11
   Compiling radsort v0.1.1
   Compiling strict-num v0.1.1
   Compiling ttf-parser v0.25.1
   Compiling xkeysym v0.2.1
   Compiling arrayref v0.3.9
   Compiling xcursor v0.3.11
   Compiling cursor-icon v1.2.0
   Compiling serde_json v1.0.152
   Compiling ab_glyph_rasterizer v0.1.10
   Compiling alsa-sys v0.4.0
   Compiling getrandom v0.3.4
   Compiling inotify-sys v0.1.8
   Compiling num-integer v0.1.47
   Compiling grid v1.0.1
   Compiling x11rb-protocol v0.13.2
   Compiling as-raw-xcb-connection v1.0.1
   Compiling taffy v0.10.1
   Compiling num-bigint v0.4.8
   Compiling x11-dl v2.21.0
   Compiling owned_ttf_parser v0.25.1
   Compiling ab_glyph v0.2.32
   Compiling libudev-sys v0.1.4
   Compiling ogg v0.8.0
   Compiling dasp_sample v0.11.0
   Compiling vec_map v0.8.2
   Compiling base64 v0.22.1
   Compiling tinyvec v1.13.3
   Compiling dpi v0.1.2
   Compiling num-rational v0.4.2
   Compiling lewton v0.10.2
   Compiling hound v3.5.1
   Compiling fnv v1.0.7
   Compiling approx v0.5.1
   Compiling bytemuck_derive v1.12.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling futures-macro v0.3.34
   Compiling zerofrom-derive v0.1.8
   Compiling encase_derive v0.12.1
   Compiling derive_more-impl v2.1.1
   Compiling variadics_please v1.1.0
   Compiling bevy_reflect_derive v0.19.1
   Compiling assert_type_match v0.1.1
   Compiling bevy_ecs_macros v0.19.1
   Compiling bevy_derive v0.19.1
   Compiling zerocopy-derive v0.8.62
   Compiling tracing-attributes v0.1.31
   Compiling bytemuck v1.25.2
   Compiling bitflags v2.13.2
   Compiling rustix v1.1.5
   Compiling nix v0.31.3
   Compiling futures-util v0.3.34
   Compiling ctrlc v3.5.2
   Compiling thiserror v2.0.21
   Compiling bevy_asset_macros v0.19.1
   Compiling encase v0.12.1
   Compiling glam v0.32.1
   Compiling png v0.18.1
   Compiling polling v3.11.0
   Compiling async-io v2.6.0
   Compiling image v0.25.10
   Compiling ktx2 v0.5.0
   Compiling bevy_encase_derive v0.19.1
   Compiling serde v1.0.229
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling zerovec-derive v0.11.6
   Compiling wgpu-types v29.0.4
   Compiling smol_str v0.2.2
   Compiling bevy_platform v0.19.1
   Compiling petgraph v0.8.3
   Compiling bevy_utils v0.19.1
   Compiling ron v0.12.2
   Compiling hexasphere v18.0.0
   Compiling spirv v0.4.0+sdk-1.4.341.0
   Compiling displaydoc v0.2.7
   Compiling gpu-descriptor-types v0.2.0
   Compiling gpu-descriptor v0.3.2
   Compiling wayland-backend v0.3.17
   Compiling gpu-allocator v0.28.0
   Compiling derive_more v2.1.1
   Compiling bevy_reflect v0.19.1
   Compiling bevy_tasks v0.19.1
   Compiling tracing v0.1.44
   Compiling zerocopy v0.8.62
   Compiling tracing-subscriber v0.3.23
   Compiling font-types v0.11.3
   Compiling wayland-client v0.31.15
   Compiling read-fonts v0.39.2
   Compiling font-types v0.12.6
   Compiling bevy_gizmos_macros v0.19.1
   Compiling bevy_material_macros v0.19.1
   Compiling read-fonts v0.41.0
   Compiling thiserror-impl v1.0.69
   Compiling half v2.7.1
   Compiling bevy_render_macros v0.19.1
   Compiling naga v29.0.4
   Compiling wayland-protocols v0.32.13
   Compiling rustix v0.38.44
   Compiling wayland-cursor v0.31.14
   Compiling bevy_ecs v0.19.1
   Compiling bevy_math v0.19.1
   Compiling skrifa v0.44.0
   Compiling yoke v0.8.3
   Compiling zerovec v0.11.8
   Compiling tinystr v0.8.4
   Compiling icu_locale_core v2.3.0
   Compiling wgpu-naga-bridge v29.0.4
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling icu_collections v2.3.0
   Compiling bevy_color v0.19.1
   Compiling icu_provider v2.3.1
   Compiling wgpu-hal v29.0.4
   Compiling naga_oil v0.22.0
   Compiling icu_properties v2.3.0
   Compiling wgpu-core-deps-windows-linux-android v29.0.4
   Compiling wgpu-core v29.0.4
   Compiling icu_locale_fallback v2.3.0
   Compiling icu_segmenter v2.3.0
   Compiling bevy_app v0.19.1
   Compiling parley_data v0.9.0
   Compiling icu_normalizer v2.3.0
   Compiling bevy_time v0.19.1
   Compiling bevy_diagnostic v0.19.1
   Compiling bevy_transform v0.19.1
   Compiling bevy_asset v0.19.1
   Compiling bevy_input v0.19.1
   Compiling bevy_log v0.19.1
   Compiling wgpu v29.0.4
   Compiling bevy_image v0.19.1
   Compiling bevy_mesh v0.19.1
   Compiling bevy_shader v0.19.1
   Compiling harfrust v0.6.2
   Compiling bevy_window v0.19.1
   Compiling bevy_camera v0.19.1
   Compiling bevy_material v0.19.1
   Compiling bevy_gizmos v0.19.1
   Compiling bevy_picking v0.19.1
   Compiling fontique v0.9.0
   Compiling bevy_light v0.19.1
   Compiling skrifa v0.42.1
   Compiling bevy_render v0.19.1
   Compiling parley v0.9.0
   Compiling bevy_clipboard v0.19.1
   Compiling thiserror v1.0.69
   Compiling swash v0.2.10
   Compiling calloop v0.13.0
   Compiling bevy_text v0.19.1
   Compiling calloop-wayland-source v0.3.0
   Compiling wayland-protocols-wlr v0.3.12
   Compiling wayland-csd-frame v0.3.0
   Compiling bevy_sprite v0.19.1
   Compiling tiny-skia-path v0.11.4
   Compiling gltf-derive v1.4.1
   Compiling tiny-skia v0.11.4
   Compiling smithay-client-toolkit v0.19.2
   Compiling gltf-json v1.4.1
   Compiling bevy_input_focus v0.19.1
   Compiling bevy_a11y v0.19.1
   Compiling gethostname v1.1.0
   Compiling bevy_animation_macros v0.19.1
   Compiling x11rb v0.13.2
   Compiling bevy_animation v0.19.1
   Compiling gltf v1.4.1
   Compiling sctk-adwaita v0.10.1
   Compiling bevy_world_serialization v0.19.1
   Compiling bevy_core_pipeline v0.19.1
   Compiling wayland-protocols-plasma v0.3.12
   Compiling ahash v0.8.12
   Compiling inotify v0.11.5
   Compiling xkbcommon-dl v0.4.2
   Compiling alsa v0.11.0
   Compiling cpal v0.17.3
   Compiling winit v0.30.13
   Compiling gilrs-core v0.6.8
   Compiling bevy_gltf v0.19.1
   Compiling bevy_ui v0.19.1
   Compiling accesskit_winit v0.32.2
   Compiling gilrs v0.11.2
   Compiling bevy_sprite_render v0.19.1
   Compiling bevy_pbr v0.19.1
   Compiling rodio v0.22.2
   Compiling bevy_state_macros v0.19.1
   Compiling bevy_scene_macros v0.19.1
   Compiling bevy_state v0.19.1
   Compiling bevy_audio v0.19.1
   Compiling bevy_ui_widgets v0.19.1
   Compiling bevy_ui_render v0.19.1
   Compiling bevy_scene v0.19.1
   Compiling bevy_post_process v0.19.1
   Compiling bevy_anti_alias v0.19.1
   Compiling bevy_gilrs v0.19.1
   Compiling bevy_winit v0.19.1
   Compiling bevy_gizmos_render v0.19.1
   Compiling bevy_internal v0.19.1
   Compiling bevy_dylib v0.19.1
   Compiling bevy v0.19.1
   Compiling learn-bevy v0.1.0 (/home/bvpelt/Develop/learn-bevy)
    Finished `dev` profile [optimized + debuginfo] target(s) in 8m 59s
     Running `target/debug/learn-bevy`
2026-10-24T18:57:19.816502Z  INFO bevy_render::renderer: AdapterInfo { name: "Intel(R) Graphics (MTL)", vendor: 32902, device: 32069, device_type: IntegratedGpu, device_pci_bus_id: "0000:00:02.0", driver: "Intel open-source Mesa driver", driver_info: "Mesa 25.2.8-0ubuntu0.24.04.4", backend: Vulkan, subgroup_min_size: 8, subgroup_max_size: 32, transient_saves_memory: false }
2026-10-24T18:57:19.931613Z  INFO bevy_pbr::cluster: GPU clustering is supported on this device.
2026-10-24T18:57:19.931645Z  INFO bevy_render::batching::gpu_preprocessing: GPU preprocessing is fully supported on this device.
2026-10-24T18:57:19.932040Z  INFO bevy_winit::system: Creating new window learn-bevy (65v0)
2026-10-24T18:57:19.932228Z  INFO winit::platform_impl::linux::x11::window: Guessed window scale factor: 1


$ ldd target/debug/learn-bevy
	linux-vdso.so.1 (0x000078d67df93000)
	libbevy_dylib-e69a47dae682ca9e.so => not found
	libstd-7e719de9c4abc98f.so => not found
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x000078d67df48000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x000078d67d800000)
	/lib64/ld-linux-x86-64.so.2 (0x000078d67df95000)

$ ls target/debug/ -altr
total 3772612
drwxrwxr-x    2 bvpelt bvpelt       4096 Oct 10 20:44 examples
-rw-rw-r--    1 bvpelt bvpelt          0 Oct 10 20:44 .cargo-lock
-rw-rw-r--    1 bvpelt bvpelt          0 Oct 10 20:44 .cargo-build-lock
-rw-rw-r--    1 bvpelt bvpelt          0 Oct 10 20:50 .cargo-artifact-lock
drwxrwxr-x  280 bvpelt bvpelt      20480 Oct 24 20:43 build
-rwxrwxr-x    2 bvpelt bvpelt 1940906584 Oct 24 20:57 libbevy_dylib.so
drwxrwxr-x 2657 bvpelt bvpelt     139264 Oct 24 21:02 .fingerprint
drwxrwxr-x   22 bvpelt bvpelt       4096 Oct 24 21:07 incremental
-rwxrwxr-x    2 bvpelt bvpelt 1921845432 Oct 24 21:07 learn-bevy
drwxrwxr-x    2 bvpelt bvpelt     430080 Oct 24 21:07 deps
drwxrwxr-x    7 bvpelt bvpelt       4096 Oct 24 21:07 .
-rw-rw-r--    1 bvpelt bvpelt       3601 Oct 24 21:07 learn-bevy.d
drwxrwxr-x    5 bvpelt bvpelt       4096 Oct 24 21:08 ..
```