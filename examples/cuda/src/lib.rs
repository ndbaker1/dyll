#![allow(unused_imports)]
#![allow(nonstandard_style)]
use libc::*;

pub mod types;
pub use types::*;

pub mod dl;
pub use dl::*;

pub mod config;
pub use config::*;

const OPT_LEVEL_FILTER: &str = "level-filter";

// The function we want to run at load time.
#[no_mangle]
pub extern "C" fn custom_init_function() {
    use std::str::FromStr;

    let level_filter = read_option(OPT_LEVEL_FILTER)
        .and_then(|f| log::LevelFilter::from_str(&f).ok())
        .unwrap_or(log::LevelFilter::Debug);

    env_logger::builder().filter_level(level_filter).init();

    log::debug!("--- MOCK INIT ---");
}

// A static reference to the initialization function pointer is placed
// in the .init_array section using linker directives.
#[used]
// This attribute ensures the compiler doesn't optimize away the static item if it thinks it's unused.
#[link_section = ".init_array"]
pub static INITIALIZER: extern "C" fn() = custom_init_function;

// Generated function stubs
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetFlags(arg0: *mut c_uint) -> CUresult {
    let cuCtxGetFlags: extern "C" fn(arg0: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetFlags"));
    eprintln!("[CALL] {}", "cuCtxGetFlags");
    cuCtxGetFlags(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAsync(
    arg0: CUdeviceptr,
    arg1: CUdeviceptr,
    arg2: usize,
    arg3: CUstream,
) -> CUresult {
    let cuMemcpyAsync: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: CUdeviceptr,
        arg2: usize,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyAsync"));
    eprintln!("[CALL] {}", "cuMemcpyAsync");
    cuMemcpyAsync(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoA_v2(
    arg0: CUarray,
    arg1: usize,
    arg2: CUdeviceptr,
    arg3: usize,
) -> CUresult {
    let cuMemcpyDtoA_v2: extern "C" fn(
        arg0: CUarray,
        arg1: usize,
        arg2: CUdeviceptr,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyDtoA_v2"));
    eprintln!("[CALL] {}", "cuMemcpyDtoA_v2");
    cuMemcpyDtoA_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncGetAttribute(
    arg0: *mut c_int,
    arg1: CUfunction_attribute,
    arg2: CUfunction,
) -> CUresult {
    let cuFuncGetAttribute: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUfunction_attribute,
        arg2: CUfunction,
    ) -> CUresult = std::mem::transmute(get_sym("cuFuncGetAttribute"));
    eprintln!("[CALL] {}", "cuFuncGetAttribute");
    cuFuncGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetCacheConfig(arg0: CUfunc_cache) -> CUresult {
    let cuCtxSetCacheConfig: extern "C" fn(arg0: CUfunc_cache) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetCacheConfig"));
    eprintln!("[CALL] {}", "cuCtxSetCacheConfig");
    cuCtxSetCacheConfig(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32Async(
    arg0: CUdeviceptr,
    arg1: c_uint,
    arg2: usize,
    arg3: CUstream,
) -> CUresult {
    let cuMemsetD32Async: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: c_uint,
        arg2: usize,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD32Async"));
    eprintln!("[CALL] {}", "cuMemsetD32Async");
    cuMemsetD32Async(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuPointerSetAttribute(
    arg0: *const c_void,
    arg1: CUpointer_attribute,
    arg2: CUdeviceptr,
) -> CUresult {
    let cuPointerSetAttribute: extern "C" fn(
        arg0: *const c_void,
        arg1: CUpointer_attribute,
        arg2: CUdeviceptr,
    ) -> CUresult = std::mem::transmute(get_sym("cuPointerSetAttribute"));
    eprintln!("[CALL] {}", "cuPointerSetAttribute");
    cuPointerSetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetToolsId(
    arg0: CUgraphNode,
    arg1: *mut c_ulonglong,
) -> CUresult {
    let cuGraphNodeGetToolsId: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeGetToolsId"));
    eprintln!("[CALL] {}", "cuGraphNodeGetToolsId");
    cuGraphNodeGetToolsId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDestroyExternalSemaphore(arg0: CUexternalSemaphore) -> CUresult {
    let cuDestroyExternalSemaphore: extern "C" fn(arg0: CUexternalSemaphore) -> CUresult =
        std::mem::transmute(get_sym("cuDestroyExternalSemaphore"));
    eprintln!("[CALL] {}", "cuDestroyExternalSemaphore");
    cuDestroyExternalSemaphore(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryLoadFromFile(
    arg0: *mut CUlibrary,
    arg1: *const c_char,
    arg2: *mut CUjit_option,
    arg3: *mut *mut c_void,
    arg4: c_uint,
    arg5: *mut CUlibraryOption,
    arg6: *mut *mut c_void,
    arg7: c_uint,
) -> CUresult {
    let cuLibraryLoadFromFile: extern "C" fn(
        arg0: *mut CUlibrary,
        arg1: *const c_char,
        arg2: *mut CUjit_option,
        arg3: *mut *mut c_void,
        arg4: c_uint,
        arg5: *mut CUlibraryOption,
        arg6: *mut *mut c_void,
        arg7: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuLibraryLoadFromFile"));
    eprintln!("[CALL] {}", "cuLibraryLoadFromFile");
    cuLibraryLoadFromFile(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceCanAccessPeer(
    arg0: *mut c_int,
    arg1: CUdevice,
    arg2: CUdevice,
) -> CUresult {
    let cuDeviceCanAccessPeer: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUdevice,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceCanAccessPeer"));
    eprintln!("[CALL] {}", "cuDeviceCanAccessPeer");
    cuDeviceCanAccessPeer(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddExternalSemaphoresWaitNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: *const CUDA_EXT_SEM_WAIT_NODE_PARAMS,
) -> CUresult {
    let cuGraphAddExternalSemaphoresWaitNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: *const CUDA_EXT_SEM_WAIT_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddExternalSemaphoresWaitNode"));
    eprintln!("[CALL] {}", "cuGraphAddExternalSemaphoresWaitNode");
    cuGraphAddExternalSemaphoresWaitNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuOccupancyMaxActiveClusters(
    arg0: *mut c_int,
    arg1: CUfunction,
    arg2: *const CUlaunchConfig,
) -> CUresult {
    let cuOccupancyMaxActiveClusters: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUfunction,
        arg2: *const CUlaunchConfig,
    ) -> CUresult = std::mem::transmute(get_sym("cuOccupancyMaxActiveClusters"));
    eprintln!("[CALL] {}", "cuOccupancyMaxActiveClusters");
    cuOccupancyMaxActiveClusters(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecUpdate_v2(
    arg0: CUgraphExec,
    arg1: CUgraph,
    arg2: *mut CUgraphExecUpdateResultInfo,
) -> CUresult {
    let cuGraphExecUpdate_v2: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraph,
        arg2: *mut CUgraphExecUpdateResultInfo,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecUpdate_v2"));
    eprintln!("[CALL] {}", "cuGraphExecUpdate_v2");
    cuGraphExecUpdate_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuThreadExchangeStreamCaptureMode(
    arg0: *mut CUstreamCaptureMode,
) -> CUresult {
    let cuThreadExchangeStreamCaptureMode: extern "C" fn(
        arg0: *mut CUstreamCaptureMode,
    ) -> CUresult = std::mem::transmute(get_sym("cuThreadExchangeStreamCaptureMode"));
    eprintln!("[CALL] {}", "cuThreadExchangeStreamCaptureMode");
    cuThreadExchangeStreamCaptureMode(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncSetBlockShape(
    arg0: CUfunction,
    arg1: c_int,
    arg2: c_int,
    arg3: c_int,
) -> CUresult {
    let cuFuncSetBlockShape: extern "C" fn(
        arg0: CUfunction,
        arg1: c_int,
        arg2: c_int,
        arg3: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuFuncSetBlockShape"));
    eprintln!("[CALL] {}", "cuFuncSetBlockShape");
    cuFuncSetBlockShape(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddEventRecordNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: CUevent,
) -> CUresult {
    let cuGraphAddEventRecordNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: CUevent,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddEventRecordNode"));
    eprintln!("[CALL] {}", "cuGraphAddEventRecordNode");
    cuGraphAddEventRecordNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchCooperativeKernelMultiDevice(
    arg0: *mut CUDA_LAUNCH_PARAMS,
    arg1: c_uint,
    arg2: c_uint,
) -> CUresult {
    let cuLaunchCooperativeKernelMultiDevice: extern "C" fn(
        arg0: *mut CUDA_LAUNCH_PARAMS,
        arg1: c_uint,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuLaunchCooperativeKernelMultiDevice"));
    eprintln!("[CALL] {}", "cuLaunchCooperativeKernelMultiDevice");
    cuLaunchCooperativeKernelMultiDevice(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTensorMapEncodeTiled(
    arg0: *mut CUtensorMap,
    arg1: CUtensorMapDataType,
    arg2: cuuint32_t,
    arg3: *mut c_void,
    arg4: *const cuuint64_t,
    arg5: *const cuuint64_t,
    arg6: *const cuuint32_t,
    arg7: *const cuuint32_t,
    arg8: CUtensorMapInterleave,
    arg9: CUtensorMapSwizzle,
    arg10: CUtensorMapL2promotion,
    arg11: CUtensorMapFloatOOBfill,
) -> CUresult {
    let cuTensorMapEncodeTiled: extern "C" fn(
        arg0: *mut CUtensorMap,
        arg1: CUtensorMapDataType,
        arg2: cuuint32_t,
        arg3: *mut c_void,
        arg4: *const cuuint64_t,
        arg5: *const cuuint64_t,
        arg6: *const cuuint32_t,
        arg7: *const cuuint32_t,
        arg8: CUtensorMapInterleave,
        arg9: CUtensorMapSwizzle,
        arg10: CUtensorMapL2promotion,
        arg11: CUtensorMapFloatOOBfill,
    ) -> CUresult = std::mem::transmute(get_sym("cuTensorMapEncodeTiled"));
    eprintln!("[CALL] {}", "cuTensorMapEncodeTiled");
    cuTensorMapEncodeTiled(
        arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11,
    )
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetExecAffinity(
    arg0: *mut CUexecAffinityParam,
    arg1: CUexecAffinityType,
) -> CUresult {
    let cuCtxGetExecAffinity: extern "C" fn(
        arg0: *mut CUexecAffinityParam,
        arg1: CUexecAffinityType,
    ) -> CUresult = std::mem::transmute(get_sym("cuCtxGetExecAffinity"));
    eprintln!("[CALL] {}", "cuCtxGetExecAffinity");
    cuCtxGetExecAffinity(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceGetMappedPointer_v2(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: CUgraphicsResource,
) -> CUresult {
    let cuGraphicsResourceGetMappedPointer_v2: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: CUgraphicsResource,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphicsResourceGetMappedPointer_v2"));
    eprintln!("[CALL] {}", "cuGraphicsResourceGetMappedPointer_v2");
    cuGraphicsResourceGetMappedPointer_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecChildGraphNodeSetParams(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: CUgraph,
) -> CUresult {
    let cuGraphExecChildGraphNodeSetParams: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: CUgraph,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecChildGraphNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExecChildGraphNodeSetParams");
    cuGraphExecChildGraphNodeSetParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncGetParamInfo(
    arg0: CUfunction,
    arg1: usize,
    arg2: *mut usize,
    arg3: *mut usize,
) -> CUresult {
    let cuFuncGetParamInfo: extern "C" fn(
        arg0: CUfunction,
        arg1: usize,
        arg2: *mut usize,
        arg3: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuFuncGetParamInfo"));
    eprintln!("[CALL] {}", "cuFuncGetParamInfo");
    cuFuncGetParamInfo(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolExportToShareableHandle(
    arg0: *mut c_void,
    arg1: CUmemoryPool,
    arg2: CUmemAllocationHandleType,
    arg3: c_ulonglong,
) -> CUresult {
    let cuMemPoolExportToShareableHandle: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUmemoryPool,
        arg2: CUmemAllocationHandleType,
        arg3: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolExportToShareableHandle"));
    eprintln!("[CALL] {}", "cuMemPoolExportToShareableHandle");
    cuMemPoolExportToShareableHandle(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetLoadingMode(arg0: *mut CUmoduleLoadingMode) -> CUresult {
    let cuModuleGetLoadingMode: extern "C" fn(arg0: *mut CUmoduleLoadingMode) -> CUresult =
        std::mem::transmute(get_sym("cuModuleGetLoadingMode"));
    eprintln!("[CALL] {}", "cuModuleGetLoadingMode");
    cuModuleGetLoadingMode(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuSurfObjectGetResourceDesc(
    arg0: *mut CUDA_RESOURCE_DESC,
    arg1: CUsurfObject,
) -> CUresult {
    let cuSurfObjectGetResourceDesc: extern "C" fn(
        arg0: *mut CUDA_RESOURCE_DESC,
        arg1: CUsurfObject,
    ) -> CUresult = std::mem::transmute(get_sym("cuSurfObjectGetResourceDesc"));
    eprintln!("[CALL] {}", "cuSurfObjectGetResourceDesc");
    cuSurfObjectGetResourceDesc(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemUnmap(arg0: CUdeviceptr, arg1: usize) -> CUresult {
    let cuMemUnmap: extern "C" fn(arg0: CUdeviceptr, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemUnmap"));
    eprintln!("[CALL] {}", "cuMemUnmap");
    cuMemUnmap(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryGetKernel(
    arg0: *mut CUkernel,
    arg1: CUlibrary,
    arg2: *const c_char,
) -> CUresult {
    let cuLibraryGetKernel: extern "C" fn(
        arg0: *mut CUkernel,
        arg1: CUlibrary,
        arg2: *const c_char,
    ) -> CUresult = std::mem::transmute(get_sym("cuLibraryGetKernel"));
    eprintln!("[CALL] {}", "cuLibraryGetKernel");
    cuLibraryGetKernel(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexObjectGetResourceDesc(
    arg0: *mut CUDA_RESOURCE_DESC,
    arg1: CUtexObject,
) -> CUresult {
    let cuTexObjectGetResourceDesc: extern "C" fn(
        arg0: *mut CUDA_RESOURCE_DESC,
        arg1: CUtexObject,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexObjectGetResourceDesc"));
    eprintln!("[CALL] {}", "cuTexObjectGetResourceDesc");
    cuTexObjectGetResourceDesc(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxRecordEvent(arg0: CUgreenCtx, arg1: CUevent) -> CUresult {
    let cuGreenCtxRecordEvent: extern "C" fn(arg0: CUgreenCtx, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuGreenCtxRecordEvent"));
    eprintln!("[CALL] {}", "cuGreenCtxRecordEvent");
    cuGreenCtxRecordEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleEnumerateFunctions(
    arg0: *mut CUfunction,
    arg1: c_uint,
    arg2: CUmodule,
) -> CUresult {
    let cuModuleEnumerateFunctions: extern "C" fn(
        arg0: *mut CUfunction,
        arg1: c_uint,
        arg2: CUmodule,
    ) -> CUresult = std::mem::transmute(get_sym("cuModuleEnumerateFunctions"));
    eprintln!("[CALL] {}", "cuModuleEnumerateFunctions");
    cuModuleEnumerateFunctions(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetInfo_v2(arg0: *mut usize, arg1: *mut usize) -> CUresult {
    let cuMemGetInfo_v2: extern "C" fn(arg0: *mut usize, arg1: *mut usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemGetInfo_v2"));
    eprintln!("[CALL] {}", "cuMemGetInfo_v2");
    cuMemGetInfo_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetBorderColor(arg0: CUtexref, arg1: *mut f32) -> CUresult {
    let cuTexRefSetBorderColor: extern "C" fn(arg0: CUtexref, arg1: *mut f32) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetBorderColor"));
    eprintln!("[CALL] {}", "cuTexRefSetBorderColor");
    cuTexRefSetBorderColor(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolGetAttribute(
    arg0: CUmemoryPool,
    arg1: CUmemPool_attribute,
    arg2: *mut c_void,
) -> CUresult {
    let cuMemPoolGetAttribute: extern "C" fn(
        arg0: CUmemoryPool,
        arg1: CUmemPool_attribute,
        arg2: *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolGetAttribute"));
    eprintln!("[CALL] {}", "cuMemPoolGetAttribute");
    cuMemPoolGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetName(arg0: *mut *const c_char, arg1: CUkernel) -> CUresult {
    let cuKernelGetName: extern "C" fn(arg0: *mut *const c_char, arg1: CUkernel) -> CUresult =
        std::mem::transmute(get_sym("cuKernelGetName"));
    eprintln!("[CALL] {}", "cuKernelGetName");
    cuKernelGetName(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMipmappedArrayGetLevel(
    arg0: *mut CUarray,
    arg1: CUmipmappedArray,
    arg2: c_uint,
) -> CUresult {
    let cuMipmappedArrayGetLevel: extern "C" fn(
        arg0: *mut CUarray,
        arg1: CUmipmappedArray,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuMipmappedArrayGetLevel"));
    eprintln!("[CALL] {}", "cuMipmappedArrayGetLevel");
    cuMipmappedArrayGetLevel(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCtx(arg0: CUstream, arg1: *mut CUcontext) -> CUresult {
    let cuStreamGetCtx: extern "C" fn(arg0: CUstream, arg1: *mut CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetCtx"));
    eprintln!("[CALL] {}", "cuStreamGetCtx");
    cuStreamGetCtx(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetSharedMemConfig(arg0: *mut CUsharedconfig) -> CUresult {
    let cuCtxGetSharedMemConfig: extern "C" fn(arg0: *mut CUsharedconfig) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetSharedMemConfig"));
    eprintln!("[CALL] {}", "cuCtxGetSharedMemConfig");
    cuCtxGetSharedMemConfig(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventQuery(arg0: CUevent) -> CUresult {
    let cuEventQuery: extern "C" fn(arg0: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuEventQuery"));
    eprintln!("[CALL] {}", "cuEventQuery");
    cuEventQuery(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphBatchMemOpNodeSetParams(
    arg0: CUgraphNode,
    arg1: *const CUDA_BATCH_MEM_OP_NODE_PARAMS,
) -> CUresult {
    let cuGraphBatchMemOpNodeSetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *const CUDA_BATCH_MEM_OP_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphBatchMemOpNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphBatchMemOpNodeSetParams");
    cuGraphBatchMemOpNodeSetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphDestroy(arg0: CUgraph) -> CUresult {
    let cuGraphDestroy: extern "C" fn(arg0: CUgraph) -> CUresult =
        std::mem::transmute(get_sym("cuGraphDestroy"));
    eprintln!("[CALL] {}", "cuGraphDestroy");
    cuGraphDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuSignalExternalSemaphoresAsync(
    arg0: *const CUexternalSemaphore,
    arg1: *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    arg2: c_uint,
    arg3: CUstream,
) -> CUresult {
    let cuSignalExternalSemaphoresAsync: extern "C" fn(
        arg0: *const CUexternalSemaphore,
        arg1: *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
        arg2: c_uint,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuSignalExternalSemaphoresAsync"));
    eprintln!("[CALL] {}", "cuSignalExternalSemaphoresAsync");
    cuSignalExternalSemaphoresAsync(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBatchMemOp_v2(
    arg0: CUstream,
    arg1: c_uint,
    arg2: *mut CUstreamBatchMemOpParams,
    arg3: c_uint,
) -> CUresult {
    let cuStreamBatchMemOp_v2: extern "C" fn(
        arg0: CUstream,
        arg1: c_uint,
        arg2: *mut CUstreamBatchMemOpParams,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamBatchMemOp_v2"));
    eprintln!("[CALL] {}", "cuStreamBatchMemOp_v2");
    cuStreamBatchMemOp_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexObjectDestroy(arg0: CUtexObject) -> CUresult {
    let cuTexObjectDestroy: extern "C" fn(arg0: CUtexObject) -> CUresult =
        std::mem::transmute(get_sym("cuTexObjectDestroy"));
    eprintln!("[CALL] {}", "cuTexObjectDestroy");
    cuTexObjectDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemRangeGetAttributes(
    arg0: *mut *mut c_void,
    arg1: *mut usize,
    arg2: *mut CUmem_range_attribute,
    arg3: usize,
    arg4: CUdeviceptr,
    arg5: usize,
) -> CUresult {
    let cuMemRangeGetAttributes: extern "C" fn(
        arg0: *mut *mut c_void,
        arg1: *mut usize,
        arg2: *mut CUmem_range_attribute,
        arg3: usize,
        arg4: CUdeviceptr,
        arg5: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemRangeGetAttributes"));
    eprintln!("[CALL] {}", "cuMemRangeGetAttributes");
    cuMemRangeGetAttributes(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddressMode(
    arg0: CUtexref,
    arg1: c_int,
    arg2: CUaddress_mode,
) -> CUresult {
    let cuTexRefSetAddressMode: extern "C" fn(
        arg0: CUtexref,
        arg1: c_int,
        arg2: CUaddress_mode,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefSetAddressMode"));
    eprintln!("[CALL] {}", "cuTexRefSetAddressMode");
    cuTexRefSetAddressMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetSurfRef(
    arg0: *mut CUsurfref,
    arg1: CUmodule,
    arg2: *const c_char,
) -> CUresult {
    let cuModuleGetSurfRef: extern "C" fn(
        arg0: *mut CUsurfref,
        arg1: CUmodule,
        arg2: *const c_char,
    ) -> CUresult = std::mem::transmute(get_sym("cuModuleGetSurfRef"));
    eprintln!("[CALL] {}", "cuModuleGetSurfRef");
    cuModuleGetSurfRef(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetEnabled(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *mut c_uint,
) -> CUresult {
    let cuGraphNodeGetEnabled: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *mut c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeGetEnabled"));
    eprintln!("[CALL] {}", "cuGraphNodeGetEnabled");
    cuGraphNodeGetEnabled(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeSetEnabled(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: c_uint,
) -> CUresult {
    let cuGraphNodeSetEnabled: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeSetEnabled"));
    eprintln!("[CALL] {}", "cuGraphNodeSetEnabled");
    cuGraphNodeSetEnabled(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetDependentNodes_v2(
    arg0: CUgraphNode,
    arg1: *mut CUgraphNode,
    arg2: *mut CUgraphEdgeData,
    arg3: *mut usize,
) -> CUresult {
    let cuGraphNodeGetDependentNodes_v2: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUgraphNode,
        arg2: *mut CUgraphEdgeData,
        arg3: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeGetDependentNodes_v2"));
    eprintln!("[CALL] {}", "cuGraphNodeGetDependentNodes_v2");
    cuGraphNodeGetDependentNodes_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuImportExternalMemory(
    arg0: *mut CUexternalMemory,
    arg1: *const CUDA_EXTERNAL_MEMORY_HANDLE_DESC,
) -> CUresult {
    let cuImportExternalMemory: extern "C" fn(
        arg0: *mut CUexternalMemory,
        arg1: *const CUDA_EXTERNAL_MEMORY_HANDLE_DESC,
    ) -> CUresult = std::mem::transmute(get_sym("cuImportExternalMemory"));
    eprintln!("[CALL] {}", "cuImportExternalMemory");
    cuImportExternalMemory(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsSubResourceGetMappedArray(
    arg0: *mut CUarray,
    arg1: CUgraphicsResource,
    arg2: c_uint,
    arg3: c_uint,
) -> CUresult {
    let cuGraphicsSubResourceGetMappedArray: extern "C" fn(
        arg0: *mut CUarray,
        arg1: CUgraphicsResource,
        arg2: c_uint,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphicsSubResourceGetMappedArray"));
    eprintln!("[CALL] {}", "cuGraphicsSubResourceGetMappedArray");
    cuGraphicsSubResourceGetMappedArray(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuArray3DCreate_v2(
    arg0: *mut CUarray,
    arg1: *const CUDA_ARRAY3D_DESCRIPTOR,
) -> CUresult {
    let cuArray3DCreate_v2: extern "C" fn(
        arg0: *mut CUarray,
        arg1: *const CUDA_ARRAY3D_DESCRIPTOR,
    ) -> CUresult = std::mem::transmute(get_sym("cuArray3DCreate_v2"));
    eprintln!("[CALL] {}", "cuArray3DCreate_v2");
    cuArray3DCreate_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuWaitExternalSemaphoresAsync(
    arg0: *const CUexternalSemaphore,
    arg1: *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    arg2: c_uint,
    arg3: CUstream,
) -> CUresult {
    let cuWaitExternalSemaphoresAsync: extern "C" fn(
        arg0: *const CUexternalSemaphore,
        arg1: *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
        arg2: c_uint,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuWaitExternalSemaphoresAsync"));
    eprintln!("[CALL] {}", "cuWaitExternalSemaphoresAsync");
    cuWaitExternalSemaphoresAsync(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddMemFreeNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: CUdeviceptr,
) -> CUresult {
    let cuGraphAddMemFreeNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: CUdeviceptr,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddMemFreeNode"));
    eprintln!("[CALL] {}", "cuGraphAddMemFreeNode");
    cuGraphAddMemFreeNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSynchronize() -> CUresult {
    let cuCtxSynchronize: extern "C" fn() -> CUresult =
        std::mem::transmute(get_sym("cuCtxSynchronize"));
    eprintln!("[CALL] {}", "cuCtxSynchronize");
    cuCtxSynchronize()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceSetMapFlags_v2(
    arg0: CUgraphicsResource,
    arg1: c_uint,
) -> CUresult {
    let cuGraphicsResourceSetMapFlags_v2: extern "C" fn(
        arg0: CUgraphicsResource,
        arg1: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphicsResourceSetMapFlags_v2"));
    eprintln!("[CALL] {}", "cuGraphicsResourceSetMapFlags_v2");
    cuGraphicsResourceSetMapFlags_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMipmappedArrayGetMemoryRequirements(
    arg0: *mut CUDA_ARRAY_MEMORY_REQUIREMENTS,
    arg1: CUmipmappedArray,
    arg2: CUdevice,
) -> CUresult {
    let cuMipmappedArrayGetMemoryRequirements: extern "C" fn(
        arg0: *mut CUDA_ARRAY_MEMORY_REQUIREMENTS,
        arg1: CUmipmappedArray,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuMipmappedArrayGetMemoryRequirements"));
    eprintln!("[CALL] {}", "cuMipmappedArrayGetMemoryRequirements");
    cuMipmappedArrayGetMemoryRequirements(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExternalSemaphoresSignalNodeSetParams(
    arg0: CUgraphNode,
    arg1: *const CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
) -> CUresult {
    let cuGraphExternalSemaphoresSignalNodeSetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *const CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExternalSemaphoresSignalNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExternalSemaphoresSignalNodeSetParams");
    cuGraphExternalSemaphoresSignalNodeSetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkAddFile_v2(
    arg0: CUlinkState,
    arg1: CUjitInputType,
    arg2: *const c_char,
    arg3: c_uint,
    arg4: *mut CUjit_option,
    arg5: *mut *mut c_void,
) -> CUresult {
    let cuLinkAddFile_v2: extern "C" fn(
        arg0: CUlinkState,
        arg1: CUjitInputType,
        arg2: *const c_char,
        arg3: c_uint,
        arg4: *mut CUjit_option,
        arg5: *mut *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuLinkAddFile_v2"));
    eprintln!("[CALL] {}", "cuLinkAddFile_v2");
    cuLinkAddFile_v2(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetMipmapLevelClamp(
    arg0: CUtexref,
    arg1: f32,
    arg2: f32,
) -> CUresult {
    let cuTexRefSetMipmapLevelClamp: extern "C" fn(
        arg0: CUtexref,
        arg1: f32,
        arg2: f32,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefSetMipmapLevelClamp"));
    eprintln!("[CALL] {}", "cuTexRefSetMipmapLevelClamp");
    cuTexRefSetMipmapLevelClamp(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDriverGetVersion(arg0: *mut c_int) -> CUresult {
    let cuDriverGetVersion: extern "C" fn(arg0: *mut c_int) -> CUresult =
        std::mem::transmute(get_sym("cuDriverGetVersion"));
    eprintln!("[CALL] {}", "cuDriverGetVersion");
    cuDriverGetVersion(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamCopyAttributes(arg0: CUstream, arg1: CUstream) -> CUresult {
    let cuStreamCopyAttributes: extern "C" fn(arg0: CUstream, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamCopyAttributes"));
    eprintln!("[CALL] {}", "cuStreamCopyAttributes");
    cuStreamCopyAttributes(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncGetName(arg0: *mut *const c_char, arg1: CUfunction) -> CUresult {
    let cuFuncGetName: extern "C" fn(arg0: *mut *const c_char, arg1: CUfunction) -> CUresult =
        std::mem::transmute(get_sym("cuFuncGetName"));
    eprintln!("[CALL] {}", "cuFuncGetName");
    cuFuncGetName(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCheckpointProcessLock(
    arg0: c_int,
    arg1: *mut CUcheckpointLockArgs,
) -> CUresult {
    let cuCheckpointProcessLock: extern "C" fn(
        arg0: c_int,
        arg1: *mut CUcheckpointLockArgs,
    ) -> CUresult = std::mem::transmute(get_sym("cuCheckpointProcessLock"));
    eprintln!("[CALL] {}", "cuCheckpointProcessLock");
    cuCheckpointProcessLock(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddHostNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: *const CUDA_HOST_NODE_PARAMS,
) -> CUresult {
    let cuGraphAddHostNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: *const CUDA_HOST_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddHostNode"));
    eprintln!("[CALL] {}", "cuGraphAddHostNode");
    cuGraphAddHostNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCaptureToCig(
    arg0: CUstream,
    arg1: *mut CUstreamCigCaptureParams,
) -> CUresult {
    let cuStreamBeginCaptureToCig: extern "C" fn(
        arg0: CUstream,
        arg1: *mut CUstreamCigCaptureParams,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamBeginCaptureToCig"));
    eprintln!("[CALL] {}", "cuStreamBeginCaptureToCig");
    cuStreamBeginCaptureToCig(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSynchronize_v2(arg0: CUcontext) -> CUresult {
    let cuCtxSynchronize_v2: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSynchronize_v2"));
    eprintln!("[CALL] {}", "cuCtxSynchronize_v2");
    cuCtxSynchronize_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMulticastBindAddr(
    arg0: CUmemGenericAllocationHandle,
    arg1: usize,
    arg2: CUdeviceptr,
    arg3: usize,
    arg4: c_ulonglong,
) -> CUresult {
    let cuMulticastBindAddr: extern "C" fn(
        arg0: CUmemGenericAllocationHandle,
        arg1: usize,
        arg2: CUdeviceptr,
        arg3: usize,
        arg4: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMulticastBindAddr"));
    eprintln!("[CALL] {}", "cuMulticastBindAddr");
    cuMulticastBindAddr(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddMemAllocNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: *mut CUDA_MEM_ALLOC_NODE_PARAMS,
) -> CUresult {
    let cuGraphAddMemAllocNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: *mut CUDA_MEM_ALLOC_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddMemAllocNode"));
    eprintln!("[CALL] {}", "cuGraphAddMemAllocNode");
    cuGraphAddMemAllocNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetP2PAttribute(
    arg0: *mut c_int,
    arg1: CUdevice_P2PAttribute,
    arg2: CUdevice,
    arg3: CUdevice,
) -> CUresult {
    let cuDeviceGetP2PAttribute: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUdevice_P2PAttribute,
        arg2: CUdevice,
        arg3: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetP2PAttribute"));
    eprintln!("[CALL] {}", "cuDeviceGetP2PAttribute");
    cuDeviceGetP2PAttribute(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetId(arg0: CUstream, arg1: *mut c_ulonglong) -> CUresult {
    let cuStreamGetId: extern "C" fn(arg0: CUstream, arg1: *mut c_ulonglong) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetId"));
    eprintln!("[CALL] {}", "cuStreamGetId");
    cuStreamGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD16Async(
    arg0: CUdeviceptr,
    arg1: c_ushort,
    arg2: usize,
    arg3: CUstream,
) -> CUresult {
    let cuMemsetD16Async: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: c_ushort,
        arg2: usize,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD16Async"));
    eprintln!("[CALL] {}", "cuMemsetD16Async");
    cuMemsetD16Async(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxResetPersistingL2Cache() -> CUresult {
    let cuCtxResetPersistingL2Cache: extern "C" fn() -> CUresult =
        std::mem::transmute(get_sym("cuCtxResetPersistingL2Cache"));
    eprintln!("[CALL] {}", "cuCtxResetPersistingL2Cache");
    cuCtxResetPersistingL2Cache()
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetCount(arg0: *mut c_int) -> CUresult {
    let cuDeviceGetCount: extern "C" fn(arg0: *mut c_int) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetCount"));
    eprintln!("[CALL] {}", "cuDeviceGetCount");
    cuDeviceGetCount(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecKernelNodeSetParams_v2(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *const CUDA_KERNEL_NODE_PARAMS,
) -> CUresult {
    let cuGraphExecKernelNodeSetParams_v2: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *const CUDA_KERNEL_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecKernelNodeSetParams_v2"));
    eprintln!("[CALL] {}", "cuGraphExecKernelNodeSetParams_v2");
    cuGraphExecKernelNodeSetParams_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCheckpointProcessCheckpoint(
    arg0: c_int,
    arg1: *mut CUcheckpointCheckpointArgs,
) -> CUresult {
    let cuCheckpointProcessCheckpoint: extern "C" fn(
        arg0: c_int,
        arg1: *mut CUcheckpointCheckpointArgs,
    ) -> CUresult = std::mem::transmute(get_sym("cuCheckpointProcessCheckpoint"));
    eprintln!("[CALL] {}", "cuCheckpointProcessCheckpoint");
    cuCheckpointProcessCheckpoint(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD16_v2(
    arg0: CUdeviceptr,
    arg1: c_ushort,
    arg2: usize,
) -> CUresult {
    let cuMemsetD16_v2: extern "C" fn(arg0: CUdeviceptr, arg1: c_ushort, arg2: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemsetD16_v2"));
    eprintln!("[CALL] {}", "cuMemsetD16_v2");
    cuMemsetD16_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolCreate(
    arg0: *mut CUmemoryPool,
    arg1: *const CUmemPoolProps,
) -> CUresult {
    let cuMemPoolCreate: extern "C" fn(
        arg0: *mut CUmemoryPool,
        arg1: *const CUmemPoolProps,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolCreate"));
    eprintln!("[CALL] {}", "cuMemPoolCreate");
    cuMemPoolCreate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxCreate_v4(
    arg0: *mut CUcontext,
    arg1: *mut CUctxCreateParams,
    arg2: c_uint,
    arg3: CUdevice,
) -> CUresult {
    let cuCtxCreate_v4: extern "C" fn(
        arg0: *mut CUcontext,
        arg1: *mut CUctxCreateParams,
        arg2: c_uint,
        arg3: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuCtxCreate_v4"));
    eprintln!("[CALL] {}", "cuCtxCreate_v4");
    cuCtxCreate_v4(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointBindMem(
    arg0: CUlogicalEndpointId,
    arg1: CUdevice,
    arg2: cuuint64_t,
    arg3: CUmemGenericAllocationHandle,
    arg4: cuuint64_t,
    arg5: cuuint64_t,
    arg6: c_ulonglong,
) -> CUresult {
    let cuLogicalEndpointBindMem: extern "C" fn(
        arg0: CUlogicalEndpointId,
        arg1: CUdevice,
        arg2: cuuint64_t,
        arg3: CUmemGenericAllocationHandle,
        arg4: cuuint64_t,
        arg5: cuuint64_t,
        arg6: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointBindMem"));
    eprintln!("[CALL] {}", "cuLogicalEndpointBindMem");
    cuLogicalEndpointBindMem(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphReleaseUserObject(
    arg0: CUgraph,
    arg1: CUuserObject,
    arg2: c_uint,
) -> CUresult {
    let cuGraphReleaseUserObject: extern "C" fn(
        arg0: CUgraph,
        arg1: CUuserObject,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphReleaseUserObject"));
    eprintln!("[CALL] {}", "cuGraphReleaseUserObject");
    cuGraphReleaseUserObject(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecDestroy(arg0: CUgraphExec) -> CUresult {
    let cuGraphExecDestroy: extern "C" fn(arg0: CUgraphExec) -> CUresult =
        std::mem::transmute(get_sym("cuGraphExecDestroy"));
    eprintln!("[CALL] {}", "cuGraphExecDestroy");
    cuGraphExecDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsUnmapResources(
    arg0: c_uint,
    arg1: *mut CUgraphicsResource,
    arg2: CUstream,
) -> CUresult {
    let cuGraphicsUnmapResources: extern "C" fn(
        arg0: c_uint,
        arg1: *mut CUgraphicsResource,
        arg2: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphicsUnmapResources"));
    eprintln!("[CALL] {}", "cuGraphicsUnmapResources");
    cuGraphicsUnmapResources(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventElapsedTime_v2(
    arg0: *mut f32,
    arg1: CUevent,
    arg2: CUevent,
) -> CUresult {
    let cuEventElapsedTime_v2: extern "C" fn(
        arg0: *mut f32,
        arg1: CUevent,
        arg2: CUevent,
    ) -> CUresult = std::mem::transmute(get_sym("cuEventElapsedTime_v2"));
    eprintln!("[CALL] {}", "cuEventElapsedTime_v2");
    cuEventElapsedTime_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecGetFlags(arg0: CUgraphExec, arg1: *mut cuuint64_t) -> CUresult {
    let cuGraphExecGetFlags: extern "C" fn(arg0: CUgraphExec, arg1: *mut cuuint64_t) -> CUresult =
        std::mem::transmute(get_sym("cuGraphExecGetFlags"));
    eprintln!("[CALL] {}", "cuGraphExecGetFlags");
    cuGraphExecGetFlags(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxAttach(arg0: *mut CUcontext, arg1: c_uint) -> CUresult {
    let cuCtxAttach: extern "C" fn(arg0: *mut CUcontext, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxAttach"));
    eprintln!("[CALL] {}", "cuCtxAttach");
    cuCtxAttach(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxRelease_v2(arg0: CUdevice) -> CUresult {
    let cuDevicePrimaryCtxRelease_v2: extern "C" fn(arg0: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxRelease_v2"));
    eprintln!("[CALL] {}", "cuDevicePrimaryCtxRelease_v2");
    cuDevicePrimaryCtxRelease_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetContainingGraph(
    arg0: CUgraphNode,
    arg1: *mut CUgraph,
) -> CUresult {
    let cuGraphNodeGetContainingGraph: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUgraph,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeGetContainingGraph"));
    eprintln!("[CALL] {}", "cuGraphNodeGetContainingGraph");
    cuGraphNodeGetContainingGraph(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetMipmappedArray(
    arg0: *mut CUmipmappedArray,
    arg1: CUtexref,
) -> CUresult {
    let cuTexRefGetMipmappedArray: extern "C" fn(
        arg0: *mut CUmipmappedArray,
        arg1: CUtexref,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefGetMipmappedArray"));
    eprintln!("[CALL] {}", "cuTexRefGetMipmappedArray");
    cuTexRefGetMipmappedArray(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchCooperativeKernel(
    arg0: CUfunction,
    arg1: c_uint,
    arg2: c_uint,
    arg3: c_uint,
    arg4: c_uint,
    arg5: c_uint,
    arg6: c_uint,
    arg7: c_uint,
    arg8: CUstream,
    arg9: *mut *mut c_void,
) -> CUresult {
    let cuLaunchCooperativeKernel: extern "C" fn(
        arg0: CUfunction,
        arg1: c_uint,
        arg2: c_uint,
        arg3: c_uint,
        arg4: c_uint,
        arg5: c_uint,
        arg6: c_uint,
        arg7: c_uint,
        arg8: CUstream,
        arg9: *mut *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuLaunchCooperativeKernel"));
    eprintln!("[CALL] {}", "cuLaunchCooperativeKernel");
    cuLaunchCooperativeKernel(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetMaxAnisotropy(arg0: *mut c_int, arg1: CUtexref) -> CUresult {
    let cuTexRefGetMaxAnisotropy: extern "C" fn(arg0: *mut c_int, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetMaxAnisotropy"));
    eprintln!("[CALL] {}", "cuTexRefGetMaxAnisotropy");
    cuTexRefGetMaxAnisotropy(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAddressReserve(
    arg0: *mut CUdeviceptr,
    arg1: usize,
    arg2: usize,
    arg3: CUdeviceptr,
    arg4: c_ulonglong,
) -> CUresult {
    let cuMemAddressReserve: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: usize,
        arg2: usize,
        arg3: CUdeviceptr,
        arg4: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemAddressReserve"));
    eprintln!("[CALL] {}", "cuMemAddressReserve");
    cuMemAddressReserve(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphInstantiateWithFlags(
    arg0: *mut CUgraphExec,
    arg1: CUgraph,
    arg2: c_ulonglong,
) -> CUresult {
    let cuGraphInstantiateWithFlags: extern "C" fn(
        arg0: *mut CUgraphExec,
        arg1: CUgraph,
        arg2: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphInstantiateWithFlags"));
    eprintln!("[CALL] {}", "cuGraphInstantiateWithFlags");
    cuGraphInstantiateWithFlags(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocHost_v2(arg0: *mut *mut c_void, arg1: usize) -> CUresult {
    let cuMemAllocHost_v2: extern "C" fn(arg0: *mut *mut c_void, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemAllocHost_v2"));
    eprintln!("[CALL] {}", "cuMemAllocHost_v2");
    cuMemAllocHost_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxDisablePeerAccess(arg0: CUcontext) -> CUresult {
    let cuCtxDisablePeerAccess: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxDisablePeerAccess"));
    eprintln!("[CALL] {}", "cuCtxDisablePeerAccess");
    cuCtxDisablePeerAccess(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxEnablePeerAccess(arg0: CUcontext, arg1: c_uint) -> CUresult {
    let cuCtxEnablePeerAccess: extern "C" fn(arg0: CUcontext, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxEnablePeerAccess"));
    eprintln!("[CALL] {}", "cuCtxEnablePeerAccess");
    cuCtxEnablePeerAccess(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuUserObjectRetain(arg0: CUuserObject, arg1: c_uint) -> CUresult {
    let cuUserObjectRetain: extern "C" fn(arg0: CUuserObject, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuUserObjectRetain"));
    eprintln!("[CALL] {}", "cuUserObjectRetain");
    cuUserObjectRetain(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevSmResourceSplitByCount(
    arg0: *mut CUdevResource,
    arg1: *mut c_uint,
    arg2: *const CUdevResource,
    arg3: *mut CUdevResource,
    arg4: c_uint,
    arg5: c_uint,
) -> CUresult {
    let cuDevSmResourceSplitByCount: extern "C" fn(
        arg0: *mut CUdevResource,
        arg1: *mut c_uint,
        arg2: *const CUdevResource,
        arg3: *mut CUdevResource,
        arg4: c_uint,
        arg5: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuDevSmResourceSplitByCount"));
    eprintln!("[CALL] {}", "cuDevSmResourceSplitByCount");
    cuDevSmResourceSplitByCount(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetFunctionCount(arg0: *mut c_uint, arg1: CUmodule) -> CUresult {
    let cuModuleGetFunctionCount: extern "C" fn(arg0: *mut c_uint, arg1: CUmodule) -> CUresult =
        std::mem::transmute(get_sym("cuModuleGetFunctionCount"));
    eprintln!("[CALL] {}", "cuModuleGetFunctionCount");
    cuModuleGetFunctionCount(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMulticastUnbind(
    arg0: CUmemGenericAllocationHandle,
    arg1: CUdevice,
    arg2: usize,
    arg3: usize,
) -> CUresult {
    let cuMulticastUnbind: extern "C" fn(
        arg0: CUmemGenericAllocationHandle,
        arg1: CUdevice,
        arg2: usize,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMulticastUnbind"));
    eprintln!("[CALL] {}", "cuMulticastUnbind");
    cuMulticastUnbind(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetMipmappedArray(
    arg0: CUtexref,
    arg1: CUmipmappedArray,
    arg2: c_uint,
) -> CUresult {
    let cuTexRefSetMipmappedArray: extern "C" fn(
        arg0: CUtexref,
        arg1: CUmipmappedArray,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefSetMipmappedArray"));
    eprintln!("[CALL] {}", "cuTexRefSetMipmappedArray");
    cuTexRefSetMipmappedArray(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyPeer(
    arg0: CUdeviceptr,
    arg1: CUcontext,
    arg2: CUdeviceptr,
    arg3: CUcontext,
    arg4: usize,
) -> CUresult {
    let cuMemcpyPeer: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: CUcontext,
        arg2: CUdeviceptr,
        arg3: CUcontext,
        arg4: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyPeer"));
    eprintln!("[CALL] {}", "cuMemcpyPeer");
    cuMemcpyPeer(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphDebugDotPrint(
    arg0: CUgraph,
    arg1: *const c_char,
    arg2: c_uint,
) -> CUresult {
    let cuGraphDebugDotPrint: extern "C" fn(
        arg0: CUgraph,
        arg1: *const c_char,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphDebugDotPrint"));
    eprintln!("[CALL] {}", "cuGraphDebugDotPrint");
    cuGraphDebugDotPrint(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncSetSharedSize(arg0: CUfunction, arg1: c_uint) -> CUresult {
    let cuFuncSetSharedSize: extern "C" fn(arg0: CUfunction, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuFuncSetSharedSize"));
    eprintln!("[CALL] {}", "cuFuncSetSharedSize");
    cuFuncSetSharedSize(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpGetAttributeGlobal(
    arg0: CUcoredumpSettings,
    arg1: *mut c_void,
    arg2: *mut usize,
) -> CUresult {
    let cuCoredumpGetAttributeGlobal: extern "C" fn(
        arg0: CUcoredumpSettings,
        arg1: *mut c_void,
        arg2: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpGetAttributeGlobal"));
    eprintln!("[CALL] {}", "cuCoredumpGetAttributeGlobal");
    cuCoredumpGetAttributeGlobal(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolImportPointer(
    arg0: *mut CUdeviceptr,
    arg1: CUmemoryPool,
    arg2: *mut CUmemPoolPtrExportData,
) -> CUresult {
    let cuMemPoolImportPointer: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: CUmemoryPool,
        arg2: *mut CUmemPoolPtrExportData,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolImportPointer"));
    eprintln!("[CALL] {}", "cuMemPoolImportPointer");
    cuMemPoolImportPointer(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleUnload(arg0: CUmodule) -> CUresult {
    let cuModuleUnload: extern "C" fn(arg0: CUmodule) -> CUresult =
        std::mem::transmute(get_sym("cuModuleUnload"));
    eprintln!("[CALL] {}", "cuModuleUnload");
    cuModuleUnload(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoHAsync_v2(
    arg0: *mut c_void,
    arg1: CUdeviceptr,
    arg2: usize,
    arg3: CUstream,
) -> CUresult {
    let cuMemcpyDtoHAsync_v2: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUdeviceptr,
        arg2: usize,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyDtoHAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpyDtoHAsync_v2");
    cuMemcpyDtoHAsync_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuPointerGetAttributes(
    arg0: c_uint,
    arg1: *mut CUpointer_attribute,
    arg2: *mut *mut c_void,
    arg3: CUdeviceptr,
) -> CUresult {
    let cuPointerGetAttributes: extern "C" fn(
        arg0: c_uint,
        arg1: *mut CUpointer_attribute,
        arg2: *mut *mut c_void,
        arg3: CUdeviceptr,
    ) -> CUresult = std::mem::transmute(get_sym("cuPointerGetAttributes"));
    eprintln!("[CALL] {}", "cuPointerGetAttributes");
    cuPointerGetAttributes(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuFlushGPUDirectRDMAWrites(
    arg0: CUflushGPUDirectRDMAWritesTarget,
    arg1: CUflushGPUDirectRDMAWritesScope,
) -> CUresult {
    let cuFlushGPUDirectRDMAWrites: extern "C" fn(
        arg0: CUflushGPUDirectRDMAWritesTarget,
        arg1: CUflushGPUDirectRDMAWritesScope,
    ) -> CUresult = std::mem::transmute(get_sym("cuFlushGPUDirectRDMAWrites"));
    eprintln!("[CALL] {}", "cuFlushGPUDirectRDMAWrites");
    cuFlushGPUDirectRDMAWrites(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetLocalId(arg0: CUgraphNode, arg1: *mut c_uint) -> CUresult {
    let cuGraphNodeGetLocalId: extern "C" fn(arg0: CUgraphNode, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuGraphNodeGetLocalId"));
    eprintln!("[CALL] {}", "cuGraphNodeGetLocalId");
    cuGraphNodeGetLocalId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DAsync_v2(
    arg0: *const CUDA_MEMCPY3D,
    arg1: CUstream,
) -> CUresult {
    let cuMemcpy3DAsync_v2: extern "C" fn(arg0: *const CUDA_MEMCPY3D, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy3DAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpy3DAsync_v2");
    cuMemcpy3DAsync_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DAsync_v2(
    arg0: *const CUDA_MEMCPY2D,
    arg1: CUstream,
) -> CUresult {
    let cuMemcpy2DAsync_v2: extern "C" fn(arg0: *const CUDA_MEMCPY2D, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy2DAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpy2DAsync_v2");
    cuMemcpy2DAsync_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkAddData_v2(
    arg0: CUlinkState,
    arg1: CUjitInputType,
    arg2: *mut c_void,
    arg3: usize,
    arg4: *const c_char,
    arg5: c_uint,
    arg6: *mut CUjit_option,
    arg7: *mut *mut c_void,
) -> CUresult {
    let cuLinkAddData_v2: extern "C" fn(
        arg0: CUlinkState,
        arg1: CUjitInputType,
        arg2: *mut c_void,
        arg3: usize,
        arg4: *const c_char,
        arg5: c_uint,
        arg6: *mut CUjit_option,
        arg7: *mut *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuLinkAddData_v2"));
    eprintln!("[CALL] {}", "cuLinkAddData_v2");
    cuLinkAddData_v2(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}
#[no_mangle]
pub unsafe extern "C" fn cuSurfRefSetArray(
    arg0: CUsurfref,
    arg1: CUarray,
    arg2: c_uint,
) -> CUresult {
    let cuSurfRefSetArray: extern "C" fn(arg0: CUsurfref, arg1: CUarray, arg2: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuSurfRefSetArray"));
    eprintln!("[CALL] {}", "cuSurfRefSetArray");
    cuSurfRefSetArray(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFreeHost(arg0: *mut c_void) -> CUresult {
    let cuMemFreeHost: extern "C" fn(arg0: *mut c_void) -> CUresult =
        std::mem::transmute(get_sym("cuMemFreeHost"));
    eprintln!("[CALL] {}", "cuMemFreeHost");
    cuMemFreeHost(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetFormat(
    arg0: *mut CUarray_format,
    arg1: *mut c_int,
    arg2: CUtexref,
) -> CUresult {
    let cuTexRefGetFormat: extern "C" fn(
        arg0: *mut CUarray_format,
        arg1: *mut c_int,
        arg2: CUtexref,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefGetFormat"));
    eprintln!("[CALL] {}", "cuTexRefGetFormat");
    cuTexRefGetFormat(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAlloc_v2(arg0: *mut CUdeviceptr, arg1: usize) -> CUresult {
    let cuMemAlloc_v2: extern "C" fn(arg0: *mut CUdeviceptr, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemAlloc_v2"));
    eprintln!("[CALL] {}", "cuMemAlloc_v2");
    cuMemAlloc_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphEventWaitNodeGetEvent(
    arg0: CUgraphNode,
    arg1: *mut CUevent,
) -> CUresult {
    let cuGraphEventWaitNodeGetEvent: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUevent,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphEventWaitNodeGetEvent"));
    eprintln!("[CALL] {}", "cuGraphEventWaitNodeGetEvent");
    cuGraphEventWaitNodeGetEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoH_v2(
    arg0: *mut c_void,
    arg1: CUarray,
    arg2: usize,
    arg3: usize,
) -> CUresult {
    let cuMemcpyAtoH_v2: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUarray,
        arg2: usize,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyAtoH_v2"));
    eprintln!("[CALL] {}", "cuMemcpyAtoH_v2");
    cuMemcpyAtoH_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMulticastCreate(
    arg0: *mut CUmemGenericAllocationHandle,
    arg1: *const CUmulticastObjectProp,
) -> CUresult {
    let cuMulticastCreate: extern "C" fn(
        arg0: *mut CUmemGenericAllocationHandle,
        arg1: *const CUmulticastObjectProp,
    ) -> CUresult = std::mem::transmute(get_sym("cuMulticastCreate"));
    eprintln!("[CALL] {}", "cuMulticastCreate");
    cuMulticastCreate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeSetParams(
    arg0: CUgraphNode,
    arg1: *mut CUgraphNodeParams,
) -> CUresult {
    let cuGraphNodeSetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUgraphNodeParams,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphNodeSetParams");
    cuGraphNodeSetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetP2PAtomicCapabilities(
    arg0: *mut c_uint,
    arg1: *const CUatomicOperation,
    arg2: c_uint,
    arg3: CUdevice,
    arg4: CUdevice,
) -> CUresult {
    let cuDeviceGetP2PAtomicCapabilities: extern "C" fn(
        arg0: *mut c_uint,
        arg1: *const CUatomicOperation,
        arg2: c_uint,
        arg3: CUdevice,
        arg4: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetP2PAtomicCapabilities"));
    eprintln!("[CALL] {}", "cuDeviceGetP2PAtomicCapabilities");
    cuDeviceGetP2PAtomicCapabilities(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddress2D_v3(
    arg0: CUtexref,
    arg1: *const CUDA_ARRAY_DESCRIPTOR,
    arg2: CUdeviceptr,
    arg3: usize,
) -> CUresult {
    let cuTexRefSetAddress2D_v3: extern "C" fn(
        arg0: CUtexref,
        arg1: *const CUDA_ARRAY_DESCRIPTOR,
        arg2: CUdeviceptr,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefSetAddress2D_v3"));
    eprintln!("[CALL] {}", "cuTexRefSetAddress2D_v3");
    cuTexRefSetAddress2D_v3(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphMemcpyNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUDA_MEMCPY3D,
) -> CUresult {
    let cuGraphMemcpyNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUDA_MEMCPY3D,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphMemcpyNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphMemcpyNodeGetParams");
    cuGraphMemcpyNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetGraphMemAttribute(
    arg0: CUdevice,
    arg1: CUgraphMem_attribute,
    arg2: *mut c_void,
) -> CUresult {
    let cuDeviceGetGraphMemAttribute: extern "C" fn(
        arg0: CUdevice,
        arg1: CUgraphMem_attribute,
        arg2: *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetGraphMemAttribute"));
    eprintln!("[CALL] {}", "cuDeviceGetGraphMemAttribute");
    cuDeviceGetGraphMemAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelSetCacheConfig(
    arg0: CUkernel,
    arg1: CUfunc_cache,
    arg2: CUdevice,
) -> CUresult {
    let cuKernelSetCacheConfig: extern "C" fn(
        arg0: CUkernel,
        arg1: CUfunc_cache,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuKernelSetCacheConfig"));
    eprintln!("[CALL] {}", "cuKernelSetCacheConfig");
    cuKernelSetCacheConfig(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexObjectGetTextureDesc(
    arg0: *mut CUDA_TEXTURE_DESC,
    arg1: CUtexObject,
) -> CUresult {
    let cuTexObjectGetTextureDesc: extern "C" fn(
        arg0: *mut CUDA_TEXTURE_DESC,
        arg1: CUtexObject,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexObjectGetTextureDesc"));
    eprintln!("[CALL] {}", "cuTexObjectGetTextureDesc");
    cuTexObjectGetTextureDesc(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetGreenCtx(arg0: CUstream, arg1: *mut CUgreenCtx) -> CUresult {
    let cuStreamGetGreenCtx: extern "C" fn(arg0: CUstream, arg1: *mut CUgreenCtx) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetGreenCtx"));
    eprintln!("[CALL] {}", "cuStreamGetGreenCtx");
    cuStreamGetGreenCtx(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleLoadDataEx(
    arg0: *mut CUmodule,
    arg1: *const c_void,
    arg2: c_uint,
    arg3: *mut CUjit_option,
    arg4: *mut *mut c_void,
) -> CUresult {
    let cuModuleLoadDataEx: extern "C" fn(
        arg0: *mut CUmodule,
        arg1: *const c_void,
        arg2: c_uint,
        arg3: *mut CUjit_option,
        arg4: *mut *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuModuleLoadDataEx"));
    eprintln!("[CALL] {}", "cuModuleLoadDataEx");
    cuModuleLoadDataEx(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleLoadFatBinary(
    arg0: *mut CUmodule,
    arg1: *const c_void,
) -> CUresult {
    let cuModuleLoadFatBinary: extern "C" fn(arg0: *mut CUmodule, arg1: *const c_void) -> CUresult =
        std::mem::transmute(get_sym("cuModuleLoadFatBinary"));
    eprintln!("[CALL] {}", "cuModuleLoadFatBinary");
    cuModuleLoadFatBinary(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayGetMemoryRequirements(
    arg0: *mut CUDA_ARRAY_MEMORY_REQUIREMENTS,
    arg1: CUarray,
    arg2: CUdevice,
) -> CUresult {
    let cuArrayGetMemoryRequirements: extern "C" fn(
        arg0: *mut CUDA_ARRAY_MEMORY_REQUIREMENTS,
        arg1: CUarray,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuArrayGetMemoryRequirements"));
    eprintln!("[CALL] {}", "cuArrayGetMemoryRequirements");
    cuArrayGetMemoryRequirements(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddEventWaitNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: CUevent,
) -> CUresult {
    let cuGraphAddEventWaitNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: CUevent,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddEventWaitNode"));
    eprintln!("[CALL] {}", "cuGraphAddEventWaitNode");
    cuGraphAddEventWaitNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceUnregisterAsyncNotification(
    arg0: CUdevice,
    arg1: CUasyncCallbackHandle,
) -> CUresult {
    let cuDeviceUnregisterAsyncNotification: extern "C" fn(
        arg0: CUdevice,
        arg1: CUasyncCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceUnregisterAsyncNotification"));
    eprintln!("[CALL] {}", "cuDeviceUnregisterAsyncNotification");
    cuDeviceUnregisterAsyncNotification(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxCreate(
    arg0: *mut CUgreenCtx,
    arg1: CUdevResourceDesc,
    arg2: CUdevice,
    arg3: c_uint,
) -> CUresult {
    let cuGreenCtxCreate: extern "C" fn(
        arg0: *mut CUgreenCtx,
        arg1: CUdevResourceDesc,
        arg2: CUdevice,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGreenCtxCreate"));
    eprintln!("[CALL] {}", "cuGreenCtxCreate");
    cuGreenCtxCreate(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuPointerGetAttribute(
    arg0: *mut c_void,
    arg1: CUpointer_attribute,
    arg2: CUdeviceptr,
) -> CUresult {
    let cuPointerGetAttribute: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUpointer_attribute,
        arg2: CUdeviceptr,
    ) -> CUresult = std::mem::transmute(get_sym("cuPointerGetAttribute"));
    eprintln!("[CALL] {}", "cuPointerGetAttribute");
    cuPointerGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCaptureToGraph(
    arg0: CUstream,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: *const CUgraphEdgeData,
    arg4: usize,
    arg5: CUstreamCaptureMode,
) -> CUresult {
    let cuStreamBeginCaptureToGraph: extern "C" fn(
        arg0: CUstream,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: *const CUgraphEdgeData,
        arg4: usize,
        arg5: CUstreamCaptureMode,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamBeginCaptureToGraph"));
    eprintln!("[CALL] {}", "cuStreamBeginCaptureToGraph");
    cuStreamBeginCaptureToGraph(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuParamSetTexRef(
    arg0: CUfunction,
    arg1: c_int,
    arg2: CUtexref,
) -> CUresult {
    let cuParamSetTexRef: extern "C" fn(arg0: CUfunction, arg1: c_int, arg2: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuParamSetTexRef"));
    eprintln!("[CALL] {}", "cuParamSetTexRef");
    cuParamSetTexRef(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetHandleForAddressRange(
    arg0: *mut c_void,
    arg1: CUdeviceptr,
    arg2: usize,
    arg3: CUmemRangeHandleType,
    arg4: c_ulonglong,
) -> CUresult {
    let cuMemGetHandleForAddressRange: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUdeviceptr,
        arg2: usize,
        arg3: CUmemRangeHandleType,
        arg4: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemGetHandleForAddressRange"));
    eprintln!("[CALL] {}", "cuMemGetHandleForAddressRange");
    cuMemGetHandleForAddressRange(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGetProcAddress_v2(
    arg0: *const c_char,
    arg1: *mut *mut c_void,
    arg2: c_int,
    arg3: cuuint64_t,
    arg4: *mut CUdriverProcAddressQueryResult,
) -> CUresult {
    let cuGetProcAddress_v2: extern "C" fn(
        arg0: *const c_char,
        arg1: *mut *mut c_void,
        arg2: c_int,
        arg3: cuuint64_t,
        arg4: *mut CUdriverProcAddressQueryResult,
    ) -> CUresult = std::mem::transmute(get_sym("cuGetProcAddress_v2"));
    eprintln!("[CALL] {}", "cuGetProcAddress_v2");
    cuGetProcAddress_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcGetMemHandle(
    arg0: *mut CUipcMemHandle,
    arg1: CUdeviceptr,
) -> CUresult {
    let cuIpcGetMemHandle: extern "C" fn(arg0: *mut CUipcMemHandle, arg1: CUdeviceptr) -> CUresult =
        std::mem::transmute(get_sym("cuIpcGetMemHandle"));
    eprintln!("[CALL] {}", "cuIpcGetMemHandle");
    cuIpcGetMemHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexObjectCreate(
    arg0: *mut CUtexObject,
    arg1: *const CUDA_RESOURCE_DESC,
    arg2: *const CUDA_TEXTURE_DESC,
    arg3: *const CUDA_RESOURCE_VIEW_DESC,
) -> CUresult {
    let cuTexObjectCreate: extern "C" fn(
        arg0: *mut CUtexObject,
        arg1: *const CUDA_RESOURCE_DESC,
        arg2: *const CUDA_TEXTURE_DESC,
        arg3: *const CUDA_RESOURCE_VIEW_DESC,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexObjectCreate"));
    eprintln!("[CALL] {}", "cuTexObjectCreate");
    cuTexObjectCreate(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetMipmapLevelBias(arg0: CUtexref, arg1: f32) -> CUresult {
    let cuTexRefSetMipmapLevelBias: extern "C" fn(arg0: CUtexref, arg1: f32) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetMipmapLevelBias"));
    eprintln!("[CALL] {}", "cuTexRefSetMipmapLevelBias");
    cuTexRefSetMipmapLevelBias(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGetExportTable(
    arg0: *mut *const c_void,
    arg1: *const CUuuid,
) -> CUresult {
    let cuGetExportTable: extern "C" fn(arg0: *mut *const c_void, arg1: *const CUuuid) -> CUresult =
        std::mem::transmute(get_sym("cuGetExportTable"));
    eprintln!("[CALL] {}", "cuGetExportTable");
    cuGetExportTable(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCheckpointProcessRestore(
    arg0: c_int,
    arg1: *mut CUcheckpointRestoreArgs,
) -> CUresult {
    let cuCheckpointProcessRestore: extern "C" fn(
        arg0: c_int,
        arg1: *mut CUcheckpointRestoreArgs,
    ) -> CUresult = std::mem::transmute(get_sym("cuCheckpointProcessRestore"));
    eprintln!("[CALL] {}", "cuCheckpointProcessRestore");
    cuCheckpointProcessRestore(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DWithAttributesAsync(
    arg0: *mut CUDA_MEMCPY3D_BATCH_OP,
    arg1: c_ulonglong,
    arg2: CUstream,
) -> CUresult {
    let cuMemcpy3DWithAttributesAsync: extern "C" fn(
        arg0: *mut CUDA_MEMCPY3D_BATCH_OP,
        arg1: c_ulonglong,
        arg2: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpy3DWithAttributesAsync"));
    eprintln!("[CALL] {}", "cuMemcpy3DWithAttributesAsync");
    cuMemcpy3DWithAttributesAsync(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpRegisterCompleteCallback(
    arg0: CUcoredumpStatusCallback,
    arg1: *mut c_void,
    arg2: *mut CUcoredumpCallbackHandle,
) -> CUresult {
    let cuCoredumpRegisterCompleteCallback: extern "C" fn(
        arg0: CUcoredumpStatusCallback,
        arg1: *mut c_void,
        arg2: *mut CUcoredumpCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpRegisterCompleteCallback"));
    eprintln!("[CALL] {}", "cuCoredumpRegisterCompleteCallback");
    cuCoredumpRegisterCompleteCallback(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecMemcpyNodeSetParams(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *const CUDA_MEMCPY3D,
    arg3: CUcontext,
) -> CUresult {
    let cuGraphExecMemcpyNodeSetParams: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *const CUDA_MEMCPY3D,
        arg3: CUcontext,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecMemcpyNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExecMemcpyNodeSetParams");
    cuGraphExecMemcpyNodeSetParams(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolExportPointer(
    arg0: *mut CUmemPoolPtrExportData,
    arg1: CUdeviceptr,
) -> CUresult {
    let cuMemPoolExportPointer: extern "C" fn(
        arg0: *mut CUmemPoolPtrExportData,
        arg1: CUdeviceptr,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolExportPointer"));
    eprintln!("[CALL] {}", "cuMemPoolExportPointer");
    cuMemPoolExportPointer(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGraphMemTrim(arg0: CUdevice) -> CUresult {
    let cuDeviceGraphMemTrim: extern "C" fn(arg0: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGraphMemTrim"));
    eprintln!("[CALL] {}", "cuDeviceGraphMemTrim");
    cuDeviceGraphMemTrim(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetFlags(arg0: CUstream, arg1: *mut c_uint) -> CUresult {
    let cuStreamGetFlags: extern "C" fn(arg0: CUstream, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetFlags"));
    eprintln!("[CALL] {}", "cuStreamGetFlags");
    cuStreamGetFlags(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxDetach(arg0: CUcontext) -> CUresult {
    let cuCtxDetach: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxDetach"));
    eprintln!("[CALL] {}", "cuCtxDetach");
    cuCtxDetach(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8Async(
    arg0: CUdeviceptr,
    arg1: c_uchar,
    arg2: usize,
    arg3: CUstream,
) -> CUresult {
    let cuMemsetD8Async: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: c_uchar,
        arg2: usize,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD8Async"));
    eprintln!("[CALL] {}", "cuMemsetD8Async");
    cuMemsetD8Async(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeSetAttribute(
    arg0: CUgraphNode,
    arg1: CUkernelNodeAttrID,
    arg2: *const CUkernelNodeAttrValue,
) -> CUresult {
    let cuGraphKernelNodeSetAttribute: extern "C" fn(
        arg0: CUgraphNode,
        arg1: CUkernelNodeAttrID,
        arg2: *const CUkernelNodeAttrValue,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphKernelNodeSetAttribute"));
    eprintln!("[CALL] {}", "cuGraphKernelNodeSetAttribute");
    cuGraphKernelNodeSetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointQuery(
    arg0: CUlogicalEndpointId,
    arg1: cuuint32_t,
    arg2: *mut c_int,
) -> CUresult {
    let cuLogicalEndpointQuery: extern "C" fn(
        arg0: CUlogicalEndpointId,
        arg1: cuuint32_t,
        arg2: *mut c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointQuery"));
    eprintln!("[CALL] {}", "cuLogicalEndpointQuery");
    cuLogicalEndpointQuery(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceGetMappedMipmappedArray(
    arg0: *mut CUmipmappedArray,
    arg1: CUgraphicsResource,
) -> CUresult {
    let cuGraphicsResourceGetMappedMipmappedArray: extern "C" fn(
        arg0: *mut CUmipmappedArray,
        arg1: CUgraphicsResource,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphicsResourceGetMappedMipmappedArray"));
    eprintln!("[CALL] {}", "cuGraphicsResourceGetMappedMipmappedArray");
    cuGraphicsResourceGetMappedMipmappedArray(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointIdRelease(
    arg0: CUlogicalEndpointId,
    arg1: cuuint32_t,
) -> CUresult {
    let cuLogicalEndpointIdRelease: extern "C" fn(
        arg0: CUlogicalEndpointId,
        arg1: cuuint32_t,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointIdRelease"));
    eprintln!("[CALL] {}", "cuLogicalEndpointIdRelease");
    cuLogicalEndpointIdRelease(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoD_v2(
    arg0: CUdeviceptr,
    arg1: CUarray,
    arg2: usize,
    arg3: usize,
) -> CUresult {
    let cuMemcpyAtoD_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: CUarray,
        arg2: usize,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyAtoD_v2"));
    eprintln!("[CALL] {}", "cuMemcpyAtoD_v2");
    cuMemcpyAtoD_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuArray3DGetDescriptor_v2(
    arg0: *mut CUDA_ARRAY3D_DESCRIPTOR,
    arg1: CUarray,
) -> CUresult {
    let cuArray3DGetDescriptor_v2: extern "C" fn(
        arg0: *mut CUDA_ARRAY3D_DESCRIPTOR,
        arg1: CUarray,
    ) -> CUresult = std::mem::transmute(get_sym("cuArray3DGetDescriptor_v2"));
    eprintln!("[CALL] {}", "cuArray3DGetDescriptor_v2");
    cuArray3DGetDescriptor_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetSharedMemConfig(arg0: CUsharedconfig) -> CUresult {
    let cuCtxSetSharedMemConfig: extern "C" fn(arg0: CUsharedconfig) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetSharedMemConfig"));
    eprintln!("[CALL] {}", "cuCtxSetSharedMemConfig");
    cuCtxSetSharedMemConfig(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCtx_v2(
    arg0: CUstream,
    arg1: *mut CUcontext,
    arg2: *mut CUgreenCtx,
) -> CUresult {
    let cuStreamGetCtx_v2: extern "C" fn(
        arg0: CUstream,
        arg1: *mut CUcontext,
        arg2: *mut CUgreenCtx,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamGetCtx_v2"));
    eprintln!("[CALL] {}", "cuStreamGetCtx_v2");
    cuStreamGetCtx_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMulticastBindAddr_v2(
    arg0: CUmemGenericAllocationHandle,
    arg1: CUdevice,
    arg2: usize,
    arg3: CUdeviceptr,
    arg4: usize,
    arg5: c_ulonglong,
) -> CUresult {
    let cuMulticastBindAddr_v2: extern "C" fn(
        arg0: CUmemGenericAllocationHandle,
        arg1: CUdevice,
        arg2: usize,
        arg3: CUdeviceptr,
        arg4: usize,
        arg5: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMulticastBindAddr_v2"));
    eprintln!("[CALL] {}", "cuMulticastBindAddr_v2");
    cuMulticastBindAddr_v2(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphChildGraphNodeGetGraph(
    arg0: CUgraphNode,
    arg1: *mut CUgraph,
) -> CUresult {
    let cuGraphChildGraphNodeGetGraph: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUgraph,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphChildGraphNodeGetGraph"));
    eprintln!("[CALL] {}", "cuGraphChildGraphNodeGetGraph");
    cuGraphChildGraphNodeGetGraph(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxFromGreenCtx(arg0: *mut CUcontext, arg1: CUgreenCtx) -> CUresult {
    let cuCtxFromGreenCtx: extern "C" fn(arg0: *mut CUcontext, arg1: CUgreenCtx) -> CUresult =
        std::mem::transmute(get_sym("cuCtxFromGreenCtx"));
    eprintln!("[CALL] {}", "cuCtxFromGreenCtx");
    cuCtxFromGreenCtx(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetId(arg0: CUcontext, arg1: *mut c_ulonglong) -> CUresult {
    let cuCtxGetId: extern "C" fn(arg0: CUcontext, arg1: *mut c_ulonglong) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetId"));
    eprintln!("[CALL] {}", "cuCtxGetId");
    cuCtxGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxDestroy(arg0: CUgreenCtx) -> CUresult {
    let cuGreenCtxDestroy: extern "C" fn(arg0: CUgreenCtx) -> CUresult =
        std::mem::transmute(get_sym("cuGreenCtxDestroy"));
    eprintln!("[CALL] {}", "cuGreenCtxDestroy");
    cuGreenCtxDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetName(
    arg0: *mut c_char,
    arg1: c_int,
    arg2: CUdevice,
) -> CUresult {
    let cuDeviceGetName: extern "C" fn(arg0: *mut c_char, arg1: c_int, arg2: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetName"));
    eprintln!("[CALL] {}", "cuDeviceGetName");
    cuDeviceGetName(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecNodeSetParams(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *mut CUgraphNodeParams,
) -> CUresult {
    let cuGraphExecNodeSetParams: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *mut CUgraphNodeParams,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExecNodeSetParams");
    cuGraphExecNodeSetParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetPCIBusId(
    arg0: *mut c_char,
    arg1: c_int,
    arg2: CUdevice,
) -> CUresult {
    let cuDeviceGetPCIBusId: extern "C" fn(
        arg0: *mut c_char,
        arg1: c_int,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetPCIBusId"));
    eprintln!("[CALL] {}", "cuDeviceGetPCIBusId");
    cuDeviceGetPCIBusId(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryGetKernelCount(arg0: *mut c_uint, arg1: CUlibrary) -> CUresult {
    let cuLibraryGetKernelCount: extern "C" fn(arg0: *mut c_uint, arg1: CUlibrary) -> CUresult =
        std::mem::transmute(get_sym("cuLibraryGetKernelCount"));
    eprintln!("[CALL] {}", "cuLibraryGetKernelCount");
    cuLibraryGetKernelCount(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryLoadData(
    arg0: *mut CUlibrary,
    arg1: *const c_void,
    arg2: *mut CUjit_option,
    arg3: *mut *mut c_void,
    arg4: c_uint,
    arg5: *mut CUlibraryOption,
    arg6: *mut *mut c_void,
    arg7: c_uint,
) -> CUresult {
    let cuLibraryLoadData: extern "C" fn(
        arg0: *mut CUlibrary,
        arg1: *const c_void,
        arg2: *mut CUjit_option,
        arg3: *mut *mut c_void,
        arg4: c_uint,
        arg5: *mut CUlibraryOption,
        arg6: *mut *mut c_void,
        arg7: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuLibraryLoadData"));
    eprintln!("[CALL] {}", "cuLibraryLoadData");
    cuLibraryLoadData(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostRegister_v2(
    arg0: *mut c_void,
    arg1: usize,
    arg2: c_uint,
) -> CUresult {
    let cuMemHostRegister_v2: extern "C" fn(
        arg0: *mut c_void,
        arg1: usize,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemHostRegister_v2"));
    eprintln!("[CALL] {}", "cuMemHostRegister_v2");
    cuMemHostRegister_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetCacheConfig(arg0: *mut CUfunc_cache) -> CUresult {
    let cuCtxGetCacheConfig: extern "C" fn(arg0: *mut CUfunc_cache) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetCacheConfig"));
    eprintln!("[CALL] {}", "cuCtxGetCacheConfig");
    cuCtxGetCacheConfig(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddMemcpyNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: *const CUDA_MEMCPY3D,
    arg5: CUcontext,
) -> CUresult {
    let cuGraphAddMemcpyNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: *const CUDA_MEMCPY3D,
        arg5: CUcontext,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddMemcpyNode"));
    eprintln!("[CALL] {}", "cuGraphAddMemcpyNode");
    cuGraphAddMemcpyNode(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkComplete(
    arg0: CUlinkState,
    arg1: *mut *mut c_void,
    arg2: *mut usize,
) -> CUresult {
    let cuLinkComplete: extern "C" fn(
        arg0: CUlinkState,
        arg1: *mut *mut c_void,
        arg2: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuLinkComplete"));
    eprintln!("[CALL] {}", "cuLinkComplete");
    cuLinkComplete(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAddressFree(arg0: CUdeviceptr, arg1: usize) -> CUresult {
    let cuMemAddressFree: extern "C" fn(arg0: CUdeviceptr, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemAddressFree"));
    eprintln!("[CALL] {}", "cuMemAddressFree");
    cuMemAddressFree(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuSurfObjectDestroy(arg0: CUsurfObject) -> CUresult {
    let cuSurfObjectDestroy: extern "C" fn(arg0: CUsurfObject) -> CUresult =
        std::mem::transmute(get_sym("cuSurfObjectDestroy"));
    eprintln!("[CALL] {}", "cuSurfObjectDestroy");
    cuSurfObjectDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddMemsetNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: *const CUDA_MEMSET_NODE_PARAMS,
    arg5: CUcontext,
) -> CUresult {
    let cuGraphAddMemsetNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: *const CUDA_MEMSET_NODE_PARAMS,
        arg5: CUcontext,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddMemsetNode"));
    eprintln!("[CALL] {}", "cuGraphAddMemsetNode");
    cuGraphAddMemsetNode(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeCopyAttributes(
    arg0: CUgraphNode,
    arg1: CUgraphNode,
) -> CUresult {
    let cuGraphKernelNodeCopyAttributes: extern "C" fn(
        arg0: CUgraphNode,
        arg1: CUgraphNode,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphKernelNodeCopyAttributes"));
    eprintln!("[CALL] {}", "cuGraphKernelNodeCopyAttributes");
    cuGraphKernelNodeCopyAttributes(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcOpenEventHandle(
    arg0: *mut CUevent,
    arg1: CUipcEventHandle,
) -> CUresult {
    let cuIpcOpenEventHandle: extern "C" fn(
        arg0: *mut CUevent,
        arg1: CUipcEventHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuIpcOpenEventHandle"));
    eprintln!("[CALL] {}", "cuIpcOpenEventHandle");
    cuIpcOpenEventHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetTexture1DLinearMaxWidth(
    arg0: *mut usize,
    arg1: CUarray_format,
    arg2: c_uint,
    arg3: CUdevice,
) -> CUresult {
    let cuDeviceGetTexture1DLinearMaxWidth: extern "C" fn(
        arg0: *mut usize,
        arg1: CUarray_format,
        arg2: c_uint,
        arg3: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetTexture1DLinearMaxWidth"));
    eprintln!("[CALL] {}", "cuDeviceGetTexture1DLinearMaxWidth");
    cuDeviceGetTexture1DLinearMaxWidth(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemSetAccess(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: *const CUmemAccessDesc,
    arg3: usize,
) -> CUresult {
    let cuMemSetAccess: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: *const CUmemAccessDesc,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemSetAccess"));
    eprintln!("[CALL] {}", "cuMemSetAccess");
    cuMemSetAccess(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuExternalMemoryGetMappedMipmappedArray(
    arg0: *mut CUmipmappedArray,
    arg1: CUexternalMemory,
    arg2: *const CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC,
) -> CUresult {
    let cuExternalMemoryGetMappedMipmappedArray: extern "C" fn(
        arg0: *mut CUmipmappedArray,
        arg1: CUexternalMemory,
        arg2: *const CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC,
    ) -> CUresult = std::mem::transmute(get_sym("cuExternalMemoryGetMappedMipmappedArray"));
    eprintln!("[CALL] {}", "cuExternalMemoryGetMappedMipmappedArray");
    cuExternalMemoryGetMappedMipmappedArray(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue64_v2(
    arg0: CUstream,
    arg1: CUdeviceptr,
    arg2: cuuint64_t,
    arg3: c_uint,
) -> CUresult {
    let cuStreamWriteValue64_v2: extern "C" fn(
        arg0: CUstream,
        arg1: CUdeviceptr,
        arg2: cuuint64_t,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamWriteValue64_v2"));
    eprintln!("[CALL] {}", "cuStreamWriteValue64_v2");
    cuStreamWriteValue64_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_v3(
    arg0: CUstream,
    arg1: *mut CUstreamCaptureStatus,
    arg2: *mut cuuint64_t,
    arg3: *mut CUgraph,
    arg4: *mut *const CUgraphNode,
    arg5: *mut *const CUgraphEdgeData,
    arg6: *mut usize,
) -> CUresult {
    let cuStreamGetCaptureInfo_v3: extern "C" fn(
        arg0: CUstream,
        arg1: *mut CUstreamCaptureStatus,
        arg2: *mut cuuint64_t,
        arg3: *mut CUgraph,
        arg4: *mut *const CUgraphNode,
        arg5: *mut *const CUgraphEdgeData,
        arg6: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamGetCaptureInfo_v3"));
    eprintln!("[CALL] {}", "cuStreamGetCaptureInfo_v3");
    cuStreamGetCaptureInfo_v3(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogsCurrent(arg0: *mut CUlogIterator, arg1: c_uint) -> CUresult {
    let cuLogsCurrent: extern "C" fn(arg0: *mut CUlogIterator, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuLogsCurrent"));
    eprintln!("[CALL] {}", "cuLogsCurrent");
    cuLogsCurrent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemBatchDecompressAsync(
    arg0: *mut CUmemDecompressParams,
    arg1: usize,
    arg2: c_uint,
    arg3: *mut usize,
    arg4: CUstream,
) -> CUresult {
    let cuMemBatchDecompressAsync: extern "C" fn(
        arg0: *mut CUmemDecompressParams,
        arg1: usize,
        arg2: c_uint,
        arg3: *mut usize,
        arg4: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemBatchDecompressAsync"));
    eprintln!("[CALL] {}", "cuMemBatchDecompressAsync");
    cuMemBatchDecompressAsync(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
    arg0: *mut c_int,
    arg1: CUfunction,
    arg2: c_int,
    arg3: usize,
    arg4: c_uint,
) -> CUresult {
    let cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUfunction,
        arg2: c_int,
        arg3: usize,
        arg4: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym(
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags",
    ));
    eprintln!(
        "[CALL] {}",
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags"
    );
    cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetDefaultMemPool(
    arg0: *mut CUmemoryPool,
    arg1: *mut CUmemLocation,
    arg2: CUmemAllocationType,
) -> CUresult {
    let cuMemGetDefaultMemPool: extern "C" fn(
        arg0: *mut CUmemoryPool,
        arg1: *mut CUmemLocation,
        arg2: CUmemAllocationType,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemGetDefaultMemPool"));
    eprintln!("[CALL] {}", "cuMemGetDefaultMemPool");
    cuMemGetDefaultMemPool(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetMipmapFilterMode(
    arg0: CUtexref,
    arg1: CUfilter_mode,
) -> CUresult {
    let cuTexRefSetMipmapFilterMode: extern "C" fn(
        arg0: CUtexref,
        arg1: CUfilter_mode,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefSetMipmapFilterMode"));
    eprintln!("[CALL] {}", "cuTexRefSetMipmapFilterMode");
    cuTexRefSetMipmapFilterMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchGrid(arg0: CUfunction, arg1: c_int, arg2: c_int) -> CUresult {
    let cuLaunchGrid: extern "C" fn(arg0: CUfunction, arg1: c_int, arg2: c_int) -> CUresult =
        std::mem::transmute(get_sym("cuLaunchGrid"));
    eprintln!("[CALL] {}", "cuLaunchGrid");
    cuLaunchGrid(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddExternalSemaphoresSignalNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: *const CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
) -> CUresult {
    let cuGraphAddExternalSemaphoresSignalNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: *const CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddExternalSemaphoresSignalNode"));
    eprintln!("[CALL] {}", "cuGraphAddExternalSemaphoresSignalNode");
    cuGraphAddExternalSemaphoresSignalNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetProperties(arg0: *mut CUdevprop, arg1: CUdevice) -> CUresult {
    let cuDeviceGetProperties: extern "C" fn(arg0: *mut CUdevprop, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetProperties"));
    eprintln!("[CALL] {}", "cuDeviceGetProperties");
    cuDeviceGetProperties(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoA_v2(
    arg0: CUarray,
    arg1: usize,
    arg2: CUarray,
    arg3: usize,
    arg4: usize,
) -> CUresult {
    let cuMemcpyAtoA_v2: extern "C" fn(
        arg0: CUarray,
        arg1: usize,
        arg2: CUarray,
        arg3: usize,
        arg4: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyAtoA_v2"));
    eprintln!("[CALL] {}", "cuMemcpyAtoA_v2");
    cuMemcpyAtoA_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddEmptyNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
) -> CUresult {
    let cuGraphAddEmptyNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddEmptyNode"));
    eprintln!("[CALL] {}", "cuGraphAddEmptyNode");
    cuGraphAddEmptyNode(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetDevResource(
    arg0: CUdevice,
    arg1: *mut CUdevResource,
    arg2: CUdevResourceType,
) -> CUresult {
    let cuDeviceGetDevResource: extern "C" fn(
        arg0: CUdevice,
        arg1: *mut CUdevResource,
        arg2: CUdevResourceType,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetDevResource"));
    eprintln!("[CALL] {}", "cuDeviceGetDevResource");
    cuDeviceGetDevResource(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphMemsetNodeSetParams(
    arg0: CUgraphNode,
    arg1: *const CUDA_MEMSET_NODE_PARAMS,
) -> CUresult {
    let cuGraphMemsetNodeSetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *const CUDA_MEMSET_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphMemsetNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphMemsetNodeSetParams");
    cuGraphMemsetNodeSetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceRegisterAsyncNotification(
    arg0: CUdevice,
    arg1: CUasyncCallback,
    arg2: *mut c_void,
    arg3: *mut CUasyncCallbackHandle,
) -> CUresult {
    let cuDeviceRegisterAsyncNotification: extern "C" fn(
        arg0: CUdevice,
        arg1: CUasyncCallback,
        arg2: *mut c_void,
        arg3: *mut CUasyncCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceRegisterAsyncNotification"));
    eprintln!("[CALL] {}", "cuDeviceRegisterAsyncNotification");
    cuDeviceRegisterAsyncNotification(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointBindAddr(
    arg0: CUlogicalEndpointId,
    arg1: CUdevice,
    arg2: cuuint64_t,
    arg3: *mut c_void,
    arg4: cuuint64_t,
    arg5: c_ulonglong,
) -> CUresult {
    let cuLogicalEndpointBindAddr: extern "C" fn(
        arg0: CUlogicalEndpointId,
        arg1: CUdevice,
        arg2: cuuint64_t,
        arg3: *mut c_void,
        arg4: cuuint64_t,
        arg5: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointBindAddr"));
    eprintln!("[CALL] {}", "cuLogicalEndpointBindAddr");
    cuLogicalEndpointBindAddr(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchHostFunc(
    arg0: CUstream,
    arg1: CUhostFn,
    arg2: *mut c_void,
) -> CUresult {
    let cuLaunchHostFunc: extern "C" fn(
        arg0: CUstream,
        arg1: CUhostFn,
        arg2: *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuLaunchHostFunc"));
    eprintln!("[CALL] {}", "cuLaunchHostFunc");
    cuLaunchHostFunc(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemDiscardAndPrefetchBatchAsync(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: usize,
    arg3: *mut CUmemLocation,
    arg4: *mut usize,
    arg5: usize,
    arg6: c_ulonglong,
    arg7: CUstream,
) -> CUresult {
    let cuMemDiscardAndPrefetchBatchAsync: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: usize,
        arg3: *mut CUmemLocation,
        arg4: *mut usize,
        arg5: usize,
        arg6: c_ulonglong,
        arg7: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemDiscardAndPrefetchBatchAsync"));
    eprintln!("[CALL] {}", "cuMemDiscardAndPrefetchBatchAsync");
    cuMemDiscardAndPrefetchBatchAsync(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetBorderColor(arg0: *mut f32, arg1: CUtexref) -> CUresult {
    let cuTexRefGetBorderColor: extern "C" fn(arg0: *mut f32, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetBorderColor"));
    eprintln!("[CALL] {}", "cuTexRefGetBorderColor");
    cuTexRefGetBorderColor(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncIsLoaded(
    arg0: *mut CUfunctionLoadingState,
    arg1: CUfunction,
) -> CUresult {
    let cuFuncIsLoaded: extern "C" fn(
        arg0: *mut CUfunctionLoadingState,
        arg1: CUfunction,
    ) -> CUresult = std::mem::transmute(get_sym("cuFuncIsLoaded"));
    eprintln!("[CALL] {}", "cuFuncIsLoaded");
    cuFuncIsLoaded(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpRegisterStartCallback(
    arg0: CUcoredumpStatusCallback,
    arg1: *mut c_void,
    arg2: *mut CUcoredumpCallbackHandle,
) -> CUresult {
    let cuCoredumpRegisterStartCallback: extern "C" fn(
        arg0: CUcoredumpStatusCallback,
        arg1: *mut c_void,
        arg2: *mut CUcoredumpCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpRegisterStartCallback"));
    eprintln!("[CALL] {}", "cuCoredumpRegisterStartCallback");
    cuCoredumpRegisterStartCallback(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8_v2(arg0: CUdeviceptr, arg1: c_uchar, arg2: usize) -> CUresult {
    let cuMemsetD8_v2: extern "C" fn(arg0: CUdeviceptr, arg1: c_uchar, arg2: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemsetD8_v2"));
    eprintln!("[CALL] {}", "cuMemsetD8_v2");
    cuMemsetD8_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemImportFromShareableHandle(
    arg0: *mut CUmemGenericAllocationHandle,
    arg1: *mut c_void,
    arg2: CUmemAllocationHandleType,
) -> CUresult {
    let cuMemImportFromShareableHandle: extern "C" fn(
        arg0: *mut CUmemGenericAllocationHandle,
        arg1: *mut c_void,
        arg2: CUmemAllocationHandleType,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemImportFromShareableHandle"));
    eprintln!("[CALL] {}", "cuMemImportFromShareableHandle");
    cuMemImportFromShareableHandle(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTensorMapReplaceAddress(
    arg0: *mut CUtensorMap,
    arg1: *mut c_void,
) -> CUresult {
    let cuTensorMapReplaceAddress: extern "C" fn(
        arg0: *mut CUtensorMap,
        arg1: *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuTensorMapReplaceAddress"));
    eprintln!("[CALL] {}", "cuTensorMapReplaceAddress");
    cuTensorMapReplaceAddress(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetExecAffinitySupport(
    arg0: *mut c_int,
    arg1: CUexecAffinityType,
    arg2: CUdevice,
) -> CUresult {
    let cuDeviceGetExecAffinitySupport: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUexecAffinityType,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetExecAffinitySupport"));
    eprintln!("[CALL] {}", "cuDeviceGetExecAffinitySupport");
    cuDeviceGetExecAffinitySupport(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventRecord(arg0: CUevent, arg1: CUstream) -> CUresult {
    let cuEventRecord: extern "C" fn(arg0: CUevent, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuEventRecord"));
    eprintln!("[CALL] {}", "cuEventRecord");
    cuEventRecord(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoDAsync_v2(
    arg0: CUdeviceptr,
    arg1: CUdeviceptr,
    arg2: usize,
    arg3: CUstream,
) -> CUresult {
    let cuMemcpyDtoDAsync_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: CUdeviceptr,
        arg2: usize,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyDtoDAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpyDtoDAsync_v2");
    cuMemcpyDtoDAsync_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemRelease(arg0: CUmemGenericAllocationHandle) -> CUresult {
    let cuMemRelease: extern "C" fn(arg0: CUmemGenericAllocationHandle) -> CUresult =
        std::mem::transmute(get_sym("cuMemRelease"));
    eprintln!("[CALL] {}", "cuMemRelease");
    cuMemRelease(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetAddressMode(
    arg0: *mut CUaddress_mode,
    arg1: CUtexref,
    arg2: c_int,
) -> CUresult {
    let cuTexRefGetAddressMode: extern "C" fn(
        arg0: *mut CUaddress_mode,
        arg1: CUtexref,
        arg2: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefGetAddressMode"));
    eprintln!("[CALL] {}", "cuTexRefGetAddressMode");
    cuTexRefGetAddressMode(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpSetAttribute(
    arg0: CUcoredumpSettings,
    arg1: *mut c_void,
    arg2: *mut usize,
) -> CUresult {
    let cuCoredumpSetAttribute: extern "C" fn(
        arg0: CUcoredumpSettings,
        arg1: *mut c_void,
        arg2: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpSetAttribute"));
    eprintln!("[CALL] {}", "cuCoredumpSetAttribute");
    cuCoredumpSetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphEventRecordNodeSetEvent(
    arg0: CUgraphNode,
    arg1: CUevent,
) -> CUresult {
    let cuGraphEventRecordNodeSetEvent: extern "C" fn(
        arg0: CUgraphNode,
        arg1: CUevent,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphEventRecordNodeSetEvent"));
    eprintln!("[CALL] {}", "cuGraphEventRecordNodeSetEvent");
    cuGraphEventRecordNodeSetEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxDestroy_v2(arg0: CUcontext) -> CUresult {
    let cuCtxDestroy_v2: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxDestroy_v2"));
    eprintln!("[CALL] {}", "cuCtxDestroy_v2");
    cuCtxDestroy_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointGetLimits(
    arg0: *mut cuuint64_t,
    arg1: *mut cuuint64_t,
    arg2: *const CUlogicalEndpointProp,
) -> CUresult {
    let cuLogicalEndpointGetLimits: extern "C" fn(
        arg0: *mut cuuint64_t,
        arg1: *mut cuuint64_t,
        arg2: *const CUlogicalEndpointProp,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointGetLimits"));
    eprintln!("[CALL] {}", "cuLogicalEndpointGetLimits");
    cuLogicalEndpointGetLimits(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamEndCaptureToCig(arg0: CUstream) -> CUresult {
    let cuStreamEndCaptureToCig: extern "C" fn(arg0: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamEndCaptureToCig"));
    eprintln!("[CALL] {}", "cuStreamEndCaptureToCig");
    cuStreamEndCaptureToCig(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamQuery(arg0: CUstream) -> CUresult {
    let cuStreamQuery: extern "C" fn(arg0: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamQuery"));
    eprintln!("[CALL] {}", "cuStreamQuery");
    cuStreamQuery(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncLoad(arg0: CUfunction) -> CUresult {
    let cuFuncLoad: extern "C" fn(arg0: CUfunction) -> CUresult =
        std::mem::transmute(get_sym("cuFuncLoad"));
    eprintln!("[CALL] {}", "cuFuncLoad");
    cuFuncLoad(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostUnregister(arg0: *mut c_void) -> CUresult {
    let cuMemHostUnregister: extern "C" fn(arg0: *mut c_void) -> CUresult =
        std::mem::transmute(get_sym("cuMemHostUnregister"));
    eprintln!("[CALL] {}", "cuMemHostUnregister");
    cuMemHostUnregister(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamAttachMemAsync(
    arg0: CUstream,
    arg1: CUdeviceptr,
    arg2: usize,
    arg3: c_uint,
) -> CUresult {
    let cuStreamAttachMemAsync: extern "C" fn(
        arg0: CUstream,
        arg1: CUdeviceptr,
        arg2: usize,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamAttachMemAsync"));
    eprintln!("[CALL] {}", "cuStreamAttachMemAsync");
    cuStreamAttachMemAsync(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecExternalSemaphoresSignalNodeSetParams(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *const CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
) -> CUresult {
    let cuGraphExecExternalSemaphoresSignalNodeSetParams: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *const CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
    ) -> CUresult =
        std::mem::transmute(get_sym("cuGraphExecExternalSemaphoresSignalNodeSetParams"));
    eprintln!(
        "[CALL] {}",
        "cuGraphExecExternalSemaphoresSignalNodeSetParams"
    );
    cuGraphExecExternalSemaphoresSignalNodeSetParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetLimit(arg0: CUlimit, arg1: usize) -> CUresult {
    let cuCtxSetLimit: extern "C" fn(arg0: CUlimit, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetLimit"));
    eprintln!("[CALL] {}", "cuCtxSetLimit");
    cuCtxSetLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxWaitEvent(arg0: CUgreenCtx, arg1: CUevent) -> CUresult {
    let cuGreenCtxWaitEvent: extern "C" fn(arg0: CUgreenCtx, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuGreenCtxWaitEvent"));
    eprintln!("[CALL] {}", "cuGreenCtxWaitEvent");
    cuGreenCtxWaitEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphRemoveDependencies_v2(
    arg0: CUgraph,
    arg1: *const CUgraphNode,
    arg2: *const CUgraphNode,
    arg3: *const CUgraphEdgeData,
    arg4: usize,
) -> CUresult {
    let cuGraphRemoveDependencies_v2: extern "C" fn(
        arg0: CUgraph,
        arg1: *const CUgraphNode,
        arg2: *const CUgraphNode,
        arg3: *const CUgraphEdgeData,
        arg4: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphRemoveDependencies_v2"));
    eprintln!("[CALL] {}", "cuGraphRemoveDependencies_v2");
    cuGraphRemoveDependencies_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D32_v2(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: c_uint,
    arg3: usize,
    arg4: usize,
) -> CUresult {
    let cuMemsetD2D32_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: c_uint,
        arg3: usize,
        arg4: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD2D32_v2"));
    eprintln!("[CALL] {}", "cuMemsetD2D32_v2");
    cuMemsetD2D32_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemExportToShareableHandle(
    arg0: *mut c_void,
    arg1: CUmemGenericAllocationHandle,
    arg2: CUmemAllocationHandleType,
    arg3: c_ulonglong,
) -> CUresult {
    let cuMemExportToShareableHandle: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUmemGenericAllocationHandle,
        arg2: CUmemAllocationHandleType,
        arg3: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemExportToShareableHandle"));
    eprintln!("[CALL] {}", "cuMemExportToShareableHandle");
    cuMemExportToShareableHandle(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetFunction(arg0: *mut CUfunction, arg1: CUkernel) -> CUresult {
    let cuKernelGetFunction: extern "C" fn(arg0: *mut CUfunction, arg1: CUkernel) -> CUresult =
        std::mem::transmute(get_sym("cuKernelGetFunction"));
    eprintln!("[CALL] {}", "cuKernelGetFunction");
    cuKernelGetFunction(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxRetain(
    arg0: *mut CUcontext,
    arg1: CUdevice,
) -> CUresult {
    let cuDevicePrimaryCtxRetain: extern "C" fn(arg0: *mut CUcontext, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxRetain"));
    eprintln!("[CALL] {}", "cuDevicePrimaryCtxRetain");
    cuDevicePrimaryCtxRetain(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefDestroy(arg0: CUtexref) -> CUresult {
    let cuTexRefDestroy: extern "C" fn(arg0: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefDestroy"));
    eprintln!("[CALL] {}", "cuTexRefDestroy");
    cuTexRefDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocManaged(
    arg0: *mut CUdeviceptr,
    arg1: usize,
    arg2: c_uint,
) -> CUresult {
    let cuMemAllocManaged: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: usize,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemAllocManaged"));
    eprintln!("[CALL] {}", "cuMemAllocManaged");
    cuMemAllocManaged(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxPopCurrent_v2(arg0: *mut CUcontext) -> CUresult {
    let cuCtxPopCurrent_v2: extern "C" fn(arg0: *mut CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxPopCurrent_v2"));
    eprintln!("[CALL] {}", "cuCtxPopCurrent_v2");
    cuCtxPopCurrent_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetArray(arg0: CUtexref, arg1: CUarray, arg2: c_uint) -> CUresult {
    let cuTexRefSetArray: extern "C" fn(arg0: CUtexref, arg1: CUarray, arg2: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetArray"));
    eprintln!("[CALL] {}", "cuTexRefSetArray");
    cuTexRefSetArray(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolSetAccess(
    arg0: CUmemoryPool,
    arg1: *const CUmemAccessDesc,
    arg2: usize,
) -> CUresult {
    let cuMemPoolSetAccess: extern "C" fn(
        arg0: CUmemoryPool,
        arg1: *const CUmemAccessDesc,
        arg2: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolSetAccess"));
    eprintln!("[CALL] {}", "cuMemPoolSetAccess");
    cuMemPoolSetAccess(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitEvent(
    arg0: CUstream,
    arg1: CUevent,
    arg2: c_uint,
) -> CUresult {
    let cuStreamWaitEvent: extern "C" fn(arg0: CUstream, arg1: CUevent, arg2: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuStreamWaitEvent"));
    eprintln!("[CALL] {}", "cuStreamWaitEvent");
    cuStreamWaitEvent(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchGridAsync(
    arg0: CUfunction,
    arg1: c_int,
    arg2: c_int,
    arg3: CUstream,
) -> CUresult {
    let cuLaunchGridAsync: extern "C" fn(
        arg0: CUfunction,
        arg1: c_int,
        arg2: c_int,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuLaunchGridAsync"));
    eprintln!("[CALL] {}", "cuLaunchGridAsync");
    cuLaunchGridAsync(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue64_v2(
    arg0: CUstream,
    arg1: CUdeviceptr,
    arg2: cuuint64_t,
    arg3: c_uint,
) -> CUresult {
    let cuStreamWaitValue64_v2: extern "C" fn(
        arg0: CUstream,
        arg1: CUdeviceptr,
        arg2: cuuint64_t,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamWaitValue64_v2"));
    eprintln!("[CALL] {}", "cuStreamWaitValue64_v2");
    cuStreamWaitValue64_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncSetCacheConfig(arg0: CUfunction, arg1: CUfunc_cache) -> CUresult {
    let cuFuncSetCacheConfig: extern "C" fn(arg0: CUfunction, arg1: CUfunc_cache) -> CUresult =
        std::mem::transmute(get_sym("cuFuncSetCacheConfig"));
    eprintln!("[CALL] {}", "cuFuncSetCacheConfig");
    cuFuncSetCacheConfig(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphInstantiateWithParams(
    arg0: *mut CUgraphExec,
    arg1: CUgraph,
    arg2: *mut CUDA_GRAPH_INSTANTIATE_PARAMS,
) -> CUresult {
    let cuGraphInstantiateWithParams: extern "C" fn(
        arg0: *mut CUgraphExec,
        arg1: CUgraph,
        arg2: *mut CUDA_GRAPH_INSTANTIATE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphInstantiateWithParams"));
    eprintln!("[CALL] {}", "cuGraphInstantiateWithParams");
    cuGraphInstantiateWithParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemMapArrayAsync(
    arg0: *mut CUarrayMapInfo,
    arg1: c_uint,
    arg2: CUstream,
) -> CUresult {
    let cuMemMapArrayAsync: extern "C" fn(
        arg0: *mut CUarrayMapInfo,
        arg1: c_uint,
        arg2: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemMapArrayAsync"));
    eprintln!("[CALL] {}", "cuMemMapArrayAsync");
    cuMemMapArrayAsync(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncGetModule(arg0: *mut CUmodule, arg1: CUfunction) -> CUresult {
    let cuFuncGetModule: extern "C" fn(arg0: *mut CUmodule, arg1: CUfunction) -> CUresult =
        std::mem::transmute(get_sym("cuFuncGetModule"));
    eprintln!("[CALL] {}", "cuFuncGetModule");
    cuFuncGetModule(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsMapResources(
    arg0: c_uint,
    arg1: *mut CUgraphicsResource,
    arg2: CUstream,
) -> CUresult {
    let cuGraphicsMapResources: extern "C" fn(
        arg0: c_uint,
        arg1: *mut CUgraphicsResource,
        arg2: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphicsMapResources"));
    eprintln!("[CALL] {}", "cuGraphicsMapResources");
    cuGraphicsMapResources(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxGetId(arg0: CUgreenCtx, arg1: *mut c_ulonglong) -> CUresult {
    let cuGreenCtxGetId: extern "C" fn(arg0: CUgreenCtx, arg1: *mut c_ulonglong) -> CUresult =
        std::mem::transmute(get_sym("cuGreenCtxGetId"));
    eprintln!("[CALL] {}", "cuGreenCtxGetId");
    cuGreenCtxGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostGetFlags(arg0: *mut c_uint, arg1: *mut c_void) -> CUresult {
    let cuMemHostGetFlags: extern "C" fn(arg0: *mut c_uint, arg1: *mut c_void) -> CUresult =
        std::mem::transmute(get_sym("cuMemHostGetFlags"));
    eprintln!("[CALL] {}", "cuMemHostGetFlags");
    cuMemHostGetFlags(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTensorMapEncodeIm2col(
    arg0: *mut CUtensorMap,
    arg1: CUtensorMapDataType,
    arg2: cuuint32_t,
    arg3: *mut c_void,
    arg4: *const cuuint64_t,
    arg5: *const cuuint64_t,
    arg6: *const c_int,
    arg7: *const c_int,
    arg8: cuuint32_t,
    arg9: cuuint32_t,
    arg10: *const cuuint32_t,
    arg11: CUtensorMapInterleave,
    arg12: CUtensorMapSwizzle,
    arg13: CUtensorMapL2promotion,
    arg14: CUtensorMapFloatOOBfill,
) -> CUresult {
    let cuTensorMapEncodeIm2col: extern "C" fn(
        arg0: *mut CUtensorMap,
        arg1: CUtensorMapDataType,
        arg2: cuuint32_t,
        arg3: *mut c_void,
        arg4: *const cuuint64_t,
        arg5: *const cuuint64_t,
        arg6: *const c_int,
        arg7: *const c_int,
        arg8: cuuint32_t,
        arg9: cuuint32_t,
        arg10: *const cuuint32_t,
        arg11: CUtensorMapInterleave,
        arg12: CUtensorMapSwizzle,
        arg13: CUtensorMapL2promotion,
        arg14: CUtensorMapFloatOOBfill,
    ) -> CUresult = std::mem::transmute(get_sym("cuTensorMapEncodeIm2col"));
    eprintln!("[CALL] {}", "cuTensorMapEncodeIm2col");
    cuTensorMapEncodeIm2col(
        arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
        arg14,
    )
}
#[no_mangle]
pub unsafe extern "C" fn cuExternalMemoryGetMappedBuffer(
    arg0: *mut CUdeviceptr,
    arg1: CUexternalMemory,
    arg2: *const CUDA_EXTERNAL_MEMORY_BUFFER_DESC,
) -> CUresult {
    let cuExternalMemoryGetMappedBuffer: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: CUexternalMemory,
        arg2: *const CUDA_EXTERNAL_MEMORY_BUFFER_DESC,
    ) -> CUresult = std::mem::transmute(get_sym("cuExternalMemoryGetMappedBuffer"));
    eprintln!("[CALL] {}", "cuExternalMemoryGetMappedBuffer");
    cuExternalMemoryGetMappedBuffer(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamSetAttribute(
    arg0: CUstream,
    arg1: CUstreamAttrID,
    arg2: *const CUstreamAttrValue,
) -> CUresult {
    let cuStreamSetAttribute: extern "C" fn(
        arg0: CUstream,
        arg1: CUstreamAttrID,
        arg2: *const CUstreamAttrValue,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamSetAttribute"));
    eprintln!("[CALL] {}", "cuStreamSetAttribute");
    cuStreamSetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryGetUnifiedFunction(
    arg0: *mut *mut c_void,
    arg1: CUlibrary,
    arg2: *const c_char,
) -> CUresult {
    let cuLibraryGetUnifiedFunction: extern "C" fn(
        arg0: *mut *mut c_void,
        arg1: CUlibrary,
        arg2: *const c_char,
    ) -> CUresult = std::mem::transmute(get_sym("cuLibraryGetUnifiedFunction"));
    eprintln!("[CALL] {}", "cuLibraryGetUnifiedFunction");
    cuLibraryGetUnifiedFunction(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuSurfRefGetArray(arg0: *mut CUarray, arg1: CUsurfref) -> CUresult {
    let cuSurfRefGetArray: extern "C" fn(arg0: *mut CUarray, arg1: CUsurfref) -> CUresult =
        std::mem::transmute(get_sym("cuSurfRefGetArray"));
    eprintln!("[CALL] {}", "cuSurfRefGetArray");
    cuSurfRefGetArray(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamAddCallback(
    arg0: CUstream,
    arg1: CUstreamCallback,
    arg2: *mut c_void,
    arg3: c_uint,
) -> CUresult {
    let cuStreamAddCallback: extern "C" fn(
        arg0: CUstream,
        arg1: CUstreamCallback,
        arg2: *mut c_void,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamAddCallback"));
    eprintln!("[CALL] {}", "cuStreamAddCallback");
    cuStreamAddCallback(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddKernelNode_v2(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: *const CUDA_KERNEL_NODE_PARAMS,
) -> CUresult {
    let cuGraphAddKernelNode_v2: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: *const CUDA_KERNEL_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddKernelNode_v2"));
    eprintln!("[CALL] {}", "cuGraphAddKernelNode_v2");
    cuGraphAddKernelNode_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetDevice(arg0: CUstream, arg1: *mut CUdevice) -> CUresult {
    let cuStreamGetDevice: extern "C" fn(arg0: CUstream, arg1: *mut CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetDevice"));
    eprintln!("[CALL] {}", "cuStreamGetDevice");
    cuStreamGetDevice(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCapture_v2(
    arg0: CUstream,
    arg1: CUstreamCaptureMode,
) -> CUresult {
    let cuStreamBeginCapture_v2: extern "C" fn(
        arg0: CUstream,
        arg1: CUstreamCaptureMode,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamBeginCapture_v2"));
    eprintln!("[CALL] {}", "cuStreamBeginCapture_v2");
    cuStreamBeginCapture_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphBatchMemOpNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUDA_BATCH_MEM_OP_NODE_PARAMS,
) -> CUresult {
    let cuGraphBatchMemOpNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUDA_BATCH_MEM_OP_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphBatchMemOpNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphBatchMemOpNodeGetParams");
    cuGraphBatchMemOpNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcGetEventHandle(
    arg0: *mut CUipcEventHandle,
    arg1: CUevent,
) -> CUresult {
    let cuIpcGetEventHandle: extern "C" fn(arg0: *mut CUipcEventHandle, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuIpcGetEventHandle"));
    eprintln!("[CALL] {}", "cuIpcGetEventHandle");
    cuIpcGetEventHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevResourceGenerateDesc(
    arg0: *mut CUdevResourceDesc,
    arg1: *mut CUdevResource,
    arg2: c_uint,
) -> CUresult {
    let cuDevResourceGenerateDesc: extern "C" fn(
        arg0: *mut CUdevResourceDesc,
        arg1: *mut CUdevResource,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuDevResourceGenerateDesc"));
    eprintln!("[CALL] {}", "cuDevResourceGenerateDesc");
    cuDevResourceGenerateDesc(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCheckpointProcessGetState(
    arg0: c_int,
    arg1: *mut CUprocessState,
) -> CUresult {
    let cuCheckpointProcessGetState: extern "C" fn(
        arg0: c_int,
        arg1: *mut CUprocessState,
    ) -> CUresult = std::mem::transmute(get_sym("cuCheckpointProcessGetState"));
    eprintln!("[CALL] {}", "cuCheckpointProcessGetState");
    cuCheckpointProcessGetState(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchBatchAsync(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: usize,
    arg3: *mut CUmemLocation,
    arg4: *mut usize,
    arg5: usize,
    arg6: c_ulonglong,
    arg7: CUstream,
) -> CUresult {
    let cuMemPrefetchBatchAsync: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: usize,
        arg3: *mut CUmemLocation,
        arg4: *mut usize,
        arg5: usize,
        arg6: c_ulonglong,
        arg7: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPrefetchBatchAsync"));
    eprintln!("[CALL] {}", "cuMemPrefetchBatchAsync");
    cuMemPrefetchBatchAsync(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphHostNodeSetParams(
    arg0: CUgraphNode,
    arg1: *const CUDA_HOST_NODE_PARAMS,
) -> CUresult {
    let cuGraphHostNodeSetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *const CUDA_HOST_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphHostNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphHostNodeSetParams");
    cuGraphHostNodeSetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamCreateWithPriority(
    arg0: *mut CUstream,
    arg1: c_uint,
    arg2: c_int,
) -> CUresult {
    let cuStreamCreateWithPriority: extern "C" fn(
        arg0: *mut CUstream,
        arg1: c_uint,
        arg2: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamCreateWithPriority"));
    eprintln!("[CALL] {}", "cuStreamCreateWithPriority");
    cuStreamCreateWithPriority(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayDestroy(arg0: CUarray) -> CUresult {
    let cuArrayDestroy: extern "C" fn(arg0: CUarray) -> CUresult =
        std::mem::transmute(get_sym("cuArrayDestroy"));
    eprintln!("[CALL] {}", "cuArrayDestroy");
    cuArrayDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamCreate(arg0: *mut CUstream, arg1: c_uint) -> CUresult {
    let cuStreamCreate: extern "C" fn(arg0: *mut CUstream, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuStreamCreate"));
    eprintln!("[CALL] {}", "cuStreamCreate");
    cuStreamCreate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecBatchMemOpNodeSetParams(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *const CUDA_BATCH_MEM_OP_NODE_PARAMS,
) -> CUresult {
    let cuGraphExecBatchMemOpNodeSetParams: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *const CUDA_BATCH_MEM_OP_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecBatchMemOpNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExecBatchMemOpNodeSetParams");
    cuGraphExecBatchMemOpNodeSetParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetPriority(arg0: CUstream, arg1: *mut c_int) -> CUresult {
    let cuStreamGetPriority: extern "C" fn(arg0: CUstream, arg1: *mut c_int) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetPriority"));
    eprintln!("[CALL] {}", "cuStreamGetPriority");
    cuStreamGetPriority(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMulticastGetGranularity(
    arg0: *mut usize,
    arg1: *const CUmulticastObjectProp,
    arg2: CUmulticastGranularity_flags,
) -> CUresult {
    let cuMulticastGetGranularity: extern "C" fn(
        arg0: *mut usize,
        arg1: *const CUmulticastObjectProp,
        arg2: CUmulticastGranularity_flags,
    ) -> CUresult = std::mem::transmute(get_sym("cuMulticastGetGranularity"));
    eprintln!("[CALL] {}", "cuMulticastGetGranularity");
    cuMulticastGetGranularity(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetDependencies_v2(
    arg0: CUgraphNode,
    arg1: *mut CUgraphNode,
    arg2: *mut CUgraphEdgeData,
    arg3: *mut usize,
) -> CUresult {
    let cuGraphNodeGetDependencies_v2: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUgraphNode,
        arg2: *mut CUgraphEdgeData,
        arg3: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeGetDependencies_v2"));
    eprintln!("[CALL] {}", "cuGraphNodeGetDependencies_v2");
    cuGraphNodeGetDependencies_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelSetAttribute(
    arg0: CUfunction_attribute,
    arg1: c_int,
    arg2: CUkernel,
    arg3: CUdevice,
) -> CUresult {
    let cuKernelSetAttribute: extern "C" fn(
        arg0: CUfunction_attribute,
        arg1: c_int,
        arg2: CUkernel,
        arg3: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuKernelSetAttribute"));
    eprintln!("[CALL] {}", "cuKernelSetAttribute");
    cuKernelSetAttribute(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxSetFlags_v2(arg0: CUdevice, arg1: c_uint) -> CUresult {
    let cuDevicePrimaryCtxSetFlags_v2: extern "C" fn(arg0: CUdevice, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxSetFlags_v2"));
    eprintln!("[CALL] {}", "cuDevicePrimaryCtxSetFlags_v2");
    cuDevicePrimaryCtxSetFlags_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuOccupancyMaxPotentialBlockSize(
    arg0: *mut c_int,
    arg1: *mut c_int,
    arg2: CUfunction,
    arg3: CUoccupancyB2DSize,
    arg4: usize,
    arg5: c_int,
) -> CUresult {
    let cuOccupancyMaxPotentialBlockSize: extern "C" fn(
        arg0: *mut c_int,
        arg1: *mut c_int,
        arg2: CUfunction,
        arg3: CUoccupancyB2DSize,
        arg4: usize,
        arg5: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuOccupancyMaxPotentialBlockSize"));
    eprintln!("[CALL] {}", "cuOccupancyMaxPotentialBlockSize");
    cuOccupancyMaxPotentialBlockSize(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DBatchAsync_v2(
    arg0: usize,
    arg1: *mut CUDA_MEMCPY3D_BATCH_OP,
    arg2: c_ulonglong,
    arg3: CUstream,
) -> CUresult {
    let cuMemcpy3DBatchAsync_v2: extern "C" fn(
        arg0: usize,
        arg1: *mut CUDA_MEMCPY3D_BATCH_OP,
        arg2: c_ulonglong,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpy3DBatchAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpy3DBatchAsync_v2");
    cuMemcpy3DBatchAsync_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2D_v2(arg0: *const CUDA_MEMCPY2D) -> CUresult {
    let cuMemcpy2D_v2: extern "C" fn(arg0: *const CUDA_MEMCPY2D) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy2D_v2"));
    eprintln!("[CALL] {}", "cuMemcpy2D_v2");
    cuMemcpy2D_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetFormat(
    arg0: CUtexref,
    arg1: CUarray_format,
    arg2: c_int,
) -> CUresult {
    let cuTexRefSetFormat: extern "C" fn(
        arg0: CUtexref,
        arg1: CUarray_format,
        arg2: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefSetFormat"));
    eprintln!("[CALL] {}", "cuTexRefSetFormat");
    cuTexRefSetFormat(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayGetDescriptor_v2(
    arg0: *mut CUDA_ARRAY_DESCRIPTOR,
    arg1: CUarray,
) -> CUresult {
    let cuArrayGetDescriptor_v2: extern "C" fn(
        arg0: *mut CUDA_ARRAY_DESCRIPTOR,
        arg1: CUarray,
    ) -> CUresult = std::mem::transmute(get_sym("cuArrayGetDescriptor_v2"));
    eprintln!("[CALL] {}", "cuArrayGetDescriptor_v2");
    cuArrayGetDescriptor_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetFilterMode(
    arg0: *mut CUfilter_mode,
    arg1: CUtexref,
) -> CUresult {
    let cuTexRefGetFilterMode: extern "C" fn(arg0: *mut CUfilter_mode, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetFilterMode"));
    eprintln!("[CALL] {}", "cuTexRefGetFilterMode");
    cuTexRefGetFilterMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D8Async(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: c_uchar,
    arg3: usize,
    arg4: usize,
    arg5: CUstream,
) -> CUresult {
    let cuMemsetD2D8Async: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: c_uchar,
        arg3: usize,
        arg4: usize,
        arg5: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD2D8Async"));
    eprintln!("[CALL] {}", "cuMemsetD2D8Async");
    cuMemsetD2D8Async(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocPitch_v2(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: usize,
    arg3: usize,
    arg4: c_uint,
) -> CUresult {
    let cuMemAllocPitch_v2: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: usize,
        arg3: usize,
        arg4: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemAllocPitch_v2"));
    eprintln!("[CALL] {}", "cuMemAllocPitch_v2");
    cuMemAllocPitch_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginRecaptureToGraph(
    arg0: CUstream,
    arg1: CUstreamCaptureMode,
    arg2: CUgraph,
    arg3: CUgraphRecaptureCallback,
    arg4: *mut c_void,
) -> CUresult {
    let cuStreamBeginRecaptureToGraph: extern "C" fn(
        arg0: CUstream,
        arg1: CUstreamCaptureMode,
        arg2: CUgraph,
        arg3: CUgraphRecaptureCallback,
        arg4: *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamBeginRecaptureToGraph"));
    eprintln!("[CALL] {}", "cuStreamBeginRecaptureToGraph");
    cuStreamBeginRecaptureToGraph(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphClone(arg0: *mut CUgraph, arg1: CUgraph) -> CUresult {
    let cuGraphClone: extern "C" fn(arg0: *mut CUgraph, arg1: CUgraph) -> CUresult =
        std::mem::transmute(get_sym("cuGraphClone"));
    eprintln!("[CALL] {}", "cuGraphClone");
    cuGraphClone(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetStreamPriorityRange(
    arg0: *mut c_int,
    arg1: *mut c_int,
) -> CUresult {
    let cuCtxGetStreamPriorityRange: extern "C" fn(arg0: *mut c_int, arg1: *mut c_int) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetStreamPriorityRange"));
    eprintln!("[CALL] {}", "cuCtxGetStreamPriorityRange");
    cuCtxGetStreamPriorityRange(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetDefaultMemPool(
    arg0: *mut CUmemoryPool,
    arg1: CUdevice,
) -> CUresult {
    let cuDeviceGetDefaultMemPool: extern "C" fn(
        arg0: *mut CUmemoryPool,
        arg1: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetDefaultMemPool"));
    eprintln!("[CALL] {}", "cuDeviceGetDefaultMemPool");
    cuDeviceGetDefaultMemPool(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuUserObjectCreate(
    arg0: *mut CUuserObject,
    arg1: *mut c_void,
    arg2: CUhostFn,
    arg3: c_uint,
    arg4: c_uint,
) -> CUresult {
    let cuUserObjectCreate: extern "C" fn(
        arg0: *mut CUuserObject,
        arg1: *mut c_void,
        arg2: CUhostFn,
        arg3: c_uint,
        arg4: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuUserObjectCreate"));
    eprintln!("[CALL] {}", "cuUserObjectCreate");
    cuUserObjectCreate(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemDiscardBatchAsync(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: usize,
    arg3: c_ulonglong,
    arg4: CUstream,
) -> CUresult {
    let cuMemDiscardBatchAsync: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: usize,
        arg3: c_ulonglong,
        arg4: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemDiscardBatchAsync"));
    eprintln!("[CALL] {}", "cuMemDiscardBatchAsync");
    cuMemDiscardBatchAsync(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphMemcpyNodeSetParams(
    arg0: CUgraphNode,
    arg1: *const CUDA_MEMCPY3D,
) -> CUresult {
    let cuGraphMemcpyNodeSetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *const CUDA_MEMCPY3D,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphMemcpyNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphMemcpyNodeSetParams");
    cuGraphMemcpyNodeSetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchAsync_v2(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: CUmemLocation,
    arg3: c_uint,
    arg4: CUstream,
) -> CUresult {
    let cuMemPrefetchAsync_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: CUmemLocation,
        arg3: c_uint,
        arg4: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPrefetchAsync_v2"));
    eprintln!("[CALL] {}", "cuMemPrefetchAsync_v2");
    cuMemPrefetchAsync_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuMulticastAddDevice(
    arg0: CUmemGenericAllocationHandle,
    arg1: CUdevice,
) -> CUresult {
    let cuMulticastAddDevice: extern "C" fn(
        arg0: CUmemGenericAllocationHandle,
        arg1: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuMulticastAddDevice"));
    eprintln!("[CALL] {}", "cuMulticastAddDevice");
    cuMulticastAddDevice(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeGetAttribute(
    arg0: CUgraphNode,
    arg1: CUkernelNodeAttrID,
    arg2: *mut CUkernelNodeAttrValue,
) -> CUresult {
    let cuGraphKernelNodeGetAttribute: extern "C" fn(
        arg0: CUgraphNode,
        arg1: CUkernelNodeAttrID,
        arg2: *mut CUkernelNodeAttrValue,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphKernelNodeGetAttribute"));
    eprintln!("[CALL] {}", "cuGraphKernelNodeGetAttribute");
    cuGraphKernelNodeGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGet(arg0: *mut CUdevice, arg1: c_int) -> CUresult {
    let cuDeviceGet: extern "C" fn(arg0: *mut CUdevice, arg1: c_int) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGet"));
    eprintln!("[CALL] {}", "cuDeviceGet");
    cuDeviceGet(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphRetainUserObject(
    arg0: CUgraph,
    arg1: CUuserObject,
    arg2: c_uint,
    arg3: c_uint,
) -> CUresult {
    let cuGraphRetainUserObject: extern "C" fn(
        arg0: CUgraph,
        arg1: CUuserObject,
        arg2: c_uint,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphRetainUserObject"));
    eprintln!("[CALL] {}", "cuGraphRetainUserObject");
    cuGraphRetainUserObject(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetUuid_v2(arg0: *mut CUuuid, arg1: CUdevice) -> CUresult {
    let cuDeviceGetUuid_v2: extern "C" fn(arg0: *mut CUuuid, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetUuid_v2"));
    eprintln!("[CALL] {}", "cuDeviceGetUuid_v2");
    cuDeviceGetUuid_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DPeerAsync(
    arg0: *const CUDA_MEMCPY3D_PEER,
    arg1: CUstream,
) -> CUresult {
    let cuMemcpy3DPeerAsync: extern "C" fn(
        arg0: *const CUDA_MEMCPY3D_PEER,
        arg1: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpy3DPeerAsync"));
    eprintln!("[CALL] {}", "cuMemcpy3DPeerAsync");
    cuMemcpy3DPeerAsync(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxPushCurrent_v2(arg0: CUcontext) -> CUresult {
    let cuCtxPushCurrent_v2: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxPushCurrent_v2"));
    eprintln!("[CALL] {}", "cuCtxPushCurrent_v2");
    cuCtxPushCurrent_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExternalSemaphoresWaitNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUDA_EXT_SEM_WAIT_NODE_PARAMS,
) -> CUresult {
    let cuGraphExternalSemaphoresWaitNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUDA_EXT_SEM_WAIT_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExternalSemaphoresWaitNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphExternalSemaphoresWaitNodeGetParams");
    cuGraphExternalSemaphoresWaitNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetTexRef(
    arg0: *mut CUtexref,
    arg1: CUmodule,
    arg2: *const c_char,
) -> CUresult {
    let cuModuleGetTexRef: extern "C" fn(
        arg0: *mut CUtexref,
        arg1: CUmodule,
        arg2: *const c_char,
    ) -> CUresult = std::mem::transmute(get_sym("cuModuleGetTexRef"));
    eprintln!("[CALL] {}", "cuModuleGetTexRef");
    cuModuleGetTexRef(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventCreate(arg0: *mut CUevent, arg1: c_uint) -> CUresult {
    let cuEventCreate: extern "C" fn(arg0: *mut CUevent, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuEventCreate"));
    eprintln!("[CALL] {}", "cuEventCreate");
    cuEventCreate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetByPCIBusId(
    arg0: *mut CUdevice,
    arg1: *const c_char,
) -> CUresult {
    let cuDeviceGetByPCIBusId: extern "C" fn(arg0: *mut CUdevice, arg1: *const c_char) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetByPCIBusId"));
    eprintln!("[CALL] {}", "cuDeviceGetByPCIBusId");
    cuDeviceGetByPCIBusId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuOccupancyMaxPotentialBlockSizeWithFlags(
    arg0: *mut c_int,
    arg1: *mut c_int,
    arg2: CUfunction,
    arg3: CUoccupancyB2DSize,
    arg4: usize,
    arg5: c_int,
    arg6: c_uint,
) -> CUresult {
    let cuOccupancyMaxPotentialBlockSizeWithFlags: extern "C" fn(
        arg0: *mut c_int,
        arg1: *mut c_int,
        arg2: CUfunction,
        arg3: CUoccupancyB2DSize,
        arg4: usize,
        arg5: c_int,
        arg6: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuOccupancyMaxPotentialBlockSizeWithFlags"));
    eprintln!("[CALL] {}", "cuOccupancyMaxPotentialBlockSizeWithFlags");
    cuOccupancyMaxPotentialBlockSizeWithFlags(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncSetSharedMemConfig(
    arg0: CUfunction,
    arg1: CUsharedconfig,
) -> CUresult {
    let cuFuncSetSharedMemConfig: extern "C" fn(
        arg0: CUfunction,
        arg1: CUsharedconfig,
    ) -> CUresult = std::mem::transmute(get_sym("cuFuncSetSharedMemConfig"));
    eprintln!("[CALL] {}", "cuFuncSetSharedMemConfig");
    cuFuncSetSharedMemConfig(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecHostNodeSetParams(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *const CUDA_HOST_NODE_PARAMS,
) -> CUresult {
    let cuGraphExecHostNodeSetParams: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *const CUDA_HOST_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecHostNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExecHostNodeSetParams");
    cuGraphExecHostNodeSetParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAdvise_v2(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: CUmem_advise,
    arg3: CUmemLocation,
) -> CUresult {
    let cuMemAdvise_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: CUmem_advise,
        arg3: CUmemLocation,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemAdvise_v2"));
    eprintln!("[CALL] {}", "cuMemAdvise_v2");
    cuMemAdvise_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetAttribute(
    arg0: *mut c_int,
    arg1: CUdevice_attribute,
    arg2: CUdevice,
) -> CUresult {
    let cuDeviceGetAttribute: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUdevice_attribute,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetAttribute"));
    eprintln!("[CALL] {}", "cuDeviceGetAttribute");
    cuDeviceGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetAddressRange_v2(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: CUdeviceptr,
) -> CUresult {
    let cuMemGetAddressRange_v2: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: CUdeviceptr,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemGetAddressRange_v2"));
    eprintln!("[CALL] {}", "cuMemGetAddressRange_v2");
    cuMemGetAddressRange_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostAlloc(
    arg0: *mut *mut c_void,
    arg1: usize,
    arg2: c_uint,
) -> CUresult {
    let cuMemHostAlloc: extern "C" fn(
        arg0: *mut *mut c_void,
        arg1: usize,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemHostAlloc"));
    eprintln!("[CALL] {}", "cuMemHostAlloc");
    cuMemHostAlloc(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGetErrorName(arg0: CUresult, arg1: *mut *const c_char) -> CUresult {
    let cuGetErrorName: extern "C" fn(arg0: CUresult, arg1: *mut *const c_char) -> CUresult =
        std::mem::transmute(get_sym("cuGetErrorName"));
    eprintln!("[CALL] {}", "cuGetErrorName");
    cuGetErrorName(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxGetDevResource(
    arg0: CUgreenCtx,
    arg1: *mut CUdevResource,
    arg2: CUdevResourceType,
) -> CUresult {
    let cuGreenCtxGetDevResource: extern "C" fn(
        arg0: CUgreenCtx,
        arg1: *mut CUdevResource,
        arg2: CUdevResourceType,
    ) -> CUresult = std::mem::transmute(get_sym("cuGreenCtxGetDevResource"));
    eprintln!("[CALL] {}", "cuGreenCtxGetDevResource");
    cuGreenCtxGetDevResource(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryEnumerateKernels(
    arg0: *mut CUkernel,
    arg1: c_uint,
    arg2: CUlibrary,
) -> CUresult {
    let cuLibraryEnumerateKernels: extern "C" fn(
        arg0: *mut CUkernel,
        arg1: c_uint,
        arg2: CUlibrary,
    ) -> CUresult = std::mem::transmute(get_sym("cuLibraryEnumerateKernels"));
    eprintln!("[CALL] {}", "cuLibraryEnumerateKernels");
    cuLibraryEnumerateKernels(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointDestroy(arg0: CUlogicalEndpointId) -> CUresult {
    let cuLogicalEndpointDestroy: extern "C" fn(arg0: CUlogicalEndpointId) -> CUresult =
        std::mem::transmute(get_sym("cuLogicalEndpointDestroy"));
    eprintln!("[CALL] {}", "cuLogicalEndpointDestroy");
    cuLogicalEndpointDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocFromPoolAsync(
    arg0: *mut CUdeviceptr,
    arg1: usize,
    arg2: CUmemoryPool,
    arg3: CUstream,
) -> CUresult {
    let cuMemAllocFromPoolAsync: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: usize,
        arg2: CUmemoryPool,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemAllocFromPoolAsync"));
    eprintln!("[CALL] {}", "cuMemAllocFromPoolAsync");
    cuMemAllocFromPoolAsync(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetAddress_v2(arg0: *mut CUdeviceptr, arg1: CUtexref) -> CUresult {
    let cuTexRefGetAddress_v2: extern "C" fn(arg0: *mut CUdeviceptr, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetAddress_v2"));
    eprintln!("[CALL] {}", "cuTexRefGetAddress_v2");
    cuTexRefGetAddress_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpDeregisterCompleteCallback(
    arg0: CUcoredumpCallbackHandle,
) -> CUresult {
    let cuCoredumpDeregisterCompleteCallback: extern "C" fn(
        arg0: CUcoredumpCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpDeregisterCompleteCallback"));
    eprintln!("[CALL] {}", "cuCoredumpDeregisterCompleteCallback");
    cuCoredumpDeregisterCompleteCallback(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuParamSetf(arg0: CUfunction, arg1: c_int, arg2: f32) -> CUresult {
    let cuParamSetf: extern "C" fn(arg0: CUfunction, arg1: c_int, arg2: f32) -> CUresult =
        std::mem::transmute(get_sym("cuParamSetf"));
    eprintln!("[CALL] {}", "cuParamSetf");
    cuParamSetf(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkDestroy(arg0: CUlinkState) -> CUresult {
    let cuLinkDestroy: extern "C" fn(arg0: CUlinkState) -> CUresult =
        std::mem::transmute(get_sym("cuLinkDestroy"));
    eprintln!("[CALL] {}", "cuLinkDestroy");
    cuLinkDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleLoad(arg0: *mut CUmodule, arg1: *const c_char) -> CUresult {
    let cuModuleLoad: extern "C" fn(arg0: *mut CUmodule, arg1: *const c_char) -> CUresult =
        std::mem::transmute(get_sym("cuModuleLoad"));
    eprintln!("[CALL] {}", "cuModuleLoad");
    cuModuleLoad(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchHostFunc_v2(
    arg0: CUstream,
    arg1: CUhostFn,
    arg2: *mut c_void,
    arg3: c_uint,
) -> CUresult {
    let cuLaunchHostFunc_v2: extern "C" fn(
        arg0: CUstream,
        arg1: CUhostFn,
        arg2: *mut c_void,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuLaunchHostFunc_v2"));
    eprintln!("[CALL] {}", "cuLaunchHostFunc_v2");
    cuLaunchHostFunc_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventRecordWithFlags(
    arg0: CUevent,
    arg1: CUstream,
    arg2: c_uint,
) -> CUresult {
    let cuEventRecordWithFlags: extern "C" fn(
        arg0: CUevent,
        arg1: CUstream,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuEventRecordWithFlags"));
    eprintln!("[CALL] {}", "cuEventRecordWithFlags");
    cuEventRecordWithFlags(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetFlags(arg0: CUtexref, arg1: c_uint) -> CUresult {
    let cuTexRefSetFlags: extern "C" fn(arg0: CUtexref, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetFlags"));
    eprintln!("[CALL] {}", "cuTexRefSetFlags");
    cuTexRefSetFlags(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuParamSeti(arg0: CUfunction, arg1: c_int, arg2: c_uint) -> CUresult {
    let cuParamSeti: extern "C" fn(arg0: CUfunction, arg1: c_int, arg2: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuParamSeti"));
    eprintln!("[CALL] {}", "cuParamSeti");
    cuParamSeti(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamEndCapture(arg0: CUstream, arg1: *mut CUgraph) -> CUresult {
    let cuStreamEndCapture: extern "C" fn(arg0: CUstream, arg1: *mut CUgraph) -> CUresult =
        std::mem::transmute(get_sym("cuStreamEndCapture"));
    eprintln!("[CALL] {}", "cuStreamEndCapture");
    cuStreamEndCapture(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphGetRootNodes(
    arg0: CUgraph,
    arg1: *mut CUgraphNode,
    arg2: *mut usize,
) -> CUresult {
    let cuGraphGetRootNodes: extern "C" fn(
        arg0: CUgraph,
        arg1: *mut CUgraphNode,
        arg2: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphGetRootNodes"));
    eprintln!("[CALL] {}", "cuGraphGetRootNodes");
    cuGraphGetRootNodes(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMipmappedArrayCreate(
    arg0: *mut CUmipmappedArray,
    arg1: *const CUDA_ARRAY3D_DESCRIPTOR,
    arg2: c_uint,
) -> CUresult {
    let cuMipmappedArrayCreate: extern "C" fn(
        arg0: *mut CUmipmappedArray,
        arg1: *const CUDA_ARRAY3D_DESCRIPTOR,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuMipmappedArrayCreate"));
    eprintln!("[CALL] {}", "cuMipmappedArrayCreate");
    cuMipmappedArrayCreate(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphGetNodes(
    arg0: CUgraph,
    arg1: *mut CUgraphNode,
    arg2: *mut usize,
) -> CUresult {
    let cuGraphGetNodes: extern "C" fn(
        arg0: CUgraph,
        arg1: *mut CUgraphNode,
        arg2: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphGetNodes"));
    eprintln!("[CALL] {}", "cuGraphGetNodes");
    cuGraphGetNodes(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemCreate(
    arg0: *mut CUmemGenericAllocationHandle,
    arg1: usize,
    arg2: *const CUmemAllocationProp,
    arg3: c_ulonglong,
) -> CUresult {
    let cuMemCreate: extern "C" fn(
        arg0: *mut CUmemGenericAllocationHandle,
        arg1: usize,
        arg2: *const CUmemAllocationProp,
        arg3: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemCreate"));
    eprintln!("[CALL] {}", "cuMemCreate");
    cuMemCreate(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryGetModule(arg0: *mut CUmodule, arg1: CUlibrary) -> CUresult {
    let cuLibraryGetModule: extern "C" fn(arg0: *mut CUmodule, arg1: CUlibrary) -> CUresult =
        std::mem::transmute(get_sym("cuLibraryGetModule"));
    eprintln!("[CALL] {}", "cuLibraryGetModule");
    cuLibraryGetModule(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuParamSetv(
    arg0: CUfunction,
    arg1: c_int,
    arg2: *mut c_void,
    arg3: c_uint,
) -> CUresult {
    let cuParamSetv: extern "C" fn(
        arg0: CUfunction,
        arg1: c_int,
        arg2: *mut c_void,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuParamSetv"));
    eprintln!("[CALL] {}", "cuParamSetv");
    cuParamSetv(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuSurfObjectCreate(
    arg0: *mut CUsurfObject,
    arg1: *const CUDA_RESOURCE_DESC,
) -> CUresult {
    let cuSurfObjectCreate: extern "C" fn(
        arg0: *mut CUsurfObject,
        arg1: *const CUDA_RESOURCE_DESC,
    ) -> CUresult = std::mem::transmute(get_sym("cuSurfObjectCreate"));
    eprintln!("[CALL] {}", "cuSurfObjectCreate");
    cuSurfObjectCreate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddNode_v2(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: *const CUgraphEdgeData,
    arg4: usize,
    arg5: *mut CUgraphNodeParams,
) -> CUresult {
    let cuGraphAddNode_v2: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: *const CUgraphEdgeData,
        arg4: usize,
        arg5: *mut CUgraphNodeParams,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddNode_v2"));
    eprintln!("[CALL] {}", "cuGraphAddNode_v2");
    cuGraphAddNode_v2(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuOccupancyMaxPotentialClusterSize(
    arg0: *mut c_int,
    arg1: CUfunction,
    arg2: *const CUlaunchConfig,
) -> CUresult {
    let cuOccupancyMaxPotentialClusterSize: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUfunction,
        arg2: *const CUlaunchConfig,
    ) -> CUresult = std::mem::transmute(get_sym("cuOccupancyMaxPotentialClusterSize"));
    eprintln!("[CALL] {}", "cuOccupancyMaxPotentialClusterSize");
    cuOccupancyMaxPotentialClusterSize(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointUnbind(
    arg0: CUlogicalEndpointId,
    arg1: CUdevice,
    arg2: cuuint64_t,
    arg3: cuuint64_t,
) -> CUresult {
    let cuLogicalEndpointUnbind: extern "C" fn(
        arg0: CUlogicalEndpointId,
        arg1: CUdevice,
        arg2: cuuint64_t,
        arg3: cuuint64_t,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointUnbind"));
    eprintln!("[CALL] {}", "cuLogicalEndpointUnbind");
    cuLogicalEndpointUnbind(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuImportExternalSemaphore(
    arg0: *mut CUexternalSemaphore,
    arg1: *const CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC,
) -> CUresult {
    let cuImportExternalSemaphore: extern "C" fn(
        arg0: *mut CUexternalSemaphore,
        arg1: *const CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC,
    ) -> CUresult = std::mem::transmute(get_sym("cuImportExternalSemaphore"));
    eprintln!("[CALL] {}", "cuImportExternalSemaphore");
    cuImportExternalSemaphore(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceSetMemPool(arg0: CUdevice, arg1: CUmemoryPool) -> CUresult {
    let cuDeviceSetMemPool: extern "C" fn(arg0: CUdevice, arg1: CUmemoryPool) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceSetMemPool"));
    eprintln!("[CALL] {}", "cuDeviceSetMemPool");
    cuDeviceSetMemPool(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayCreate_v2(
    arg0: *mut CUarray,
    arg1: *const CUDA_ARRAY_DESCRIPTOR,
) -> CUresult {
    let cuArrayCreate_v2: extern "C" fn(
        arg0: *mut CUarray,
        arg1: *const CUDA_ARRAY_DESCRIPTOR,
    ) -> CUresult = std::mem::transmute(get_sym("cuArrayCreate_v2"));
    eprintln!("[CALL] {}", "cuArrayCreate_v2");
    cuArrayCreate_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMipmappedArrayGetSparseProperties(
    arg0: *mut CUDA_ARRAY_SPARSE_PROPERTIES,
    arg1: CUmipmappedArray,
) -> CUresult {
    let cuMipmappedArrayGetSparseProperties: extern "C" fn(
        arg0: *mut CUDA_ARRAY_SPARSE_PROPERTIES,
        arg1: CUmipmappedArray,
    ) -> CUresult = std::mem::transmute(get_sym("cuMipmappedArrayGetSparseProperties"));
    eprintln!("[CALL] {}", "cuMipmappedArrayGetSparseProperties");
    cuMipmappedArrayGetSparseProperties(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetGlobal_v2(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: CUmodule,
    arg3: *const c_char,
) -> CUresult {
    let cuModuleGetGlobal_v2: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: CUmodule,
        arg3: *const c_char,
    ) -> CUresult = std::mem::transmute(get_sym("cuModuleGetGlobal_v2"));
    eprintln!("[CALL] {}", "cuModuleGetGlobal_v2");
    cuModuleGetGlobal_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUgraphNodeParams,
) -> CUresult {
    let cuGraphNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUgraphNodeParams,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphNodeGetParams");
    cuGraphNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetType(
    arg0: CUgraphNode,
    arg1: *mut CUgraphNodeType,
) -> CUresult {
    let cuGraphNodeGetType: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUgraphNodeType,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeGetType"));
    eprintln!("[CALL] {}", "cuGraphNodeGetType");
    cuGraphNodeGetType(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceComputeCapability(
    arg0: *mut c_int,
    arg1: *mut c_int,
    arg2: CUdevice,
) -> CUresult {
    let cuDeviceComputeCapability: extern "C" fn(
        arg0: *mut c_int,
        arg1: *mut c_int,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceComputeCapability"));
    eprintln!("[CALL] {}", "cuDeviceComputeCapability");
    cuDeviceComputeCapability(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetMipmapLevelClamp(
    arg0: *mut f32,
    arg1: *mut f32,
    arg2: CUtexref,
) -> CUresult {
    let cuTexRefGetMipmapLevelClamp: extern "C" fn(
        arg0: *mut f32,
        arg1: *mut f32,
        arg2: CUtexref,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefGetMipmapLevelClamp"));
    eprintln!("[CALL] {}", "cuTexRefGetMipmapLevelClamp");
    cuTexRefGetMipmapLevelClamp(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuUserObjectRelease(arg0: CUuserObject, arg1: c_uint) -> CUresult {
    let cuUserObjectRelease: extern "C" fn(arg0: CUuserObject, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuUserObjectRelease"));
    eprintln!("[CALL] {}", "cuUserObjectRelease");
    cuUserObjectRelease(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetApiVersion(arg0: CUcontext, arg1: *mut c_uint) -> CUresult {
    let cuCtxGetApiVersion: extern "C" fn(arg0: CUcontext, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetApiVersion"));
    eprintln!("[CALL] {}", "cuCtxGetApiVersion");
    cuCtxGetApiVersion(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuOccupancyAvailableDynamicSMemPerBlock(
    arg0: *mut usize,
    arg1: CUfunction,
    arg2: c_int,
    arg3: c_int,
) -> CUresult {
    let cuOccupancyAvailableDynamicSMemPerBlock: extern "C" fn(
        arg0: *mut usize,
        arg1: CUfunction,
        arg2: c_int,
        arg3: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuOccupancyAvailableDynamicSMemPerBlock"));
    eprintln!("[CALL] {}", "cuOccupancyAvailableDynamicSMemPerBlock");
    cuOccupancyAvailableDynamicSMemPerBlock(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamDestroy_v2(arg0: CUstream) -> CUresult {
    let cuStreamDestroy_v2: extern "C" fn(arg0: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamDestroy_v2"));
    eprintln!("[CALL] {}", "cuStreamDestroy_v2");
    cuStreamDestroy_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetFilterMode(arg0: CUtexref, arg1: CUfilter_mode) -> CUresult {
    let cuTexRefSetFilterMode: extern "C" fn(arg0: CUtexref, arg1: CUfilter_mode) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetFilterMode"));
    eprintln!("[CALL] {}", "cuTexRefSetFilterMode");
    cuTexRefSetFilterMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddBatchMemOpNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: *const CUDA_BATCH_MEM_OP_NODE_PARAMS,
) -> CUresult {
    let cuGraphAddBatchMemOpNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: *const CUDA_BATCH_MEM_OP_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddBatchMemOpNode"));
    eprintln!("[CALL] {}", "cuGraphAddBatchMemOpNode");
    cuGraphAddBatchMemOpNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpSetAttributeGlobal(
    arg0: CUcoredumpSettings,
    arg1: *mut c_void,
    arg2: *mut usize,
) -> CUresult {
    let cuCoredumpSetAttributeGlobal: extern "C" fn(
        arg0: CUcoredumpSettings,
        arg1: *mut c_void,
        arg2: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpSetAttributeGlobal"));
    eprintln!("[CALL] {}", "cuCoredumpSetAttributeGlobal");
    cuCoredumpSetAttributeGlobal(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue32_v2(
    arg0: CUstream,
    arg1: CUdeviceptr,
    arg2: cuuint32_t,
    arg3: c_uint,
) -> CUresult {
    let cuStreamWriteValue32_v2: extern "C" fn(
        arg0: CUstream,
        arg1: CUdeviceptr,
        arg2: cuuint32_t,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamWriteValue32_v2"));
    eprintln!("[CALL] {}", "cuStreamWriteValue32_v2");
    cuStreamWriteValue32_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeSetParams_v2(
    arg0: CUgraphNode,
    arg1: *const CUDA_KERNEL_NODE_PARAMS,
) -> CUresult {
    let cuGraphKernelNodeSetParams_v2: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *const CUDA_KERNEL_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphKernelNodeSetParams_v2"));
    eprintln!("[CALL] {}", "cuGraphKernelNodeSetParams_v2");
    cuGraphKernelNodeSetParams_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphEventRecordNodeGetEvent(
    arg0: CUgraphNode,
    arg1: *mut CUevent,
) -> CUresult {
    let cuGraphEventRecordNodeGetEvent: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUevent,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphEventRecordNodeGetEvent"));
    eprintln!("[CALL] {}", "cuGraphEventRecordNodeGetEvent");
    cuGraphEventRecordNodeGetEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointCreate(
    arg0: CUlogicalEndpointId,
    arg1: *const CUlogicalEndpointProp,
) -> CUresult {
    let cuLogicalEndpointCreate: extern "C" fn(
        arg0: CUlogicalEndpointId,
        arg1: *const CUlogicalEndpointProp,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointCreate"));
    eprintln!("[CALL] {}", "cuLogicalEndpointCreate");
    cuLogicalEndpointCreate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogsDumpToFile(
    arg0: *mut CUlogIterator,
    arg1: *const c_char,
    arg2: c_uint,
) -> CUresult {
    let cuLogsDumpToFile: extern "C" fn(
        arg0: *mut CUlogIterator,
        arg1: *const c_char,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogsDumpToFile"));
    eprintln!("[CALL] {}", "cuLogsDumpToFile");
    cuLogsDumpToFile(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32_v2(arg0: CUdeviceptr, arg1: c_uint, arg2: usize) -> CUresult {
    let cuMemsetD32_v2: extern "C" fn(arg0: CUdeviceptr, arg1: c_uint, arg2: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemsetD32_v2"));
    eprintln!("[CALL] {}", "cuMemsetD32_v2");
    cuMemsetD32_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamUpdateCaptureDependencies_v2(
    arg0: CUstream,
    arg1: *mut CUgraphNode,
    arg2: *const CUgraphEdgeData,
    arg3: usize,
    arg4: c_uint,
) -> CUresult {
    let cuStreamUpdateCaptureDependencies_v2: extern "C" fn(
        arg0: CUstream,
        arg1: *mut CUgraphNode,
        arg2: *const CUgraphEdgeData,
        arg3: usize,
        arg4: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamUpdateCaptureDependencies_v2"));
    eprintln!("[CALL] {}", "cuStreamUpdateCaptureDependencies_v2");
    cuStreamUpdateCaptureDependencies_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsUnregisterResource(arg0: CUgraphicsResource) -> CUresult {
    let cuGraphicsUnregisterResource: extern "C" fn(arg0: CUgraphicsResource) -> CUresult =
        std::mem::transmute(get_sym("cuGraphicsUnregisterResource"));
    eprintln!("[CALL] {}", "cuGraphicsUnregisterResource");
    cuGraphicsUnregisterResource(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemRetainAllocationHandle(
    arg0: *mut CUmemGenericAllocationHandle,
    arg1: *mut c_void,
) -> CUresult {
    let cuMemRetainAllocationHandle: extern "C" fn(
        arg0: *mut CUmemGenericAllocationHandle,
        arg1: *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemRetainAllocationHandle"));
    eprintln!("[CALL] {}", "cuMemRetainAllocationHandle");
    cuMemRetainAllocationHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecEventWaitNodeSetEvent(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: CUevent,
) -> CUresult {
    let cuGraphExecEventWaitNodeSetEvent: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: CUevent,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecEventWaitNodeSetEvent"));
    eprintln!("[CALL] {}", "cuGraphExecEventWaitNodeSetEvent");
    cuGraphExecEventWaitNodeSetEvent(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoAAsync_v2(
    arg0: CUarray,
    arg1: usize,
    arg2: *const c_void,
    arg3: usize,
    arg4: CUstream,
) -> CUresult {
    let cuMemcpyHtoAAsync_v2: extern "C" fn(
        arg0: CUarray,
        arg1: usize,
        arg2: *const c_void,
        arg3: usize,
        arg4: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyHtoAAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpyHtoAAsync_v2");
    cuMemcpyHtoAAsync_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceSetGraphMemAttribute(
    arg0: CUdevice,
    arg1: CUgraphMem_attribute,
    arg2: *mut c_void,
) -> CUresult {
    let cuDeviceSetGraphMemAttribute: extern "C" fn(
        arg0: CUdevice,
        arg1: CUgraphMem_attribute,
        arg2: *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceSetGraphMemAttribute"));
    eprintln!("[CALL] {}", "cuDeviceSetGraphMemAttribute");
    cuDeviceSetGraphMemAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetLimit(arg0: *mut usize, arg1: CUlimit) -> CUresult {
    let cuCtxGetLimit: extern "C" fn(arg0: *mut usize, arg1: CUlimit) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetLimit"));
    eprintln!("[CALL] {}", "cuCtxGetLimit");
    cuCtxGetLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolTrimTo(arg0: CUmemoryPool, arg1: usize) -> CUresult {
    let cuMemPoolTrimTo: extern "C" fn(arg0: CUmemoryPool, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemPoolTrimTo"));
    eprintln!("[CALL] {}", "cuMemPoolTrimTo");
    cuMemPoolTrimTo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncGetParamCount(arg0: CUfunction, arg1: *mut usize) -> CUresult {
    let cuFuncGetParamCount: extern "C" fn(arg0: CUfunction, arg1: *mut usize) -> CUresult =
        std::mem::transmute(get_sym("cuFuncGetParamCount"));
    eprintln!("[CALL] {}", "cuFuncGetParamCount");
    cuFuncGetParamCount(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetFlags(arg0: c_uint) -> CUresult {
    let cuCtxSetFlags: extern "C" fn(arg0: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetFlags"));
    eprintln!("[CALL] {}", "cuCtxSetFlags");
    cuCtxSetFlags(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostGetDevicePointer_v2(
    arg0: *mut CUdeviceptr,
    arg1: *mut c_void,
    arg2: c_uint,
) -> CUresult {
    let cuMemHostGetDevicePointer_v2: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut c_void,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemHostGetDevicePointer_v2"));
    eprintln!("[CALL] {}", "cuMemHostGetDevicePointer_v2");
    cuMemHostGetDevicePointer_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchKernelEx(
    arg0: *const CUlaunchConfig,
    arg1: CUfunction,
    arg2: *mut *mut c_void,
    arg3: *mut *mut c_void,
) -> CUresult {
    let cuLaunchKernelEx: extern "C" fn(
        arg0: *const CUlaunchConfig,
        arg1: CUfunction,
        arg2: *mut *mut c_void,
        arg3: *mut *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuLaunchKernelEx"));
    eprintln!("[CALL] {}", "cuLaunchKernelEx");
    cuLaunchKernelEx(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceTotalMem_v2(arg0: *mut usize, arg1: CUdevice) -> CUresult {
    let cuDeviceTotalMem_v2: extern "C" fn(arg0: *mut usize, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceTotalMem_v2"));
    eprintln!("[CALL] {}", "cuDeviceTotalMem_v2");
    cuDeviceTotalMem_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointIdReserve(
    arg0: *mut CUlogicalEndpointId,
    arg1: cuuint32_t,
) -> CUresult {
    let cuLogicalEndpointIdReserve: extern "C" fn(
        arg0: *mut CUlogicalEndpointId,
        arg1: cuuint32_t,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointIdReserve"));
    eprintln!("[CALL] {}", "cuLogicalEndpointIdReserve");
    cuLogicalEndpointIdReserve(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxWaitEvent(arg0: CUcontext, arg1: CUevent) -> CUresult {
    let cuCtxWaitEvent: extern "C" fn(arg0: CUcontext, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuCtxWaitEvent"));
    eprintln!("[CALL] {}", "cuCtxWaitEvent");
    cuCtxWaitEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogsDumpToMemory(
    arg0: *mut CUlogIterator,
    arg1: *mut c_char,
    arg2: *mut usize,
    arg3: c_uint,
) -> CUresult {
    let cuLogsDumpToMemory: extern "C" fn(
        arg0: *mut CUlogIterator,
        arg1: *mut c_char,
        arg2: *mut usize,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogsDumpToMemory"));
    eprintln!("[CALL] {}", "cuLogsDumpToMemory");
    cuLogsDumpToMemory(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncSetAttribute(
    arg0: CUfunction,
    arg1: CUfunction_attribute,
    arg2: c_int,
) -> CUresult {
    let cuFuncSetAttribute: extern "C" fn(
        arg0: CUfunction,
        arg1: CUfunction_attribute,
        arg2: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuFuncSetAttribute"));
    eprintln!("[CALL] {}", "cuFuncSetAttribute");
    cuFuncSetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetLibrary(arg0: *mut CUlibrary, arg1: CUkernel) -> CUresult {
    let cuKernelGetLibrary: extern "C" fn(arg0: *mut CUlibrary, arg1: CUkernel) -> CUresult =
        std::mem::transmute(get_sym("cuKernelGetLibrary"));
    eprintln!("[CALL] {}", "cuKernelGetLibrary");
    cuKernelGetLibrary(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogsRegisterCallback(
    arg0: CUlogsCallback,
    arg1: *mut c_void,
    arg2: *mut CUlogsCallbackHandle,
) -> CUresult {
    let cuLogsRegisterCallback: extern "C" fn(
        arg0: CUlogsCallback,
        arg1: *mut c_void,
        arg2: *mut CUlogsCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogsRegisterCallback"));
    eprintln!("[CALL] {}", "cuLogsRegisterCallback");
    cuLogsRegisterCallback(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetArray(arg0: *mut CUarray, arg1: CUtexref) -> CUresult {
    let cuTexRefGetArray: extern "C" fn(arg0: *mut CUarray, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetArray"));
    eprintln!("[CALL] {}", "cuTexRefGetArray");
    cuTexRefGetArray(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy(arg0: CUdeviceptr, arg1: CUdeviceptr, arg2: usize) -> CUresult {
    let cuMemcpy: extern "C" fn(arg0: CUdeviceptr, arg1: CUdeviceptr, arg2: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy"));
    eprintln!("[CALL] {}", "cuMemcpy");
    cuMemcpy(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxStreamCreate(
    arg0: *mut CUstream,
    arg1: CUgreenCtx,
    arg2: c_uint,
    arg3: c_int,
) -> CUresult {
    let cuGreenCtxStreamCreate: extern "C" fn(
        arg0: *mut CUstream,
        arg1: CUgreenCtx,
        arg2: c_uint,
        arg3: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuGreenCtxStreamCreate"));
    eprintln!("[CALL] {}", "cuGreenCtxStreamCreate");
    cuGreenCtxStreamCreate(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyBatchAsync_v2(
    arg0: *mut CUdeviceptr,
    arg1: *mut CUdeviceptr,
    arg2: *mut usize,
    arg3: usize,
    arg4: *mut CUmemcpyAttributes,
    arg5: *mut usize,
    arg6: usize,
    arg7: CUstream,
) -> CUresult {
    let cuMemcpyBatchAsync_v2: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut CUdeviceptr,
        arg2: *mut usize,
        arg3: usize,
        arg4: *mut CUmemcpyAttributes,
        arg5: *mut usize,
        arg6: usize,
        arg7: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyBatchAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpyBatchAsync_v2");
    cuMemcpyBatchAsync_v2(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D32Async(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: c_uint,
    arg3: usize,
    arg4: usize,
    arg5: CUstream,
) -> CUresult {
    let cuMemsetD2D32Async: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: c_uint,
        arg3: usize,
        arg4: usize,
        arg5: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD2D32Async"));
    eprintln!("[CALL] {}", "cuMemsetD2D32Async");
    cuMemsetD2D32Async(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DPeer(arg0: *const CUDA_MEMCPY3D_PEER) -> CUresult {
    let cuMemcpy3DPeer: extern "C" fn(arg0: *const CUDA_MEMCPY3D_PEER) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy3DPeer"));
    eprintln!("[CALL] {}", "cuMemcpy3DPeer");
    cuMemcpy3DPeer(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExternalSemaphoresSignalNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
) -> CUresult {
    let cuGraphExternalSemaphoresSignalNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExternalSemaphoresSignalNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphExternalSemaphoresSignalNodeGetParams");
    cuGraphExternalSemaphoresSignalNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetDevice(arg0: *mut CUdevice) -> CUresult {
    let cuCtxGetDevice: extern "C" fn(arg0: *mut CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetDevice"));
    eprintln!("[CALL] {}", "cuCtxGetDevice");
    cuCtxGetDevice(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddChildGraphNode(
    arg0: *mut CUgraphNode,
    arg1: CUgraph,
    arg2: *const CUgraphNode,
    arg3: usize,
    arg4: CUgraph,
) -> CUresult {
    let cuGraphAddChildGraphNode: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraph,
        arg2: *const CUgraphNode,
        arg3: usize,
        arg4: CUgraph,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddChildGraphNode"));
    eprintln!("[CALL] {}", "cuGraphAddChildGraphNode");
    cuGraphAddChildGraphNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphMemAllocNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUDA_MEM_ALLOC_NODE_PARAMS,
) -> CUresult {
    let cuGraphMemAllocNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUDA_MEM_ALLOC_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphMemAllocNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphMemAllocNodeGetParams");
    cuGraphMemAllocNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTensorMapEncodeIm2colWide(
    arg0: *mut CUtensorMap,
    arg1: CUtensorMapDataType,
    arg2: cuuint32_t,
    arg3: *mut c_void,
    arg4: *const cuuint64_t,
    arg5: *const cuuint64_t,
    arg6: c_int,
    arg7: c_int,
    arg8: cuuint32_t,
    arg9: cuuint32_t,
    arg10: *const cuuint32_t,
    arg11: CUtensorMapInterleave,
    arg12: CUtensorMapIm2ColWideMode,
    arg13: CUtensorMapSwizzle,
    arg14: CUtensorMapL2promotion,
    arg15: CUtensorMapFloatOOBfill,
) -> CUresult {
    let cuTensorMapEncodeIm2colWide: extern "C" fn(
        arg0: *mut CUtensorMap,
        arg1: CUtensorMapDataType,
        arg2: cuuint32_t,
        arg3: *mut c_void,
        arg4: *const cuuint64_t,
        arg5: *const cuuint64_t,
        arg6: c_int,
        arg7: c_int,
        arg8: cuuint32_t,
        arg9: cuuint32_t,
        arg10: *const cuuint32_t,
        arg11: CUtensorMapInterleave,
        arg12: CUtensorMapIm2ColWideMode,
        arg13: CUtensorMapSwizzle,
        arg14: CUtensorMapL2promotion,
        arg15: CUtensorMapFloatOOBfill,
    ) -> CUresult = std::mem::transmute(get_sym("cuTensorMapEncodeIm2colWide"));
    eprintln!("[CALL] {}", "cuTensorMapEncodeIm2colWide");
    cuTensorMapEncodeIm2colWide(
        arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13,
        arg14, arg15,
    )
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddress_v2(
    arg0: *mut usize,
    arg1: CUtexref,
    arg2: CUdeviceptr,
    arg3: usize,
) -> CUresult {
    let cuTexRefSetAddress_v2: extern "C" fn(
        arg0: *mut usize,
        arg1: CUtexref,
        arg2: CUdeviceptr,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefSetAddress_v2"));
    eprintln!("[CALL] {}", "cuTexRefSetAddress_v2");
    cuTexRefSetAddress_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayGetPlane(
    arg0: *mut CUarray,
    arg1: CUarray,
    arg2: c_uint,
) -> CUresult {
    let cuArrayGetPlane: extern "C" fn(
        arg0: *mut CUarray,
        arg1: CUarray,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuArrayGetPlane"));
    eprintln!("[CALL] {}", "cuArrayGetPlane");
    cuArrayGetPlane(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexObjectGetResourceViewDesc(
    arg0: *mut CUDA_RESOURCE_VIEW_DESC,
    arg1: CUtexObject,
) -> CUresult {
    let cuTexObjectGetResourceViewDesc: extern "C" fn(
        arg0: *mut CUDA_RESOURCE_VIEW_DESC,
        arg1: CUtexObject,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexObjectGetResourceViewDesc"));
    eprintln!("[CALL] {}", "cuTexObjectGetResourceViewDesc");
    cuTexObjectGetResourceViewDesc(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphConditionalHandleCreate(
    arg0: *mut CUgraphConditionalHandle,
    arg1: CUgraph,
    arg2: CUcontext,
    arg3: c_uint,
    arg4: c_uint,
) -> CUresult {
    let cuGraphConditionalHandleCreate: extern "C" fn(
        arg0: *mut CUgraphConditionalHandle,
        arg1: CUgraph,
        arg2: CUcontext,
        arg3: c_uint,
        arg4: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphConditionalHandleCreate"));
    eprintln!("[CALL] {}", "cuGraphConditionalHandleCreate");
    cuGraphConditionalHandleCreate(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetDevResource(
    arg0: CUcontext,
    arg1: *mut CUdevResource,
    arg2: CUdevResourceType,
) -> CUresult {
    let cuCtxGetDevResource: extern "C" fn(
        arg0: CUcontext,
        arg1: *mut CUdevResource,
        arg2: CUdevResourceType,
    ) -> CUresult = std::mem::transmute(get_sym("cuCtxGetDevResource"));
    eprintln!("[CALL] {}", "cuCtxGetDevResource");
    cuCtxGetDevResource(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3D_v2(arg0: *const CUDA_MEMCPY3D) -> CUresult {
    let cuMemcpy3D_v2: extern "C" fn(arg0: *const CUDA_MEMCPY3D) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy3D_v2"));
    eprintln!("[CALL] {}", "cuMemcpy3D_v2");
    cuMemcpy3D_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryGetGlobal(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: CUlibrary,
    arg3: *const c_char,
) -> CUresult {
    let cuLibraryGetGlobal: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: CUlibrary,
        arg3: *const c_char,
    ) -> CUresult = std::mem::transmute(get_sym("cuLibraryGetGlobal"));
    eprintln!("[CALL] {}", "cuLibraryGetGlobal");
    cuLibraryGetGlobal(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphGetId(arg0: CUgraph, arg1: *mut c_uint) -> CUresult {
    let cuGraphGetId: extern "C" fn(arg0: CUgraph, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuGraphGetId"));
    eprintln!("[CALL] {}", "cuGraphGetId");
    cuGraphGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDestroyExternalMemory(arg0: CUexternalMemory) -> CUresult {
    let cuDestroyExternalMemory: extern "C" fn(arg0: CUexternalMemory) -> CUresult =
        std::mem::transmute(get_sym("cuDestroyExternalMemory"));
    eprintln!("[CALL] {}", "cuDestroyExternalMemory");
    cuDestroyExternalMemory(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetMipmapLevelBias(arg0: *mut f32, arg1: CUtexref) -> CUresult {
    let cuTexRefGetMipmapLevelBias: extern "C" fn(arg0: *mut f32, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetMipmapLevelBias"));
    eprintln!("[CALL] {}", "cuTexRefGetMipmapLevelBias");
    cuTexRefGetMipmapLevelBias(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuOccupancyMaxActiveBlocksPerMultiprocessor(
    arg0: *mut c_int,
    arg1: CUfunction,
    arg2: c_int,
    arg3: usize,
) -> CUresult {
    let cuOccupancyMaxActiveBlocksPerMultiprocessor: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUfunction,
        arg2: c_int,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuOccupancyMaxActiveBlocksPerMultiprocessor"));
    eprintln!("[CALL] {}", "cuOccupancyMaxActiveBlocksPerMultiprocessor");
    cuOccupancyMaxActiveBlocksPerMultiprocessor(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxGetState(
    arg0: CUdevice,
    arg1: *mut c_uint,
    arg2: *mut c_int,
) -> CUresult {
    let cuDevicePrimaryCtxGetState: extern "C" fn(
        arg0: CUdevice,
        arg1: *mut c_uint,
        arg2: *mut c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuDevicePrimaryCtxGetState"));
    eprintln!("[CALL] {}", "cuDevicePrimaryCtxGetState");
    cuDevicePrimaryCtxGetState(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolSetAttribute(
    arg0: CUmemoryPool,
    arg1: CUmemPool_attribute,
    arg2: *mut c_void,
) -> CUresult {
    let cuMemPoolSetAttribute: extern "C" fn(
        arg0: CUmemoryPool,
        arg1: CUmemPool_attribute,
        arg2: *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolSetAttribute"));
    eprintln!("[CALL] {}", "cuMemPoolSetAttribute");
    cuMemPoolSetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemMap(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: usize,
    arg3: CUmemGenericAllocationHandle,
    arg4: c_ulonglong,
) -> CUresult {
    let cuMemMap: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: usize,
        arg3: CUmemGenericAllocationHandle,
        arg4: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemMap"));
    eprintln!("[CALL] {}", "cuMemMap");
    cuMemMap(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue32_v2(
    arg0: CUstream,
    arg1: CUdeviceptr,
    arg2: cuuint32_t,
    arg3: c_uint,
) -> CUresult {
    let cuStreamWaitValue32_v2: extern "C" fn(
        arg0: CUstream,
        arg1: CUdeviceptr,
        arg2: cuuint32_t,
        arg3: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamWaitValue32_v2"));
    eprintln!("[CALL] {}", "cuStreamWaitValue32_v2");
    cuStreamWaitValue32_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetAttribute(
    arg0: *mut c_int,
    arg1: CUfunction_attribute,
    arg2: CUkernel,
    arg3: CUdevice,
) -> CUresult {
    let cuKernelGetAttribute: extern "C" fn(
        arg0: *mut c_int,
        arg1: CUfunction_attribute,
        arg2: CUkernel,
        arg3: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuKernelGetAttribute"));
    eprintln!("[CALL] {}", "cuKernelGetAttribute");
    cuKernelGetAttribute(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFreeAsync(arg0: CUdeviceptr, arg1: CUstream) -> CUresult {
    let cuMemFreeAsync: extern "C" fn(arg0: CUdeviceptr, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuMemFreeAsync"));
    eprintln!("[CALL] {}", "cuMemFreeAsync");
    cuMemFreeAsync(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetParamCount(arg0: CUkernel, arg1: *mut usize) -> CUresult {
    let cuKernelGetParamCount: extern "C" fn(arg0: CUkernel, arg1: *mut usize) -> CUresult =
        std::mem::transmute(get_sym("cuKernelGetParamCount"));
    eprintln!("[CALL] {}", "cuKernelGetParamCount");
    cuKernelGetParamCount(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchKernel(
    arg0: CUfunction,
    arg1: c_uint,
    arg2: c_uint,
    arg3: c_uint,
    arg4: c_uint,
    arg5: c_uint,
    arg6: c_uint,
    arg7: c_uint,
    arg8: CUstream,
    arg9: *mut *mut c_void,
    arg10: *mut *mut c_void,
) -> CUresult {
    let cuLaunchKernel: extern "C" fn(
        arg0: CUfunction,
        arg1: c_uint,
        arg2: c_uint,
        arg3: c_uint,
        arg4: c_uint,
        arg5: c_uint,
        arg6: c_uint,
        arg7: c_uint,
        arg8: CUstream,
        arg9: *mut *mut c_void,
        arg10: *mut *mut c_void,
    ) -> CUresult = std::mem::transmute(get_sym("cuLaunchKernel"));
    eprintln!("[CALL] {}", "cuLaunchKernel");
    cuLaunchKernel(
        arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10,
    )
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetDevResource(
    arg0: CUstream,
    arg1: *mut CUdevResource,
    arg2: CUdevResourceType,
) -> CUresult {
    let cuStreamGetDevResource: extern "C" fn(
        arg0: CUstream,
        arg1: *mut CUdevResource,
        arg2: CUdevResourceType,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamGetDevResource"));
    eprintln!("[CALL] {}", "cuStreamGetDevResource");
    cuStreamGetDevResource(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoD_v2(
    arg0: CUdeviceptr,
    arg1: *const c_void,
    arg2: usize,
) -> CUresult {
    let cuMemcpyHtoD_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: *const c_void,
        arg2: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyHtoD_v2"));
    eprintln!("[CALL] {}", "cuMemcpyHtoD_v2");
    cuMemcpyHtoD_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogsUnregisterCallback(arg0: CUlogsCallbackHandle) -> CUresult {
    let cuLogsUnregisterCallback: extern "C" fn(arg0: CUlogsCallbackHandle) -> CUresult =
        std::mem::transmute(get_sym("cuLogsUnregisterCallback"));
    eprintln!("[CALL] {}", "cuLogsUnregisterCallback");
    cuLogsUnregisterCallback(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetAccess(
    arg0: *mut c_ulonglong,
    arg1: *const CUmemLocation,
    arg2: CUdeviceptr,
) -> CUresult {
    let cuMemGetAccess: extern "C" fn(
        arg0: *mut c_ulonglong,
        arg1: *const CUmemLocation,
        arg2: CUdeviceptr,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemGetAccess"));
    eprintln!("[CALL] {}", "cuMemGetAccess");
    cuMemGetAccess(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpDeregisterStartCallback(
    arg0: CUcoredumpCallbackHandle,
) -> CUresult {
    let cuCoredumpDeregisterStartCallback: extern "C" fn(
        arg0: CUcoredumpCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpDeregisterStartCallback"));
    eprintln!("[CALL] {}", "cuCoredumpDeregisterStartCallback");
    cuCoredumpDeregisterStartCallback(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetAllocationPropertiesFromHandle(
    arg0: *mut CUmemAllocationProp,
    arg1: CUmemGenericAllocationHandle,
) -> CUresult {
    let cuMemGetAllocationPropertiesFromHandle: extern "C" fn(
        arg0: *mut CUmemAllocationProp,
        arg1: CUmemGenericAllocationHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemGetAllocationPropertiesFromHandle"));
    eprintln!("[CALL] {}", "cuMemGetAllocationPropertiesFromHandle");
    cuMemGetAllocationPropertiesFromHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetDevice_v2(arg0: *mut CUdevice, arg1: CUcontext) -> CUresult {
    let cuCtxGetDevice_v2: extern "C" fn(arg0: *mut CUdevice, arg1: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetDevice_v2"));
    eprintln!("[CALL] {}", "cuCtxGetDevice_v2");
    cuCtxGetDevice_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunch(arg0: CUfunction) -> CUresult {
    let cuLaunch: extern "C" fn(arg0: CUfunction) -> CUresult =
        std::mem::transmute(get_sym("cuLaunch"));
    eprintln!("[CALL] {}", "cuLaunch");
    cuLaunch(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetMaxAnisotropy(arg0: CUtexref, arg1: c_uint) -> CUresult {
    let cuTexRefSetMaxAnisotropy: extern "C" fn(arg0: CUtexref, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetMaxAnisotropy"));
    eprintln!("[CALL] {}", "cuTexRefSetMaxAnisotropy");
    cuTexRefSetMaxAnisotropy(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetParamInfo(
    arg0: CUkernel,
    arg1: usize,
    arg2: *mut usize,
    arg3: *mut usize,
) -> CUresult {
    let cuKernelGetParamInfo: extern "C" fn(
        arg0: CUkernel,
        arg1: usize,
        arg2: *mut usize,
        arg3: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuKernelGetParamInfo"));
    eprintln!("[CALL] {}", "cuKernelGetParamInfo");
    cuKernelGetParamInfo(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemRangeGetAttribute(
    arg0: *mut c_void,
    arg1: usize,
    arg2: CUmem_range_attribute,
    arg3: CUdeviceptr,
    arg4: usize,
) -> CUresult {
    let cuMemRangeGetAttribute: extern "C" fn(
        arg0: *mut c_void,
        arg1: usize,
        arg2: CUmem_range_attribute,
        arg3: CUdeviceptr,
        arg4: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemRangeGetAttribute"));
    eprintln!("[CALL] {}", "cuMemRangeGetAttribute");
    cuMemRangeGetAttribute(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddDependencies_v2(
    arg0: CUgraph,
    arg1: *const CUgraphNode,
    arg2: *const CUgraphNode,
    arg3: *const CUgraphEdgeData,
    arg4: usize,
) -> CUresult {
    let cuGraphAddDependencies_v2: extern "C" fn(
        arg0: CUgraph,
        arg1: *const CUgraphNode,
        arg2: *const CUgraphNode,
        arg3: *const CUgraphEdgeData,
        arg4: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphAddDependencies_v2"));
    eprintln!("[CALL] {}", "cuGraphAddDependencies_v2");
    cuGraphAddDependencies_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecExternalSemaphoresWaitNodeSetParams(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *const CUDA_EXT_SEM_WAIT_NODE_PARAMS,
) -> CUresult {
    let cuGraphExecExternalSemaphoresWaitNodeSetParams: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *const CUDA_EXT_SEM_WAIT_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecExternalSemaphoresWaitNodeSetParams"));
    eprintln!(
        "[CALL] {}",
        "cuGraphExecExternalSemaphoresWaitNodeSetParams"
    );
    cuGraphExecExternalSemaphoresWaitNodeSetParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoH_v2(
    arg0: *mut c_void,
    arg1: CUdeviceptr,
    arg2: usize,
) -> CUresult {
    let cuMemcpyDtoH_v2: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUdeviceptr,
        arg2: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyDtoH_v2"));
    eprintln!("[CALL] {}", "cuMemcpyDtoH_v2");
    cuMemcpyDtoH_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DUnaligned_v2(arg0: *const CUDA_MEMCPY2D) -> CUresult {
    let cuMemcpy2DUnaligned_v2: extern "C" fn(arg0: *const CUDA_MEMCPY2D) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy2DUnaligned_v2"));
    eprintln!("[CALL] {}", "cuMemcpy2DUnaligned_v2");
    cuMemcpy2DUnaligned_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMulticastBindMem_v2(
    arg0: CUmemGenericAllocationHandle,
    arg1: CUdevice,
    arg2: usize,
    arg3: CUmemGenericAllocationHandle,
    arg4: usize,
    arg5: usize,
    arg6: c_ulonglong,
) -> CUresult {
    let cuMulticastBindMem_v2: extern "C" fn(
        arg0: CUmemGenericAllocationHandle,
        arg1: CUdevice,
        arg2: usize,
        arg3: CUmemGenericAllocationHandle,
        arg4: usize,
        arg5: usize,
        arg6: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMulticastBindMem_v2"));
    eprintln!("[CALL] {}", "cuMulticastBindMem_v2");
    cuMulticastBindMem_v2(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevSmResourceSplit(
    arg0: *mut CUdevResource,
    arg1: c_uint,
    arg2: *const CUdevResource,
    arg3: *mut CUdevResource,
    arg4: c_uint,
    arg5: *mut CU_DEV_SM_RESOURCE_GROUP_PARAMS,
) -> CUresult {
    let cuDevSmResourceSplit: extern "C" fn(
        arg0: *mut CUdevResource,
        arg1: c_uint,
        arg2: *const CUdevResource,
        arg3: *mut CUdevResource,
        arg4: c_uint,
        arg5: *mut CU_DEV_SM_RESOURCE_GROUP_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuDevSmResourceSplit"));
    eprintln!("[CALL] {}", "cuDevSmResourceSplit");
    cuDevSmResourceSplit(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphMemsetNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUDA_MEMSET_NODE_PARAMS,
) -> CUresult {
    let cuGraphMemsetNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUDA_MEMSET_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphMemsetNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphMemsetNodeGetParams");
    cuGraphMemsetNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphLaunch(arg0: CUgraphExec, arg1: CUstream) -> CUresult {
    let cuGraphLaunch: extern "C" fn(arg0: CUgraphExec, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuGraphLaunch"));
    eprintln!("[CALL] {}", "cuGraphLaunch");
    cuGraphLaunch(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyWithAttributesAsync(
    arg0: CUdeviceptr,
    arg1: CUdeviceptr,
    arg2: usize,
    arg3: *mut CUmemcpyAttributes,
    arg4: CUstream,
) -> CUresult {
    let cuMemcpyWithAttributesAsync: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: CUdeviceptr,
        arg2: usize,
        arg3: *mut CUmemcpyAttributes,
        arg4: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyWithAttributesAsync"));
    eprintln!("[CALL] {}", "cuMemcpyWithAttributesAsync");
    cuMemcpyWithAttributesAsync(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventSynchronize(arg0: CUevent) -> CUresult {
    let cuEventSynchronize: extern "C" fn(arg0: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuEventSynchronize"));
    eprintln!("[CALL] {}", "cuEventSynchronize");
    cuEventSynchronize(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuCheckpointProcessUnlock(
    arg0: c_int,
    arg1: *mut CUcheckpointUnlockArgs,
) -> CUresult {
    let cuCheckpointProcessUnlock: extern "C" fn(
        arg0: c_int,
        arg1: *mut CUcheckpointUnlockArgs,
    ) -> CUresult = std::mem::transmute(get_sym("cuCheckpointProcessUnlock"));
    eprintln!("[CALL] {}", "cuCheckpointProcessUnlock");
    cuCheckpointProcessUnlock(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetAllocationGranularity(
    arg0: *mut usize,
    arg1: *const CUmemAllocationProp,
    arg2: CUmemAllocationGranularity_flags,
) -> CUresult {
    let cuMemGetAllocationGranularity: extern "C" fn(
        arg0: *mut usize,
        arg1: *const CUmemAllocationProp,
        arg2: CUmemAllocationGranularity_flags,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemGetAllocationGranularity"));
    eprintln!("[CALL] {}", "cuMemGetAllocationGranularity");
    cuMemGetAllocationGranularity(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamIsCapturing(
    arg0: CUstream,
    arg1: *mut CUstreamCaptureStatus,
) -> CUresult {
    let cuStreamIsCapturing: extern "C" fn(
        arg0: CUstream,
        arg1: *mut CUstreamCaptureStatus,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamIsCapturing"));
    eprintln!("[CALL] {}", "cuStreamIsCapturing");
    cuStreamIsCapturing(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphUpload(arg0: CUgraphExec, arg1: CUstream) -> CUresult {
    let cuGraphUpload: extern "C" fn(arg0: CUgraphExec, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuGraphUpload"));
    eprintln!("[CALL] {}", "cuGraphUpload");
    cuGraphUpload(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryUnload(arg0: CUlibrary) -> CUresult {
    let cuLibraryUnload: extern "C" fn(arg0: CUlibrary) -> CUresult =
        std::mem::transmute(get_sym("cuLibraryUnload"));
    eprintln!("[CALL] {}", "cuLibraryUnload");
    cuLibraryUnload(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D16_v2(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: c_ushort,
    arg3: usize,
    arg4: usize,
) -> CUresult {
    let cuMemsetD2D16_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: c_ushort,
        arg3: usize,
        arg4: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD2D16_v2"));
    eprintln!("[CALL] {}", "cuMemsetD2D16_v2");
    cuMemsetD2D16_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeFindInClone(
    arg0: *mut CUgraphNode,
    arg1: CUgraphNode,
    arg2: CUgraph,
) -> CUresult {
    let cuGraphNodeFindInClone: extern "C" fn(
        arg0: *mut CUgraphNode,
        arg1: CUgraphNode,
        arg2: CUgraph,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphNodeFindInClone"));
    eprintln!("[CALL] {}", "cuGraphNodeFindInClone");
    cuGraphNodeFindInClone(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D16Async(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: c_ushort,
    arg3: usize,
    arg4: usize,
    arg5: CUstream,
) -> CUresult {
    let cuMemsetD2D16Async: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: c_ushort,
        arg3: usize,
        arg4: usize,
        arg5: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD2D16Async"));
    eprintln!("[CALL] {}", "cuMemsetD2D16Async");
    cuMemsetD2D16Async(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoA_v2(
    arg0: CUarray,
    arg1: usize,
    arg2: *const c_void,
    arg3: usize,
) -> CUresult {
    let cuMemcpyHtoA_v2: extern "C" fn(
        arg0: CUarray,
        arg1: usize,
        arg2: *const c_void,
        arg3: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyHtoA_v2"));
    eprintln!("[CALL] {}", "cuMemcpyHtoA_v2");
    cuMemcpyHtoA_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetFunction(
    arg0: *mut CUfunction,
    arg1: CUmodule,
    arg2: *const c_char,
) -> CUresult {
    let cuModuleGetFunction: extern "C" fn(
        arg0: *mut CUfunction,
        arg1: CUmodule,
        arg2: *const c_char,
    ) -> CUresult = std::mem::transmute(get_sym("cuModuleGetFunction"));
    eprintln!("[CALL] {}", "cuModuleGetFunction");
    cuModuleGetFunction(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoHAsync_v2(
    arg0: *mut c_void,
    arg1: CUarray,
    arg2: usize,
    arg3: usize,
    arg4: CUstream,
) -> CUresult {
    let cuMemcpyAtoHAsync_v2: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUarray,
        arg2: usize,
        arg3: usize,
        arg4: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyAtoHAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpyAtoHAsync_v2");
    cuMemcpyAtoHAsync_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphMemFreeNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUdeviceptr,
) -> CUresult {
    let cuGraphMemFreeNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUdeviceptr,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphMemFreeNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphMemFreeNodeGetParams");
    cuGraphMemFreeNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetHostAtomicCapabilities(
    arg0: *mut c_uint,
    arg1: *const CUatomicOperation,
    arg2: c_uint,
    arg3: CUdevice,
) -> CUresult {
    let cuDeviceGetHostAtomicCapabilities: extern "C" fn(
        arg0: *mut c_uint,
        arg1: *const CUatomicOperation,
        arg2: c_uint,
        arg3: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetHostAtomicCapabilities"));
    eprintln!("[CALL] {}", "cuDeviceGetHostAtomicCapabilities");
    cuDeviceGetHostAtomicCapabilities(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventDestroy_v2(arg0: CUevent) -> CUresult {
    let cuEventDestroy_v2: extern "C" fn(arg0: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuEventDestroy_v2"));
    eprintln!("[CALL] {}", "cuEventDestroy_v2");
    cuEventDestroy_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointExport(
    arg0: *mut c_void,
    arg1: CUlogicalEndpointId,
    arg2: CUlogicalEndpointIpcHandleType,
) -> CUresult {
    let cuLogicalEndpointExport: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUlogicalEndpointId,
        arg2: CUlogicalEndpointIpcHandleType,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointExport"));
    eprintln!("[CALL] {}", "cuLogicalEndpointExport");
    cuLogicalEndpointExport(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxReset_v2(arg0: CUdevice) -> CUresult {
    let cuDevicePrimaryCtxReset_v2: extern "C" fn(arg0: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxReset_v2"));
    eprintln!("[CALL] {}", "cuDevicePrimaryCtxReset_v2");
    cuDevicePrimaryCtxReset_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecEventRecordNodeSetEvent(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: CUevent,
) -> CUresult {
    let cuGraphExecEventRecordNodeSetEvent: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: CUevent,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecEventRecordNodeSetEvent"));
    eprintln!("[CALL] {}", "cuGraphExecEventRecordNodeSetEvent");
    cuGraphExecEventRecordNodeSetEvent(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetMemPool(arg0: *mut CUmemoryPool, arg1: CUdevice) -> CUresult {
    let cuDeviceGetMemPool: extern "C" fn(arg0: *mut CUmemoryPool, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetMemPool"));
    eprintln!("[CALL] {}", "cuDeviceGetMemPool");
    cuDeviceGetMemPool(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecMemsetNodeSetParams(
    arg0: CUgraphExec,
    arg1: CUgraphNode,
    arg2: *const CUDA_MEMSET_NODE_PARAMS,
    arg3: CUcontext,
) -> CUresult {
    let cuGraphExecMemsetNodeSetParams: extern "C" fn(
        arg0: CUgraphExec,
        arg1: CUgraphNode,
        arg2: *const CUDA_MEMSET_NODE_PARAMS,
        arg3: CUcontext,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExecMemsetNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExecMemsetNodeSetParams");
    cuGraphExecMemsetNodeSetParams(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolImportFromShareableHandle(
    arg0: *mut CUmemoryPool,
    arg1: *mut c_void,
    arg2: CUmemAllocationHandleType,
    arg3: c_ulonglong,
) -> CUresult {
    let cuMemPoolImportFromShareableHandle: extern "C" fn(
        arg0: *mut CUmemoryPool,
        arg1: *mut c_void,
        arg2: CUmemAllocationHandleType,
        arg3: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolImportFromShareableHandle"));
    eprintln!("[CALL] {}", "cuMemPoolImportFromShareableHandle");
    cuMemPoolImportFromShareableHandle(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayGetSparseProperties(
    arg0: *mut CUDA_ARRAY_SPARSE_PROPERTIES,
    arg1: CUarray,
) -> CUresult {
    let cuArrayGetSparseProperties: extern "C" fn(
        arg0: *mut CUDA_ARRAY_SPARSE_PROPERTIES,
        arg1: CUarray,
    ) -> CUresult = std::mem::transmute(get_sym("cuArrayGetSparseProperties"));
    eprintln!("[CALL] {}", "cuArrayGetSparseProperties");
    cuArrayGetSparseProperties(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuInit(arg0: c_uint) -> CUresult {
    let cuInit: extern "C" fn(arg0: c_uint) -> CUresult = std::mem::transmute(get_sym("cuInit"));
    eprintln!("[CALL] {}", "cuInit");
    cuInit(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolGetAccess(
    arg0: *mut CUmemAccess_flags,
    arg1: CUmemoryPool,
    arg2: *mut CUmemLocation,
) -> CUresult {
    let cuMemPoolGetAccess: extern "C" fn(
        arg0: *mut CUmemAccess_flags,
        arg1: CUmemoryPool,
        arg2: *mut CUmemLocation,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemPoolGetAccess"));
    eprintln!("[CALL] {}", "cuMemPoolGetAccess");
    cuMemPoolGetAccess(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphGetEdges_v2(
    arg0: CUgraph,
    arg1: *mut CUgraphNode,
    arg2: *mut CUgraphNode,
    arg3: *mut CUgraphEdgeData,
    arg4: *mut usize,
) -> CUresult {
    let cuGraphGetEdges_v2: extern "C" fn(
        arg0: CUgraph,
        arg1: *mut CUgraphNode,
        arg2: *mut CUgraphNode,
        arg3: *mut CUgraphEdgeData,
        arg4: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphGetEdges_v2"));
    eprintln!("[CALL] {}", "cuGraphGetEdges_v2");
    cuGraphGetEdges_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetFlags(arg0: *mut c_uint, arg1: CUtexref) -> CUresult {
    let cuTexRefGetFlags: extern "C" fn(arg0: *mut c_uint, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetFlags"));
    eprintln!("[CALL] {}", "cuTexRefGetFlags");
    cuTexRefGetFlags(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpGetAttribute(
    arg0: CUcoredumpSettings,
    arg1: *mut c_void,
    arg2: *mut usize,
) -> CUresult {
    let cuCoredumpGetAttribute: extern "C" fn(
        arg0: CUcoredumpSettings,
        arg1: *mut c_void,
        arg2: *mut usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpGetAttribute"));
    eprintln!("[CALL] {}", "cuCoredumpGetAttribute");
    cuCoredumpGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMipmappedArrayDestroy(arg0: CUmipmappedArray) -> CUresult {
    let cuMipmappedArrayDestroy: extern "C" fn(arg0: CUmipmappedArray) -> CUresult =
        std::mem::transmute(get_sym("cuMipmappedArrayDestroy"));
    eprintln!("[CALL] {}", "cuMipmappedArrayDestroy");
    cuMipmappedArrayDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointImport(
    arg0: CUlogicalEndpointId,
    arg1: *const c_void,
    arg2: CUlogicalEndpointIpcHandleType,
) -> CUresult {
    let cuLogicalEndpointImport: extern "C" fn(
        arg0: CUlogicalEndpointId,
        arg1: *const c_void,
        arg2: CUlogicalEndpointIpcHandleType,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointImport"));
    eprintln!("[CALL] {}", "cuLogicalEndpointImport");
    cuLogicalEndpointImport(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyPeerAsync(
    arg0: CUdeviceptr,
    arg1: CUcontext,
    arg2: CUdeviceptr,
    arg3: CUcontext,
    arg4: usize,
    arg5: CUstream,
) -> CUresult {
    let cuMemcpyPeerAsync: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: CUcontext,
        arg2: CUdeviceptr,
        arg3: CUcontext,
        arg4: usize,
        arg5: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyPeerAsync"));
    eprintln!("[CALL] {}", "cuMemcpyPeerAsync");
    cuMemcpyPeerAsync(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetCurrent(arg0: *mut CUcontext) -> CUresult {
    let cuCtxGetCurrent: extern "C" fn(arg0: *mut CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetCurrent"));
    eprintln!("[CALL] {}", "cuCtxGetCurrent");
    cuCtxGetCurrent(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetCurrent(arg0: CUcontext) -> CUresult {
    let cuCtxSetCurrent: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetCurrent"));
    eprintln!("[CALL] {}", "cuCtxSetCurrent");
    cuCtxSetCurrent(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoD_v2(
    arg0: CUdeviceptr,
    arg1: CUdeviceptr,
    arg2: usize,
) -> CUresult {
    let cuMemcpyDtoD_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: CUdeviceptr,
        arg2: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyDtoD_v2"));
    eprintln!("[CALL] {}", "cuMemcpyDtoD_v2");
    cuMemcpyDtoD_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecGetId(arg0: CUgraphExec, arg1: *mut c_uint) -> CUresult {
    let cuGraphExecGetId: extern "C" fn(arg0: CUgraphExec, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuGraphExecGetId"));
    eprintln!("[CALL] {}", "cuGraphExecGetId");
    cuGraphExecGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxRecordEvent(arg0: CUcontext, arg1: CUevent) -> CUresult {
    let cuCtxRecordEvent: extern "C" fn(arg0: CUcontext, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuCtxRecordEvent"));
    eprintln!("[CALL] {}", "cuCtxRecordEvent");
    cuCtxRecordEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphCreate(arg0: *mut CUgraph, arg1: c_uint) -> CUresult {
    let cuGraphCreate: extern "C" fn(arg0: *mut CUgraph, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuGraphCreate"));
    eprintln!("[CALL] {}", "cuGraphCreate");
    cuGraphCreate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoDAsync_v2(
    arg0: CUdeviceptr,
    arg1: *const c_void,
    arg2: usize,
    arg3: CUstream,
) -> CUresult {
    let cuMemcpyHtoDAsync_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: *const c_void,
        arg2: usize,
        arg3: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemcpyHtoDAsync_v2"));
    eprintln!("[CALL] {}", "cuMemcpyHtoDAsync_v2");
    cuMemcpyHtoDAsync_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExternalSemaphoresWaitNodeSetParams(
    arg0: CUgraphNode,
    arg1: *const CUDA_EXT_SEM_WAIT_NODE_PARAMS,
) -> CUresult {
    let cuGraphExternalSemaphoresWaitNodeSetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *const CUDA_EXT_SEM_WAIT_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphExternalSemaphoresWaitNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExternalSemaphoresWaitNodeSetParams");
    cuGraphExternalSemaphoresWaitNodeSetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCheckpointProcessGetRestoreThreadId(
    arg0: c_int,
    arg1: *mut c_int,
) -> CUresult {
    let cuCheckpointProcessGetRestoreThreadId: extern "C" fn(
        arg0: c_int,
        arg1: *mut c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuCheckpointProcessGetRestoreThreadId"));
    eprintln!("[CALL] {}", "cuCheckpointProcessGetRestoreThreadId");
    cuCheckpointProcessGetRestoreThreadId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcOpenMemHandle_v2(
    arg0: *mut CUdeviceptr,
    arg1: CUipcMemHandle,
    arg2: c_uint,
) -> CUresult {
    let cuIpcOpenMemHandle_v2: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: CUipcMemHandle,
        arg2: c_uint,
    ) -> CUresult = std::mem::transmute(get_sym("cuIpcOpenMemHandle_v2"));
    eprintln!("[CALL] {}", "cuIpcOpenMemHandle_v2");
    cuIpcOpenMemHandle_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFree_v2(arg0: CUdeviceptr) -> CUresult {
    let cuMemFree_v2: extern "C" fn(arg0: CUdeviceptr) -> CUresult =
        std::mem::transmute(get_sym("cuMemFree_v2"));
    eprintln!("[CALL] {}", "cuMemFree_v2");
    cuMemFree_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetLuid(
    arg0: *mut c_char,
    arg1: *mut c_uint,
    arg2: CUdevice,
) -> CUresult {
    let cuDeviceGetLuid: extern "C" fn(
        arg0: *mut c_char,
        arg1: *mut c_uint,
        arg2: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetLuid"));
    eprintln!("[CALL] {}", "cuDeviceGetLuid");
    cuDeviceGetLuid(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMulticastBindMem(
    arg0: CUmemGenericAllocationHandle,
    arg1: usize,
    arg2: CUmemGenericAllocationHandle,
    arg3: usize,
    arg4: usize,
    arg5: c_ulonglong,
) -> CUresult {
    let cuMulticastBindMem: extern "C" fn(
        arg0: CUmemGenericAllocationHandle,
        arg1: usize,
        arg2: CUmemGenericAllocationHandle,
        arg3: usize,
        arg4: usize,
        arg5: c_ulonglong,
    ) -> CUresult = std::mem::transmute(get_sym("cuMulticastBindMem"));
    eprintln!("[CALL] {}", "cuMulticastBindMem");
    cuMulticastBindMem(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuGetErrorString(arg0: CUresult, arg1: *mut *const c_char) -> CUresult {
    let cuGetErrorString: extern "C" fn(arg0: CUresult, arg1: *mut *const c_char) -> CUresult =
        std::mem::transmute(get_sym("cuGetErrorString"));
    eprintln!("[CALL] {}", "cuGetErrorString");
    cuGetErrorString(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocAsync(
    arg0: *mut CUdeviceptr,
    arg1: usize,
    arg2: CUstream,
) -> CUresult {
    let cuMemAllocAsync: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: usize,
        arg2: CUstream,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemAllocAsync"));
    eprintln!("[CALL] {}", "cuMemAllocAsync");
    cuMemAllocAsync(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphEventWaitNodeSetEvent(
    arg0: CUgraphNode,
    arg1: CUevent,
) -> CUresult {
    let cuGraphEventWaitNodeSetEvent: extern "C" fn(arg0: CUgraphNode, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuGraphEventWaitNodeSetEvent"));
    eprintln!("[CALL] {}", "cuGraphEventWaitNodeSetEvent");
    cuGraphEventWaitNodeSetEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetNvSciSyncAttributes(
    arg0: *mut c_void,
    arg1: CUdevice,
    arg2: c_int,
) -> CUresult {
    let cuDeviceGetNvSciSyncAttributes: extern "C" fn(
        arg0: *mut c_void,
        arg1: CUdevice,
        arg2: c_int,
    ) -> CUresult = std::mem::transmute(get_sym("cuDeviceGetNvSciSyncAttributes"));
    eprintln!("[CALL] {}", "cuDeviceGetNvSciSyncAttributes");
    cuDeviceGetNvSciSyncAttributes(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemSetMemPool(
    arg0: *mut CUmemLocation,
    arg1: CUmemAllocationType,
    arg2: CUmemoryPool,
) -> CUresult {
    let cuMemSetMemPool: extern "C" fn(
        arg0: *mut CUmemLocation,
        arg1: CUmemAllocationType,
        arg2: CUmemoryPool,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemSetMemPool"));
    eprintln!("[CALL] {}", "cuMemSetMemPool");
    cuMemSetMemPool(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetMemPool(
    arg0: *mut CUmemoryPool,
    arg1: *mut CUmemLocation,
    arg2: CUmemAllocationType,
) -> CUresult {
    let cuMemGetMemPool: extern "C" fn(
        arg0: *mut CUmemoryPool,
        arg1: *mut CUmemLocation,
        arg2: CUmemAllocationType,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemGetMemPool"));
    eprintln!("[CALL] {}", "cuMemGetMemPool");
    cuMemGetMemPool(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D8_v2(
    arg0: CUdeviceptr,
    arg1: usize,
    arg2: c_uchar,
    arg3: usize,
    arg4: usize,
) -> CUresult {
    let cuMemsetD2D8_v2: extern "C" fn(
        arg0: CUdeviceptr,
        arg1: usize,
        arg2: c_uchar,
        arg3: usize,
        arg4: usize,
    ) -> CUresult = std::mem::transmute(get_sym("cuMemsetD2D8_v2"));
    eprintln!("[CALL] {}", "cuMemsetD2D8_v2");
    cuMemsetD2D8_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefCreate(arg0: *mut CUtexref) -> CUresult {
    let cuTexRefCreate: extern "C" fn(arg0: *mut CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefCreate"));
    eprintln!("[CALL] {}", "cuTexRefCreate");
    cuTexRefCreate(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphHostNodeGetParams(
    arg0: CUgraphNode,
    arg1: *mut CUDA_HOST_NODE_PARAMS,
) -> CUresult {
    let cuGraphHostNodeGetParams: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUDA_HOST_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphHostNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphHostNodeGetParams");
    cuGraphHostNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkCreate_v2(
    arg0: c_uint,
    arg1: *mut CUjit_option,
    arg2: *mut *mut c_void,
    arg3: *mut CUlinkState,
) -> CUresult {
    let cuLinkCreate_v2: extern "C" fn(
        arg0: c_uint,
        arg1: *mut CUjit_option,
        arg2: *mut *mut c_void,
        arg3: *mut CUlinkState,
    ) -> CUresult = std::mem::transmute(get_sym("cuLinkCreate_v2"));
    eprintln!("[CALL] {}", "cuLinkCreate_v2");
    cuLinkCreate_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleLoadData(arg0: *mut CUmodule, arg1: *const c_void) -> CUresult {
    let cuModuleLoadData: extern "C" fn(arg0: *mut CUmodule, arg1: *const c_void) -> CUresult =
        std::mem::transmute(get_sym("cuModuleLoadData"));
    eprintln!("[CALL] {}", "cuModuleLoadData");
    cuModuleLoadData(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeGetParams_v2(
    arg0: CUgraphNode,
    arg1: *mut CUDA_KERNEL_NODE_PARAMS,
) -> CUresult {
    let cuGraphKernelNodeGetParams_v2: extern "C" fn(
        arg0: CUgraphNode,
        arg1: *mut CUDA_KERNEL_NODE_PARAMS,
    ) -> CUresult = std::mem::transmute(get_sym("cuGraphKernelNodeGetParams_v2"));
    eprintln!("[CALL] {}", "cuGraphKernelNodeGetParams_v2");
    cuGraphKernelNodeGetParams_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamSynchronize(arg0: CUstream) -> CUresult {
    let cuStreamSynchronize: extern "C" fn(arg0: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamSynchronize"));
    eprintln!("[CALL] {}", "cuStreamSynchronize");
    cuStreamSynchronize(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetAttribute(
    arg0: CUstream,
    arg1: CUstreamAttrID,
    arg2: *mut CUstreamAttrValue,
) -> CUresult {
    let cuStreamGetAttribute: extern "C" fn(
        arg0: CUstream,
        arg1: CUstreamAttrID,
        arg2: *mut CUstreamAttrValue,
    ) -> CUresult = std::mem::transmute(get_sym("cuStreamGetAttribute"));
    eprintln!("[CALL] {}", "cuStreamGetAttribute");
    cuStreamGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuParamSetSize(arg0: CUfunction, arg1: c_uint) -> CUresult {
    let cuParamSetSize: extern "C" fn(arg0: CUfunction, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuParamSetSize"));
    eprintln!("[CALL] {}", "cuParamSetSize");
    cuParamSetSize(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryGetManaged(
    arg0: *mut CUdeviceptr,
    arg1: *mut usize,
    arg2: CUlibrary,
    arg3: *const c_char,
) -> CUresult {
    let cuLibraryGetManaged: extern "C" fn(
        arg0: *mut CUdeviceptr,
        arg1: *mut usize,
        arg2: CUlibrary,
        arg3: *const c_char,
    ) -> CUresult = std::mem::transmute(get_sym("cuLibraryGetManaged"));
    eprintln!("[CALL] {}", "cuLibraryGetManaged");
    cuLibraryGetManaged(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetMipmapFilterMode(
    arg0: *mut CUfilter_mode,
    arg1: CUtexref,
) -> CUresult {
    let cuTexRefGetMipmapFilterMode: extern "C" fn(
        arg0: *mut CUfilter_mode,
        arg1: CUtexref,
    ) -> CUresult = std::mem::transmute(get_sym("cuTexRefGetMipmapFilterMode"));
    eprintln!("[CALL] {}", "cuTexRefGetMipmapFilterMode");
    cuTexRefGetMipmapFilterMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcCloseMemHandle(arg0: CUdeviceptr) -> CUresult {
    let cuIpcCloseMemHandle: extern "C" fn(arg0: CUdeviceptr) -> CUresult =
        std::mem::transmute(get_sym("cuIpcCloseMemHandle"));
    eprintln!("[CALL] {}", "cuIpcCloseMemHandle");
    cuIpcCloseMemHandle(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphDestroyNode(arg0: CUgraphNode) -> CUresult {
    let cuGraphDestroyNode: extern "C" fn(arg0: CUgraphNode) -> CUresult =
        std::mem::transmute(get_sym("cuGraphDestroyNode"));
    eprintln!("[CALL] {}", "cuGraphDestroyNode");
    cuGraphDestroyNode(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointAddDevice(
    arg0: CUlogicalEndpointId,
    arg1: CUdevice,
) -> CUresult {
    let cuLogicalEndpointAddDevice: extern "C" fn(
        arg0: CUlogicalEndpointId,
        arg1: CUdevice,
    ) -> CUresult = std::mem::transmute(get_sym("cuLogicalEndpointAddDevice"));
    eprintln!("[CALL] {}", "cuLogicalEndpointAddDevice");
    cuLogicalEndpointAddDevice(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolDestroy(arg0: CUmemoryPool) -> CUresult {
    let cuMemPoolDestroy: extern "C" fn(arg0: CUmemoryPool) -> CUresult =
        std::mem::transmute(get_sym("cuMemPoolDestroy"));
    eprintln!("[CALL] {}", "cuMemPoolDestroy");
    cuMemPoolDestroy(arg0)
}

// Unknown function stubs (functions not found in header)
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetGlobal() {
    let cuModuleGetGlobal: extern "C" fn() = std::mem::transmute(get_sym("cuModuleGetGlobal"));
    eprintln!("[CALL] {}", "cuModuleGetGlobal");
    cuModuleGetGlobal()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetInfo() {
    let cuMemGetInfo: extern "C" fn() = std::mem::transmute(get_sym("cuMemGetInfo"));
    eprintln!("[CALL] {}", "cuMemGetInfo");
    cuMemGetInfo()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecKernelNodeSetParams() {
    let cuGraphExecKernelNodeSetParams: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphExecKernelNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphExecKernelNodeSetParams");
    cuGraphExecKernelNodeSetParams()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchKernelEx_ptsz() {
    let cuLaunchKernelEx_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuLaunchKernelEx_ptsz"));
    eprintln!("[CALL] {}", "cuLaunchKernelEx_ptsz");
    cuLaunchKernelEx_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphGetEdges() {
    let cuGraphGetEdges: extern "C" fn() = std::mem::transmute(get_sym("cuGraphGetEdges"));
    eprintln!("[CALL] {}", "cuGraphGetEdges");
    cuGraphGetEdges()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObjectAsync() {
    let cuGLMapBufferObjectAsync: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObjectAsync"));
    eprintln!("[CALL] {}", "cuGLMapBufferObjectAsync");
    cuGLMapBufferObjectAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoA_v2_ptds() {
    let cuMemcpyAtoA_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyAtoA_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyAtoA_v2_ptds");
    cuMemcpyAtoA_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLUnmapBufferObject() {
    let cuGLUnmapBufferObject: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLUnmapBufferObject"));
    eprintln!("[CALL] {}", "cuGLUnmapBufferObject");
    cuGLUnmapBufferObject()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerConnectWithFlags() {
    let cuEGLStreamConsumerConnectWithFlags: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerConnectWithFlags"));
    eprintln!("[CALL] {}", "cuEGLStreamConsumerConnectWithFlags");
    cuEGLStreamConsumerConnectWithFlags()
}
#[no_mangle]
pub unsafe extern "C" fn cuSignalExternalSemaphoresAsync_ptsz() {
    let cuSignalExternalSemaphoresAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuSignalExternalSemaphoresAsync_ptsz"));
    eprintln!("[CALL] {}", "cuSignalExternalSemaphoresAsync_ptsz");
    cuSignalExternalSemaphoresAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsVDPAURegisterOutputSurface() {
    let cuGraphicsVDPAURegisterOutputSurface: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsVDPAURegisterOutputSurface"));
    eprintln!("[CALL] {}", "cuGraphicsVDPAURegisterOutputSurface");
    cuGraphicsVDPAURegisterOutputSurface()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventCreateFromEGLSync() {
    let cuEventCreateFromEGLSync: extern "C" fn() =
        std::mem::transmute(get_sym("cuEventCreateFromEGLSync"));
    eprintln!("[CALL] {}", "cuEventCreateFromEGLSync");
    cuEventCreateFromEGLSync()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsGLRegisterBuffer() {
    let cuGraphicsGLRegisterBuffer: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsGLRegisterBuffer"));
    eprintln!("[CALL] {}", "cuGraphicsGLRegisterBuffer");
    cuGraphicsGLRegisterBuffer()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAlloc() {
    let cuMemAlloc: extern "C" fn() = std::mem::transmute(get_sym("cuMemAlloc"));
    eprintln!("[CALL] {}", "cuMemAlloc");
    cuMemAlloc()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchAsync_ptsz() {
    let cuMemPrefetchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemPrefetchAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemPrefetchAsync_ptsz");
    cuMemPrefetchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoHAsync_v2_ptsz() {
    let cuMemcpyAtoHAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyAtoHAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyAtoHAsync_v2_ptsz");
    cuMemcpyAtoHAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DAsync() {
    let cuMemcpy2DAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy2DAsync"));
    eprintln!("[CALL] {}", "cuMemcpy2DAsync");
    cuMemcpy2DAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoA_v2_ptds() {
    let cuMemcpyHtoA_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyHtoA_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyHtoA_v2_ptds");
    cuMemcpyHtoA_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventDestroy() {
    let cuEventDestroy: extern "C" fn() = std::mem::transmute(get_sym("cuEventDestroy"));
    eprintln!("[CALL] {}", "cuEventDestroy");
    cuEventDestroy()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamDestroy() {
    let cuStreamDestroy: extern "C" fn() = std::mem::transmute(get_sym("cuStreamDestroy"));
    eprintln!("[CALL] {}", "cuStreamDestroy");
    cuStreamDestroy()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBatchMemOp_v2_ptsz() {
    let cuStreamBatchMemOp_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBatchMemOp_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamBatchMemOp_v2_ptsz");
    cuStreamBatchMemOp_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObjectAsync_v2() {
    let cuGLMapBufferObjectAsync_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObjectAsync_v2"));
    eprintln!("[CALL] {}", "cuGLMapBufferObjectAsync_v2");
    cuGLMapBufferObjectAsync_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphUpload_ptsz() {
    let cuGraphUpload_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuGraphUpload_ptsz"));
    eprintln!("[CALL] {}", "cuGraphUpload_ptsz");
    cuGraphUpload_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamUpdateCaptureDependencies_ptsz() {
    let cuStreamUpdateCaptureDependencies_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamUpdateCaptureDependencies_ptsz"));
    eprintln!("[CALL] {}", "cuStreamUpdateCaptureDependencies_ptsz");
    cuStreamUpdateCaptureDependencies_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamEndCaptureToCig_ptsz() {
    let cuStreamEndCaptureToCig_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamEndCaptureToCig_ptsz"));
    eprintln!("[CALL] {}", "cuStreamEndCaptureToCig_ptsz");
    cuStreamEndCaptureToCig_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkAddData() {
    let cuLinkAddData: extern "C" fn() = std::mem::transmute(get_sym("cuLinkAddData"));
    eprintln!("[CALL] {}", "cuLinkAddData");
    cuLinkAddData()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D32Async_ptsz() {
    let cuMemsetD2D32Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D32Async_ptsz"));
    eprintln!("[CALL] {}", "cuMemsetD2D32Async_ptsz");
    cuMemsetD2D32Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyPeer_ptds() {
    let cuMemcpyPeer_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyPeer_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyPeer_ptds");
    cuMemcpyPeer_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8() {
    let cuMemsetD8: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD8"));
    eprintln!("[CALL] {}", "cuMemsetD8");
    cuMemsetD8()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamEndCapture_ptsz() {
    let cuStreamEndCapture_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamEndCapture_ptsz"));
    eprintln!("[CALL] {}", "cuStreamEndCapture_ptsz");
    cuStreamEndCapture_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxCreate_v2() {
    let cuCtxCreate_v2: extern "C" fn() = std::mem::transmute(get_sym("cuCtxCreate_v2"));
    eprintln!("[CALL] {}", "cuCtxCreate_v2");
    cuCtxCreate_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxCreate_v3() {
    let cuCtxCreate_v3: extern "C" fn() = std::mem::transmute(get_sym("cuCtxCreate_v3"));
    eprintln!("[CALL] {}", "cuCtxCreate_v3");
    cuCtxCreate_v3()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgReportDriverApiError() {
    let cudbgReportDriverApiError: extern "C" fn() =
        std::mem::transmute(get_sym("cudbgReportDriverApiError"));
    eprintln!("[CALL] {}", "cudbgReportDriverApiError");
    cudbgReportDriverApiError()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchBatchAsync_ptsz() {
    let cuMemPrefetchBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemPrefetchBatchAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemPrefetchBatchAsync_ptsz");
    cuMemPrefetchBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuVDPAUCtxCreate() {
    let cuVDPAUCtxCreate: extern "C" fn() = std::mem::transmute(get_sym("cuVDPAUCtxCreate"));
    eprintln!("[CALL] {}", "cuVDPAUCtxCreate");
    cuVDPAUCtxCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsMapResources_ptsz() {
    let cuGraphicsMapResources_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsMapResources_ptsz"));
    eprintln!("[CALL] {}", "cuGraphicsMapResources_ptsz");
    cuGraphicsMapResources_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAsync_ptsz() {
    let cuMemcpyAsync_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyAsync_ptsz");
    cuMemcpyAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginRecaptureToGraph_ptsz() {
    let cuStreamBeginRecaptureToGraph_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginRecaptureToGraph_ptsz"));
    eprintln!("[CALL] {}", "cuStreamBeginRecaptureToGraph_ptsz");
    cuStreamBeginRecaptureToGraph_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemBatchDecompressAsync_ptsz() {
    let cuMemBatchDecompressAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemBatchDecompressAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemBatchDecompressAsync_ptsz");
    cuMemBatchDecompressAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetDependentNodes() {
    let cuGraphNodeGetDependentNodes: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphNodeGetDependentNodes"));
    eprintln!("[CALL] {}", "cuGraphNodeGetDependentNodes");
    cuGraphNodeGetDependentNodes()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy_ptds() {
    let cuMemcpy_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy_ptds"));
    eprintln!("[CALL] {}", "cuMemcpy_ptds");
    cuMemcpy_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGetProcAddress() {
    let cuGetProcAddress: extern "C" fn() = std::mem::transmute(get_sym("cuGetProcAddress"));
    eprintln!("[CALL] {}", "cuGetProcAddress");
    cuGetProcAddress()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostRegister() {
    let cuMemHostRegister: extern "C" fn() = std::mem::transmute(get_sym("cuMemHostRegister"));
    eprintln!("[CALL] {}", "cuMemHostRegister");
    cuMemHostRegister()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue32() {
    let cuStreamWaitValue32: extern "C" fn() = std::mem::transmute(get_sym("cuStreamWaitValue32"));
    eprintln!("[CALL] {}", "cuStreamWaitValue32");
    cuStreamWaitValue32()
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddress2D_v2() {
    let cuTexRefSetAddress2D_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuTexRefSetAddress2D_v2"));
    eprintln!("[CALL] {}", "cuTexRefSetAddress2D_v2");
    cuTexRefSetAddress2D_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DAsync_v2_ptsz() {
    let cuMemcpy2DAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy2DAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpy2DAsync_v2_ptsz");
    cuMemcpy2DAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamAttachMemAsync_ptsz() {
    let cuStreamAttachMemAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamAttachMemAsync_ptsz"));
    eprintln!("[CALL] {}", "cuStreamAttachMemAsync_ptsz");
    cuStreamAttachMemAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoAAsync_v2_ptsz() {
    let cuMemcpyHtoAAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyHtoAAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyHtoAAsync_v2_ptsz");
    cuMemcpyHtoAAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocHost() {
    let cuMemAllocHost: extern "C" fn() = std::mem::transmute(get_sym("cuMemAllocHost"));
    eprintln!("[CALL] {}", "cuMemAllocHost");
    cuMemAllocHost()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocAsync_ptsz() {
    let cuMemAllocAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemAllocAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemAllocAsync_ptsz");
    cuMemAllocAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DUnaligned_v2_ptds() {
    let cuMemcpy2DUnaligned_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy2DUnaligned_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpy2DUnaligned_v2_ptds");
    cuMemcpy2DUnaligned_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoDAsync_v2_ptsz() {
    let cuMemcpyDtoDAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoDAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyDtoDAsync_v2_ptsz");
    cuMemcpyDtoDAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBatchMemOp() {
    let cuStreamBatchMemOp: extern "C" fn() = std::mem::transmute(get_sym("cuStreamBatchMemOp"));
    eprintln!("[CALL] {}", "cuStreamBatchMemOp");
    cuStreamBatchMemOp()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoH_v2_ptds() {
    let cuMemcpyAtoH_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyAtoH_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyAtoH_v2_ptds");
    cuMemcpyAtoH_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayGetDescriptor() {
    let cuArrayGetDescriptor: extern "C" fn() =
        std::mem::transmute(get_sym("cuArrayGetDescriptor"));
    eprintln!("[CALL] {}", "cuArrayGetDescriptor");
    cuArrayGetDescriptor()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocFromPoolAsync_ptsz() {
    let cuMemAllocFromPoolAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemAllocFromPoolAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemAllocFromPoolAsync_ptsz");
    cuMemAllocFromPoolAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue32() {
    let cuStreamWriteValue32: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue32"));
    eprintln!("[CALL] {}", "cuStreamWriteValue32");
    cuStreamWriteValue32()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeSetParams() {
    let cuGraphKernelNodeSetParams: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphKernelNodeSetParams"));
    eprintln!("[CALL] {}", "cuGraphKernelNodeSetParams");
    cuGraphKernelNodeSetParams()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsGLRegisterImage() {
    let cuGraphicsGLRegisterImage: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsGLRegisterImage"));
    eprintln!("[CALL] {}", "cuGraphicsGLRegisterImage");
    cuGraphicsGLRegisterImage()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoA() {
    let cuMemcpyHtoA: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyHtoA"));
    eprintln!("[CALL] {}", "cuMemcpyHtoA");
    cuMemcpyHtoA()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamProducerReturnFrame() {
    let cuEGLStreamProducerReturnFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamProducerReturnFrame"));
    eprintln!("[CALL] {}", "cuEGLStreamProducerReturnFrame");
    cuEGLStreamProducerReturnFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue64_v2_ptsz() {
    let cuStreamWriteValue64_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue64_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWriteValue64_v2_ptsz");
    cuStreamWriteValue64_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD16() {
    let cuMemsetD16: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD16"));
    eprintln!("[CALL] {}", "cuMemsetD16");
    cuMemsetD16()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoD() {
    let cuMemcpyHtoD: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyHtoD"));
    eprintln!("[CALL] {}", "cuMemcpyHtoD");
    cuMemcpyHtoD()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgReportAttachProcedureFinished() {
    let cudbgReportAttachProcedureFinished: extern "C" fn() =
        std::mem::transmute(get_sym("cudbgReportAttachProcedureFinished"));
    eprintln!("[CALL] {}", "cudbgReportAttachProcedureFinished");
    cudbgReportAttachProcedureFinished()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoHAsync() {
    let cuMemcpyAtoHAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAtoHAsync"));
    eprintln!("[CALL] {}", "cuMemcpyAtoHAsync");
    cuMemcpyAtoHAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue32_ptsz() {
    let cuStreamWriteValue32_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue32_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWriteValue32_ptsz");
    cuStreamWriteValue32_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8Async_ptsz() {
    let cuMemsetD8Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD8Async_ptsz"));
    eprintln!("[CALL] {}", "cuMemsetD8Async_ptsz");
    cuMemsetD8Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFree() {
    let cuMemFree: extern "C" fn() = std::mem::transmute(get_sym("cuMemFree"));
    eprintln!("[CALL] {}", "cuMemFree");
    cuMemFree()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsVDPAURegisterVideoSurface() {
    let cuGraphicsVDPAURegisterVideoSurface: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsVDPAURegisterVideoSurface"));
    eprintln!("[CALL] {}", "cuGraphicsVDPAURegisterVideoSurface");
    cuGraphicsVDPAURegisterVideoSurface()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DBatchAsync_ptsz() {
    let cuMemcpy3DBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DBatchAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpy3DBatchAsync_ptsz");
    cuMemcpy3DBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue64_ptsz() {
    let cuStreamWaitValue64_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitValue64_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWaitValue64_ptsz");
    cuStreamWaitValue64_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoD_v2_ptds() {
    let cuMemcpyHtoD_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyHtoD_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyHtoD_v2_ptds");
    cuMemcpyHtoD_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkAddFile() {
    let cuLinkAddFile: extern "C" fn() = std::mem::transmute(get_sym("cuLinkAddFile"));
    eprintln!("[CALL] {}", "cuLinkAddFile");
    cuLinkAddFile()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD16_v2_ptds() {
    let cuMemsetD16_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD16_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemsetD16_v2_ptds");
    cuMemsetD16_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecUpdate() {
    let cuGraphExecUpdate: extern "C" fn() = std::mem::transmute(get_sym("cuGraphExecUpdate"));
    eprintln!("[CALL] {}", "cuGraphExecUpdate");
    cuGraphExecUpdate()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D16Async_ptsz() {
    let cuMemsetD2D16Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D16Async_ptsz"));
    eprintln!("[CALL] {}", "cuMemsetD2D16Async_ptsz");
    cuMemsetD2D16Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchCooperativeKernel_ptsz() {
    let cuLaunchCooperativeKernel_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuLaunchCooperativeKernel_ptsz"));
    eprintln!("[CALL] {}", "cuLaunchCooperativeKernel_ptsz");
    cuLaunchCooperativeKernel_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLUnmapBufferObjectAsync() {
    let cuGLUnmapBufferObjectAsync: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLUnmapBufferObjectAsync"));
    eprintln!("[CALL] {}", "cuGLUnmapBufferObjectAsync");
    cuGLUnmapBufferObjectAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddress2D() {
    let cuTexRefSetAddress2D: extern "C" fn() =
        std::mem::transmute(get_sym("cuTexRefSetAddress2D"));
    eprintln!("[CALL] {}", "cuTexRefSetAddress2D");
    cuTexRefSetAddress2D()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObjectAsync_v2_ptsz() {
    let cuGLMapBufferObjectAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObjectAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuGLMapBufferObjectAsync_v2_ptsz");
    cuGLMapBufferObjectAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceSetMapFlags() {
    let cuGraphicsResourceSetMapFlags: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsResourceSetMapFlags"));
    eprintln!("[CALL] {}", "cuGraphicsResourceSetMapFlags");
    cuGraphicsResourceSetMapFlags()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamUpdateCaptureDependencies() {
    let cuStreamUpdateCaptureDependencies: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamUpdateCaptureDependencies"));
    eprintln!("[CALL] {}", "cuStreamUpdateCaptureDependencies");
    cuStreamUpdateCaptureDependencies()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgReportDriverInternalError() {
    let cudbgReportDriverInternalError: extern "C" fn() =
        std::mem::transmute(get_sym("cudbgReportDriverInternalError"));
    eprintln!("[CALL] {}", "cudbgReportDriverInternalError");
    cudbgReportDriverInternalError()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DAsync() {
    let cuMemcpy3DAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy3DAsync"));
    eprintln!("[CALL] {}", "cuMemcpy3DAsync");
    cuMemcpy3DAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoA_v2_ptds() {
    let cuMemcpyDtoA_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoA_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyDtoA_v2_ptds");
    cuMemcpyDtoA_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitEvent_ptsz() {
    let cuStreamWaitEvent_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitEvent_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWaitEvent_ptsz");
    cuStreamWaitEvent_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgGetAPI() {
    let cudbgGetAPI: extern "C" fn() = std::mem::transmute(get_sym("cudbgGetAPI"));
    eprintln!("[CALL] {}", "cudbgGetAPI");
    cudbgGetAPI()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoA() {
    let cuMemcpyDtoA: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoA"));
    eprintln!("[CALL] {}", "cuMemcpyDtoA");
    cuMemcpyDtoA()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoD() {
    let cuMemcpyDtoD: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoD"));
    eprintln!("[CALL] {}", "cuMemcpyDtoD");
    cuMemcpyDtoD()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerReleaseFrame() {
    let cuEGLStreamConsumerReleaseFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerReleaseFrame"));
    eprintln!("[CALL] {}", "cuEGLStreamConsumerReleaseFrame");
    cuEGLStreamConsumerReleaseFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_v2() {
    let cuStreamGetCaptureInfo_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo_v2"));
    eprintln!("[CALL] {}", "cuStreamGetCaptureInfo_v2");
    cuStreamGetCaptureInfo_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCtx_ptsz() {
    let cuStreamGetCtx_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuStreamGetCtx_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetCtx_ptsz");
    cuStreamGetCtx_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2D() {
    let cuMemcpy2D: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy2D"));
    eprintln!("[CALL] {}", "cuMemcpy2D");
    cuMemcpy2D()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoH() {
    let cuMemcpyDtoH: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoH"));
    eprintln!("[CALL] {}", "cuMemcpyDtoH");
    cuMemcpyDtoH()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCaptureToGraph_ptsz() {
    let cuStreamBeginCaptureToGraph_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCaptureToGraph_ptsz"));
    eprintln!("[CALL] {}", "cuStreamBeginCaptureToGraph_ptsz");
    cuStreamBeginCaptureToGraph_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceGetMappedPointer() {
    let cuGraphicsResourceGetMappedPointer: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsResourceGetMappedPointer"));
    eprintln!("[CALL] {}", "cuGraphicsResourceGetMappedPointer");
    cuGraphicsResourceGetMappedPointer()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue32_v2_ptsz() {
    let cuStreamWriteValue32_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue32_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWriteValue32_v2_ptsz");
    cuStreamWriteValue32_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLRegisterBufferObject() {
    let cuGLRegisterBufferObject: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLRegisterBufferObject"));
    eprintln!("[CALL] {}", "cuGLRegisterBufferObject");
    cuGLRegisterBufferObject()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCapture() {
    let cuStreamBeginCapture: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCapture"));
    eprintln!("[CALL] {}", "cuStreamBeginCapture");
    cuStreamBeginCapture()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32() {
    let cuMemsetD32: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD32"));
    eprintln!("[CALL] {}", "cuMemsetD32");
    cuMemsetD32()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLCtxCreate() {
    let cuGLCtxCreate: extern "C" fn() = std::mem::transmute(get_sym("cuGLCtxCreate"));
    eprintln!("[CALL] {}", "cuGLCtxCreate");
    cuGLCtxCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo() {
    let cuStreamGetCaptureInfo: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo"));
    eprintln!("[CALL] {}", "cuStreamGetCaptureInfo");
    cuStreamGetCaptureInfo()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue64() {
    let cuStreamWaitValue64: extern "C" fn() = std::mem::transmute(get_sym("cuStreamWaitValue64"));
    eprintln!("[CALL] {}", "cuStreamWaitValue64");
    cuStreamWaitValue64()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLSetBufferObjectMapFlags() {
    let cuGLSetBufferObjectMapFlags: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLSetBufferObjectMapFlags"));
    eprintln!("[CALL] {}", "cuGLSetBufferObjectMapFlags");
    cuGLSetBufferObjectMapFlags()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoA() {
    let cuMemcpyAtoA: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAtoA"));
    eprintln!("[CALL] {}", "cuMemcpyAtoA");
    cuMemcpyAtoA()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamProducerDisconnect() {
    let cuEGLStreamProducerDisconnect: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamProducerDisconnect"));
    eprintln!("[CALL] {}", "cuEGLStreamProducerDisconnect");
    cuEGLStreamProducerDisconnect()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphRemoveDependencies() {
    let cuGraphRemoveDependencies: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphRemoveDependencies"));
    eprintln!("[CALL] {}", "cuGraphRemoveDependencies");
    cuGraphRemoveDependencies()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoD() {
    let cuMemcpyAtoD: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAtoD"));
    eprintln!("[CALL] {}", "cuMemcpyAtoD");
    cuMemcpyAtoD()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3D() {
    let cuMemcpy3D: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy3D"));
    eprintln!("[CALL] {}", "cuMemcpy3D");
    cuMemcpy3D()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphInstantiateWithParams_ptsz() {
    let cuGraphInstantiateWithParams_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphInstantiateWithParams_ptsz"));
    eprintln!("[CALL] {}", "cuGraphInstantiateWithParams_ptsz");
    cuGraphInstantiateWithParams_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoH() {
    let cuMemcpyAtoH: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAtoH"));
    eprintln!("[CALL] {}", "cuMemcpyAtoH");
    cuMemcpyAtoH()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgApiAttach() {
    let cudbgApiAttach: extern "C" fn() = std::mem::transmute(get_sym("cudbgApiAttach"));
    eprintln!("[CALL] {}", "cudbgApiAttach");
    cudbgApiAttach()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_ptsz() {
    let cuStreamGetCaptureInfo_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetCaptureInfo_ptsz");
    cuStreamGetCaptureInfo_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerAcquireFrame() {
    let cuEGLStreamConsumerAcquireFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerAcquireFrame"));
    eprintln!("[CALL] {}", "cuEGLStreamConsumerAcquireFrame");
    cuEGLStreamConsumerAcquireFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamProducerConnect() {
    let cuEGLStreamProducerConnect: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamProducerConnect"));
    eprintln!("[CALL] {}", "cuEGLStreamProducerConnect");
    cuEGLStreamProducerConnect()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue64() {
    let cuStreamWriteValue64: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue64"));
    eprintln!("[CALL] {}", "cuStreamWriteValue64");
    cuStreamWriteValue64()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphLaunch_ptsz() {
    let cuGraphLaunch_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuGraphLaunch_ptsz"));
    eprintln!("[CALL] {}", "cuGraphLaunch_ptsz");
    cuGraphLaunch_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxRelease() {
    let cuDevicePrimaryCtxRelease: extern "C" fn() =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxRelease"));
    eprintln!("[CALL] {}", "cuDevicePrimaryCtxRelease");
    cuDevicePrimaryCtxRelease()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoHAsync_v2_ptsz() {
    let cuMemcpyDtoHAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoHAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyDtoHAsync_v2_ptsz");
    cuMemcpyDtoHAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DBatchAsync() {
    let cuMemcpy3DBatchAsync: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DBatchAsync"));
    eprintln!("[CALL] {}", "cuMemcpy3DBatchAsync");
    cuMemcpy3DBatchAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuProfilerInitialize() {
    let cuProfilerInitialize: extern "C" fn() =
        std::mem::transmute(get_sym("cuProfilerInitialize"));
    eprintln!("[CALL] {}", "cuProfilerInitialize");
    cuProfilerInitialize()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D16_v2_ptds() {
    let cuMemsetD2D16_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D16_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemsetD2D16_v2_ptds");
    cuMemsetD2D16_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObject_v2() {
    let cuGLMapBufferObject_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObject_v2"));
    eprintln!("[CALL] {}", "cuGLMapBufferObject_v2");
    cuGLMapBufferObject_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoD_v2_ptds() {
    let cuMemcpyAtoD_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyAtoD_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyAtoD_v2_ptds");
    cuMemcpyAtoD_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLGetDevices() {
    let cuGLGetDevices: extern "C" fn() = std::mem::transmute(get_sym("cuGLGetDevices"));
    eprintln!("[CALL] {}", "cuGLGetDevices");
    cuGLGetDevices()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObject_v2_ptds() {
    let cuGLMapBufferObject_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObject_v2_ptds"));
    eprintln!("[CALL] {}", "cuGLMapBufferObject_v2_ptds");
    cuGLMapBufferObject_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DWithAttributesAsync_ptsz() {
    let cuMemcpy3DWithAttributesAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DWithAttributesAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpy3DWithAttributesAsync_ptsz");
    cuMemcpy3DWithAttributesAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayCreate() {
    let cuArrayCreate: extern "C" fn() = std::mem::transmute(get_sym("cuArrayCreate"));
    eprintln!("[CALL] {}", "cuArrayCreate");
    cuArrayCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetDevice_ptsz() {
    let cuStreamGetDevice_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetDevice_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetDevice_ptsz");
    cuStreamGetDevice_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoHAsync() {
    let cuMemcpyDtoHAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoHAsync"));
    eprintln!("[CALL] {}", "cuMemcpyDtoHAsync");
    cuMemcpyDtoHAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuSubgridCreate() {
    let cuSubgridCreate: extern "C" fn() = std::mem::transmute(get_sym("cuSubgridCreate"));
    eprintln!("[CALL] {}", "cuSubgridCreate");
    cuSubgridCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D8Async_ptsz() {
    let cuMemsetD2D8Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D8Async_ptsz"));
    eprintln!("[CALL] {}", "cuMemsetD2D8Async_ptsz");
    cuMemsetD2D8Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DPeerAsync_ptsz() {
    let cuMemcpy3DPeerAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DPeerAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpy3DPeerAsync_ptsz");
    cuMemcpy3DPeerAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObject() {
    let cuGLMapBufferObject: extern "C" fn() = std::mem::transmute(get_sym("cuGLMapBufferObject"));
    eprintln!("[CALL] {}", "cuGLMapBufferObject");
    cuGLMapBufferObject()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgApiInit() {
    let cudbgApiInit: extern "C" fn() = std::mem::transmute(get_sym("cudbgApiInit"));
    eprintln!("[CALL] {}", "cudbgApiInit");
    cudbgApiInit()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyBatchAsync_ptsz() {
    let cuMemcpyBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyBatchAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyBatchAsync_ptsz");
    cuMemcpyBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32_v2_ptds() {
    let cuMemsetD32_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD32_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemsetD32_v2_ptds");
    cuMemsetD32_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuDriverGetGpuCodeIsaVersion() {
    let cuDriverGetGpuCodeIsaVersion: extern "C" fn() =
        std::mem::transmute(get_sym("cuDriverGetGpuCodeIsaVersion"));
    eprintln!("[CALL] {}", "cuDriverGetGpuCodeIsaVersion");
    cuDriverGetGpuCodeIsaVersion()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostGetDevicePointer() {
    let cuMemHostGetDevicePointer: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemHostGetDevicePointer"));
    eprintln!("[CALL] {}", "cuMemHostGetDevicePointer");
    cuMemHostGetDevicePointer()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgGetAPIVersion() {
    let cudbgGetAPIVersion: extern "C" fn() = std::mem::transmute(get_sym("cudbgGetAPIVersion"));
    eprintln!("[CALL] {}", "cudbgGetAPIVersion");
    cudbgGetAPIVersion()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue64_v2_ptsz() {
    let cuStreamWaitValue64_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitValue64_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWaitValue64_v2_ptsz");
    cuStreamWaitValue64_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchHostFunc_v2_ptsz() {
    let cuLaunchHostFunc_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuLaunchHostFunc_v2_ptsz"));
    eprintln!("[CALL] {}", "cuLaunchHostFunc_v2_ptsz");
    cuLaunchHostFunc_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetPriority_ptsz() {
    let cuStreamGetPriority_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetPriority_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetPriority_ptsz");
    cuStreamGetPriority_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoH_v2_ptds() {
    let cuMemcpyDtoH_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoH_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyDtoH_v2_ptds");
    cuMemcpyDtoH_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3D_v2_ptds() {
    let cuMemcpy3D_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy3D_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpy3D_v2_ptds");
    cuMemcpy3D_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2D_v2_ptds() {
    let cuMemcpy2D_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy2D_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpy2D_v2_ptds");
    cuMemcpy2D_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLGetDevices_v2() {
    let cuGLGetDevices_v2: extern "C" fn() = std::mem::transmute(get_sym("cuGLGetDevices_v2"));
    eprintln!("[CALL] {}", "cuGLGetDevices_v2");
    cuGLGetDevices_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoDAsync_v2_ptsz() {
    let cuMemcpyHtoDAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyHtoDAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyHtoDAsync_v2_ptsz");
    cuMemcpyHtoDAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemMapArrayAsync_ptsz() {
    let cuMemMapArrayAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemMapArrayAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemMapArrayAsync_ptsz");
    cuMemMapArrayAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgApiDetach() {
    let cudbgApiDetach: extern "C" fn() = std::mem::transmute(get_sym("cudbgApiDetach"));
    eprintln!("[CALL] {}", "cudbgApiDetach");
    cudbgApiDetach()
}
#[no_mangle]
pub unsafe extern "C" fn cuSubgridWorksetCreate() {
    let cuSubgridWorksetCreate: extern "C" fn() =
        std::mem::transmute(get_sym("cuSubgridWorksetCreate"));
    eprintln!("[CALL] {}", "cuSubgridWorksetCreate");
    cuSubgridWorksetCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoDAsync() {
    let cuMemcpyHtoDAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyHtoDAsync"));
    eprintln!("[CALL] {}", "cuMemcpyHtoDAsync");
    cuMemcpyHtoDAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuArray3DGetDescriptor() {
    let cuArray3DGetDescriptor: extern "C" fn() =
        std::mem::transmute(get_sym("cuArray3DGetDescriptor"));
    eprintln!("[CALL] {}", "cuArray3DGetDescriptor");
    cuArray3DGetDescriptor()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLInit() {
    let cuGLInit: extern "C" fn() = std::mem::transmute(get_sym("cuGLInit"));
    eprintln!("[CALL] {}", "cuGLInit");
    cuGLInit()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchAsync() {
    let cuMemPrefetchAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemPrefetchAsync"));
    eprintln!("[CALL] {}", "cuMemPrefetchAsync");
    cuMemPrefetchAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamIsCapturing_ptsz() {
    let cuStreamIsCapturing_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamIsCapturing_ptsz"));
    eprintln!("[CALL] {}", "cuStreamIsCapturing_ptsz");
    cuStreamIsCapturing_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamQuery_ptsz() {
    let cuStreamQuery_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuStreamQuery_ptsz"));
    eprintln!("[CALL] {}", "cuStreamQuery_ptsz");
    cuStreamQuery_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_v3_ptsz() {
    let cuStreamGetCaptureInfo_v3_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo_v3_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetCaptureInfo_v3_ptsz");
    cuStreamGetCaptureInfo_v3_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgPreInit() {
    let cudbgPreInit: extern "C" fn() = std::mem::transmute(get_sym("cudbgPreInit"));
    eprintln!("[CALL] {}", "cudbgPreInit");
    cudbgPreInit()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocPitch() {
    let cuMemAllocPitch: extern "C" fn() = std::mem::transmute(get_sym("cuMemAllocPitch"));
    eprintln!("[CALL] {}", "cuMemAllocPitch");
    cuMemAllocPitch()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamSynchronize_ptsz() {
    let cuStreamSynchronize_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamSynchronize_ptsz"));
    eprintln!("[CALL] {}", "cuStreamSynchronize_ptsz");
    cuStreamSynchronize_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCapture_v2_ptsz() {
    let cuStreamBeginCapture_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCapture_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamBeginCapture_v2_ptsz");
    cuStreamBeginCapture_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemDiscardBatchAsync_ptsz() {
    let cuMemDiscardBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemDiscardBatchAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemDiscardBatchAsync_ptsz");
    cuMemDiscardBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D8() {
    let cuMemsetD2D8: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD2D8"));
    eprintln!("[CALL] {}", "cuMemsetD2D8");
    cuMemsetD2D8()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue32_v2_ptsz() {
    let cuStreamWaitValue32_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitValue32_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWaitValue32_v2_ptsz");
    cuStreamWaitValue32_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventRecordWithFlags_ptsz() {
    let cuEventRecordWithFlags_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuEventRecordWithFlags_ptsz"));
    eprintln!("[CALL] {}", "cuEventRecordWithFlags_ptsz");
    cuEventRecordWithFlags_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyPeerAsync_ptsz() {
    let cuMemcpyPeerAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyPeerAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyPeerAsync_ptsz");
    cuMemcpyPeerAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyBatchAsync_v2_ptsz() {
    let cuMemcpyBatchAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyBatchAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyBatchAsync_v2_ptsz");
    cuMemcpyBatchAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchAsync_v2_ptsz() {
    let cuMemPrefetchAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemPrefetchAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemPrefetchAsync_v2_ptsz");
    cuMemPrefetchAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D32_v2_ptds() {
    let cuMemsetD2D32_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D32_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemsetD2D32_v2_ptds");
    cuMemsetD2D32_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLUnregisterBufferObject() {
    let cuGLUnregisterBufferObject: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLUnregisterBufferObject"));
    eprintln!("[CALL] {}", "cuGLUnregisterBufferObject");
    cuGLUnregisterBufferObject()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetDevResource_ptsz() {
    let cuStreamGetDevResource_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetDevResource_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetDevResource_ptsz");
    cuStreamGetDevResource_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32Async_ptsz() {
    let cuMemsetD32Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD32Async_ptsz"));
    eprintln!("[CALL] {}", "cuMemsetD32Async_ptsz");
    cuMemsetD32Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsUnmapResources_ptsz() {
    let cuGraphicsUnmapResources_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsUnmapResources_ptsz"));
    eprintln!("[CALL] {}", "cuGraphicsUnmapResources_ptsz");
    cuGraphicsUnmapResources_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_v2_ptsz() {
    let cuStreamGetCaptureInfo_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetCaptureInfo_v2_ptsz");
    cuStreamGetCaptureInfo_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventElapsedTime() {
    let cuEventElapsedTime: extern "C" fn() = std::mem::transmute(get_sym("cuEventElapsedTime"));
    eprintln!("[CALL] {}", "cuEventElapsedTime");
    cuEventElapsedTime()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue64_ptsz() {
    let cuStreamWriteValue64_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue64_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWriteValue64_ptsz");
    cuStreamWriteValue64_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetDependencies() {
    let cuGraphNodeGetDependencies: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphNodeGetDependencies"));
    eprintln!("[CALL] {}", "cuGraphNodeGetDependencies");
    cuGraphNodeGetDependencies()
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkCreate() {
    let cuLinkCreate: extern "C" fn() = std::mem::transmute(get_sym("cuLinkCreate"));
    eprintln!("[CALL] {}", "cuLinkCreate");
    cuLinkCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchHostFunc_ptsz() {
    let cuLaunchHostFunc_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuLaunchHostFunc_ptsz"));
    eprintln!("[CALL] {}", "cuLaunchHostFunc_ptsz");
    cuLaunchHostFunc_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuProfilerStop() {
    let cuProfilerStop: extern "C" fn() = std::mem::transmute(get_sym("cuProfilerStop"));
    eprintln!("[CALL] {}", "cuProfilerStop");
    cuProfilerStop()
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddress() {
    let cuTexRefSetAddress: extern "C" fn() = std::mem::transmute(get_sym("cuTexRefSetAddress"));
    eprintln!("[CALL] {}", "cuTexRefSetAddress");
    cuTexRefSetAddress()
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetAddress() {
    let cuTexRefGetAddress: extern "C" fn() = std::mem::transmute(get_sym("cuTexRefGetAddress"));
    eprintln!("[CALL] {}", "cuTexRefGetAddress");
    cuTexRefGetAddress()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxDestroy() {
    let cuCtxDestroy: extern "C" fn() = std::mem::transmute(get_sym("cuCtxDestroy"));
    eprintln!("[CALL] {}", "cuCtxDestroy");
    cuCtxDestroy()
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetUuid() {
    let cuDeviceGetUuid: extern "C" fn() = std::mem::transmute(get_sym("cuDeviceGetUuid"));
    eprintln!("[CALL] {}", "cuDeviceGetUuid");
    cuDeviceGetUuid()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBatchMemOp_ptsz() {
    let cuStreamBatchMemOp_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBatchMemOp_ptsz"));
    eprintln!("[CALL] {}", "cuStreamBatchMemOp_ptsz");
    cuStreamBatchMemOp_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphInstantiate() {
    let cuGraphInstantiate: extern "C" fn() = std::mem::transmute(get_sym("cuGraphInstantiate"));
    eprintln!("[CALL] {}", "cuGraphInstantiate");
    cuGraphInstantiate()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8_v2_ptds() {
    let cuMemsetD8_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD8_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemsetD8_v2_ptds");
    cuMemsetD8_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCtx_v2_ptsz() {
    let cuStreamGetCtx_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCtx_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetCtx_v2_ptsz");
    cuStreamGetCtx_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuVDPAUGetDevice() {
    let cuVDPAUGetDevice: extern "C" fn() = std::mem::transmute(get_sym("cuVDPAUGetDevice"));
    eprintln!("[CALL] {}", "cuVDPAUGetDevice");
    cuVDPAUGetDevice()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamAddCallback_ptsz() {
    let cuStreamAddCallback_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamAddCallback_ptsz"));
    eprintln!("[CALL] {}", "cuStreamAddCallback_ptsz");
    cuStreamAddCallback_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D16() {
    let cuMemsetD2D16: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD2D16"));
    eprintln!("[CALL] {}", "cuMemsetD2D16");
    cuMemsetD2D16()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventRecord_ptsz() {
    let cuEventRecord_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuEventRecord_ptsz"));
    eprintln!("[CALL] {}", "cuEventRecord_ptsz");
    cuEventRecord_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerDisconnect() {
    let cuEGLStreamConsumerDisconnect: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerDisconnect"));
    eprintln!("[CALL] {}", "cuEGLStreamConsumerDisconnect");
    cuEGLStreamConsumerDisconnect()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamSetAttribute_ptsz() {
    let cuStreamSetAttribute_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamSetAttribute_ptsz"));
    eprintln!("[CALL] {}", "cuStreamSetAttribute_ptsz");
    cuStreamSetAttribute_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuVDPAUCtxCreate_v2() {
    let cuVDPAUCtxCreate_v2: extern "C" fn() = std::mem::transmute(get_sym("cuVDPAUCtxCreate_v2"));
    eprintln!("[CALL] {}", "cuVDPAUCtxCreate_v2");
    cuVDPAUCtxCreate_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxPushCurrent() {
    let cuCtxPushCurrent: extern "C" fn() = std::mem::transmute(get_sym("cuCtxPushCurrent"));
    eprintln!("[CALL] {}", "cuCtxPushCurrent");
    cuCtxPushCurrent()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphInstantiate_v2() {
    let cuGraphInstantiate_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphInstantiate_v2"));
    eprintln!("[CALL] {}", "cuGraphInstantiate_v2");
    cuGraphInstantiate_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxSetFlags() {
    let cuDevicePrimaryCtxSetFlags: extern "C" fn() =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxSetFlags"));
    eprintln!("[CALL] {}", "cuDevicePrimaryCtxSetFlags");
    cuDevicePrimaryCtxSetFlags()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DPeer_ptds() {
    let cuMemcpy3DPeer_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy3DPeer_ptds"));
    eprintln!("[CALL] {}", "cuMemcpy3DPeer_ptds");
    cuMemcpy3DPeer_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoD_v2_ptds() {
    let cuMemcpyDtoD_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoD_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemcpyDtoD_v2_ptds");
    cuMemcpyDtoD_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamProducerPresentFrame() {
    let cuEGLStreamProducerPresentFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamProducerPresentFrame"));
    eprintln!("[CALL] {}", "cuEGLStreamProducerPresentFrame");
    cuEGLStreamProducerPresentFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCapture_ptsz() {
    let cuStreamBeginCapture_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCapture_ptsz"));
    eprintln!("[CALL] {}", "cuStreamBeginCapture_ptsz");
    cuStreamBeginCapture_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddNode() {
    let cuGraphAddNode: extern "C" fn() = std::mem::transmute(get_sym("cuGraphAddNode"));
    eprintln!("[CALL] {}", "cuGraphAddNode");
    cuGraphAddNode()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DAsync_v2_ptsz() {
    let cuMemcpy3DAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpy3DAsync_v2_ptsz");
    cuMemcpy3DAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFreeAsync_ptsz() {
    let cuMemFreeAsync_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuMemFreeAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemFreeAsync_ptsz");
    cuMemFreeAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D8_v2_ptds() {
    let cuMemsetD2D8_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D8_v2_ptds"));
    eprintln!("[CALL] {}", "cuMemsetD2D8_v2_ptds");
    cuMemsetD2D8_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLCtxCreate_v2() {
    let cuGLCtxCreate_v2: extern "C" fn() = std::mem::transmute(get_sym("cuGLCtxCreate_v2"));
    eprintln!("[CALL] {}", "cuGLCtxCreate_v2");
    cuGLCtxCreate_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamUpdateCaptureDependencies_v2_ptsz() {
    let cuStreamUpdateCaptureDependencies_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamUpdateCaptureDependencies_v2_ptsz"));
    eprintln!("[CALL] {}", "cuStreamUpdateCaptureDependencies_v2_ptsz");
    cuStreamUpdateCaptureDependencies_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoDAsync() {
    let cuMemcpyDtoDAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoDAsync"));
    eprintln!("[CALL] {}", "cuMemcpyDtoDAsync");
    cuMemcpyDtoDAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerConnect() {
    let cuEGLStreamConsumerConnect: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerConnect"));
    eprintln!("[CALL] {}", "cuEGLStreamConsumerConnect");
    cuEGLStreamConsumerConnect()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchKernel_ptsz() {
    let cuLaunchKernel_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuLaunchKernel_ptsz"));
    eprintln!("[CALL] {}", "cuLaunchKernel_ptsz");
    cuLaunchKernel_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyBatchAsync() {
    let cuMemcpyBatchAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyBatchAsync"));
    eprintln!("[CALL] {}", "cuMemcpyBatchAsync");
    cuMemcpyBatchAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuProfilerStart() {
    let cuProfilerStart: extern "C" fn() = std::mem::transmute(get_sym("cuProfilerStart"));
    eprintln!("[CALL] {}", "cuProfilerStart");
    cuProfilerStart()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoAAsync() {
    let cuMemcpyHtoAAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyHtoAAsync"));
    eprintln!("[CALL] {}", "cuMemcpyHtoAAsync");
    cuMemcpyHtoAAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue32_ptsz() {
    let cuStreamWaitValue32_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitValue32_ptsz"));
    eprintln!("[CALL] {}", "cuStreamWaitValue32_ptsz");
    cuStreamWaitValue32_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxPopCurrent() {
    let cuCtxPopCurrent: extern "C" fn() = std::mem::transmute(get_sym("cuCtxPopCurrent"));
    eprintln!("[CALL] {}", "cuCtxPopCurrent");
    cuCtxPopCurrent()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemDiscardAndPrefetchBatchAsync_ptsz() {
    let cuMemDiscardAndPrefetchBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemDiscardAndPrefetchBatchAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemDiscardAndPrefetchBatchAsync_ptsz");
    cuMemDiscardAndPrefetchBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceGetMappedEglFrame() {
    let cuGraphicsResourceGetMappedEglFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsResourceGetMappedEglFrame"));
    eprintln!("[CALL] {}", "cuGraphicsResourceGetMappedEglFrame");
    cuGraphicsResourceGetMappedEglFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D32() {
    let cuMemsetD2D32: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD2D32"));
    eprintln!("[CALL] {}", "cuMemsetD2D32");
    cuMemsetD2D32()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCaptureToCig_ptsz() {
    let cuStreamBeginCaptureToCig_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCaptureToCig_ptsz"));
    eprintln!("[CALL] {}", "cuStreamBeginCaptureToCig_ptsz");
    cuStreamBeginCaptureToCig_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddDependencies() {
    let cuGraphAddDependencies: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphAddDependencies"));
    eprintln!("[CALL] {}", "cuGraphAddDependencies");
    cuGraphAddDependencies()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DBatchAsync_v2_ptsz() {
    let cuMemcpy3DBatchAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DBatchAsync_v2_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpy3DBatchAsync_v2_ptsz");
    cuMemcpy3DBatchAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyWithAttributesAsync_ptsz() {
    let cuMemcpyWithAttributesAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyWithAttributesAsync_ptsz"));
    eprintln!("[CALL] {}", "cuMemcpyWithAttributesAsync_ptsz");
    cuMemcpyWithAttributesAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddKernelNode() {
    let cuGraphAddKernelNode: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphAddKernelNode"));
    eprintln!("[CALL] {}", "cuGraphAddKernelNode");
    cuGraphAddKernelNode()
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceTotalMem() {
    let cuDeviceTotalMem: extern "C" fn() = std::mem::transmute(get_sym("cuDeviceTotalMem"));
    eprintln!("[CALL] {}", "cuDeviceTotalMem");
    cuDeviceTotalMem()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD16Async_ptsz() {
    let cuMemsetD16Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD16Async_ptsz"));
    eprintln!("[CALL] {}", "cuMemsetD16Async_ptsz");
    cuMemsetD16Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsEGLRegisterImage() {
    let cuGraphicsEGLRegisterImage: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsEGLRegisterImage"));
    eprintln!("[CALL] {}", "cuGraphicsEGLRegisterImage");
    cuGraphicsEGLRegisterImage()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetAddressRange() {
    let cuMemGetAddressRange: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemGetAddressRange"));
    eprintln!("[CALL] {}", "cuMemGetAddressRange");
    cuMemGetAddressRange()
}
#[no_mangle]
pub unsafe extern "C" fn cuArray3DCreate() {
    let cuArray3DCreate: extern "C" fn() = std::mem::transmute(get_sym("cuArray3DCreate"));
    eprintln!("[CALL] {}", "cuArray3DCreate");
    cuArray3DCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcOpenMemHandle() {
    let cuIpcOpenMemHandle: extern "C" fn() = std::mem::transmute(get_sym("cuIpcOpenMemHandle"));
    eprintln!("[CALL] {}", "cuIpcOpenMemHandle");
    cuIpcOpenMemHandle()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxCreate() {
    let cuCtxCreate: extern "C" fn() = std::mem::transmute(get_sym("cuCtxCreate"));
    eprintln!("[CALL] {}", "cuCtxCreate");
    cuCtxCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamCopyAttributes_ptsz() {
    let cuStreamCopyAttributes_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamCopyAttributes_ptsz"));
    eprintln!("[CALL] {}", "cuStreamCopyAttributes_ptsz");
    cuStreamCopyAttributes_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeGetParams() {
    let cuGraphKernelNodeGetParams: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphKernelNodeGetParams"));
    eprintln!("[CALL] {}", "cuGraphKernelNodeGetParams");
    cuGraphKernelNodeGetParams()
}
#[no_mangle]
pub unsafe extern "C" fn cuSubgridWorkerGridCreate() {
    let cuSubgridWorkerGridCreate: extern "C" fn() =
        std::mem::transmute(get_sym("cuSubgridWorkerGridCreate"));
    eprintln!("[CALL] {}", "cuSubgridWorkerGridCreate");
    cuSubgridWorkerGridCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetFlags_ptsz() {
    let cuStreamGetFlags_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetFlags_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetFlags_ptsz");
    cuStreamGetFlags_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DUnaligned() {
    let cuMemcpy2DUnaligned: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy2DUnaligned"));
    eprintln!("[CALL] {}", "cuMemcpy2DUnaligned");
    cuMemcpy2DUnaligned()
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxReset() {
    let cuDevicePrimaryCtxReset: extern "C" fn() =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxReset"));
    eprintln!("[CALL] {}", "cuDevicePrimaryCtxReset");
    cuDevicePrimaryCtxReset()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAdvise() {
    let cuMemAdvise: extern "C" fn() = std::mem::transmute(get_sym("cuMemAdvise"));
    eprintln!("[CALL] {}", "cuMemAdvise");
    cuMemAdvise()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetAttribute_ptsz() {
    let cuStreamGetAttribute_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetAttribute_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetAttribute_ptsz");
    cuStreamGetAttribute_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetId_ptsz() {
    let cuStreamGetId_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuStreamGetId_ptsz"));
    eprintln!("[CALL] {}", "cuStreamGetId_ptsz");
    cuStreamGetId_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuWaitExternalSemaphoresAsync_ptsz() {
    let cuWaitExternalSemaphoresAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuWaitExternalSemaphoresAsync_ptsz"));
    eprintln!("[CALL] {}", "cuWaitExternalSemaphoresAsync_ptsz");
    cuWaitExternalSemaphoresAsync_ptsz()
}
