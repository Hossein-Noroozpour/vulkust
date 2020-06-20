<img align="left" alt="" src="https://github.com/Hossein-Noroozpour/vulkust-static-files/raw/master/vulkust_logo.png" height="150" />

# Vulkust (SUSPENDED! go to Amethyst)

An experimental tiny engine for **Vulkan in Rust**.

<br>
<br>

## 1- Missions

 1. **Safety:**
Only FFI related stuff are `unsafe` and in addition several other validation on runtime occur for checking correct API usage and for performance reason it happens only in debug mode.
 2. **Quality:** It's gonna use highest available hardware features for providing better graphics, some of the feature may not be appropriate for poor devices (e.g. deferred rendering).
 3. **Performance:** It does every thing to bring highest possible performance.
 4. **Fast development:** Easy to develop new features.
 5. **Small executable binary file:** Abstained from chunky big external dependencies.
 6. **Cross-platform:** It works for Linux, Android, Windows, MacOS, iOS, (note: current focus is mostly on Linux and Android but other platforms get support after a while after a new feature added)
 7. **Maintainable code**

### 2- Status:

It is under lots of changes.

(Until current milestone feature is not implemented it is not stable.)

### 2-1- Current milestone

#### 2-1-1- Current features:

- Cross platform (Linux, Windows, MacOs, Android, iOS)
- Deferred Rendering
- Multithreaded Rendering
- Support current version of GX3D.
- Supports font rendering
- UI system (early version)
- A safe interface over Vulkan
- Vulkan Memory management (early version)
- Occlusion culling (Frustum culling)
- Shadowing
- Cascaded Shadow
- Shadow Accumulation
- Soft Shadowing
- SSAO
- PBR lighting
- Texture mipmapping (2D)

#### 2-1-2- Underdeveloment features:

- Forward transparent pass

#### 2-1-3- In near future:

- Auto exposure
- Light blooming
- Light streak
- PBR IBL
- More UI widgets
- Supporting GLTF

## 3- Examples

- First of all master branch may become unstable or even uncompilable.
  (I'm gonna create a release branch in the first stable version)
- You must have **glslangValidator** in you PATH environment variable.
- For **iOS** and **Android**, you must have
  [vulkust-ios](https://github.com/Hossein-Noroozpour/vulkust-ios) and
  [vulkust-android](https://github.com/Hossein-Noroozpour/vulkust-android)
  projects next to the vulkust root.

## 4- License

- You can do whatever you want to do with it, but every consequences are on you.
- You can not say that you have written this.
- Make some promotion for this project and me (I'm a job seeker).
