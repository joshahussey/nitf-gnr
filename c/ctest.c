#include <stdio.h>
#include <dlfcn.h>
#include <stdint.h>
#include <string.h>

typedef void (*extract_jp2_fn)(const char *inputpath, const char *outputpath);
typedef void (*extract_des_fn)(const char *inputpath, const char *outputpath);
typedef void (*get_version_fn)(const char *inputpath);
typedef int (*get_num_images_from_file_fn)(const char *inputpath);
typedef int (*get_num_graphics_from_file_fn)(const char *inputpath);
typedef int (*get_num_text_files_from_file_fn)(const char *inputpath);
typedef int (*get_num_des_from_file_fn)(const char *inputpath);
typedef int (*get_num_res_from_file_fn)(const char *inputpath);

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

    extract_des_fn extract_des = (extract_des_fn)dlsym(lib_handle, "extract_des");
    if (!extract_des) {
        fprintf(stderr, "Failed to locate function: %s\n", dlerror());
        dlclose(lib_handle);
        return 1;
    }

    get_version_fn get_version = (get_version_fn)dlsym(lib_handle, "get_version");
    if (!get_version) {
        fprintf(stderr, "Failed to locate function: %s\n", dlerror());
        dlclose(lib_handle);
        return 1;
    }

    get_num_images_from_file_fn get_num_images = (get_num_images_from_file_fn)dlsym(lib_handle, "get_num_images_from_file");
    if (!get_num_images) {
        fprintf(stderr, "Failed to locate function: %s\n", dlerror());
        dlclose(lib_handle);
        return 1;
    }
    get_num_graphics_from_file_fn get_num_graphics = (get_num_graphics_from_file_fn)dlsym(lib_handle, "get_num_graphics_from_file");
    if (!get_num_graphics) {
        fprintf(stderr, "Failed to locate function: %s\n", dlerror());
        dlclose(lib_handle);
        return 1;
    }
    get_num_text_files_from_file_fn get_num_text_files = (get_num_text_files_from_file_fn)dlsym(lib_handle, "get_num_text_files_from_file");
    if (!get_num_text_files) {
        fprintf(stderr, "Failed to locate function: %s\n", dlerror());
        dlclose(lib_handle);
        return 1;
    }
    get_num_des_from_file_fn get_num_des = (get_num_des_from_file_fn)dlsym(lib_handle, "get_num_des_from_file");
    if (!get_num_des) {
        fprintf(stderr, "Failed to locate function: %s\n", dlerror());
        dlclose(lib_handle);
        return 1;
    }
    get_num_res_from_file_fn get_num_res = (get_num_res_from_file_fn)dlsym(lib_handle, "get_num_res_from_file");
    if (!get_num_res) {
        fprintf(stderr, "Failed to locate function: %s\n", dlerror());
        dlclose(lib_handle);
        return 1;
    }

    // Prepare the string input
    const char *path = "./nitfs/test.nitf";
    const char *out = "/opt/nitf-gnr/ctest/TESTOUT/";

    // Call the Rust function
    extract_jp2(path, out);
    extract_des(path, out);
    get_version(path);
    int img = get_num_images(path);
    printf("Number of images: %d\n", img);
    int graphics = get_num_graphics(path);
    printf("Number of graphics: %d\n", graphics);
    int texts = get_num_text_files(path);
    printf("Number of text files: %d\n", texts);
    int des = get_num_des(path);
    printf("Number of Data Extensions: %d\n", des);
    int res = get_num_res(path);
    printf("Number of  Reserved Extensions: %d\n", res);
    printf("Done\n");
    // Close the shared object
    dlclose(lib_handle);
    
    return 0;
}
