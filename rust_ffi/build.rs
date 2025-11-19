use cc::Build;

fn main() {
    //CBuild::new().file("c_libs/math.c").compile("math");
    Build::new()
        .cpp(true)
        .file("cpp_libs/math_cpp.cpp")
        .file("cpp_libs/math_cpp_wrapper.cpp")
        .compile("libmath_cpp_wrapper.a");

    Build::new().file("c_libs/math.c").compile("math");
}
