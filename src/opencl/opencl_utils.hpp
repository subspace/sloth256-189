#define CL_TARGET_OPENCL_VERSION 220

#include <string>
#include <vector>
#include <fstream>
#include <sstream>
#include <cstring>
#include <iostream>

#ifdef __APPLE__
#include <Open/opencl.h>
#else
#include <CL/cl_ext.h>
#endif

enum sloth256_189_opencl_error_codes: cl_int {
    SLOTH_NO_OPENCL_COMPATIBLE_GPUS        = (cl_int)2026,
    SLOTH_NO_OPENCL_COMPATIBLE_NVIDIA_GPUS = (cl_int)2027,
    SLOTH_NO_OPENCL_COMPATIBLE_AMD_GPUS    = (cl_int)2028,
    SLOTH_NO_OPENCL_COMPATIBLE_INTEL_GPUS  = (cl_int)2029,
    SLOTH_PIECES_NOT_MULTIPLE_OF_1024      = (cl_int)2031,
    SLOTH_PINNED_MEMORY_ALLOCATION_FAILURE = (cl_int)2035,
    SLOTH_NO_ALLOCATED_PINNED_MEMORY       = (cl_int)2036,
};

void printProgramBuildLog(const cl_int err, cl_device_id& device, cl_program& program) {

    if (err == CL_BUILD_PROGRAM_FAILURE) {
        size_t log_size;
        clGetProgramBuildInfo(program, device, CL_PROGRAM_BUILD_LOG, 0, NULL, &log_size);

        char* log = (char*)malloc(log_size);

        clGetProgramBuildInfo(program, device, CL_PROGRAM_BUILD_LOG, log_size, log, NULL);

        printf("\nError while building OpenCL program:\n%s\n", log);
    }
}

void getAllPlatforms(cl_platform_id*& platforms, cl_uint& num_platforms, cl_int& err) {

    err = clGetPlatformIDs(0, NULL, &num_platforms); if (err != CL_SUCCESS) return;
    platforms = (cl_platform_id*)malloc(sizeof(cl_platform_id) * (size_t)num_platforms);
    err = clGetPlatformIDs(num_platforms, platforms, NULL); if (err != CL_SUCCESS) return;
}

std::string getPlatformName(cl_platform_id& platform_id, cl_int& err) {

    std::string platform_name = "0";

    size_t platform_name_length;
    err = clGetPlatformInfo(platform_id, CL_PLATFORM_NAME, 0, NULL, &platform_name_length); 
    if (err != CL_SUCCESS) return platform_name;

    platform_name.resize(platform_name_length);

    err = clGetPlatformInfo(platform_id, CL_PLATFORM_NAME, platform_name_length, 
                            (void*)platform_name.data(), NULL); 
    if (err != CL_SUCCESS) return platform_name;;

    return platform_name;
}

std::string getDeviceName(cl_device_id& device_id, cl_int& err) {

    std::string device_name = "0";

    size_t device_name_length;
    err = clGetDeviceInfo(device_id, CL_DEVICE_NAME, 0, NULL, &device_name_length); 
    if (err != CL_SUCCESS) return device_name;

    device_name.resize(device_name_length);

    err = clGetDeviceInfo(device_id, CL_DEVICE_NAME, device_name_length, 
                          (void*)device_name.data(), NULL); 
    if (err != CL_SUCCESS) return device_name;

    return device_name;
}

std::vector<cl_device_id> getAllGPUs(cl_int& err) {

    std::vector<cl_device_id> devices;

    cl_platform_id* platforms;
    cl_uint num_platforms;

    getAllPlatforms(platforms, num_platforms, err); 
    if (err != CL_SUCCESS) return devices;

    for (size_t i = 0; i < num_platforms; i++) {

        std::string platform_name = getPlatformName(platforms[i], err); 
        if (err != CL_SUCCESS) return devices;

        if (platform_name.find("NVIDIA CUDA") == std::string::npos && 
            platform_name.find("Intel(R) OpenCL HD Graphics") == std::string::npos && 
            platform_name.find("Amd") == std::string::npos && 
            platform_name.find("AMD") == std::string::npos) {

            continue;
        }

        cl_uint num_devices;

        err = clGetDeviceIDs(platforms[i], CL_DEVICE_TYPE_GPU, 0, NULL, &num_devices); 
        if (err != CL_SUCCESS) return devices;

        cl_device_id* devices_curr = (cl_device_id*)malloc(sizeof(cl_device_id) * (size_t)num_devices);

        err = clGetDeviceIDs(platforms[i], CL_DEVICE_TYPE_GPU, num_devices, devices_curr, NULL); 
        if (err != CL_SUCCESS) return devices;

        for (size_t j = 0; j < num_devices; j++) {
            devices.push_back(devices_curr[j]);
        }
    }

    if (devices.size() == 0)
        err = SLOTH_NO_OPENCL_COMPATIBLE_GPUS;

    return devices;
}

std::vector<cl_device_id> getAllNvidiaGPUs(cl_int& err) {

    std::vector<cl_device_id> devices;

    cl_platform_id* platforms;
    cl_uint num_platforms;

    getAllPlatforms(platforms, num_platforms, err); 
    if (err != CL_SUCCESS) return devices;

    for (size_t i = 0; i < num_platforms; i++) {

        std::string platform_name = getPlatformName(platforms[i], err); 
        if (err != CL_SUCCESS) return devices;

        if (platform_name.find("NVIDIA CUDA") == std::string::npos) {
            continue;
        }

        cl_uint num_devices;
        err = clGetDeviceIDs(platforms[i], CL_DEVICE_TYPE_GPU, 0, NULL, &num_devices); 
        if (err != CL_SUCCESS) return devices;

        cl_device_id* devices_curr = (cl_device_id*)malloc(sizeof(cl_device_id) * (size_t)num_devices);

        err = clGetDeviceIDs(platforms[i], CL_DEVICE_TYPE_GPU, num_devices, devices_curr, NULL); 
        if (err != CL_SUCCESS) return devices;

        for (size_t j = 0; j < num_devices; j++) {
            devices.push_back(devices_curr[j]);
        }
    }

    if (devices.size() == 0)
        err = SLOTH_NO_OPENCL_COMPATIBLE_NVIDIA_GPUS;

    return devices;
}

std::vector<cl_device_id> getAllAMDGPUs(cl_int& err) {

    std::vector<cl_device_id> devices;

    cl_platform_id* platforms;
    cl_uint num_platforms;

    getAllPlatforms(platforms, num_platforms, err); 
    if (err != CL_SUCCESS) return devices;

    for (size_t i = 0; i < num_platforms; i++) {

        std::string platform_name = getPlatformName(platforms[i], err); 
        if (err != CL_SUCCESS) return devices;

        if (platform_name.find("Amd") == std::string::npos && 
            platform_name.find("AMD") == std::string::npos) {

            continue;
        }

        cl_uint num_devices;

        err = clGetDeviceIDs(platforms[i], CL_DEVICE_TYPE_GPU, 0, NULL, &num_devices); 
        if (err != CL_SUCCESS) return devices;

        cl_device_id* devices_curr = (cl_device_id*)malloc(sizeof(cl_device_id) * (size_t)num_devices);

        err = clGetDeviceIDs(platforms[i], CL_DEVICE_TYPE_GPU, num_devices, devices_curr, NULL); 
        if (err != CL_SUCCESS) return devices;

        for (size_t j = 0; j < num_devices; j++) {
            devices.push_back(devices_curr[j]);
        }
    }

    if (devices.size() == 0)
        err = SLOTH_NO_OPENCL_COMPATIBLE_AMD_GPUS;

    return devices;
}

std::vector<cl_device_id> getAllIntelGPUs(cl_int& err) {

    std::vector<cl_device_id> devices;

    cl_platform_id* platforms;
    cl_uint num_platforms;

    getAllPlatforms(platforms, num_platforms, err); 
    if (err != CL_SUCCESS) return devices;

    for (size_t i = 0; i < num_platforms; i++) {
        std::string platform_name = getPlatformName(platforms[i], err); 
        if (err != CL_SUCCESS) return devices;

        if (platform_name.find("Intel(R) OpenCL HD Graphics") == std::string::npos) {
            continue;
        }

        cl_uint num_devices;
        err = clGetDeviceIDs(platforms[i], CL_DEVICE_TYPE_GPU, 0, NULL, &num_devices); 
        if (err != CL_SUCCESS) return devices;

        cl_device_id* devices_curr = (cl_device_id*)malloc(sizeof(cl_device_id) * (size_t)num_devices);

        err = clGetDeviceIDs(platforms[i], CL_DEVICE_TYPE_GPU, num_devices, devices_curr, NULL); 
        if (err != CL_SUCCESS) return devices;

        for (size_t j = 0; j < num_devices; j++) {
            devices.push_back(devices_curr[j]);
        }
    }

    if (devices.size() == 0)
        err = SLOTH_NO_OPENCL_COMPATIBLE_INTEL_GPUS;

    return devices;
}