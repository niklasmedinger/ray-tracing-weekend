<h1 align="center">Ray Tracing in One Weekend</h1>

<div align="center">
 <a href="https://github.com/niklasmedinger/ray-tracing-weekend/actions/workflows/CI.yml">
        <img src="https://github.com/niklasmedinger/ray-tracing-weekend/actions/workflows/CI.yml/badge.svg" alt="GitHub branch checks state">
 </a>
 |
 <a href="https://github.com/niklasmedinger/ray-tracing-weekend/actions/workflows/Bencher.yml">
        <img src="https://github.com/niklasmedinger/ray-tracing-weekend/actions/workflows/Bencher.yml/badge.svg" alt="GitHub branch checks state">
 </a>
</div>

This is my Rust implementation of the ray tracer developed in the
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
book series. The goal of this project is to learn a bit about ray tracing, Rust,
and benchmarking in Rust.

## Rendered Example Scenes

Here a few selected scenes from the book series rendered with this implementation.

| ![image](./assets/final.png) |
|:--:|
| *The final scene of the first book.* |

--------------------------------------------------------------------------------


| ![image](./assets/metal.png) |
|:--:|
| *A scene with a lambertian sphere in the center and two metal spheres left and right of it.* |

--------------------------------------------------------------------------------

| ![image](./assets/hollow_glass.png) |
|:--:|
| *A scene with a lambertian sphere in the center, a  metal sphere on the left, and a dieletric sphere on the right. The dieletric sphere contains another dieletric sphere, modelling a hollow glass sphere with air inside.* |

--------------------------------------------------------------------------------

| ![image](./assets/viewport.png) |
|:--:|
| *The scene with two metal spheres from an alternative viewpoint.* |


--------------------------------------------------------------------------------

| ![image](./assets/defocus.png) |
|:--:|
| *The same scene with defocus (i.e., depth of field) and a smaller field-of-view.* |


To render the scenes yourself, install [Rust](https://www.rust-lang.org/tools/install) and use
```
cargo run --example scene > scene.ppm --release
```
to render the file `scene` in the example folder into the file `scene.ppm`.
Take a look at the `./examples` folder for sample scenes. Use an image viewer of your choice
which can view `.ppm` files or, if you have `convert` from [ImageMagick](https://imagemagick.org/script/convert.php) installed,
convert them to `.png` files via
```
convert scene.ppm scene.png
```

## Benchmarking
I used this project to experiment a bit with benchmarking in Rust. There are
four popular options for benchmarking in Rust: [libtest bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html), [Criterion](https://github.com/bheisler/criterion.rs) and [Iai](https://github.com/bheisler/iai), and [Divan](https://github.com/nvzqz/divan). Since libtest requires the nightly toolchain,
it is often not used in favor of crates like Criterion, Iai, and Divan, which
work on stable rust.

Both Criterion and Divan are statistics-driven benchmarking libraries which allow
their users to measure the latency and throughput of their projects. Iai is an
experimental one-shot benchmarking library that uses Cachegrind to measure
the cache accesses of your code. For futher information about the respective
libraries, I recommend their githubs, crate documentation, and, for Divan,
this [blogpost](https://nikolaivazquez.com/blog/divan/).

I ended up choosing Criterion and Iai due to their compatability with [Bencher](https://github.com/bencherdev/bencher);
another benchmarking tool I'm exploring in this project.

To bench the ray tracer, I'm using two macro benchmarks and two micro benchmarks:
* A complete render of the [_hollow\_glass_](./examples/hollow_glass.rs) scene.
* A complete render of a grid of spheres.
* TODO: A single pixel from the [_hollow\_glass_](./examples/hollow_glass.rs) scene.
* TODO: A single pixel from the grid of spheres scene.
See the [benches folder](./benches/) for the code of these benchmarks. Each
benchmark is executed with both Criterion and Iai.

TODO: Paragraph about comparing two consecutive commits via actions.

TODO: Paragraph about continuous statistical benchmarking via Bencher.

Here, you can see the benchmarking results over time for this project.
<a href="https://bencher.dev/perf/raytracing-weekend?lower_value=false&upper_value=false&lower_boundary=false&upper_boundary=false&x_axis=date_time&branches=e272e4b9-7e97-46b2-a403-35e73893ef4f&testbeds=42132742-158d-4e64-8c2e-47984b27798f&benchmarks=584d3db9-2f38-4302-8c61-83db3d791bb1%2C5cca1689-0371-4dde-a031-89a8b3b9b5a1&measures=bd087070-50c6-40ff-aede-60d4fb58e39a&start_time=1723390191794&end_time=1723391190794&tab=plots&plots_search=d497d089-03bd-43b9-9c43-b026126c40d5&key=true&reports_per_page=4&branches_per_page=8&testbeds_per_page=8&benchmarks_per_page=8&plots_per_page=8&reports_page=1&branches_page=1&testbeds_page=1&benchmarks_page=1&plots_page=1"><img src="https://api.bencher.dev/v0/projects/raytracing-weekend/perf/img?branches=e272e4b9-7e97-46b2-a403-35e73893ef4f&testbeds=42132742-158d-4e64-8c2e-47984b27798f&benchmarks=584d3db9-2f38-4302-8c61-83db3d791bb1%2C5cca1689-0371-4dde-a031-89a8b3b9b5a1&measures=bd087070-50c6-40ff-aede-60d4fb58e39a&start_time=1723390191794&end_time=1723391190794" title="Raytracing Weekend" alt="Raytracing Weekend - Bencher" /></a>
