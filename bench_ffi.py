from timeit import timeit

from cffi import FFI

ffi = FFI()
ffi.cdef("""
    double euro_vanilla_put(double, double, double, double, double);
""")

C = ffi.dlopen("target/release/libblack_scholes_ffi.so")

n = 72000000
def bench_euro_vanilla_put():
	v = n*[None]
	for i in range(n):
		v[i] = C.euro_vanilla_put(50, 100, 1, 0.05, 0.25)

print("euro_vanilla_put", str(timeit(bench_euro_vanilla_put, number=1)/1.))
