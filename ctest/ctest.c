#include <stdio.h>
#include <dlfcn.h>
#include <stdint.h>
#include <string.h>

typedef void (*extract_jp2_fn)(const char *inputpath, const char *outputpath);

int main() {
    // Load the shared object (.so file)
    void *lib_handle = dlopen("/opt/nitf-gnr/target/release/libnitf_gnr.so", RTLD_LAZY);
    if (!lib_handle) {
        fprintf(stderr, "Failed to open .so file: %s\n", dlerror());
        return 1;
    }

    // Resolve the function pointer
    extract_jp2_fn extract_jp2 = (extract_jp2_fn)dlsym(lib_handle, "extract_jp2");
    if (!extract_jp2) {
        fprintf(stderr, "Failed to locate function: %s\n", dlerror());
        dlclose(lib_handle);
        return 1;
    }

    // Prepare the string input
    const char *path = "/opt/nitf-gnr/testdata/input.nitf";
    const char *out = "/opt/nitf-gnr/ctest/TESTOUT";

    // Call the Rust function
    extract_jp2(path, out);
    printf("Done\n");
    // Close the shared object
    dlclose(lib_handle);
    
    return 0;
}
