mod renderer_gpu;

fn main() {
    pollster::block_on(renderer_gpu::run());
}
