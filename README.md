# Oxidized Pixel Dungeon

Oxidized Pixel Dungeon is an attempt to reimplement [00-Evan's Shattered Pixel Dungeon](https://github.com/00-Evan/shattered-pixel-dungeon) in Rust and Bevy.

The project is in pre-alpha state and currently unplayable. It contains demos on [examples](examples/) directory. In order to run demos, make sure you have installed Rust toolchain and then run examples as so:

```bash
cargo run --example EXAMPLE_NAME --features bevy/dynamic_linking
```

It should take several minutes to compile. `--features bevy/dynamic_linking` is required to skip rebuilding Bevy in the subsequent compilation so it significantly reduces compilation duration.

 > [!WARNING]
 > If the performance is poor, Bevy might have fallen back to CPU to render with Vulkan backend (as Bevy defaults to use Vulkan). You can also observe this with console logs where it says `device_type: Cpu` and `backend: Vulkan`.
 > 
 > You can switch back to OpenGL by using `WGPU_BACKEND=gl` environment variable.
 > 
 > ```bash
 > WGPU_BACKEND=gl cargo run --example EXAMPLE_NAME --features bevy/dynamic_linking
 > ```

## Demos

You can also see the demos in action without downloading anything. Here's what's working:

https://github.com/user-attachments/assets/0509252d-f003-4ad5-84ce-2937ec5c8ba1


https://github.com/user-attachments/assets/bd87624b-0625-45f7-adee-7235d0ea655c


https://github.com/user-attachments/assets/95be4862-5380-4eb5-8462-69b2daba1777
