<img align="left" alt="" src="https://github.com/Hossein-Noroozpour/vulkust-static-files/raw/master/vulkust_logo.png" height="150" />

# Vulkust (unstable)
An experimental tiny engine for **Vulkan in Rust**.

<br>
<br>

## Missions
 1. Safty
 1. Performance
 2. Fast development
 3. Small executable binary file
 4. Cross-platform
 5. Maintainable code

## Progress
Measurements are based on the current reaching milestone, not total road-map.

| Platforms             | Progress |
| --------------------- |:--------:|
| Linux (Ubuntu, Fedora)| 70%      |
| Android               | 60%      |
| Windows               | 40%      |
| Macos                 | 60%      |
| iOS                   | 60%      |

## FAQ
- Why Rust?
  - Because I like it!
  - Lots of other reasons that lots of other guys have already given.
- Does it work?
  - If your question point to the current version in the repository, Maybe!
  - But in the end of each milestones I'm gonna publish it in [crates.io](https://crates.io). (except the first version that was only for name-reserving purpose.)
- Examples need a binary file, How can I have one?
  - Yes of course it needs a binary file, Vulkust stores all of its assets in a file and retrieves them in runtime, right now the binaries have some sensitive data, but I'm gonna remove them soon for guys who wants to see how it works.
- Why you didn't use available crates and developing everything from scratch?
  - Because I like it this way!
  - There is lots of prestigious crates out there (e.g: [Vulkano](https://github.com/vulkano-rs/vulkano), [GFX-RS](https://github.com/gfx-rs/gfx) and [etc](https://github.com/rust-unofficial/awesome-rust#graphics)) but I didn't like to add them because of missions.
  - This is not a strict rule e.g I will use an available video and audio decoder crates, whenever I reach to implementation of audio and video part.


## License
You can do whatever you want to do with it and every consequences are on you, **But** If you used it and it was useful for you, please make an acknowledgment and promotion for this project and me, I'm really need that because I'm currently seeking for a job in the graphic and game programing fields.
