# Changelog

<!--
Please add your PR to the changelog! Choose from a top level and bottom
level category, then write your changes like follows:

- Describe your change in a user friendly format by @yourslug in [#99999](https://github.com/cohaereo/alkahest/pull/99999)

You can add additional user facing information if it's a major breaking change. You can use the following to help:

```diff
- Old code
+ New code
```

Change types:
    - `✨ Major Changes` for version-defining changes.
    - `Added` for new features.
    - `Changed` for changes in existing functionality.
    - `Deprecated` for soon-to-be removed features.
    - `Removed` for now removed features.
    - `Fixed` for any bug fixes.
    - `Security` in case of vulnerabilities.

-->

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)

## Unreleased / Rolling Release

### Added
- Added Routes debug object. Routes can be traversed, exported, and shared(manually). They can swap to the correct activity, but currently must be reloaded afterwards. This will be fixed in a future version. by @Froggy618157725 in [#23](https://github.com/cohaereo/alkahest/pull/23)
- Added Freeroam Activity view in Activity Selector by @Froggy618157725 in [#25](https://github.com/cohaereo/alkahest/pull/25)

### Fixed
- Fixed Activity issue introduced in routes. Internal tracking is now nicer. by @Froggy618157725 in [#24](https://github.com/cohaereo/alkahest/pull/24)

## 0.4.1 - 2024-03-27

### ✨ Major Changes
- ⚠ Alkahest is no longer compatible with Avvy's Alkgui. The features provided by Alkgui are now available in Alkahest itself. 
- Reworked the map loading mechanism to allow for maps to be loaded individually by @cohaereo
- Added a map and activity browser by @cohaereo
- Added a game installation detector by @cohaereo

### Added

- Added the ability to load maps from packages by name (eg. `throneworld` or `dungeon_prophecy`) through the `-p` argument by @cohaereo
- Added draw_crosshair to the config by @Froggy618157725 in [#21](https://github.com/cohaereo/alkahest/pull/21)
- Added 'I' Key shortcut to swap to previous map by @Froggy618157725 in [#21](https://github.com/cohaereo/alkahest/pull/21)
- Added Controls under Help Menu @Froggy618157725 in [#22](https://github.com/cohaereo/alkahest/pull/22)
- Added version information to panic log by @cohaereo
- Package directory is now persisted in the config by @cohaereo

### Deprecated
- Passing a package file is deprecated in favor of the `-p` switch. In the future, Alkahest will only accept package directory paths

### Changed

- Create window before initializing the package manager by @cohaereo
- Rework transparent(_advanced) scopes by @cohaereo
- Change allocator to [mimalloc](https://github.com/microsoft/mimalloc) by @cohaereo
- Configuration files are now stored in the system config directories (see [directories API](https://docs.rs/directories/5.0.1/directories/struct.ProjectDirs.html#method.config_dir)) by @cohaereo
- The tag dumper and bulk texture dumper windows are now hidden by default, and can be toggled from the View menu by @cohaereo

### Fixed

- Fixed the GitHub URL for stable releases by @cohaereo
- Copy missing sections in nightly changelog diffs by @cohaereo
- Fixed build date/timestamp generation by @cohaereo
- Reset update check indicator timer when starting a new check by @cohaereo
- Fixed a crash when creating render targets with a zero size by @cohaereo
- Fixed a map loading crash on Disjunction by @cohaereo

## 0.4.0 - 2024-02-18

### Added

- Auto updater by @cohaereo
- Control lights as an FPS camera by @cohaereo

### Changed

- Enable TFX bytecode evaluation by default by @cohaereo
- Changed the parsing system from `binrw` to [tiger-parse](https://github.com/v4nguard/tiger-parse) by @cohaereo

### Fixed

- Fixed cubemap level selection that made surfaces too glossy by @cohaereo
- Lights now obey the Visible component
- Fixed a TFX parameter that was causing some lights to not be visible by @cohaereo
- Fixed depth linearization in the transparent scope by @cohaereo

### Removed

- Removed pointless world ID component from static instances by @cohaereo

## 0.3.0 - 2024-01-25

### Added

- Add Sphere Utility tool by @Froggy618157725 in [#7](https://github.com/cohaereo/alkahest/pull/7)
- Basic technique viewer with texture list by @cohaereo
- Implement map resources Unk80808246 and Unk80806ac2 by @cohaereo
- Add Delete Button on Inspector Panel by @Froggy618157725 in [#11](https://github.com/cohaereo/alkahest/pull/11)
- Show Havok shapes for 80809121 (Push Surfaces) by @DeltaDesigns in [#9](https://github.com/cohaereo/alkahest/pull/9)
- Add Global Utility Objects by @Froggy618167725 in [#12](https://github.com/cohaereo/alkahest/pull/12)
- Lazy entity updating by @cohaereo
- Global entity tag by @cohaereo
- Add Beacon Utility tool by @Froggy618157725 in [#13](https://github.com/cohaereo/alkahest/pull/13)
- Use `fs-err` wrapper for more descriptive filesystem error messages by @cohaereo in [#14](https://github.com/cohaereo/alkahest/pull/14)
- Print version information in console by @cohaereo
- Add a window and taskbar icon by @cohaereo
- Make Utility Objects work with the picker by @Froggy618157725 in [#16](https://github.com/cohaereo/alkahest/pull/16)
- Variable width line rendering by @cohaereo
- Partial light_specular_ibl implementation for cubemap support in deferred_shading_no_atm by @cohaereo
- Ability to query depth buffer by @Froggy618157725 in [#17](https://github.com/cohaereo/alkahest/pull/17)
- Added crosshair (off by default) by @Froggy618157725 in [#17](https://github.com/cohaereo/alkahest/pull/17)
- Utility Objects now flash briefly while selected by @Froggy618157725 in [#17](https://github.com/cohaereo/alkahest/pull/17)
- Add about menu by @cohaereo in [#19](https://github.com/cohaereo/alkahest/pull/19)
- Add changelog window by @cohaereo in [#19](https://github.com/cohaereo/alkahest/pull/19)
- Added GitHub actions nightly build workflow
- Add matcap pseudo-shading to custom debug shapes by @cohaereo

### Changed

- Spruce up Camera Controls by @Froggy618157725 in [#8](https://github.com/cohaereo/alkahest/pull/8)
- Changed the matcap texture to one with better lighting by @cohaereo
- Ruler is spawned extending from where you're looking to you by @Froggy618157725 in [#17](https://github.com/cohaereo/alkahest/pull/17)
- Sphere is spawned at 24m away, or on the first map piece encountered by @Froggy618157725 in [#17](https://github.com/cohaereo/alkahest/pull/17)
- Update egui to 0.25 by @cohaereo

### Removed

- Removed CTRL+Q quit shortcut by @Froggy618157725 in [#8](https://github.com/cohaereo/alkahest/pull/8)
- Disable render globals prints by @cohaereo

### Fixed

- Fix camera right and up axis by @cohaereo
- Fix Utility Visibility by @Froggy618157725 in [#10](https://github.com/cohaereo/alkahest/pull/10)
- Fixed Sphere Icon in Inspector Panel by @Froggy618167725 in [#12](https://github.com/cohaereo/alkahest/pull/12)
- Fixed shader warnings by @cohaereo
- Fix pickbuffer not respecting d3d mapped row pitch by @cohaereo
- Fixed Selector behavior on screens with scaling factors @Froggy618157725 in [#16](https://github.com/cohaereo/alkahest/pull/16)
- Fix cubemap view not rotating by @cohaereo
- Fixed a potential GUI memory leak when using unmanaged DirectX textures by @cohaereo