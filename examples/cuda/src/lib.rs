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
    log::debug!("[CALL] {}", "cuCtxGetFlags");
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
    log::debug!("[CALL] {}", "cuMemcpyAsync");
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
    log::debug!("[CALL] {}", "cuMemcpyDtoA_v2");
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
    log::debug!("[CALL] {}", "cuFuncGetAttribute");
    cuFuncGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetCacheConfig(arg0: CUfunc_cache) -> CUresult {
    let cuCtxSetCacheConfig: extern "C" fn(arg0: CUfunc_cache) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetCacheConfig"));
    log::debug!("[CALL] {}", "cuCtxSetCacheConfig");
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
    log::debug!("[CALL] {}", "cuMemsetD32Async");
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
    log::debug!("[CALL] {}", "cuPointerSetAttribute");
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
    log::debug!("[CALL] {}", "cuGraphNodeGetToolsId");
    cuGraphNodeGetToolsId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDestroyExternalSemaphore(arg0: CUexternalSemaphore) -> CUresult {
    let cuDestroyExternalSemaphore: extern "C" fn(arg0: CUexternalSemaphore) -> CUresult =
        std::mem::transmute(get_sym("cuDestroyExternalSemaphore"));
    log::debug!("[CALL] {}", "cuDestroyExternalSemaphore");
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
    log::debug!("[CALL] {}", "cuLibraryLoadFromFile");
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
    log::debug!("[CALL] {}", "cuDeviceCanAccessPeer");
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
    log::debug!("[CALL] {}", "cuGraphAddExternalSemaphoresWaitNode");
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
    log::debug!("[CALL] {}", "cuOccupancyMaxActiveClusters");
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
    log::debug!("[CALL] {}", "cuGraphExecUpdate_v2");
    cuGraphExecUpdate_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuThreadExchangeStreamCaptureMode(
    arg0: *mut CUstreamCaptureMode,
) -> CUresult {
    let cuThreadExchangeStreamCaptureMode: extern "C" fn(
        arg0: *mut CUstreamCaptureMode,
    ) -> CUresult = std::mem::transmute(get_sym("cuThreadExchangeStreamCaptureMode"));
    log::debug!("[CALL] {}", "cuThreadExchangeStreamCaptureMode");
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
    log::debug!("[CALL] {}", "cuFuncSetBlockShape");
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
    log::debug!("[CALL] {}", "cuGraphAddEventRecordNode");
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
    log::debug!("[CALL] {}", "cuLaunchCooperativeKernelMultiDevice");
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
    log::debug!("[CALL] {}", "cuTensorMapEncodeTiled");
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
    log::debug!("[CALL] {}", "cuCtxGetExecAffinity");
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
    log::debug!("[CALL] {}", "cuGraphicsResourceGetMappedPointer_v2");
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
    log::debug!("[CALL] {}", "cuGraphExecChildGraphNodeSetParams");
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
    log::debug!("[CALL] {}", "cuFuncGetParamInfo");
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
    log::debug!("[CALL] {}", "cuMemPoolExportToShareableHandle");
    cuMemPoolExportToShareableHandle(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetLoadingMode(arg0: *mut CUmoduleLoadingMode) -> CUresult {
    let cuModuleGetLoadingMode: extern "C" fn(arg0: *mut CUmoduleLoadingMode) -> CUresult =
        std::mem::transmute(get_sym("cuModuleGetLoadingMode"));
    log::debug!("[CALL] {}", "cuModuleGetLoadingMode");
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
    log::debug!("[CALL] {}", "cuSurfObjectGetResourceDesc");
    cuSurfObjectGetResourceDesc(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemUnmap(arg0: CUdeviceptr, arg1: usize) -> CUresult {
    let cuMemUnmap: extern "C" fn(arg0: CUdeviceptr, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemUnmap"));
    log::debug!("[CALL] {}", "cuMemUnmap");
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
    log::debug!("[CALL] {}", "cuLibraryGetKernel");
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
    log::debug!("[CALL] {}", "cuTexObjectGetResourceDesc");
    cuTexObjectGetResourceDesc(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxRecordEvent(arg0: CUgreenCtx, arg1: CUevent) -> CUresult {
    let cuGreenCtxRecordEvent: extern "C" fn(arg0: CUgreenCtx, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuGreenCtxRecordEvent"));
    log::debug!("[CALL] {}", "cuGreenCtxRecordEvent");
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
    log::debug!("[CALL] {}", "cuModuleEnumerateFunctions");
    cuModuleEnumerateFunctions(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetInfo_v2(arg0: *mut usize, arg1: *mut usize) -> CUresult {
    let cuMemGetInfo_v2: extern "C" fn(arg0: *mut usize, arg1: *mut usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemGetInfo_v2"));
    log::debug!("[CALL] {}", "cuMemGetInfo_v2");
    cuMemGetInfo_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetBorderColor(arg0: CUtexref, arg1: *mut f32) -> CUresult {
    let cuTexRefSetBorderColor: extern "C" fn(arg0: CUtexref, arg1: *mut f32) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetBorderColor"));
    log::debug!("[CALL] {}", "cuTexRefSetBorderColor");
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
    log::debug!("[CALL] {}", "cuMemPoolGetAttribute");
    cuMemPoolGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetName(arg0: *mut *const c_char, arg1: CUkernel) -> CUresult {
    let cuKernelGetName: extern "C" fn(arg0: *mut *const c_char, arg1: CUkernel) -> CUresult =
        std::mem::transmute(get_sym("cuKernelGetName"));
    log::debug!("[CALL] {}", "cuKernelGetName");
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
    log::debug!("[CALL] {}", "cuMipmappedArrayGetLevel");
    cuMipmappedArrayGetLevel(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCtx(arg0: CUstream, arg1: *mut CUcontext) -> CUresult {
    let cuStreamGetCtx: extern "C" fn(arg0: CUstream, arg1: *mut CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetCtx"));
    log::debug!("[CALL] {}", "cuStreamGetCtx");
    cuStreamGetCtx(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetSharedMemConfig(arg0: *mut CUsharedconfig) -> CUresult {
    let cuCtxGetSharedMemConfig: extern "C" fn(arg0: *mut CUsharedconfig) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetSharedMemConfig"));
    log::debug!("[CALL] {}", "cuCtxGetSharedMemConfig");
    cuCtxGetSharedMemConfig(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventQuery(arg0: CUevent) -> CUresult {
    let cuEventQuery: extern "C" fn(arg0: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuEventQuery"));
    log::debug!("[CALL] {}", "cuEventQuery");
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
    log::debug!("[CALL] {}", "cuGraphBatchMemOpNodeSetParams");
    cuGraphBatchMemOpNodeSetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphDestroy(arg0: CUgraph) -> CUresult {
    let cuGraphDestroy: extern "C" fn(arg0: CUgraph) -> CUresult =
        std::mem::transmute(get_sym("cuGraphDestroy"));
    log::debug!("[CALL] {}", "cuGraphDestroy");
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
    log::debug!("[CALL] {}", "cuSignalExternalSemaphoresAsync");
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
    log::debug!("[CALL] {}", "cuStreamBatchMemOp_v2");
    cuStreamBatchMemOp_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexObjectDestroy(arg0: CUtexObject) -> CUresult {
    let cuTexObjectDestroy: extern "C" fn(arg0: CUtexObject) -> CUresult =
        std::mem::transmute(get_sym("cuTexObjectDestroy"));
    log::debug!("[CALL] {}", "cuTexObjectDestroy");
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
    log::debug!("[CALL] {}", "cuMemRangeGetAttributes");
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
    log::debug!("[CALL] {}", "cuTexRefSetAddressMode");
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
    log::debug!("[CALL] {}", "cuModuleGetSurfRef");
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
    log::debug!("[CALL] {}", "cuGraphNodeGetEnabled");
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
    log::debug!("[CALL] {}", "cuGraphNodeSetEnabled");
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
    log::debug!("[CALL] {}", "cuGraphNodeGetDependentNodes_v2");
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
    log::debug!("[CALL] {}", "cuImportExternalMemory");
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
    log::debug!("[CALL] {}", "cuGraphicsSubResourceGetMappedArray");
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
    log::debug!("[CALL] {}", "cuArray3DCreate_v2");
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
    log::debug!("[CALL] {}", "cuWaitExternalSemaphoresAsync");
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
    log::debug!("[CALL] {}", "cuGraphAddMemFreeNode");
    cuGraphAddMemFreeNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSynchronize() -> CUresult {
    let cuCtxSynchronize: extern "C" fn() -> CUresult =
        std::mem::transmute(get_sym("cuCtxSynchronize"));
    log::debug!("[CALL] {}", "cuCtxSynchronize");
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
    log::debug!("[CALL] {}", "cuGraphicsResourceSetMapFlags_v2");
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
    log::debug!("[CALL] {}", "cuMipmappedArrayGetMemoryRequirements");
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
    log::debug!("[CALL] {}", "cuGraphExternalSemaphoresSignalNodeSetParams");
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
    log::debug!("[CALL] {}", "cuLinkAddFile_v2");
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
    log::debug!("[CALL] {}", "cuTexRefSetMipmapLevelClamp");
    cuTexRefSetMipmapLevelClamp(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDriverGetVersion(version: *mut c_int) -> CUresult {
    log::debug!("[CALL] {}", "cuDriverGetVersion");
    *version = 13200;
    CUDA_SUCCESS
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamCopyAttributes(arg0: CUstream, arg1: CUstream) -> CUresult {
    let cuStreamCopyAttributes: extern "C" fn(arg0: CUstream, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamCopyAttributes"));
    log::debug!("[CALL] {}", "cuStreamCopyAttributes");
    cuStreamCopyAttributes(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncGetName(arg0: *mut *const c_char, arg1: CUfunction) -> CUresult {
    let cuFuncGetName: extern "C" fn(arg0: *mut *const c_char, arg1: CUfunction) -> CUresult =
        std::mem::transmute(get_sym("cuFuncGetName"));
    log::debug!("[CALL] {}", "cuFuncGetName");
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
    log::debug!("[CALL] {}", "cuCheckpointProcessLock");
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
    log::debug!("[CALL] {}", "cuGraphAddHostNode");
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
    log::debug!("[CALL] {}", "cuStreamBeginCaptureToCig");
    cuStreamBeginCaptureToCig(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSynchronize_v2(arg0: CUcontext) -> CUresult {
    let cuCtxSynchronize_v2: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSynchronize_v2"));
    log::debug!("[CALL] {}", "cuCtxSynchronize_v2");
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
    log::debug!("[CALL] {}", "cuMulticastBindAddr");
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
    log::debug!("[CALL] {}", "cuGraphAddMemAllocNode");
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
    log::debug!("[CALL] {}", "cuDeviceGetP2PAttribute");
    cuDeviceGetP2PAttribute(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetId(arg0: CUstream, arg1: *mut c_ulonglong) -> CUresult {
    let cuStreamGetId: extern "C" fn(arg0: CUstream, arg1: *mut c_ulonglong) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetId"));
    log::debug!("[CALL] {}", "cuStreamGetId");
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
    log::debug!("[CALL] {}", "cuMemsetD16Async");
    cuMemsetD16Async(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxResetPersistingL2Cache() -> CUresult {
    let cuCtxResetPersistingL2Cache: extern "C" fn() -> CUresult =
        std::mem::transmute(get_sym("cuCtxResetPersistingL2Cache"));
    log::debug!("[CALL] {}", "cuCtxResetPersistingL2Cache");
    cuCtxResetPersistingL2Cache()
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetCount(device_count: *mut c_int) -> CUresult {
    log::debug!("[CALL] {}", "cuDeviceGetCount");
    *device_count = 4;
    CUDA_SUCCESS
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
    log::debug!("[CALL] {}", "cuGraphExecKernelNodeSetParams_v2");
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
    log::debug!("[CALL] {}", "cuCheckpointProcessCheckpoint");
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
    log::debug!("[CALL] {}", "cuMemsetD16_v2");
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
    log::debug!("[CALL] {}", "cuMemPoolCreate");
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
    log::debug!("[CALL] {}", "cuCtxCreate_v4");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointBindMem");
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
    log::debug!("[CALL] {}", "cuGraphReleaseUserObject");
    cuGraphReleaseUserObject(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecDestroy(arg0: CUgraphExec) -> CUresult {
    let cuGraphExecDestroy: extern "C" fn(arg0: CUgraphExec) -> CUresult =
        std::mem::transmute(get_sym("cuGraphExecDestroy"));
    log::debug!("[CALL] {}", "cuGraphExecDestroy");
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
    log::debug!("[CALL] {}", "cuGraphicsUnmapResources");
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
    log::debug!("[CALL] {}", "cuEventElapsedTime_v2");
    cuEventElapsedTime_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecGetFlags(arg0: CUgraphExec, arg1: *mut cuuint64_t) -> CUresult {
    let cuGraphExecGetFlags: extern "C" fn(arg0: CUgraphExec, arg1: *mut cuuint64_t) -> CUresult =
        std::mem::transmute(get_sym("cuGraphExecGetFlags"));
    log::debug!("[CALL] {}", "cuGraphExecGetFlags");
    cuGraphExecGetFlags(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxAttach(arg0: *mut CUcontext, arg1: c_uint) -> CUresult {
    let cuCtxAttach: extern "C" fn(arg0: *mut CUcontext, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxAttach"));
    log::debug!("[CALL] {}", "cuCtxAttach");
    cuCtxAttach(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxRelease_v2(arg0: CUdevice) -> CUresult {
    let cuDevicePrimaryCtxRelease_v2: extern "C" fn(arg0: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxRelease_v2"));
    log::debug!("[CALL] {}", "cuDevicePrimaryCtxRelease_v2");
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
    log::debug!("[CALL] {}", "cuGraphNodeGetContainingGraph");
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
    log::debug!("[CALL] {}", "cuTexRefGetMipmappedArray");
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
    log::debug!("[CALL] {}", "cuLaunchCooperativeKernel");
    cuLaunchCooperativeKernel(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetMaxAnisotropy(arg0: *mut c_int, arg1: CUtexref) -> CUresult {
    let cuTexRefGetMaxAnisotropy: extern "C" fn(arg0: *mut c_int, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetMaxAnisotropy"));
    log::debug!("[CALL] {}", "cuTexRefGetMaxAnisotropy");
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
    log::debug!("[CALL] {}", "cuMemAddressReserve");
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
    log::debug!("[CALL] {}", "cuGraphInstantiateWithFlags");
    cuGraphInstantiateWithFlags(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocHost_v2(arg0: *mut *mut c_void, arg1: usize) -> CUresult {
    let cuMemAllocHost_v2: extern "C" fn(arg0: *mut *mut c_void, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemAllocHost_v2"));
    log::debug!("[CALL] {}", "cuMemAllocHost_v2");
    cuMemAllocHost_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxDisablePeerAccess(arg0: CUcontext) -> CUresult {
    let cuCtxDisablePeerAccess: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxDisablePeerAccess"));
    log::debug!("[CALL] {}", "cuCtxDisablePeerAccess");
    cuCtxDisablePeerAccess(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxEnablePeerAccess(arg0: CUcontext, arg1: c_uint) -> CUresult {
    let cuCtxEnablePeerAccess: extern "C" fn(arg0: CUcontext, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxEnablePeerAccess"));
    log::debug!("[CALL] {}", "cuCtxEnablePeerAccess");
    cuCtxEnablePeerAccess(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuUserObjectRetain(arg0: CUuserObject, arg1: c_uint) -> CUresult {
    let cuUserObjectRetain: extern "C" fn(arg0: CUuserObject, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuUserObjectRetain"));
    log::debug!("[CALL] {}", "cuUserObjectRetain");
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
    log::debug!("[CALL] {}", "cuDevSmResourceSplitByCount");
    cuDevSmResourceSplitByCount(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetFunctionCount(arg0: *mut c_uint, arg1: CUmodule) -> CUresult {
    let cuModuleGetFunctionCount: extern "C" fn(arg0: *mut c_uint, arg1: CUmodule) -> CUresult =
        std::mem::transmute(get_sym("cuModuleGetFunctionCount"));
    log::debug!("[CALL] {}", "cuModuleGetFunctionCount");
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
    log::debug!("[CALL] {}", "cuMulticastUnbind");
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
    log::debug!("[CALL] {}", "cuTexRefSetMipmappedArray");
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
    log::debug!("[CALL] {}", "cuMemcpyPeer");
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
    log::debug!("[CALL] {}", "cuGraphDebugDotPrint");
    cuGraphDebugDotPrint(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncSetSharedSize(arg0: CUfunction, arg1: c_uint) -> CUresult {
    let cuFuncSetSharedSize: extern "C" fn(arg0: CUfunction, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuFuncSetSharedSize"));
    log::debug!("[CALL] {}", "cuFuncSetSharedSize");
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
    log::debug!("[CALL] {}", "cuCoredumpGetAttributeGlobal");
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
    log::debug!("[CALL] {}", "cuMemPoolImportPointer");
    cuMemPoolImportPointer(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleUnload(arg0: CUmodule) -> CUresult {
    let cuModuleUnload: extern "C" fn(arg0: CUmodule) -> CUresult =
        std::mem::transmute(get_sym("cuModuleUnload"));
    log::debug!("[CALL] {}", "cuModuleUnload");
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
    log::debug!("[CALL] {}", "cuMemcpyDtoHAsync_v2");
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
    log::debug!("[CALL] {}", "cuPointerGetAttributes");
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
    log::debug!("[CALL] {}", "cuFlushGPUDirectRDMAWrites");
    cuFlushGPUDirectRDMAWrites(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetLocalId(arg0: CUgraphNode, arg1: *mut c_uint) -> CUresult {
    let cuGraphNodeGetLocalId: extern "C" fn(arg0: CUgraphNode, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuGraphNodeGetLocalId"));
    log::debug!("[CALL] {}", "cuGraphNodeGetLocalId");
    cuGraphNodeGetLocalId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DAsync_v2(
    arg0: *const CUDA_MEMCPY3D,
    arg1: CUstream,
) -> CUresult {
    let cuMemcpy3DAsync_v2: extern "C" fn(arg0: *const CUDA_MEMCPY3D, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy3DAsync_v2"));
    log::debug!("[CALL] {}", "cuMemcpy3DAsync_v2");
    cuMemcpy3DAsync_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DAsync_v2(
    arg0: *const CUDA_MEMCPY2D,
    arg1: CUstream,
) -> CUresult {
    let cuMemcpy2DAsync_v2: extern "C" fn(arg0: *const CUDA_MEMCPY2D, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy2DAsync_v2"));
    log::debug!("[CALL] {}", "cuMemcpy2DAsync_v2");
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
    log::debug!("[CALL] {}", "cuLinkAddData_v2");
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
    log::debug!("[CALL] {}", "cuSurfRefSetArray");
    cuSurfRefSetArray(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFreeHost(arg0: *mut c_void) -> CUresult {
    let cuMemFreeHost: extern "C" fn(arg0: *mut c_void) -> CUresult =
        std::mem::transmute(get_sym("cuMemFreeHost"));
    log::debug!("[CALL] {}", "cuMemFreeHost");
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
    log::debug!("[CALL] {}", "cuTexRefGetFormat");
    cuTexRefGetFormat(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAlloc_v2(arg0: *mut CUdeviceptr, arg1: usize) -> CUresult {
    let cuMemAlloc_v2: extern "C" fn(arg0: *mut CUdeviceptr, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemAlloc_v2"));
    log::debug!("[CALL] {}", "cuMemAlloc_v2");
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
    log::debug!("[CALL] {}", "cuGraphEventWaitNodeGetEvent");
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
    log::debug!("[CALL] {}", "cuMemcpyAtoH_v2");
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
    log::debug!("[CALL] {}", "cuMulticastCreate");
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
    log::debug!("[CALL] {}", "cuGraphNodeSetParams");
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
    log::debug!("[CALL] {}", "cuDeviceGetP2PAtomicCapabilities");
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
    log::debug!("[CALL] {}", "cuTexRefSetAddress2D_v3");
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
    log::debug!("[CALL] {}", "cuGraphMemcpyNodeGetParams");
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
    log::debug!("[CALL] {}", "cuDeviceGetGraphMemAttribute");
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
    log::debug!("[CALL] {}", "cuKernelSetCacheConfig");
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
    log::debug!("[CALL] {}", "cuTexObjectGetTextureDesc");
    cuTexObjectGetTextureDesc(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetGreenCtx(arg0: CUstream, arg1: *mut CUgreenCtx) -> CUresult {
    let cuStreamGetGreenCtx: extern "C" fn(arg0: CUstream, arg1: *mut CUgreenCtx) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetGreenCtx"));
    log::debug!("[CALL] {}", "cuStreamGetGreenCtx");
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
    log::debug!("[CALL] {}", "cuModuleLoadDataEx");
    cuModuleLoadDataEx(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleLoadFatBinary(
    arg0: *mut CUmodule,
    arg1: *const c_void,
) -> CUresult {
    let cuModuleLoadFatBinary: extern "C" fn(arg0: *mut CUmodule, arg1: *const c_void) -> CUresult =
        std::mem::transmute(get_sym("cuModuleLoadFatBinary"));
    log::debug!("[CALL] {}", "cuModuleLoadFatBinary");
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
    log::debug!("[CALL] {}", "cuArrayGetMemoryRequirements");
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
    log::debug!("[CALL] {}", "cuGraphAddEventWaitNode");
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
    log::debug!("[CALL] {}", "cuDeviceUnregisterAsyncNotification");
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
    log::debug!("[CALL] {}", "cuGreenCtxCreate");
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
    log::debug!("[CALL] {}", "cuPointerGetAttribute");
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
    log::debug!("[CALL] {}", "cuStreamBeginCaptureToGraph");
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
    log::debug!("[CALL] {}", "cuParamSetTexRef");
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
    log::debug!("[CALL] {}", "cuMemGetHandleForAddressRange");
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
    log::debug!("[CALL] {}", "cuGetProcAddress_v2");
    cuGetProcAddress_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcGetMemHandle(
    arg0: *mut CUipcMemHandle,
    arg1: CUdeviceptr,
) -> CUresult {
    let cuIpcGetMemHandle: extern "C" fn(arg0: *mut CUipcMemHandle, arg1: CUdeviceptr) -> CUresult =
        std::mem::transmute(get_sym("cuIpcGetMemHandle"));
    log::debug!("[CALL] {}", "cuIpcGetMemHandle");
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
    log::debug!("[CALL] {}", "cuTexObjectCreate");
    cuTexObjectCreate(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetMipmapLevelBias(arg0: CUtexref, arg1: f32) -> CUresult {
    let cuTexRefSetMipmapLevelBias: extern "C" fn(arg0: CUtexref, arg1: f32) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetMipmapLevelBias"));
    log::debug!("[CALL] {}", "cuTexRefSetMipmapLevelBias");
    cuTexRefSetMipmapLevelBias(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGetExportTable(
    arg0: *mut *const c_void,
    arg1: *const CUuuid,
) -> CUresult {
    let cuGetExportTable: extern "C" fn(arg0: *mut *const c_void, arg1: *const CUuuid) -> CUresult =
        std::mem::transmute(get_sym("cuGetExportTable"));
    log::debug!("[CALL] {}", "cuGetExportTable");
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
    log::debug!("[CALL] {}", "cuCheckpointProcessRestore");
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
    log::debug!("[CALL] {}", "cuMemcpy3DWithAttributesAsync");
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
    log::debug!("[CALL] {}", "cuCoredumpRegisterCompleteCallback");
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
    log::debug!("[CALL] {}", "cuGraphExecMemcpyNodeSetParams");
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
    log::debug!("[CALL] {}", "cuMemPoolExportPointer");
    cuMemPoolExportPointer(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGraphMemTrim(arg0: CUdevice) -> CUresult {
    let cuDeviceGraphMemTrim: extern "C" fn(arg0: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGraphMemTrim"));
    log::debug!("[CALL] {}", "cuDeviceGraphMemTrim");
    cuDeviceGraphMemTrim(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetFlags(arg0: CUstream, arg1: *mut c_uint) -> CUresult {
    let cuStreamGetFlags: extern "C" fn(arg0: CUstream, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetFlags"));
    log::debug!("[CALL] {}", "cuStreamGetFlags");
    cuStreamGetFlags(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxDetach(arg0: CUcontext) -> CUresult {
    let cuCtxDetach: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxDetach"));
    log::debug!("[CALL] {}", "cuCtxDetach");
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
    log::debug!("[CALL] {}", "cuMemsetD8Async");
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
    log::debug!("[CALL] {}", "cuGraphKernelNodeSetAttribute");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointQuery");
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
    log::debug!("[CALL] {}", "cuGraphicsResourceGetMappedMipmappedArray");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointIdRelease");
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
    log::debug!("[CALL] {}", "cuMemcpyAtoD_v2");
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
    log::debug!("[CALL] {}", "cuArray3DGetDescriptor_v2");
    cuArray3DGetDescriptor_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetSharedMemConfig(arg0: CUsharedconfig) -> CUresult {
    let cuCtxSetSharedMemConfig: extern "C" fn(arg0: CUsharedconfig) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetSharedMemConfig"));
    log::debug!("[CALL] {}", "cuCtxSetSharedMemConfig");
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
    log::debug!("[CALL] {}", "cuStreamGetCtx_v2");
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
    log::debug!("[CALL] {}", "cuMulticastBindAddr_v2");
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
    log::debug!("[CALL] {}", "cuGraphChildGraphNodeGetGraph");
    cuGraphChildGraphNodeGetGraph(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxFromGreenCtx(arg0: *mut CUcontext, arg1: CUgreenCtx) -> CUresult {
    let cuCtxFromGreenCtx: extern "C" fn(arg0: *mut CUcontext, arg1: CUgreenCtx) -> CUresult =
        std::mem::transmute(get_sym("cuCtxFromGreenCtx"));
    log::debug!("[CALL] {}", "cuCtxFromGreenCtx");
    cuCtxFromGreenCtx(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetId(arg0: CUcontext, arg1: *mut c_ulonglong) -> CUresult {
    let cuCtxGetId: extern "C" fn(arg0: CUcontext, arg1: *mut c_ulonglong) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetId"));
    log::debug!("[CALL] {}", "cuCtxGetId");
    cuCtxGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxDestroy(arg0: CUgreenCtx) -> CUresult {
    let cuGreenCtxDestroy: extern "C" fn(arg0: CUgreenCtx) -> CUresult =
        std::mem::transmute(get_sym("cuGreenCtxDestroy"));
    log::debug!("[CALL] {}", "cuGreenCtxDestroy");
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
    log::debug!("[CALL] {}", "cuDeviceGetName");
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
    log::debug!("[CALL] {}", "cuGraphExecNodeSetParams");
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
    log::debug!("[CALL] {}", "cuDeviceGetPCIBusId");
    cuDeviceGetPCIBusId(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryGetKernelCount(arg0: *mut c_uint, arg1: CUlibrary) -> CUresult {
    let cuLibraryGetKernelCount: extern "C" fn(arg0: *mut c_uint, arg1: CUlibrary) -> CUresult =
        std::mem::transmute(get_sym("cuLibraryGetKernelCount"));
    log::debug!("[CALL] {}", "cuLibraryGetKernelCount");
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
    log::debug!("[CALL] {}", "cuLibraryLoadData");
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
    log::debug!("[CALL] {}", "cuMemHostRegister_v2");
    cuMemHostRegister_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetCacheConfig(arg0: *mut CUfunc_cache) -> CUresult {
    let cuCtxGetCacheConfig: extern "C" fn(arg0: *mut CUfunc_cache) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetCacheConfig"));
    log::debug!("[CALL] {}", "cuCtxGetCacheConfig");
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
    log::debug!("[CALL] {}", "cuGraphAddMemcpyNode");
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
    log::debug!("[CALL] {}", "cuLinkComplete");
    cuLinkComplete(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAddressFree(arg0: CUdeviceptr, arg1: usize) -> CUresult {
    let cuMemAddressFree: extern "C" fn(arg0: CUdeviceptr, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemAddressFree"));
    log::debug!("[CALL] {}", "cuMemAddressFree");
    cuMemAddressFree(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuSurfObjectDestroy(arg0: CUsurfObject) -> CUresult {
    let cuSurfObjectDestroy: extern "C" fn(arg0: CUsurfObject) -> CUresult =
        std::mem::transmute(get_sym("cuSurfObjectDestroy"));
    log::debug!("[CALL] {}", "cuSurfObjectDestroy");
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
    log::debug!("[CALL] {}", "cuGraphAddMemsetNode");
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
    log::debug!("[CALL] {}", "cuGraphKernelNodeCopyAttributes");
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
    log::debug!("[CALL] {}", "cuIpcOpenEventHandle");
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
    log::debug!("[CALL] {}", "cuDeviceGetTexture1DLinearMaxWidth");
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
    log::debug!("[CALL] {}", "cuMemSetAccess");
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
    log::debug!("[CALL] {}", "cuExternalMemoryGetMappedMipmappedArray");
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
    log::debug!("[CALL] {}", "cuStreamWriteValue64_v2");
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
    log::debug!("[CALL] {}", "cuStreamGetCaptureInfo_v3");
    cuStreamGetCaptureInfo_v3(arg0, arg1, arg2, arg3, arg4, arg5, arg6)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogsCurrent(arg0: *mut CUlogIterator, arg1: c_uint) -> CUresult {
    let cuLogsCurrent: extern "C" fn(arg0: *mut CUlogIterator, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuLogsCurrent"));
    log::debug!("[CALL] {}", "cuLogsCurrent");
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
    log::debug!("[CALL] {}", "cuMemBatchDecompressAsync");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "cuMemGetDefaultMemPool");
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
    log::debug!("[CALL] {}", "cuTexRefSetMipmapFilterMode");
    cuTexRefSetMipmapFilterMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchGrid(arg0: CUfunction, arg1: c_int, arg2: c_int) -> CUresult {
    let cuLaunchGrid: extern "C" fn(arg0: CUfunction, arg1: c_int, arg2: c_int) -> CUresult =
        std::mem::transmute(get_sym("cuLaunchGrid"));
    log::debug!("[CALL] {}", "cuLaunchGrid");
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
    log::debug!("[CALL] {}", "cuGraphAddExternalSemaphoresSignalNode");
    cuGraphAddExternalSemaphoresSignalNode(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetProperties(arg0: *mut CUdevprop, arg1: CUdevice) -> CUresult {
    let cuDeviceGetProperties: extern "C" fn(arg0: *mut CUdevprop, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetProperties"));
    log::debug!("[CALL] {}", "cuDeviceGetProperties");
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
    log::debug!("[CALL] {}", "cuMemcpyAtoA_v2");
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
    log::debug!("[CALL] {}", "cuGraphAddEmptyNode");
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
    log::debug!("[CALL] {}", "cuDeviceGetDevResource");
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
    log::debug!("[CALL] {}", "cuGraphMemsetNodeSetParams");
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
    log::debug!("[CALL] {}", "cuDeviceRegisterAsyncNotification");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointBindAddr");
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
    log::debug!("[CALL] {}", "cuLaunchHostFunc");
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
    log::debug!("[CALL] {}", "cuMemDiscardAndPrefetchBatchAsync");
    cuMemDiscardAndPrefetchBatchAsync(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetBorderColor(arg0: *mut f32, arg1: CUtexref) -> CUresult {
    let cuTexRefGetBorderColor: extern "C" fn(arg0: *mut f32, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetBorderColor"));
    log::debug!("[CALL] {}", "cuTexRefGetBorderColor");
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
    log::debug!("[CALL] {}", "cuFuncIsLoaded");
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
    log::debug!("[CALL] {}", "cuCoredumpRegisterStartCallback");
    cuCoredumpRegisterStartCallback(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8_v2(arg0: CUdeviceptr, arg1: c_uchar, arg2: usize) -> CUresult {
    let cuMemsetD8_v2: extern "C" fn(arg0: CUdeviceptr, arg1: c_uchar, arg2: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemsetD8_v2"));
    log::debug!("[CALL] {}", "cuMemsetD8_v2");
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
    log::debug!("[CALL] {}", "cuMemImportFromShareableHandle");
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
    log::debug!("[CALL] {}", "cuTensorMapReplaceAddress");
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
    log::debug!("[CALL] {}", "cuDeviceGetExecAffinitySupport");
    cuDeviceGetExecAffinitySupport(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventRecord(arg0: CUevent, arg1: CUstream) -> CUresult {
    let cuEventRecord: extern "C" fn(arg0: CUevent, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuEventRecord"));
    log::debug!("[CALL] {}", "cuEventRecord");
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
    log::debug!("[CALL] {}", "cuMemcpyDtoDAsync_v2");
    cuMemcpyDtoDAsync_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemRelease(arg0: CUmemGenericAllocationHandle) -> CUresult {
    let cuMemRelease: extern "C" fn(arg0: CUmemGenericAllocationHandle) -> CUresult =
        std::mem::transmute(get_sym("cuMemRelease"));
    log::debug!("[CALL] {}", "cuMemRelease");
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
    log::debug!("[CALL] {}", "cuTexRefGetAddressMode");
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
    log::debug!("[CALL] {}", "cuCoredumpSetAttribute");
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
    log::debug!("[CALL] {}", "cuGraphEventRecordNodeSetEvent");
    cuGraphEventRecordNodeSetEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxDestroy_v2(arg0: CUcontext) -> CUresult {
    let cuCtxDestroy_v2: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxDestroy_v2"));
    log::debug!("[CALL] {}", "cuCtxDestroy_v2");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointGetLimits");
    cuLogicalEndpointGetLimits(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamEndCaptureToCig(arg0: CUstream) -> CUresult {
    let cuStreamEndCaptureToCig: extern "C" fn(arg0: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamEndCaptureToCig"));
    log::debug!("[CALL] {}", "cuStreamEndCaptureToCig");
    cuStreamEndCaptureToCig(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamQuery(arg0: CUstream) -> CUresult {
    let cuStreamQuery: extern "C" fn(arg0: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamQuery"));
    log::debug!("[CALL] {}", "cuStreamQuery");
    cuStreamQuery(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncLoad(arg0: CUfunction) -> CUresult {
    let cuFuncLoad: extern "C" fn(arg0: CUfunction) -> CUresult =
        std::mem::transmute(get_sym("cuFuncLoad"));
    log::debug!("[CALL] {}", "cuFuncLoad");
    cuFuncLoad(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostUnregister(arg0: *mut c_void) -> CUresult {
    let cuMemHostUnregister: extern "C" fn(arg0: *mut c_void) -> CUresult =
        std::mem::transmute(get_sym("cuMemHostUnregister"));
    log::debug!("[CALL] {}", "cuMemHostUnregister");
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
    log::debug!("[CALL] {}", "cuStreamAttachMemAsync");
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
    log::debug!(
        "[CALL] {}",
        "cuGraphExecExternalSemaphoresSignalNodeSetParams"
    );
    cuGraphExecExternalSemaphoresSignalNodeSetParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetLimit(arg0: CUlimit, arg1: usize) -> CUresult {
    let cuCtxSetLimit: extern "C" fn(arg0: CUlimit, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetLimit"));
    log::debug!("[CALL] {}", "cuCtxSetLimit");
    cuCtxSetLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxWaitEvent(arg0: CUgreenCtx, arg1: CUevent) -> CUresult {
    let cuGreenCtxWaitEvent: extern "C" fn(arg0: CUgreenCtx, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuGreenCtxWaitEvent"));
    log::debug!("[CALL] {}", "cuGreenCtxWaitEvent");
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
    log::debug!("[CALL] {}", "cuGraphRemoveDependencies_v2");
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
    log::debug!("[CALL] {}", "cuMemsetD2D32_v2");
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
    log::debug!("[CALL] {}", "cuMemExportToShareableHandle");
    cuMemExportToShareableHandle(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetFunction(arg0: *mut CUfunction, arg1: CUkernel) -> CUresult {
    let cuKernelGetFunction: extern "C" fn(arg0: *mut CUfunction, arg1: CUkernel) -> CUresult =
        std::mem::transmute(get_sym("cuKernelGetFunction"));
    log::debug!("[CALL] {}", "cuKernelGetFunction");
    cuKernelGetFunction(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxRetain(
    arg0: *mut CUcontext,
    arg1: CUdevice,
) -> CUresult {
    let cuDevicePrimaryCtxRetain: extern "C" fn(arg0: *mut CUcontext, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxRetain"));
    log::debug!("[CALL] {}", "cuDevicePrimaryCtxRetain");
    cuDevicePrimaryCtxRetain(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefDestroy(arg0: CUtexref) -> CUresult {
    let cuTexRefDestroy: extern "C" fn(arg0: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefDestroy"));
    log::debug!("[CALL] {}", "cuTexRefDestroy");
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
    log::debug!("[CALL] {}", "cuMemAllocManaged");
    cuMemAllocManaged(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxPopCurrent_v2(arg0: *mut CUcontext) -> CUresult {
    let cuCtxPopCurrent_v2: extern "C" fn(arg0: *mut CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxPopCurrent_v2"));
    log::debug!("[CALL] {}", "cuCtxPopCurrent_v2");
    cuCtxPopCurrent_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetArray(arg0: CUtexref, arg1: CUarray, arg2: c_uint) -> CUresult {
    let cuTexRefSetArray: extern "C" fn(arg0: CUtexref, arg1: CUarray, arg2: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetArray"));
    log::debug!("[CALL] {}", "cuTexRefSetArray");
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
    log::debug!("[CALL] {}", "cuMemPoolSetAccess");
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
    log::debug!("[CALL] {}", "cuStreamWaitEvent");
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
    log::debug!("[CALL] {}", "cuLaunchGridAsync");
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
    log::debug!("[CALL] {}", "cuStreamWaitValue64_v2");
    cuStreamWaitValue64_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncSetCacheConfig(arg0: CUfunction, arg1: CUfunc_cache) -> CUresult {
    let cuFuncSetCacheConfig: extern "C" fn(arg0: CUfunction, arg1: CUfunc_cache) -> CUresult =
        std::mem::transmute(get_sym("cuFuncSetCacheConfig"));
    log::debug!("[CALL] {}", "cuFuncSetCacheConfig");
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
    log::debug!("[CALL] {}", "cuGraphInstantiateWithParams");
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
    log::debug!("[CALL] {}", "cuMemMapArrayAsync");
    cuMemMapArrayAsync(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncGetModule(arg0: *mut CUmodule, arg1: CUfunction) -> CUresult {
    let cuFuncGetModule: extern "C" fn(arg0: *mut CUmodule, arg1: CUfunction) -> CUresult =
        std::mem::transmute(get_sym("cuFuncGetModule"));
    log::debug!("[CALL] {}", "cuFuncGetModule");
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
    log::debug!("[CALL] {}", "cuGraphicsMapResources");
    cuGraphicsMapResources(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGreenCtxGetId(arg0: CUgreenCtx, arg1: *mut c_ulonglong) -> CUresult {
    let cuGreenCtxGetId: extern "C" fn(arg0: CUgreenCtx, arg1: *mut c_ulonglong) -> CUresult =
        std::mem::transmute(get_sym("cuGreenCtxGetId"));
    log::debug!("[CALL] {}", "cuGreenCtxGetId");
    cuGreenCtxGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostGetFlags(arg0: *mut c_uint, arg1: *mut c_void) -> CUresult {
    let cuMemHostGetFlags: extern "C" fn(arg0: *mut c_uint, arg1: *mut c_void) -> CUresult =
        std::mem::transmute(get_sym("cuMemHostGetFlags"));
    log::debug!("[CALL] {}", "cuMemHostGetFlags");
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
    log::debug!("[CALL] {}", "cuTensorMapEncodeIm2col");
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
    log::debug!("[CALL] {}", "cuExternalMemoryGetMappedBuffer");
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
    log::debug!("[CALL] {}", "cuStreamSetAttribute");
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
    log::debug!("[CALL] {}", "cuLibraryGetUnifiedFunction");
    cuLibraryGetUnifiedFunction(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuSurfRefGetArray(arg0: *mut CUarray, arg1: CUsurfref) -> CUresult {
    let cuSurfRefGetArray: extern "C" fn(arg0: *mut CUarray, arg1: CUsurfref) -> CUresult =
        std::mem::transmute(get_sym("cuSurfRefGetArray"));
    log::debug!("[CALL] {}", "cuSurfRefGetArray");
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
    log::debug!("[CALL] {}", "cuStreamAddCallback");
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
    log::debug!("[CALL] {}", "cuGraphAddKernelNode_v2");
    cuGraphAddKernelNode_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetDevice(arg0: CUstream, arg1: *mut CUdevice) -> CUresult {
    let cuStreamGetDevice: extern "C" fn(arg0: CUstream, arg1: *mut CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetDevice"));
    log::debug!("[CALL] {}", "cuStreamGetDevice");
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
    log::debug!("[CALL] {}", "cuStreamBeginCapture_v2");
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
    log::debug!("[CALL] {}", "cuGraphBatchMemOpNodeGetParams");
    cuGraphBatchMemOpNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcGetEventHandle(
    arg0: *mut CUipcEventHandle,
    arg1: CUevent,
) -> CUresult {
    let cuIpcGetEventHandle: extern "C" fn(arg0: *mut CUipcEventHandle, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuIpcGetEventHandle"));
    log::debug!("[CALL] {}", "cuIpcGetEventHandle");
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
    log::debug!("[CALL] {}", "cuDevResourceGenerateDesc");
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
    log::debug!("[CALL] {}", "cuCheckpointProcessGetState");
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
    log::debug!("[CALL] {}", "cuMemPrefetchBatchAsync");
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
    log::debug!("[CALL] {}", "cuGraphHostNodeSetParams");
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
    log::debug!("[CALL] {}", "cuStreamCreateWithPriority");
    cuStreamCreateWithPriority(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayDestroy(arg0: CUarray) -> CUresult {
    let cuArrayDestroy: extern "C" fn(arg0: CUarray) -> CUresult =
        std::mem::transmute(get_sym("cuArrayDestroy"));
    log::debug!("[CALL] {}", "cuArrayDestroy");
    cuArrayDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamCreate(arg0: *mut CUstream, arg1: c_uint) -> CUresult {
    let cuStreamCreate: extern "C" fn(arg0: *mut CUstream, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuStreamCreate"));
    log::debug!("[CALL] {}", "cuStreamCreate");
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
    log::debug!("[CALL] {}", "cuGraphExecBatchMemOpNodeSetParams");
    cuGraphExecBatchMemOpNodeSetParams(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetPriority(arg0: CUstream, arg1: *mut c_int) -> CUresult {
    let cuStreamGetPriority: extern "C" fn(arg0: CUstream, arg1: *mut c_int) -> CUresult =
        std::mem::transmute(get_sym("cuStreamGetPriority"));
    log::debug!("[CALL] {}", "cuStreamGetPriority");
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
    log::debug!("[CALL] {}", "cuMulticastGetGranularity");
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
    log::debug!("[CALL] {}", "cuGraphNodeGetDependencies_v2");
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
    log::debug!("[CALL] {}", "cuKernelSetAttribute");
    cuKernelSetAttribute(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxSetFlags_v2(arg0: CUdevice, arg1: c_uint) -> CUresult {
    let cuDevicePrimaryCtxSetFlags_v2: extern "C" fn(arg0: CUdevice, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxSetFlags_v2"));
    log::debug!("[CALL] {}", "cuDevicePrimaryCtxSetFlags_v2");
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
    log::debug!("[CALL] {}", "cuOccupancyMaxPotentialBlockSize");
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
    log::debug!("[CALL] {}", "cuMemcpy3DBatchAsync_v2");
    cuMemcpy3DBatchAsync_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2D_v2(arg0: *const CUDA_MEMCPY2D) -> CUresult {
    let cuMemcpy2D_v2: extern "C" fn(arg0: *const CUDA_MEMCPY2D) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy2D_v2"));
    log::debug!("[CALL] {}", "cuMemcpy2D_v2");
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
    log::debug!("[CALL] {}", "cuTexRefSetFormat");
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
    log::debug!("[CALL] {}", "cuArrayGetDescriptor_v2");
    cuArrayGetDescriptor_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetFilterMode(
    arg0: *mut CUfilter_mode,
    arg1: CUtexref,
) -> CUresult {
    let cuTexRefGetFilterMode: extern "C" fn(arg0: *mut CUfilter_mode, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetFilterMode"));
    log::debug!("[CALL] {}", "cuTexRefGetFilterMode");
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
    log::debug!("[CALL] {}", "cuMemsetD2D8Async");
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
    log::debug!("[CALL] {}", "cuMemAllocPitch_v2");
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
    log::debug!("[CALL] {}", "cuStreamBeginRecaptureToGraph");
    cuStreamBeginRecaptureToGraph(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphClone(arg0: *mut CUgraph, arg1: CUgraph) -> CUresult {
    let cuGraphClone: extern "C" fn(arg0: *mut CUgraph, arg1: CUgraph) -> CUresult =
        std::mem::transmute(get_sym("cuGraphClone"));
    log::debug!("[CALL] {}", "cuGraphClone");
    cuGraphClone(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetStreamPriorityRange(
    arg0: *mut c_int,
    arg1: *mut c_int,
) -> CUresult {
    let cuCtxGetStreamPriorityRange: extern "C" fn(arg0: *mut c_int, arg1: *mut c_int) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetStreamPriorityRange"));
    log::debug!("[CALL] {}", "cuCtxGetStreamPriorityRange");
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
    log::debug!("[CALL] {}", "cuDeviceGetDefaultMemPool");
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
    log::debug!("[CALL] {}", "cuUserObjectCreate");
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
    log::debug!("[CALL] {}", "cuMemDiscardBatchAsync");
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
    log::debug!("[CALL] {}", "cuGraphMemcpyNodeSetParams");
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
    log::debug!("[CALL] {}", "cuMemPrefetchAsync_v2");
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
    log::debug!("[CALL] {}", "cuMulticastAddDevice");
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
    log::debug!("[CALL] {}", "cuGraphKernelNodeGetAttribute");
    cuGraphKernelNodeGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGet(device: *mut CUdevice, ordinal: c_int) -> CUresult {
    log::debug!("[CALL] {}", "cuDeviceGet");
    // TODO: test mapping
    *device = ordinal;
    CUDA_SUCCESS
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
    log::debug!("[CALL] {}", "cuGraphRetainUserObject");
    cuGraphRetainUserObject(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetUuid_v2(arg0: *mut CUuuid, arg1: CUdevice) -> CUresult {
    let cuDeviceGetUuid_v2: extern "C" fn(arg0: *mut CUuuid, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetUuid_v2"));
    log::debug!("[CALL] {}", "cuDeviceGetUuid_v2");
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
    log::debug!("[CALL] {}", "cuMemcpy3DPeerAsync");
    cuMemcpy3DPeerAsync(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxPushCurrent_v2(arg0: CUcontext) -> CUresult {
    let cuCtxPushCurrent_v2: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxPushCurrent_v2"));
    log::debug!("[CALL] {}", "cuCtxPushCurrent_v2");
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
    log::debug!("[CALL] {}", "cuGraphExternalSemaphoresWaitNodeGetParams");
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
    log::debug!("[CALL] {}", "cuModuleGetTexRef");
    cuModuleGetTexRef(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventCreate(arg0: *mut CUevent, arg1: c_uint) -> CUresult {
    let cuEventCreate: extern "C" fn(arg0: *mut CUevent, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuEventCreate"));
    log::debug!("[CALL] {}", "cuEventCreate");
    cuEventCreate(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetByPCIBusId(
    arg0: *mut CUdevice,
    arg1: *const c_char,
) -> CUresult {
    let cuDeviceGetByPCIBusId: extern "C" fn(arg0: *mut CUdevice, arg1: *const c_char) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetByPCIBusId"));
    log::debug!("[CALL] {}", "cuDeviceGetByPCIBusId");
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
    log::debug!("[CALL] {}", "cuOccupancyMaxPotentialBlockSizeWithFlags");
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
    log::debug!("[CALL] {}", "cuFuncSetSharedMemConfig");
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
    log::debug!("[CALL] {}", "cuGraphExecHostNodeSetParams");
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
    log::debug!("[CALL] {}", "cuMemAdvise_v2");
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
    log::debug!("[CALL] {}", "cuDeviceGetAttribute");
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
    log::debug!("[CALL] {}", "cuMemGetAddressRange_v2");
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
    log::debug!("[CALL] {}", "cuMemHostAlloc");
    cuMemHostAlloc(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGetErrorName(arg0: CUresult, arg1: *mut *const c_char) -> CUresult {
    let cuGetErrorName: extern "C" fn(arg0: CUresult, arg1: *mut *const c_char) -> CUresult =
        std::mem::transmute(get_sym("cuGetErrorName"));
    log::debug!("[CALL] {}", "cuGetErrorName");
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
    log::debug!("[CALL] {}", "cuGreenCtxGetDevResource");
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
    log::debug!("[CALL] {}", "cuLibraryEnumerateKernels");
    cuLibraryEnumerateKernels(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogicalEndpointDestroy(arg0: CUlogicalEndpointId) -> CUresult {
    let cuLogicalEndpointDestroy: extern "C" fn(arg0: CUlogicalEndpointId) -> CUresult =
        std::mem::transmute(get_sym("cuLogicalEndpointDestroy"));
    log::debug!("[CALL] {}", "cuLogicalEndpointDestroy");
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
    log::debug!("[CALL] {}", "cuMemAllocFromPoolAsync");
    cuMemAllocFromPoolAsync(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetAddress_v2(arg0: *mut CUdeviceptr, arg1: CUtexref) -> CUresult {
    let cuTexRefGetAddress_v2: extern "C" fn(arg0: *mut CUdeviceptr, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetAddress_v2"));
    log::debug!("[CALL] {}", "cuTexRefGetAddress_v2");
    cuTexRefGetAddress_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpDeregisterCompleteCallback(
    arg0: CUcoredumpCallbackHandle,
) -> CUresult {
    let cuCoredumpDeregisterCompleteCallback: extern "C" fn(
        arg0: CUcoredumpCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpDeregisterCompleteCallback"));
    log::debug!("[CALL] {}", "cuCoredumpDeregisterCompleteCallback");
    cuCoredumpDeregisterCompleteCallback(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuParamSetf(arg0: CUfunction, arg1: c_int, arg2: f32) -> CUresult {
    let cuParamSetf: extern "C" fn(arg0: CUfunction, arg1: c_int, arg2: f32) -> CUresult =
        std::mem::transmute(get_sym("cuParamSetf"));
    log::debug!("[CALL] {}", "cuParamSetf");
    cuParamSetf(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkDestroy(arg0: CUlinkState) -> CUresult {
    let cuLinkDestroy: extern "C" fn(arg0: CUlinkState) -> CUresult =
        std::mem::transmute(get_sym("cuLinkDestroy"));
    log::debug!("[CALL] {}", "cuLinkDestroy");
    cuLinkDestroy(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleLoad(arg0: *mut CUmodule, arg1: *const c_char) -> CUresult {
    let cuModuleLoad: extern "C" fn(arg0: *mut CUmodule, arg1: *const c_char) -> CUresult =
        std::mem::transmute(get_sym("cuModuleLoad"));
    log::debug!("[CALL] {}", "cuModuleLoad");
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
    log::debug!("[CALL] {}", "cuLaunchHostFunc_v2");
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
    log::debug!("[CALL] {}", "cuEventRecordWithFlags");
    cuEventRecordWithFlags(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetFlags(arg0: CUtexref, arg1: c_uint) -> CUresult {
    let cuTexRefSetFlags: extern "C" fn(arg0: CUtexref, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetFlags"));
    log::debug!("[CALL] {}", "cuTexRefSetFlags");
    cuTexRefSetFlags(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuParamSeti(arg0: CUfunction, arg1: c_int, arg2: c_uint) -> CUresult {
    let cuParamSeti: extern "C" fn(arg0: CUfunction, arg1: c_int, arg2: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuParamSeti"));
    log::debug!("[CALL] {}", "cuParamSeti");
    cuParamSeti(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamEndCapture(arg0: CUstream, arg1: *mut CUgraph) -> CUresult {
    let cuStreamEndCapture: extern "C" fn(arg0: CUstream, arg1: *mut CUgraph) -> CUresult =
        std::mem::transmute(get_sym("cuStreamEndCapture"));
    log::debug!("[CALL] {}", "cuStreamEndCapture");
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
    log::debug!("[CALL] {}", "cuGraphGetRootNodes");
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
    log::debug!("[CALL] {}", "cuMipmappedArrayCreate");
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
    log::debug!("[CALL] {}", "cuGraphGetNodes");
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
    log::debug!("[CALL] {}", "cuMemCreate");
    cuMemCreate(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryGetModule(arg0: *mut CUmodule, arg1: CUlibrary) -> CUresult {
    let cuLibraryGetModule: extern "C" fn(arg0: *mut CUmodule, arg1: CUlibrary) -> CUresult =
        std::mem::transmute(get_sym("cuLibraryGetModule"));
    log::debug!("[CALL] {}", "cuLibraryGetModule");
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
    log::debug!("[CALL] {}", "cuParamSetv");
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
    log::debug!("[CALL] {}", "cuSurfObjectCreate");
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
    log::debug!("[CALL] {}", "cuGraphAddNode_v2");
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
    log::debug!("[CALL] {}", "cuOccupancyMaxPotentialClusterSize");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointUnbind");
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
    log::debug!("[CALL] {}", "cuImportExternalSemaphore");
    cuImportExternalSemaphore(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceSetMemPool(arg0: CUdevice, arg1: CUmemoryPool) -> CUresult {
    let cuDeviceSetMemPool: extern "C" fn(arg0: CUdevice, arg1: CUmemoryPool) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceSetMemPool"));
    log::debug!("[CALL] {}", "cuDeviceSetMemPool");
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
    log::debug!("[CALL] {}", "cuArrayCreate_v2");
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
    log::debug!("[CALL] {}", "cuMipmappedArrayGetSparseProperties");
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
    log::debug!("[CALL] {}", "cuModuleGetGlobal_v2");
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
    log::debug!("[CALL] {}", "cuGraphNodeGetParams");
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
    log::debug!("[CALL] {}", "cuGraphNodeGetType");
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
    log::debug!("[CALL] {}", "cuDeviceComputeCapability");
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
    log::debug!("[CALL] {}", "cuTexRefGetMipmapLevelClamp");
    cuTexRefGetMipmapLevelClamp(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuUserObjectRelease(arg0: CUuserObject, arg1: c_uint) -> CUresult {
    let cuUserObjectRelease: extern "C" fn(arg0: CUuserObject, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuUserObjectRelease"));
    log::debug!("[CALL] {}", "cuUserObjectRelease");
    cuUserObjectRelease(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetApiVersion(arg0: CUcontext, arg1: *mut c_uint) -> CUresult {
    let cuCtxGetApiVersion: extern "C" fn(arg0: CUcontext, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetApiVersion"));
    log::debug!("[CALL] {}", "cuCtxGetApiVersion");
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
    log::debug!("[CALL] {}", "cuOccupancyAvailableDynamicSMemPerBlock");
    cuOccupancyAvailableDynamicSMemPerBlock(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamDestroy_v2(arg0: CUstream) -> CUresult {
    let cuStreamDestroy_v2: extern "C" fn(arg0: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamDestroy_v2"));
    log::debug!("[CALL] {}", "cuStreamDestroy_v2");
    cuStreamDestroy_v2(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetFilterMode(arg0: CUtexref, arg1: CUfilter_mode) -> CUresult {
    let cuTexRefSetFilterMode: extern "C" fn(arg0: CUtexref, arg1: CUfilter_mode) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetFilterMode"));
    log::debug!("[CALL] {}", "cuTexRefSetFilterMode");
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
    log::debug!("[CALL] {}", "cuGraphAddBatchMemOpNode");
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
    log::debug!("[CALL] {}", "cuCoredumpSetAttributeGlobal");
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
    log::debug!("[CALL] {}", "cuStreamWriteValue32_v2");
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
    log::debug!("[CALL] {}", "cuGraphKernelNodeSetParams_v2");
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
    log::debug!("[CALL] {}", "cuGraphEventRecordNodeGetEvent");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointCreate");
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
    log::debug!("[CALL] {}", "cuLogsDumpToFile");
    cuLogsDumpToFile(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32_v2(arg0: CUdeviceptr, arg1: c_uint, arg2: usize) -> CUresult {
    let cuMemsetD32_v2: extern "C" fn(arg0: CUdeviceptr, arg1: c_uint, arg2: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemsetD32_v2"));
    log::debug!("[CALL] {}", "cuMemsetD32_v2");
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
    log::debug!("[CALL] {}", "cuStreamUpdateCaptureDependencies_v2");
    cuStreamUpdateCaptureDependencies_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsUnregisterResource(arg0: CUgraphicsResource) -> CUresult {
    let cuGraphicsUnregisterResource: extern "C" fn(arg0: CUgraphicsResource) -> CUresult =
        std::mem::transmute(get_sym("cuGraphicsUnregisterResource"));
    log::debug!("[CALL] {}", "cuGraphicsUnregisterResource");
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
    log::debug!("[CALL] {}", "cuMemRetainAllocationHandle");
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
    log::debug!("[CALL] {}", "cuGraphExecEventWaitNodeSetEvent");
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
    log::debug!("[CALL] {}", "cuMemcpyHtoAAsync_v2");
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
    log::debug!("[CALL] {}", "cuDeviceSetGraphMemAttribute");
    cuDeviceSetGraphMemAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetLimit(arg0: *mut usize, arg1: CUlimit) -> CUresult {
    let cuCtxGetLimit: extern "C" fn(arg0: *mut usize, arg1: CUlimit) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetLimit"));
    log::debug!("[CALL] {}", "cuCtxGetLimit");
    cuCtxGetLimit(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolTrimTo(arg0: CUmemoryPool, arg1: usize) -> CUresult {
    let cuMemPoolTrimTo: extern "C" fn(arg0: CUmemoryPool, arg1: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemPoolTrimTo"));
    log::debug!("[CALL] {}", "cuMemPoolTrimTo");
    cuMemPoolTrimTo(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuFuncGetParamCount(arg0: CUfunction, arg1: *mut usize) -> CUresult {
    let cuFuncGetParamCount: extern "C" fn(arg0: CUfunction, arg1: *mut usize) -> CUresult =
        std::mem::transmute(get_sym("cuFuncGetParamCount"));
    log::debug!("[CALL] {}", "cuFuncGetParamCount");
    cuFuncGetParamCount(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetFlags(arg0: c_uint) -> CUresult {
    let cuCtxSetFlags: extern "C" fn(arg0: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetFlags"));
    log::debug!("[CALL] {}", "cuCtxSetFlags");
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
    log::debug!("[CALL] {}", "cuMemHostGetDevicePointer_v2");
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
    log::debug!("[CALL] {}", "cuLaunchKernelEx");
    cuLaunchKernelEx(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceTotalMem_v2(arg0: *mut usize, arg1: CUdevice) -> CUresult {
    let cuDeviceTotalMem_v2: extern "C" fn(arg0: *mut usize, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceTotalMem_v2"));
    log::debug!("[CALL] {}", "cuDeviceTotalMem_v2");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointIdReserve");
    cuLogicalEndpointIdReserve(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxWaitEvent(arg0: CUcontext, arg1: CUevent) -> CUresult {
    let cuCtxWaitEvent: extern "C" fn(arg0: CUcontext, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuCtxWaitEvent"));
    log::debug!("[CALL] {}", "cuCtxWaitEvent");
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
    log::debug!("[CALL] {}", "cuLogsDumpToMemory");
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
    log::debug!("[CALL] {}", "cuFuncSetAttribute");
    cuFuncSetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetLibrary(arg0: *mut CUlibrary, arg1: CUkernel) -> CUresult {
    let cuKernelGetLibrary: extern "C" fn(arg0: *mut CUlibrary, arg1: CUkernel) -> CUresult =
        std::mem::transmute(get_sym("cuKernelGetLibrary"));
    log::debug!("[CALL] {}", "cuKernelGetLibrary");
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
    log::debug!("[CALL] {}", "cuLogsRegisterCallback");
    cuLogsRegisterCallback(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetArray(arg0: *mut CUarray, arg1: CUtexref) -> CUresult {
    let cuTexRefGetArray: extern "C" fn(arg0: *mut CUarray, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetArray"));
    log::debug!("[CALL] {}", "cuTexRefGetArray");
    cuTexRefGetArray(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy(arg0: CUdeviceptr, arg1: CUdeviceptr, arg2: usize) -> CUresult {
    let cuMemcpy: extern "C" fn(arg0: CUdeviceptr, arg1: CUdeviceptr, arg2: usize) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy"));
    log::debug!("[CALL] {}", "cuMemcpy");
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
    log::debug!("[CALL] {}", "cuGreenCtxStreamCreate");
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
    log::debug!("[CALL] {}", "cuMemcpyBatchAsync_v2");
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
    log::debug!("[CALL] {}", "cuMemsetD2D32Async");
    cuMemsetD2D32Async(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DPeer(arg0: *const CUDA_MEMCPY3D_PEER) -> CUresult {
    let cuMemcpy3DPeer: extern "C" fn(arg0: *const CUDA_MEMCPY3D_PEER) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy3DPeer"));
    log::debug!("[CALL] {}", "cuMemcpy3DPeer");
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
    log::debug!("[CALL] {}", "cuGraphExternalSemaphoresSignalNodeGetParams");
    cuGraphExternalSemaphoresSignalNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetDevice(arg0: *mut CUdevice) -> CUresult {
    let cuCtxGetDevice: extern "C" fn(arg0: *mut CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetDevice"));
    log::debug!("[CALL] {}", "cuCtxGetDevice");
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
    log::debug!("[CALL] {}", "cuGraphAddChildGraphNode");
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
    log::debug!("[CALL] {}", "cuGraphMemAllocNodeGetParams");
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
    log::debug!("[CALL] {}", "cuTensorMapEncodeIm2colWide");
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
    log::debug!("[CALL] {}", "cuTexRefSetAddress_v2");
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
    log::debug!("[CALL] {}", "cuArrayGetPlane");
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
    log::debug!("[CALL] {}", "cuTexObjectGetResourceViewDesc");
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
    log::debug!("[CALL] {}", "cuGraphConditionalHandleCreate");
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
    log::debug!("[CALL] {}", "cuCtxGetDevResource");
    cuCtxGetDevResource(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3D_v2(arg0: *const CUDA_MEMCPY3D) -> CUresult {
    let cuMemcpy3D_v2: extern "C" fn(arg0: *const CUDA_MEMCPY3D) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy3D_v2"));
    log::debug!("[CALL] {}", "cuMemcpy3D_v2");
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
    log::debug!("[CALL] {}", "cuLibraryGetGlobal");
    cuLibraryGetGlobal(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphGetId(arg0: CUgraph, arg1: *mut c_uint) -> CUresult {
    let cuGraphGetId: extern "C" fn(arg0: CUgraph, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuGraphGetId"));
    log::debug!("[CALL] {}", "cuGraphGetId");
    cuGraphGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuDestroyExternalMemory(arg0: CUexternalMemory) -> CUresult {
    let cuDestroyExternalMemory: extern "C" fn(arg0: CUexternalMemory) -> CUresult =
        std::mem::transmute(get_sym("cuDestroyExternalMemory"));
    log::debug!("[CALL] {}", "cuDestroyExternalMemory");
    cuDestroyExternalMemory(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetMipmapLevelBias(arg0: *mut f32, arg1: CUtexref) -> CUresult {
    let cuTexRefGetMipmapLevelBias: extern "C" fn(arg0: *mut f32, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetMipmapLevelBias"));
    log::debug!("[CALL] {}", "cuTexRefGetMipmapLevelBias");
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
    log::debug!("[CALL] {}", "cuOccupancyMaxActiveBlocksPerMultiprocessor");
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
    log::debug!("[CALL] {}", "cuDevicePrimaryCtxGetState");
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
    log::debug!("[CALL] {}", "cuMemPoolSetAttribute");
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
    log::debug!("[CALL] {}", "cuMemMap");
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
    log::debug!("[CALL] {}", "cuStreamWaitValue32_v2");
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
    log::debug!("[CALL] {}", "cuKernelGetAttribute");
    cuKernelGetAttribute(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFreeAsync(arg0: CUdeviceptr, arg1: CUstream) -> CUresult {
    let cuMemFreeAsync: extern "C" fn(arg0: CUdeviceptr, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuMemFreeAsync"));
    log::debug!("[CALL] {}", "cuMemFreeAsync");
    cuMemFreeAsync(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuKernelGetParamCount(arg0: CUkernel, arg1: *mut usize) -> CUresult {
    let cuKernelGetParamCount: extern "C" fn(arg0: CUkernel, arg1: *mut usize) -> CUresult =
        std::mem::transmute(get_sym("cuKernelGetParamCount"));
    log::debug!("[CALL] {}", "cuKernelGetParamCount");
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
    log::debug!("[CALL] {}", "cuLaunchKernel");
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
    log::debug!("[CALL] {}", "cuStreamGetDevResource");
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
    log::debug!("[CALL] {}", "cuMemcpyHtoD_v2");
    cuMemcpyHtoD_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuLogsUnregisterCallback(arg0: CUlogsCallbackHandle) -> CUresult {
    let cuLogsUnregisterCallback: extern "C" fn(arg0: CUlogsCallbackHandle) -> CUresult =
        std::mem::transmute(get_sym("cuLogsUnregisterCallback"));
    log::debug!("[CALL] {}", "cuLogsUnregisterCallback");
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
    log::debug!("[CALL] {}", "cuMemGetAccess");
    cuMemGetAccess(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuCoredumpDeregisterStartCallback(
    arg0: CUcoredumpCallbackHandle,
) -> CUresult {
    let cuCoredumpDeregisterStartCallback: extern "C" fn(
        arg0: CUcoredumpCallbackHandle,
    ) -> CUresult = std::mem::transmute(get_sym("cuCoredumpDeregisterStartCallback"));
    log::debug!("[CALL] {}", "cuCoredumpDeregisterStartCallback");
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
    log::debug!("[CALL] {}", "cuMemGetAllocationPropertiesFromHandle");
    cuMemGetAllocationPropertiesFromHandle(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetDevice_v2(arg0: *mut CUdevice, arg1: CUcontext) -> CUresult {
    let cuCtxGetDevice_v2: extern "C" fn(arg0: *mut CUdevice, arg1: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetDevice_v2"));
    log::debug!("[CALL] {}", "cuCtxGetDevice_v2");
    cuCtxGetDevice_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunch(arg0: CUfunction) -> CUresult {
    let cuLaunch: extern "C" fn(arg0: CUfunction) -> CUresult =
        std::mem::transmute(get_sym("cuLaunch"));
    log::debug!("[CALL] {}", "cuLaunch");
    cuLaunch(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetMaxAnisotropy(arg0: CUtexref, arg1: c_uint) -> CUresult {
    let cuTexRefSetMaxAnisotropy: extern "C" fn(arg0: CUtexref, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefSetMaxAnisotropy"));
    log::debug!("[CALL] {}", "cuTexRefSetMaxAnisotropy");
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
    log::debug!("[CALL] {}", "cuKernelGetParamInfo");
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
    log::debug!("[CALL] {}", "cuMemRangeGetAttribute");
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
    log::debug!("[CALL] {}", "cuGraphAddDependencies_v2");
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
    log::debug!(
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
    log::debug!("[CALL] {}", "cuMemcpyDtoH_v2");
    cuMemcpyDtoH_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DUnaligned_v2(arg0: *const CUDA_MEMCPY2D) -> CUresult {
    let cuMemcpy2DUnaligned_v2: extern "C" fn(arg0: *const CUDA_MEMCPY2D) -> CUresult =
        std::mem::transmute(get_sym("cuMemcpy2DUnaligned_v2"));
    log::debug!("[CALL] {}", "cuMemcpy2DUnaligned_v2");
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
    log::debug!("[CALL] {}", "cuMulticastBindMem_v2");
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
    log::debug!("[CALL] {}", "cuDevSmResourceSplit");
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
    log::debug!("[CALL] {}", "cuGraphMemsetNodeGetParams");
    cuGraphMemsetNodeGetParams(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphLaunch(arg0: CUgraphExec, arg1: CUstream) -> CUresult {
    let cuGraphLaunch: extern "C" fn(arg0: CUgraphExec, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuGraphLaunch"));
    log::debug!("[CALL] {}", "cuGraphLaunch");
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
    log::debug!("[CALL] {}", "cuMemcpyWithAttributesAsync");
    cuMemcpyWithAttributesAsync(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventSynchronize(arg0: CUevent) -> CUresult {
    let cuEventSynchronize: extern "C" fn(arg0: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuEventSynchronize"));
    log::debug!("[CALL] {}", "cuEventSynchronize");
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
    log::debug!("[CALL] {}", "cuCheckpointProcessUnlock");
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
    log::debug!("[CALL] {}", "cuMemGetAllocationGranularity");
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
    log::debug!("[CALL] {}", "cuStreamIsCapturing");
    cuStreamIsCapturing(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphUpload(arg0: CUgraphExec, arg1: CUstream) -> CUresult {
    let cuGraphUpload: extern "C" fn(arg0: CUgraphExec, arg1: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuGraphUpload"));
    log::debug!("[CALL] {}", "cuGraphUpload");
    cuGraphUpload(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuLibraryUnload(arg0: CUlibrary) -> CUresult {
    let cuLibraryUnload: extern "C" fn(arg0: CUlibrary) -> CUresult =
        std::mem::transmute(get_sym("cuLibraryUnload"));
    log::debug!("[CALL] {}", "cuLibraryUnload");
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
    log::debug!("[CALL] {}", "cuMemsetD2D16_v2");
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
    log::debug!("[CALL] {}", "cuGraphNodeFindInClone");
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
    log::debug!("[CALL] {}", "cuMemsetD2D16Async");
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
    log::debug!("[CALL] {}", "cuMemcpyHtoA_v2");
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
    log::debug!("[CALL] {}", "cuModuleGetFunction");
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
    log::debug!("[CALL] {}", "cuMemcpyAtoHAsync_v2");
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
    log::debug!("[CALL] {}", "cuGraphMemFreeNodeGetParams");
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
    log::debug!("[CALL] {}", "cuDeviceGetHostAtomicCapabilities");
    cuDeviceGetHostAtomicCapabilities(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuEventDestroy_v2(arg0: CUevent) -> CUresult {
    let cuEventDestroy_v2: extern "C" fn(arg0: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuEventDestroy_v2"));
    log::debug!("[CALL] {}", "cuEventDestroy_v2");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointExport");
    cuLogicalEndpointExport(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxReset_v2(arg0: CUdevice) -> CUresult {
    let cuDevicePrimaryCtxReset_v2: extern "C" fn(arg0: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxReset_v2"));
    log::debug!("[CALL] {}", "cuDevicePrimaryCtxReset_v2");
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
    log::debug!("[CALL] {}", "cuGraphExecEventRecordNodeSetEvent");
    cuGraphExecEventRecordNodeSetEvent(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetMemPool(arg0: *mut CUmemoryPool, arg1: CUdevice) -> CUresult {
    let cuDeviceGetMemPool: extern "C" fn(arg0: *mut CUmemoryPool, arg1: CUdevice) -> CUresult =
        std::mem::transmute(get_sym("cuDeviceGetMemPool"));
    log::debug!("[CALL] {}", "cuDeviceGetMemPool");
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
    log::debug!("[CALL] {}", "cuGraphExecMemsetNodeSetParams");
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
    log::debug!("[CALL] {}", "cuMemPoolImportFromShareableHandle");
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
    log::debug!("[CALL] {}", "cuArrayGetSparseProperties");
    cuArrayGetSparseProperties(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuInit(arg0: c_uint) -> CUresult {
    let cuInit: extern "C" fn(arg0: c_uint) -> CUresult = std::mem::transmute(get_sym("cuInit"));
    log::debug!("[CALL] {}", "cuInit");
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
    log::debug!("[CALL] {}", "cuMemPoolGetAccess");
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
    log::debug!("[CALL] {}", "cuGraphGetEdges_v2");
    cuGraphGetEdges_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetFlags(arg0: *mut c_uint, arg1: CUtexref) -> CUresult {
    let cuTexRefGetFlags: extern "C" fn(arg0: *mut c_uint, arg1: CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefGetFlags"));
    log::debug!("[CALL] {}", "cuTexRefGetFlags");
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
    log::debug!("[CALL] {}", "cuCoredumpGetAttribute");
    cuCoredumpGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMipmappedArrayDestroy(arg0: CUmipmappedArray) -> CUresult {
    let cuMipmappedArrayDestroy: extern "C" fn(arg0: CUmipmappedArray) -> CUresult =
        std::mem::transmute(get_sym("cuMipmappedArrayDestroy"));
    log::debug!("[CALL] {}", "cuMipmappedArrayDestroy");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointImport");
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
    log::debug!("[CALL] {}", "cuMemcpyPeerAsync");
    cuMemcpyPeerAsync(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxGetCurrent(arg0: *mut CUcontext) -> CUresult {
    let cuCtxGetCurrent: extern "C" fn(arg0: *mut CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxGetCurrent"));
    log::debug!("[CALL] {}", "cuCtxGetCurrent");
    cuCtxGetCurrent(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxSetCurrent(arg0: CUcontext) -> CUresult {
    let cuCtxSetCurrent: extern "C" fn(arg0: CUcontext) -> CUresult =
        std::mem::transmute(get_sym("cuCtxSetCurrent"));
    log::debug!("[CALL] {}", "cuCtxSetCurrent");
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
    log::debug!("[CALL] {}", "cuMemcpyDtoD_v2");
    cuMemcpyDtoD_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecGetId(arg0: CUgraphExec, arg1: *mut c_uint) -> CUresult {
    let cuGraphExecGetId: extern "C" fn(arg0: CUgraphExec, arg1: *mut c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuGraphExecGetId"));
    log::debug!("[CALL] {}", "cuGraphExecGetId");
    cuGraphExecGetId(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxRecordEvent(arg0: CUcontext, arg1: CUevent) -> CUresult {
    let cuCtxRecordEvent: extern "C" fn(arg0: CUcontext, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuCtxRecordEvent"));
    log::debug!("[CALL] {}", "cuCtxRecordEvent");
    cuCtxRecordEvent(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphCreate(arg0: *mut CUgraph, arg1: c_uint) -> CUresult {
    let cuGraphCreate: extern "C" fn(arg0: *mut CUgraph, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuGraphCreate"));
    log::debug!("[CALL] {}", "cuGraphCreate");
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
    log::debug!("[CALL] {}", "cuMemcpyHtoDAsync_v2");
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
    log::debug!("[CALL] {}", "cuGraphExternalSemaphoresWaitNodeSetParams");
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
    log::debug!("[CALL] {}", "cuCheckpointProcessGetRestoreThreadId");
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
    log::debug!("[CALL] {}", "cuIpcOpenMemHandle_v2");
    cuIpcOpenMemHandle_v2(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFree_v2(arg0: CUdeviceptr) -> CUresult {
    let cuMemFree_v2: extern "C" fn(arg0: CUdeviceptr) -> CUresult =
        std::mem::transmute(get_sym("cuMemFree_v2"));
    log::debug!("[CALL] {}", "cuMemFree_v2");
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
    log::debug!("[CALL] {}", "cuDeviceGetLuid");
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
    log::debug!("[CALL] {}", "cuMulticastBindMem");
    cuMulticastBindMem(arg0, arg1, arg2, arg3, arg4, arg5)
}
#[no_mangle]
pub unsafe extern "C" fn cuGetErrorString(arg0: CUresult, arg1: *mut *const c_char) -> CUresult {
    let cuGetErrorString: extern "C" fn(arg0: CUresult, arg1: *mut *const c_char) -> CUresult =
        std::mem::transmute(get_sym("cuGetErrorString"));
    log::debug!("[CALL] {}", "cuGetErrorString");
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
    log::debug!("[CALL] {}", "cuMemAllocAsync");
    cuMemAllocAsync(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphEventWaitNodeSetEvent(
    arg0: CUgraphNode,
    arg1: CUevent,
) -> CUresult {
    let cuGraphEventWaitNodeSetEvent: extern "C" fn(arg0: CUgraphNode, arg1: CUevent) -> CUresult =
        std::mem::transmute(get_sym("cuGraphEventWaitNodeSetEvent"));
    log::debug!("[CALL] {}", "cuGraphEventWaitNodeSetEvent");
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
    log::debug!("[CALL] {}", "cuDeviceGetNvSciSyncAttributes");
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
    log::debug!("[CALL] {}", "cuMemSetMemPool");
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
    log::debug!("[CALL] {}", "cuMemGetMemPool");
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
    log::debug!("[CALL] {}", "cuMemsetD2D8_v2");
    cuMemsetD2D8_v2(arg0, arg1, arg2, arg3, arg4)
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefCreate(arg0: *mut CUtexref) -> CUresult {
    let cuTexRefCreate: extern "C" fn(arg0: *mut CUtexref) -> CUresult =
        std::mem::transmute(get_sym("cuTexRefCreate"));
    log::debug!("[CALL] {}", "cuTexRefCreate");
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
    log::debug!("[CALL] {}", "cuGraphHostNodeGetParams");
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
    log::debug!("[CALL] {}", "cuLinkCreate_v2");
    cuLinkCreate_v2(arg0, arg1, arg2, arg3)
}
#[no_mangle]
pub unsafe extern "C" fn cuModuleLoadData(arg0: *mut CUmodule, arg1: *const c_void) -> CUresult {
    let cuModuleLoadData: extern "C" fn(arg0: *mut CUmodule, arg1: *const c_void) -> CUresult =
        std::mem::transmute(get_sym("cuModuleLoadData"));
    log::debug!("[CALL] {}", "cuModuleLoadData");
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
    log::debug!("[CALL] {}", "cuGraphKernelNodeGetParams_v2");
    cuGraphKernelNodeGetParams_v2(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamSynchronize(arg0: CUstream) -> CUresult {
    let cuStreamSynchronize: extern "C" fn(arg0: CUstream) -> CUresult =
        std::mem::transmute(get_sym("cuStreamSynchronize"));
    log::debug!("[CALL] {}", "cuStreamSynchronize");
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
    log::debug!("[CALL] {}", "cuStreamGetAttribute");
    cuStreamGetAttribute(arg0, arg1, arg2)
}
#[no_mangle]
pub unsafe extern "C" fn cuParamSetSize(arg0: CUfunction, arg1: c_uint) -> CUresult {
    let cuParamSetSize: extern "C" fn(arg0: CUfunction, arg1: c_uint) -> CUresult =
        std::mem::transmute(get_sym("cuParamSetSize"));
    log::debug!("[CALL] {}", "cuParamSetSize");
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
    log::debug!("[CALL] {}", "cuLibraryGetManaged");
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
    log::debug!("[CALL] {}", "cuTexRefGetMipmapFilterMode");
    cuTexRefGetMipmapFilterMode(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcCloseMemHandle(arg0: CUdeviceptr) -> CUresult {
    let cuIpcCloseMemHandle: extern "C" fn(arg0: CUdeviceptr) -> CUresult =
        std::mem::transmute(get_sym("cuIpcCloseMemHandle"));
    log::debug!("[CALL] {}", "cuIpcCloseMemHandle");
    cuIpcCloseMemHandle(arg0)
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphDestroyNode(arg0: CUgraphNode) -> CUresult {
    let cuGraphDestroyNode: extern "C" fn(arg0: CUgraphNode) -> CUresult =
        std::mem::transmute(get_sym("cuGraphDestroyNode"));
    log::debug!("[CALL] {}", "cuGraphDestroyNode");
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
    log::debug!("[CALL] {}", "cuLogicalEndpointAddDevice");
    cuLogicalEndpointAddDevice(arg0, arg1)
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPoolDestroy(arg0: CUmemoryPool) -> CUresult {
    let cuMemPoolDestroy: extern "C" fn(arg0: CUmemoryPool) -> CUresult =
        std::mem::transmute(get_sym("cuMemPoolDestroy"));
    log::debug!("[CALL] {}", "cuMemPoolDestroy");
    cuMemPoolDestroy(arg0)
}

// Unknown function stubs (functions not found in header)
#[no_mangle]
pub unsafe extern "C" fn cuModuleGetGlobal() {
    let cuModuleGetGlobal: extern "C" fn() = std::mem::transmute(get_sym("cuModuleGetGlobal"));
    log::debug!("[CALL] {}", "cuModuleGetGlobal");
    cuModuleGetGlobal()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetInfo() {
    let cuMemGetInfo: extern "C" fn() = std::mem::transmute(get_sym("cuMemGetInfo"));
    log::debug!("[CALL] {}", "cuMemGetInfo");
    cuMemGetInfo()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecKernelNodeSetParams() {
    let cuGraphExecKernelNodeSetParams: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphExecKernelNodeSetParams"));
    log::debug!("[CALL] {}", "cuGraphExecKernelNodeSetParams");
    cuGraphExecKernelNodeSetParams()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchKernelEx_ptsz() {
    let cuLaunchKernelEx_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuLaunchKernelEx_ptsz"));
    log::debug!("[CALL] {}", "cuLaunchKernelEx_ptsz");
    cuLaunchKernelEx_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphGetEdges() {
    let cuGraphGetEdges: extern "C" fn() = std::mem::transmute(get_sym("cuGraphGetEdges"));
    log::debug!("[CALL] {}", "cuGraphGetEdges");
    cuGraphGetEdges()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObjectAsync() {
    let cuGLMapBufferObjectAsync: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObjectAsync"));
    log::debug!("[CALL] {}", "cuGLMapBufferObjectAsync");
    cuGLMapBufferObjectAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoA_v2_ptds() {
    let cuMemcpyAtoA_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyAtoA_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyAtoA_v2_ptds");
    cuMemcpyAtoA_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLUnmapBufferObject() {
    let cuGLUnmapBufferObject: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLUnmapBufferObject"));
    log::debug!("[CALL] {}", "cuGLUnmapBufferObject");
    cuGLUnmapBufferObject()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerConnectWithFlags() {
    let cuEGLStreamConsumerConnectWithFlags: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerConnectWithFlags"));
    log::debug!("[CALL] {}", "cuEGLStreamConsumerConnectWithFlags");
    cuEGLStreamConsumerConnectWithFlags()
}
#[no_mangle]
pub unsafe extern "C" fn cuSignalExternalSemaphoresAsync_ptsz() {
    let cuSignalExternalSemaphoresAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuSignalExternalSemaphoresAsync_ptsz"));
    log::debug!("[CALL] {}", "cuSignalExternalSemaphoresAsync_ptsz");
    cuSignalExternalSemaphoresAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsVDPAURegisterOutputSurface() {
    let cuGraphicsVDPAURegisterOutputSurface: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsVDPAURegisterOutputSurface"));
    log::debug!("[CALL] {}", "cuGraphicsVDPAURegisterOutputSurface");
    cuGraphicsVDPAURegisterOutputSurface()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventCreateFromEGLSync() {
    let cuEventCreateFromEGLSync: extern "C" fn() =
        std::mem::transmute(get_sym("cuEventCreateFromEGLSync"));
    log::debug!("[CALL] {}", "cuEventCreateFromEGLSync");
    cuEventCreateFromEGLSync()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsGLRegisterBuffer() {
    let cuGraphicsGLRegisterBuffer: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsGLRegisterBuffer"));
    log::debug!("[CALL] {}", "cuGraphicsGLRegisterBuffer");
    cuGraphicsGLRegisterBuffer()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAlloc() {
    let cuMemAlloc: extern "C" fn() = std::mem::transmute(get_sym("cuMemAlloc"));
    log::debug!("[CALL] {}", "cuMemAlloc");
    cuMemAlloc()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchAsync_ptsz() {
    let cuMemPrefetchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemPrefetchAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemPrefetchAsync_ptsz");
    cuMemPrefetchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoHAsync_v2_ptsz() {
    let cuMemcpyAtoHAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyAtoHAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyAtoHAsync_v2_ptsz");
    cuMemcpyAtoHAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DAsync() {
    let cuMemcpy2DAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy2DAsync"));
    log::debug!("[CALL] {}", "cuMemcpy2DAsync");
    cuMemcpy2DAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoA_v2_ptds() {
    let cuMemcpyHtoA_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyHtoA_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyHtoA_v2_ptds");
    cuMemcpyHtoA_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventDestroy() {
    let cuEventDestroy: extern "C" fn() = std::mem::transmute(get_sym("cuEventDestroy"));
    log::debug!("[CALL] {}", "cuEventDestroy");
    cuEventDestroy()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamDestroy() {
    let cuStreamDestroy: extern "C" fn() = std::mem::transmute(get_sym("cuStreamDestroy"));
    log::debug!("[CALL] {}", "cuStreamDestroy");
    cuStreamDestroy()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBatchMemOp_v2_ptsz() {
    let cuStreamBatchMemOp_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBatchMemOp_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamBatchMemOp_v2_ptsz");
    cuStreamBatchMemOp_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObjectAsync_v2() {
    let cuGLMapBufferObjectAsync_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObjectAsync_v2"));
    log::debug!("[CALL] {}", "cuGLMapBufferObjectAsync_v2");
    cuGLMapBufferObjectAsync_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphUpload_ptsz() {
    let cuGraphUpload_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuGraphUpload_ptsz"));
    log::debug!("[CALL] {}", "cuGraphUpload_ptsz");
    cuGraphUpload_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamUpdateCaptureDependencies_ptsz() {
    let cuStreamUpdateCaptureDependencies_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamUpdateCaptureDependencies_ptsz"));
    log::debug!("[CALL] {}", "cuStreamUpdateCaptureDependencies_ptsz");
    cuStreamUpdateCaptureDependencies_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamEndCaptureToCig_ptsz() {
    let cuStreamEndCaptureToCig_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamEndCaptureToCig_ptsz"));
    log::debug!("[CALL] {}", "cuStreamEndCaptureToCig_ptsz");
    cuStreamEndCaptureToCig_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkAddData() {
    let cuLinkAddData: extern "C" fn() = std::mem::transmute(get_sym("cuLinkAddData"));
    log::debug!("[CALL] {}", "cuLinkAddData");
    cuLinkAddData()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D32Async_ptsz() {
    let cuMemsetD2D32Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D32Async_ptsz"));
    log::debug!("[CALL] {}", "cuMemsetD2D32Async_ptsz");
    cuMemsetD2D32Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyPeer_ptds() {
    let cuMemcpyPeer_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyPeer_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyPeer_ptds");
    cuMemcpyPeer_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8() {
    let cuMemsetD8: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD8"));
    log::debug!("[CALL] {}", "cuMemsetD8");
    cuMemsetD8()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamEndCapture_ptsz() {
    let cuStreamEndCapture_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamEndCapture_ptsz"));
    log::debug!("[CALL] {}", "cuStreamEndCapture_ptsz");
    cuStreamEndCapture_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxCreate_v2() {
    let cuCtxCreate_v2: extern "C" fn() = std::mem::transmute(get_sym("cuCtxCreate_v2"));
    log::debug!("[CALL] {}", "cuCtxCreate_v2");
    cuCtxCreate_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxCreate_v3() {
    let cuCtxCreate_v3: extern "C" fn() = std::mem::transmute(get_sym("cuCtxCreate_v3"));
    log::debug!("[CALL] {}", "cuCtxCreate_v3");
    cuCtxCreate_v3()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgReportDriverApiError() {
    let cudbgReportDriverApiError: extern "C" fn() =
        std::mem::transmute(get_sym("cudbgReportDriverApiError"));
    log::debug!("[CALL] {}", "cudbgReportDriverApiError");
    cudbgReportDriverApiError()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchBatchAsync_ptsz() {
    let cuMemPrefetchBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemPrefetchBatchAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemPrefetchBatchAsync_ptsz");
    cuMemPrefetchBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuVDPAUCtxCreate() {
    let cuVDPAUCtxCreate: extern "C" fn() = std::mem::transmute(get_sym("cuVDPAUCtxCreate"));
    log::debug!("[CALL] {}", "cuVDPAUCtxCreate");
    cuVDPAUCtxCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsMapResources_ptsz() {
    let cuGraphicsMapResources_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsMapResources_ptsz"));
    log::debug!("[CALL] {}", "cuGraphicsMapResources_ptsz");
    cuGraphicsMapResources_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAsync_ptsz() {
    let cuMemcpyAsync_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyAsync_ptsz");
    cuMemcpyAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginRecaptureToGraph_ptsz() {
    let cuStreamBeginRecaptureToGraph_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginRecaptureToGraph_ptsz"));
    log::debug!("[CALL] {}", "cuStreamBeginRecaptureToGraph_ptsz");
    cuStreamBeginRecaptureToGraph_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemBatchDecompressAsync_ptsz() {
    let cuMemBatchDecompressAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemBatchDecompressAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemBatchDecompressAsync_ptsz");
    cuMemBatchDecompressAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetDependentNodes() {
    let cuGraphNodeGetDependentNodes: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphNodeGetDependentNodes"));
    log::debug!("[CALL] {}", "cuGraphNodeGetDependentNodes");
    cuGraphNodeGetDependentNodes()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy_ptds() {
    let cuMemcpy_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy_ptds"));
    log::debug!("[CALL] {}", "cuMemcpy_ptds");
    cuMemcpy_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGetProcAddress() {
    let cuGetProcAddress: extern "C" fn() = std::mem::transmute(get_sym("cuGetProcAddress"));
    log::debug!("[CALL] {}", "cuGetProcAddress");
    cuGetProcAddress()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostRegister() {
    let cuMemHostRegister: extern "C" fn() = std::mem::transmute(get_sym("cuMemHostRegister"));
    log::debug!("[CALL] {}", "cuMemHostRegister");
    cuMemHostRegister()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue32() {
    let cuStreamWaitValue32: extern "C" fn() = std::mem::transmute(get_sym("cuStreamWaitValue32"));
    log::debug!("[CALL] {}", "cuStreamWaitValue32");
    cuStreamWaitValue32()
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddress2D_v2() {
    let cuTexRefSetAddress2D_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuTexRefSetAddress2D_v2"));
    log::debug!("[CALL] {}", "cuTexRefSetAddress2D_v2");
    cuTexRefSetAddress2D_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DAsync_v2_ptsz() {
    let cuMemcpy2DAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy2DAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpy2DAsync_v2_ptsz");
    cuMemcpy2DAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamAttachMemAsync_ptsz() {
    let cuStreamAttachMemAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamAttachMemAsync_ptsz"));
    log::debug!("[CALL] {}", "cuStreamAttachMemAsync_ptsz");
    cuStreamAttachMemAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoAAsync_v2_ptsz() {
    let cuMemcpyHtoAAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyHtoAAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyHtoAAsync_v2_ptsz");
    cuMemcpyHtoAAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocHost() {
    let cuMemAllocHost: extern "C" fn() = std::mem::transmute(get_sym("cuMemAllocHost"));
    log::debug!("[CALL] {}", "cuMemAllocHost");
    cuMemAllocHost()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocAsync_ptsz() {
    let cuMemAllocAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemAllocAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemAllocAsync_ptsz");
    cuMemAllocAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DUnaligned_v2_ptds() {
    let cuMemcpy2DUnaligned_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy2DUnaligned_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpy2DUnaligned_v2_ptds");
    cuMemcpy2DUnaligned_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoDAsync_v2_ptsz() {
    let cuMemcpyDtoDAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoDAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyDtoDAsync_v2_ptsz");
    cuMemcpyDtoDAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBatchMemOp() {
    let cuStreamBatchMemOp: extern "C" fn() = std::mem::transmute(get_sym("cuStreamBatchMemOp"));
    log::debug!("[CALL] {}", "cuStreamBatchMemOp");
    cuStreamBatchMemOp()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoH_v2_ptds() {
    let cuMemcpyAtoH_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyAtoH_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyAtoH_v2_ptds");
    cuMemcpyAtoH_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayGetDescriptor() {
    let cuArrayGetDescriptor: extern "C" fn() =
        std::mem::transmute(get_sym("cuArrayGetDescriptor"));
    log::debug!("[CALL] {}", "cuArrayGetDescriptor");
    cuArrayGetDescriptor()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocFromPoolAsync_ptsz() {
    let cuMemAllocFromPoolAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemAllocFromPoolAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemAllocFromPoolAsync_ptsz");
    cuMemAllocFromPoolAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue32() {
    let cuStreamWriteValue32: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue32"));
    log::debug!("[CALL] {}", "cuStreamWriteValue32");
    cuStreamWriteValue32()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeSetParams() {
    let cuGraphKernelNodeSetParams: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphKernelNodeSetParams"));
    log::debug!("[CALL] {}", "cuGraphKernelNodeSetParams");
    cuGraphKernelNodeSetParams()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsGLRegisterImage() {
    let cuGraphicsGLRegisterImage: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsGLRegisterImage"));
    log::debug!("[CALL] {}", "cuGraphicsGLRegisterImage");
    cuGraphicsGLRegisterImage()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoA() {
    let cuMemcpyHtoA: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyHtoA"));
    log::debug!("[CALL] {}", "cuMemcpyHtoA");
    cuMemcpyHtoA()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamProducerReturnFrame() {
    let cuEGLStreamProducerReturnFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamProducerReturnFrame"));
    log::debug!("[CALL] {}", "cuEGLStreamProducerReturnFrame");
    cuEGLStreamProducerReturnFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue64_v2_ptsz() {
    let cuStreamWriteValue64_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue64_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWriteValue64_v2_ptsz");
    cuStreamWriteValue64_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD16() {
    let cuMemsetD16: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD16"));
    log::debug!("[CALL] {}", "cuMemsetD16");
    cuMemsetD16()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoD() {
    let cuMemcpyHtoD: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyHtoD"));
    log::debug!("[CALL] {}", "cuMemcpyHtoD");
    cuMemcpyHtoD()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgReportAttachProcedureFinished() {
    let cudbgReportAttachProcedureFinished: extern "C" fn() =
        std::mem::transmute(get_sym("cudbgReportAttachProcedureFinished"));
    log::debug!("[CALL] {}", "cudbgReportAttachProcedureFinished");
    cudbgReportAttachProcedureFinished()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoHAsync() {
    let cuMemcpyAtoHAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAtoHAsync"));
    log::debug!("[CALL] {}", "cuMemcpyAtoHAsync");
    cuMemcpyAtoHAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue32_ptsz() {
    let cuStreamWriteValue32_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue32_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWriteValue32_ptsz");
    cuStreamWriteValue32_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8Async_ptsz() {
    let cuMemsetD8Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD8Async_ptsz"));
    log::debug!("[CALL] {}", "cuMemsetD8Async_ptsz");
    cuMemsetD8Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFree() {
    let cuMemFree: extern "C" fn() = std::mem::transmute(get_sym("cuMemFree"));
    log::debug!("[CALL] {}", "cuMemFree");
    cuMemFree()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsVDPAURegisterVideoSurface() {
    let cuGraphicsVDPAURegisterVideoSurface: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsVDPAURegisterVideoSurface"));
    log::debug!("[CALL] {}", "cuGraphicsVDPAURegisterVideoSurface");
    cuGraphicsVDPAURegisterVideoSurface()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DBatchAsync_ptsz() {
    let cuMemcpy3DBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DBatchAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpy3DBatchAsync_ptsz");
    cuMemcpy3DBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue64_ptsz() {
    let cuStreamWaitValue64_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitValue64_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWaitValue64_ptsz");
    cuStreamWaitValue64_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoD_v2_ptds() {
    let cuMemcpyHtoD_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyHtoD_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyHtoD_v2_ptds");
    cuMemcpyHtoD_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkAddFile() {
    let cuLinkAddFile: extern "C" fn() = std::mem::transmute(get_sym("cuLinkAddFile"));
    log::debug!("[CALL] {}", "cuLinkAddFile");
    cuLinkAddFile()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD16_v2_ptds() {
    let cuMemsetD16_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD16_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemsetD16_v2_ptds");
    cuMemsetD16_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphExecUpdate() {
    let cuGraphExecUpdate: extern "C" fn() = std::mem::transmute(get_sym("cuGraphExecUpdate"));
    log::debug!("[CALL] {}", "cuGraphExecUpdate");
    cuGraphExecUpdate()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D16Async_ptsz() {
    let cuMemsetD2D16Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D16Async_ptsz"));
    log::debug!("[CALL] {}", "cuMemsetD2D16Async_ptsz");
    cuMemsetD2D16Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchCooperativeKernel_ptsz() {
    let cuLaunchCooperativeKernel_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuLaunchCooperativeKernel_ptsz"));
    log::debug!("[CALL] {}", "cuLaunchCooperativeKernel_ptsz");
    cuLaunchCooperativeKernel_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLUnmapBufferObjectAsync() {
    let cuGLUnmapBufferObjectAsync: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLUnmapBufferObjectAsync"));
    log::debug!("[CALL] {}", "cuGLUnmapBufferObjectAsync");
    cuGLUnmapBufferObjectAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddress2D() {
    let cuTexRefSetAddress2D: extern "C" fn() =
        std::mem::transmute(get_sym("cuTexRefSetAddress2D"));
    log::debug!("[CALL] {}", "cuTexRefSetAddress2D");
    cuTexRefSetAddress2D()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObjectAsync_v2_ptsz() {
    let cuGLMapBufferObjectAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObjectAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuGLMapBufferObjectAsync_v2_ptsz");
    cuGLMapBufferObjectAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceSetMapFlags() {
    let cuGraphicsResourceSetMapFlags: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsResourceSetMapFlags"));
    log::debug!("[CALL] {}", "cuGraphicsResourceSetMapFlags");
    cuGraphicsResourceSetMapFlags()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamUpdateCaptureDependencies() {
    let cuStreamUpdateCaptureDependencies: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamUpdateCaptureDependencies"));
    log::debug!("[CALL] {}", "cuStreamUpdateCaptureDependencies");
    cuStreamUpdateCaptureDependencies()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgReportDriverInternalError() {
    let cudbgReportDriverInternalError: extern "C" fn() =
        std::mem::transmute(get_sym("cudbgReportDriverInternalError"));
    log::debug!("[CALL] {}", "cudbgReportDriverInternalError");
    cudbgReportDriverInternalError()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DAsync() {
    let cuMemcpy3DAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy3DAsync"));
    log::debug!("[CALL] {}", "cuMemcpy3DAsync");
    cuMemcpy3DAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoA_v2_ptds() {
    let cuMemcpyDtoA_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoA_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyDtoA_v2_ptds");
    cuMemcpyDtoA_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitEvent_ptsz() {
    let cuStreamWaitEvent_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitEvent_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWaitEvent_ptsz");
    cuStreamWaitEvent_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgGetAPI() {
    let cudbgGetAPI: extern "C" fn() = std::mem::transmute(get_sym("cudbgGetAPI"));
    log::debug!("[CALL] {}", "cudbgGetAPI");
    cudbgGetAPI()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoA() {
    let cuMemcpyDtoA: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoA"));
    log::debug!("[CALL] {}", "cuMemcpyDtoA");
    cuMemcpyDtoA()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoD() {
    let cuMemcpyDtoD: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoD"));
    log::debug!("[CALL] {}", "cuMemcpyDtoD");
    cuMemcpyDtoD()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerReleaseFrame() {
    let cuEGLStreamConsumerReleaseFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerReleaseFrame"));
    log::debug!("[CALL] {}", "cuEGLStreamConsumerReleaseFrame");
    cuEGLStreamConsumerReleaseFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_v2() {
    let cuStreamGetCaptureInfo_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo_v2"));
    log::debug!("[CALL] {}", "cuStreamGetCaptureInfo_v2");
    cuStreamGetCaptureInfo_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCtx_ptsz() {
    let cuStreamGetCtx_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuStreamGetCtx_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetCtx_ptsz");
    cuStreamGetCtx_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2D() {
    let cuMemcpy2D: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy2D"));
    log::debug!("[CALL] {}", "cuMemcpy2D");
    cuMemcpy2D()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoH() {
    let cuMemcpyDtoH: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoH"));
    log::debug!("[CALL] {}", "cuMemcpyDtoH");
    cuMemcpyDtoH()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCaptureToGraph_ptsz() {
    let cuStreamBeginCaptureToGraph_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCaptureToGraph_ptsz"));
    log::debug!("[CALL] {}", "cuStreamBeginCaptureToGraph_ptsz");
    cuStreamBeginCaptureToGraph_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceGetMappedPointer() {
    let cuGraphicsResourceGetMappedPointer: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsResourceGetMappedPointer"));
    log::debug!("[CALL] {}", "cuGraphicsResourceGetMappedPointer");
    cuGraphicsResourceGetMappedPointer()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue32_v2_ptsz() {
    let cuStreamWriteValue32_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue32_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWriteValue32_v2_ptsz");
    cuStreamWriteValue32_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLRegisterBufferObject() {
    let cuGLRegisterBufferObject: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLRegisterBufferObject"));
    log::debug!("[CALL] {}", "cuGLRegisterBufferObject");
    cuGLRegisterBufferObject()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCapture() {
    let cuStreamBeginCapture: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCapture"));
    log::debug!("[CALL] {}", "cuStreamBeginCapture");
    cuStreamBeginCapture()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32() {
    let cuMemsetD32: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD32"));
    log::debug!("[CALL] {}", "cuMemsetD32");
    cuMemsetD32()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLCtxCreate() {
    let cuGLCtxCreate: extern "C" fn() = std::mem::transmute(get_sym("cuGLCtxCreate"));
    log::debug!("[CALL] {}", "cuGLCtxCreate");
    cuGLCtxCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo() {
    let cuStreamGetCaptureInfo: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo"));
    log::debug!("[CALL] {}", "cuStreamGetCaptureInfo");
    cuStreamGetCaptureInfo()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue64() {
    let cuStreamWaitValue64: extern "C" fn() = std::mem::transmute(get_sym("cuStreamWaitValue64"));
    log::debug!("[CALL] {}", "cuStreamWaitValue64");
    cuStreamWaitValue64()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLSetBufferObjectMapFlags() {
    let cuGLSetBufferObjectMapFlags: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLSetBufferObjectMapFlags"));
    log::debug!("[CALL] {}", "cuGLSetBufferObjectMapFlags");
    cuGLSetBufferObjectMapFlags()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoA() {
    let cuMemcpyAtoA: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAtoA"));
    log::debug!("[CALL] {}", "cuMemcpyAtoA");
    cuMemcpyAtoA()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamProducerDisconnect() {
    let cuEGLStreamProducerDisconnect: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamProducerDisconnect"));
    log::debug!("[CALL] {}", "cuEGLStreamProducerDisconnect");
    cuEGLStreamProducerDisconnect()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphRemoveDependencies() {
    let cuGraphRemoveDependencies: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphRemoveDependencies"));
    log::debug!("[CALL] {}", "cuGraphRemoveDependencies");
    cuGraphRemoveDependencies()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoD() {
    let cuMemcpyAtoD: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAtoD"));
    log::debug!("[CALL] {}", "cuMemcpyAtoD");
    cuMemcpyAtoD()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3D() {
    let cuMemcpy3D: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy3D"));
    log::debug!("[CALL] {}", "cuMemcpy3D");
    cuMemcpy3D()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphInstantiateWithParams_ptsz() {
    let cuGraphInstantiateWithParams_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphInstantiateWithParams_ptsz"));
    log::debug!("[CALL] {}", "cuGraphInstantiateWithParams_ptsz");
    cuGraphInstantiateWithParams_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoH() {
    let cuMemcpyAtoH: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyAtoH"));
    log::debug!("[CALL] {}", "cuMemcpyAtoH");
    cuMemcpyAtoH()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgApiAttach() {
    let cudbgApiAttach: extern "C" fn() = std::mem::transmute(get_sym("cudbgApiAttach"));
    log::debug!("[CALL] {}", "cudbgApiAttach");
    cudbgApiAttach()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_ptsz() {
    let cuStreamGetCaptureInfo_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetCaptureInfo_ptsz");
    cuStreamGetCaptureInfo_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerAcquireFrame() {
    let cuEGLStreamConsumerAcquireFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerAcquireFrame"));
    log::debug!("[CALL] {}", "cuEGLStreamConsumerAcquireFrame");
    cuEGLStreamConsumerAcquireFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamProducerConnect() {
    let cuEGLStreamProducerConnect: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamProducerConnect"));
    log::debug!("[CALL] {}", "cuEGLStreamProducerConnect");
    cuEGLStreamProducerConnect()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue64() {
    let cuStreamWriteValue64: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue64"));
    log::debug!("[CALL] {}", "cuStreamWriteValue64");
    cuStreamWriteValue64()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphLaunch_ptsz() {
    let cuGraphLaunch_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuGraphLaunch_ptsz"));
    log::debug!("[CALL] {}", "cuGraphLaunch_ptsz");
    cuGraphLaunch_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxRelease() {
    let cuDevicePrimaryCtxRelease: extern "C" fn() =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxRelease"));
    log::debug!("[CALL] {}", "cuDevicePrimaryCtxRelease");
    cuDevicePrimaryCtxRelease()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoHAsync_v2_ptsz() {
    let cuMemcpyDtoHAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoHAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyDtoHAsync_v2_ptsz");
    cuMemcpyDtoHAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DBatchAsync() {
    let cuMemcpy3DBatchAsync: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DBatchAsync"));
    log::debug!("[CALL] {}", "cuMemcpy3DBatchAsync");
    cuMemcpy3DBatchAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuProfilerInitialize() {
    let cuProfilerInitialize: extern "C" fn() =
        std::mem::transmute(get_sym("cuProfilerInitialize"));
    log::debug!("[CALL] {}", "cuProfilerInitialize");
    cuProfilerInitialize()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D16_v2_ptds() {
    let cuMemsetD2D16_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D16_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemsetD2D16_v2_ptds");
    cuMemsetD2D16_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObject_v2() {
    let cuGLMapBufferObject_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObject_v2"));
    log::debug!("[CALL] {}", "cuGLMapBufferObject_v2");
    cuGLMapBufferObject_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyAtoD_v2_ptds() {
    let cuMemcpyAtoD_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyAtoD_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyAtoD_v2_ptds");
    cuMemcpyAtoD_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLGetDevices() {
    let cuGLGetDevices: extern "C" fn() = std::mem::transmute(get_sym("cuGLGetDevices"));
    log::debug!("[CALL] {}", "cuGLGetDevices");
    cuGLGetDevices()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObject_v2_ptds() {
    let cuGLMapBufferObject_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLMapBufferObject_v2_ptds"));
    log::debug!("[CALL] {}", "cuGLMapBufferObject_v2_ptds");
    cuGLMapBufferObject_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DWithAttributesAsync_ptsz() {
    let cuMemcpy3DWithAttributesAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DWithAttributesAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpy3DWithAttributesAsync_ptsz");
    cuMemcpy3DWithAttributesAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuArrayCreate() {
    let cuArrayCreate: extern "C" fn() = std::mem::transmute(get_sym("cuArrayCreate"));
    log::debug!("[CALL] {}", "cuArrayCreate");
    cuArrayCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetDevice_ptsz() {
    let cuStreamGetDevice_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetDevice_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetDevice_ptsz");
    cuStreamGetDevice_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoHAsync() {
    let cuMemcpyDtoHAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoHAsync"));
    log::debug!("[CALL] {}", "cuMemcpyDtoHAsync");
    cuMemcpyDtoHAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuSubgridCreate() {
    let cuSubgridCreate: extern "C" fn() = std::mem::transmute(get_sym("cuSubgridCreate"));
    log::debug!("[CALL] {}", "cuSubgridCreate");
    cuSubgridCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D8Async_ptsz() {
    let cuMemsetD2D8Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D8Async_ptsz"));
    log::debug!("[CALL] {}", "cuMemsetD2D8Async_ptsz");
    cuMemsetD2D8Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DPeerAsync_ptsz() {
    let cuMemcpy3DPeerAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DPeerAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpy3DPeerAsync_ptsz");
    cuMemcpy3DPeerAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLMapBufferObject() {
    let cuGLMapBufferObject: extern "C" fn() = std::mem::transmute(get_sym("cuGLMapBufferObject"));
    log::debug!("[CALL] {}", "cuGLMapBufferObject");
    cuGLMapBufferObject()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgApiInit() {
    let cudbgApiInit: extern "C" fn() = std::mem::transmute(get_sym("cudbgApiInit"));
    log::debug!("[CALL] {}", "cudbgApiInit");
    cudbgApiInit()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyBatchAsync_ptsz() {
    let cuMemcpyBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyBatchAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyBatchAsync_ptsz");
    cuMemcpyBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32_v2_ptds() {
    let cuMemsetD32_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD32_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemsetD32_v2_ptds");
    cuMemsetD32_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuDriverGetGpuCodeIsaVersion() {
    let cuDriverGetGpuCodeIsaVersion: extern "C" fn() =
        std::mem::transmute(get_sym("cuDriverGetGpuCodeIsaVersion"));
    log::debug!("[CALL] {}", "cuDriverGetGpuCodeIsaVersion");
    cuDriverGetGpuCodeIsaVersion()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemHostGetDevicePointer() {
    let cuMemHostGetDevicePointer: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemHostGetDevicePointer"));
    log::debug!("[CALL] {}", "cuMemHostGetDevicePointer");
    cuMemHostGetDevicePointer()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgGetAPIVersion() {
    let cudbgGetAPIVersion: extern "C" fn() = std::mem::transmute(get_sym("cudbgGetAPIVersion"));
    log::debug!("[CALL] {}", "cudbgGetAPIVersion");
    cudbgGetAPIVersion()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue64_v2_ptsz() {
    let cuStreamWaitValue64_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitValue64_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWaitValue64_v2_ptsz");
    cuStreamWaitValue64_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchHostFunc_v2_ptsz() {
    let cuLaunchHostFunc_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuLaunchHostFunc_v2_ptsz"));
    log::debug!("[CALL] {}", "cuLaunchHostFunc_v2_ptsz");
    cuLaunchHostFunc_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetPriority_ptsz() {
    let cuStreamGetPriority_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetPriority_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetPriority_ptsz");
    cuStreamGetPriority_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoH_v2_ptds() {
    let cuMemcpyDtoH_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoH_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyDtoH_v2_ptds");
    cuMemcpyDtoH_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3D_v2_ptds() {
    let cuMemcpy3D_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy3D_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpy3D_v2_ptds");
    cuMemcpy3D_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2D_v2_ptds() {
    let cuMemcpy2D_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy2D_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpy2D_v2_ptds");
    cuMemcpy2D_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLGetDevices_v2() {
    let cuGLGetDevices_v2: extern "C" fn() = std::mem::transmute(get_sym("cuGLGetDevices_v2"));
    log::debug!("[CALL] {}", "cuGLGetDevices_v2");
    cuGLGetDevices_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoDAsync_v2_ptsz() {
    let cuMemcpyHtoDAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyHtoDAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyHtoDAsync_v2_ptsz");
    cuMemcpyHtoDAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemMapArrayAsync_ptsz() {
    let cuMemMapArrayAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemMapArrayAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemMapArrayAsync_ptsz");
    cuMemMapArrayAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgApiDetach() {
    let cudbgApiDetach: extern "C" fn() = std::mem::transmute(get_sym("cudbgApiDetach"));
    log::debug!("[CALL] {}", "cudbgApiDetach");
    cudbgApiDetach()
}
#[no_mangle]
pub unsafe extern "C" fn cuSubgridWorksetCreate() {
    let cuSubgridWorksetCreate: extern "C" fn() =
        std::mem::transmute(get_sym("cuSubgridWorksetCreate"));
    log::debug!("[CALL] {}", "cuSubgridWorksetCreate");
    cuSubgridWorksetCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoDAsync() {
    let cuMemcpyHtoDAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyHtoDAsync"));
    log::debug!("[CALL] {}", "cuMemcpyHtoDAsync");
    cuMemcpyHtoDAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuArray3DGetDescriptor() {
    let cuArray3DGetDescriptor: extern "C" fn() =
        std::mem::transmute(get_sym("cuArray3DGetDescriptor"));
    log::debug!("[CALL] {}", "cuArray3DGetDescriptor");
    cuArray3DGetDescriptor()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLInit() {
    let cuGLInit: extern "C" fn() = std::mem::transmute(get_sym("cuGLInit"));
    log::debug!("[CALL] {}", "cuGLInit");
    cuGLInit()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchAsync() {
    let cuMemPrefetchAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemPrefetchAsync"));
    log::debug!("[CALL] {}", "cuMemPrefetchAsync");
    cuMemPrefetchAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamIsCapturing_ptsz() {
    let cuStreamIsCapturing_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamIsCapturing_ptsz"));
    log::debug!("[CALL] {}", "cuStreamIsCapturing_ptsz");
    cuStreamIsCapturing_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamQuery_ptsz() {
    let cuStreamQuery_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuStreamQuery_ptsz"));
    log::debug!("[CALL] {}", "cuStreamQuery_ptsz");
    cuStreamQuery_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_v3_ptsz() {
    let cuStreamGetCaptureInfo_v3_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo_v3_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetCaptureInfo_v3_ptsz");
    cuStreamGetCaptureInfo_v3_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cudbgPreInit() {
    let cudbgPreInit: extern "C" fn() = std::mem::transmute(get_sym("cudbgPreInit"));
    log::debug!("[CALL] {}", "cudbgPreInit");
    cudbgPreInit()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAllocPitch() {
    let cuMemAllocPitch: extern "C" fn() = std::mem::transmute(get_sym("cuMemAllocPitch"));
    log::debug!("[CALL] {}", "cuMemAllocPitch");
    cuMemAllocPitch()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamSynchronize_ptsz() {
    let cuStreamSynchronize_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamSynchronize_ptsz"));
    log::debug!("[CALL] {}", "cuStreamSynchronize_ptsz");
    cuStreamSynchronize_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCapture_v2_ptsz() {
    let cuStreamBeginCapture_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCapture_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamBeginCapture_v2_ptsz");
    cuStreamBeginCapture_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemDiscardBatchAsync_ptsz() {
    let cuMemDiscardBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemDiscardBatchAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemDiscardBatchAsync_ptsz");
    cuMemDiscardBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D8() {
    let cuMemsetD2D8: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD2D8"));
    log::debug!("[CALL] {}", "cuMemsetD2D8");
    cuMemsetD2D8()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue32_v2_ptsz() {
    let cuStreamWaitValue32_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitValue32_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWaitValue32_v2_ptsz");
    cuStreamWaitValue32_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventRecordWithFlags_ptsz() {
    let cuEventRecordWithFlags_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuEventRecordWithFlags_ptsz"));
    log::debug!("[CALL] {}", "cuEventRecordWithFlags_ptsz");
    cuEventRecordWithFlags_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyPeerAsync_ptsz() {
    let cuMemcpyPeerAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyPeerAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyPeerAsync_ptsz");
    cuMemcpyPeerAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyBatchAsync_v2_ptsz() {
    let cuMemcpyBatchAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyBatchAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyBatchAsync_v2_ptsz");
    cuMemcpyBatchAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemPrefetchAsync_v2_ptsz() {
    let cuMemPrefetchAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemPrefetchAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemPrefetchAsync_v2_ptsz");
    cuMemPrefetchAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D32_v2_ptds() {
    let cuMemsetD2D32_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D32_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemsetD2D32_v2_ptds");
    cuMemsetD2D32_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLUnregisterBufferObject() {
    let cuGLUnregisterBufferObject: extern "C" fn() =
        std::mem::transmute(get_sym("cuGLUnregisterBufferObject"));
    log::debug!("[CALL] {}", "cuGLUnregisterBufferObject");
    cuGLUnregisterBufferObject()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetDevResource_ptsz() {
    let cuStreamGetDevResource_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetDevResource_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetDevResource_ptsz");
    cuStreamGetDevResource_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD32Async_ptsz() {
    let cuMemsetD32Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD32Async_ptsz"));
    log::debug!("[CALL] {}", "cuMemsetD32Async_ptsz");
    cuMemsetD32Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsUnmapResources_ptsz() {
    let cuGraphicsUnmapResources_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsUnmapResources_ptsz"));
    log::debug!("[CALL] {}", "cuGraphicsUnmapResources_ptsz");
    cuGraphicsUnmapResources_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCaptureInfo_v2_ptsz() {
    let cuStreamGetCaptureInfo_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCaptureInfo_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetCaptureInfo_v2_ptsz");
    cuStreamGetCaptureInfo_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventElapsedTime() {
    let cuEventElapsedTime: extern "C" fn() = std::mem::transmute(get_sym("cuEventElapsedTime"));
    log::debug!("[CALL] {}", "cuEventElapsedTime");
    cuEventElapsedTime()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWriteValue64_ptsz() {
    let cuStreamWriteValue64_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWriteValue64_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWriteValue64_ptsz");
    cuStreamWriteValue64_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphNodeGetDependencies() {
    let cuGraphNodeGetDependencies: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphNodeGetDependencies"));
    log::debug!("[CALL] {}", "cuGraphNodeGetDependencies");
    cuGraphNodeGetDependencies()
}
#[no_mangle]
pub unsafe extern "C" fn cuLinkCreate() {
    let cuLinkCreate: extern "C" fn() = std::mem::transmute(get_sym("cuLinkCreate"));
    log::debug!("[CALL] {}", "cuLinkCreate");
    cuLinkCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchHostFunc_ptsz() {
    let cuLaunchHostFunc_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuLaunchHostFunc_ptsz"));
    log::debug!("[CALL] {}", "cuLaunchHostFunc_ptsz");
    cuLaunchHostFunc_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuProfilerStop() {
    let cuProfilerStop: extern "C" fn() = std::mem::transmute(get_sym("cuProfilerStop"));
    log::debug!("[CALL] {}", "cuProfilerStop");
    cuProfilerStop()
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefSetAddress() {
    let cuTexRefSetAddress: extern "C" fn() = std::mem::transmute(get_sym("cuTexRefSetAddress"));
    log::debug!("[CALL] {}", "cuTexRefSetAddress");
    cuTexRefSetAddress()
}
#[no_mangle]
pub unsafe extern "C" fn cuTexRefGetAddress() {
    let cuTexRefGetAddress: extern "C" fn() = std::mem::transmute(get_sym("cuTexRefGetAddress"));
    log::debug!("[CALL] {}", "cuTexRefGetAddress");
    cuTexRefGetAddress()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxDestroy() {
    let cuCtxDestroy: extern "C" fn() = std::mem::transmute(get_sym("cuCtxDestroy"));
    log::debug!("[CALL] {}", "cuCtxDestroy");
    cuCtxDestroy()
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceGetUuid() {
    let cuDeviceGetUuid: extern "C" fn() = std::mem::transmute(get_sym("cuDeviceGetUuid"));
    log::debug!("[CALL] {}", "cuDeviceGetUuid");
    cuDeviceGetUuid()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBatchMemOp_ptsz() {
    let cuStreamBatchMemOp_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBatchMemOp_ptsz"));
    log::debug!("[CALL] {}", "cuStreamBatchMemOp_ptsz");
    cuStreamBatchMemOp_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphInstantiate() {
    let cuGraphInstantiate: extern "C" fn() = std::mem::transmute(get_sym("cuGraphInstantiate"));
    log::debug!("[CALL] {}", "cuGraphInstantiate");
    cuGraphInstantiate()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD8_v2_ptds() {
    let cuMemsetD8_v2_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD8_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemsetD8_v2_ptds");
    cuMemsetD8_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetCtx_v2_ptsz() {
    let cuStreamGetCtx_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetCtx_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetCtx_v2_ptsz");
    cuStreamGetCtx_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuVDPAUGetDevice() {
    let cuVDPAUGetDevice: extern "C" fn() = std::mem::transmute(get_sym("cuVDPAUGetDevice"));
    log::debug!("[CALL] {}", "cuVDPAUGetDevice");
    cuVDPAUGetDevice()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamAddCallback_ptsz() {
    let cuStreamAddCallback_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamAddCallback_ptsz"));
    log::debug!("[CALL] {}", "cuStreamAddCallback_ptsz");
    cuStreamAddCallback_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D16() {
    let cuMemsetD2D16: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD2D16"));
    log::debug!("[CALL] {}", "cuMemsetD2D16");
    cuMemsetD2D16()
}
#[no_mangle]
pub unsafe extern "C" fn cuEventRecord_ptsz() {
    let cuEventRecord_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuEventRecord_ptsz"));
    log::debug!("[CALL] {}", "cuEventRecord_ptsz");
    cuEventRecord_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerDisconnect() {
    let cuEGLStreamConsumerDisconnect: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerDisconnect"));
    log::debug!("[CALL] {}", "cuEGLStreamConsumerDisconnect");
    cuEGLStreamConsumerDisconnect()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamSetAttribute_ptsz() {
    let cuStreamSetAttribute_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamSetAttribute_ptsz"));
    log::debug!("[CALL] {}", "cuStreamSetAttribute_ptsz");
    cuStreamSetAttribute_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuVDPAUCtxCreate_v2() {
    let cuVDPAUCtxCreate_v2: extern "C" fn() = std::mem::transmute(get_sym("cuVDPAUCtxCreate_v2"));
    log::debug!("[CALL] {}", "cuVDPAUCtxCreate_v2");
    cuVDPAUCtxCreate_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxPushCurrent() {
    let cuCtxPushCurrent: extern "C" fn() = std::mem::transmute(get_sym("cuCtxPushCurrent"));
    log::debug!("[CALL] {}", "cuCtxPushCurrent");
    cuCtxPushCurrent()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphInstantiate_v2() {
    let cuGraphInstantiate_v2: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphInstantiate_v2"));
    log::debug!("[CALL] {}", "cuGraphInstantiate_v2");
    cuGraphInstantiate_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxSetFlags() {
    let cuDevicePrimaryCtxSetFlags: extern "C" fn() =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxSetFlags"));
    log::debug!("[CALL] {}", "cuDevicePrimaryCtxSetFlags");
    cuDevicePrimaryCtxSetFlags()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DPeer_ptds() {
    let cuMemcpy3DPeer_ptds: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy3DPeer_ptds"));
    log::debug!("[CALL] {}", "cuMemcpy3DPeer_ptds");
    cuMemcpy3DPeer_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoD_v2_ptds() {
    let cuMemcpyDtoD_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyDtoD_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemcpyDtoD_v2_ptds");
    cuMemcpyDtoD_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamProducerPresentFrame() {
    let cuEGLStreamProducerPresentFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamProducerPresentFrame"));
    log::debug!("[CALL] {}", "cuEGLStreamProducerPresentFrame");
    cuEGLStreamProducerPresentFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCapture_ptsz() {
    let cuStreamBeginCapture_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCapture_ptsz"));
    log::debug!("[CALL] {}", "cuStreamBeginCapture_ptsz");
    cuStreamBeginCapture_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddNode() {
    let cuGraphAddNode: extern "C" fn() = std::mem::transmute(get_sym("cuGraphAddNode"));
    log::debug!("[CALL] {}", "cuGraphAddNode");
    cuGraphAddNode()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DAsync_v2_ptsz() {
    let cuMemcpy3DAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpy3DAsync_v2_ptsz");
    cuMemcpy3DAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemFreeAsync_ptsz() {
    let cuMemFreeAsync_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuMemFreeAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemFreeAsync_ptsz");
    cuMemFreeAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D8_v2_ptds() {
    let cuMemsetD2D8_v2_ptds: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD2D8_v2_ptds"));
    log::debug!("[CALL] {}", "cuMemsetD2D8_v2_ptds");
    cuMemsetD2D8_v2_ptds()
}
#[no_mangle]
pub unsafe extern "C" fn cuGLCtxCreate_v2() {
    let cuGLCtxCreate_v2: extern "C" fn() = std::mem::transmute(get_sym("cuGLCtxCreate_v2"));
    log::debug!("[CALL] {}", "cuGLCtxCreate_v2");
    cuGLCtxCreate_v2()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamUpdateCaptureDependencies_v2_ptsz() {
    let cuStreamUpdateCaptureDependencies_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamUpdateCaptureDependencies_v2_ptsz"));
    log::debug!("[CALL] {}", "cuStreamUpdateCaptureDependencies_v2_ptsz");
    cuStreamUpdateCaptureDependencies_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyDtoDAsync() {
    let cuMemcpyDtoDAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyDtoDAsync"));
    log::debug!("[CALL] {}", "cuMemcpyDtoDAsync");
    cuMemcpyDtoDAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuEGLStreamConsumerConnect() {
    let cuEGLStreamConsumerConnect: extern "C" fn() =
        std::mem::transmute(get_sym("cuEGLStreamConsumerConnect"));
    log::debug!("[CALL] {}", "cuEGLStreamConsumerConnect");
    cuEGLStreamConsumerConnect()
}
#[no_mangle]
pub unsafe extern "C" fn cuLaunchKernel_ptsz() {
    let cuLaunchKernel_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuLaunchKernel_ptsz"));
    log::debug!("[CALL] {}", "cuLaunchKernel_ptsz");
    cuLaunchKernel_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyBatchAsync() {
    let cuMemcpyBatchAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyBatchAsync"));
    log::debug!("[CALL] {}", "cuMemcpyBatchAsync");
    cuMemcpyBatchAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuProfilerStart() {
    let cuProfilerStart: extern "C" fn() = std::mem::transmute(get_sym("cuProfilerStart"));
    log::debug!("[CALL] {}", "cuProfilerStart");
    cuProfilerStart()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyHtoAAsync() {
    let cuMemcpyHtoAAsync: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpyHtoAAsync"));
    log::debug!("[CALL] {}", "cuMemcpyHtoAAsync");
    cuMemcpyHtoAAsync()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamWaitValue32_ptsz() {
    let cuStreamWaitValue32_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamWaitValue32_ptsz"));
    log::debug!("[CALL] {}", "cuStreamWaitValue32_ptsz");
    cuStreamWaitValue32_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxPopCurrent() {
    let cuCtxPopCurrent: extern "C" fn() = std::mem::transmute(get_sym("cuCtxPopCurrent"));
    log::debug!("[CALL] {}", "cuCtxPopCurrent");
    cuCtxPopCurrent()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemDiscardAndPrefetchBatchAsync_ptsz() {
    let cuMemDiscardAndPrefetchBatchAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemDiscardAndPrefetchBatchAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemDiscardAndPrefetchBatchAsync_ptsz");
    cuMemDiscardAndPrefetchBatchAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsResourceGetMappedEglFrame() {
    let cuGraphicsResourceGetMappedEglFrame: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsResourceGetMappedEglFrame"));
    log::debug!("[CALL] {}", "cuGraphicsResourceGetMappedEglFrame");
    cuGraphicsResourceGetMappedEglFrame()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD2D32() {
    let cuMemsetD2D32: extern "C" fn() = std::mem::transmute(get_sym("cuMemsetD2D32"));
    log::debug!("[CALL] {}", "cuMemsetD2D32");
    cuMemsetD2D32()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamBeginCaptureToCig_ptsz() {
    let cuStreamBeginCaptureToCig_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamBeginCaptureToCig_ptsz"));
    log::debug!("[CALL] {}", "cuStreamBeginCaptureToCig_ptsz");
    cuStreamBeginCaptureToCig_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddDependencies() {
    let cuGraphAddDependencies: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphAddDependencies"));
    log::debug!("[CALL] {}", "cuGraphAddDependencies");
    cuGraphAddDependencies()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy3DBatchAsync_v2_ptsz() {
    let cuMemcpy3DBatchAsync_v2_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpy3DBatchAsync_v2_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpy3DBatchAsync_v2_ptsz");
    cuMemcpy3DBatchAsync_v2_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpyWithAttributesAsync_ptsz() {
    let cuMemcpyWithAttributesAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemcpyWithAttributesAsync_ptsz"));
    log::debug!("[CALL] {}", "cuMemcpyWithAttributesAsync_ptsz");
    cuMemcpyWithAttributesAsync_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphAddKernelNode() {
    let cuGraphAddKernelNode: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphAddKernelNode"));
    log::debug!("[CALL] {}", "cuGraphAddKernelNode");
    cuGraphAddKernelNode()
}
#[no_mangle]
pub unsafe extern "C" fn cuDeviceTotalMem() {
    let cuDeviceTotalMem: extern "C" fn() = std::mem::transmute(get_sym("cuDeviceTotalMem"));
    log::debug!("[CALL] {}", "cuDeviceTotalMem");
    cuDeviceTotalMem()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemsetD16Async_ptsz() {
    let cuMemsetD16Async_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemsetD16Async_ptsz"));
    log::debug!("[CALL] {}", "cuMemsetD16Async_ptsz");
    cuMemsetD16Async_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphicsEGLRegisterImage() {
    let cuGraphicsEGLRegisterImage: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphicsEGLRegisterImage"));
    log::debug!("[CALL] {}", "cuGraphicsEGLRegisterImage");
    cuGraphicsEGLRegisterImage()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemGetAddressRange() {
    let cuMemGetAddressRange: extern "C" fn() =
        std::mem::transmute(get_sym("cuMemGetAddressRange"));
    log::debug!("[CALL] {}", "cuMemGetAddressRange");
    cuMemGetAddressRange()
}
#[no_mangle]
pub unsafe extern "C" fn cuArray3DCreate() {
    let cuArray3DCreate: extern "C" fn() = std::mem::transmute(get_sym("cuArray3DCreate"));
    log::debug!("[CALL] {}", "cuArray3DCreate");
    cuArray3DCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuIpcOpenMemHandle() {
    let cuIpcOpenMemHandle: extern "C" fn() = std::mem::transmute(get_sym("cuIpcOpenMemHandle"));
    log::debug!("[CALL] {}", "cuIpcOpenMemHandle");
    cuIpcOpenMemHandle()
}
#[no_mangle]
pub unsafe extern "C" fn cuCtxCreate() {
    let cuCtxCreate: extern "C" fn() = std::mem::transmute(get_sym("cuCtxCreate"));
    log::debug!("[CALL] {}", "cuCtxCreate");
    cuCtxCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamCopyAttributes_ptsz() {
    let cuStreamCopyAttributes_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamCopyAttributes_ptsz"));
    log::debug!("[CALL] {}", "cuStreamCopyAttributes_ptsz");
    cuStreamCopyAttributes_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuGraphKernelNodeGetParams() {
    let cuGraphKernelNodeGetParams: extern "C" fn() =
        std::mem::transmute(get_sym("cuGraphKernelNodeGetParams"));
    log::debug!("[CALL] {}", "cuGraphKernelNodeGetParams");
    cuGraphKernelNodeGetParams()
}
#[no_mangle]
pub unsafe extern "C" fn cuSubgridWorkerGridCreate() {
    let cuSubgridWorkerGridCreate: extern "C" fn() =
        std::mem::transmute(get_sym("cuSubgridWorkerGridCreate"));
    log::debug!("[CALL] {}", "cuSubgridWorkerGridCreate");
    cuSubgridWorkerGridCreate()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetFlags_ptsz() {
    let cuStreamGetFlags_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetFlags_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetFlags_ptsz");
    cuStreamGetFlags_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemcpy2DUnaligned() {
    let cuMemcpy2DUnaligned: extern "C" fn() = std::mem::transmute(get_sym("cuMemcpy2DUnaligned"));
    log::debug!("[CALL] {}", "cuMemcpy2DUnaligned");
    cuMemcpy2DUnaligned()
}
#[no_mangle]
pub unsafe extern "C" fn cuDevicePrimaryCtxReset() {
    let cuDevicePrimaryCtxReset: extern "C" fn() =
        std::mem::transmute(get_sym("cuDevicePrimaryCtxReset"));
    log::debug!("[CALL] {}", "cuDevicePrimaryCtxReset");
    cuDevicePrimaryCtxReset()
}
#[no_mangle]
pub unsafe extern "C" fn cuMemAdvise() {
    let cuMemAdvise: extern "C" fn() = std::mem::transmute(get_sym("cuMemAdvise"));
    log::debug!("[CALL] {}", "cuMemAdvise");
    cuMemAdvise()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetAttribute_ptsz() {
    let cuStreamGetAttribute_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuStreamGetAttribute_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetAttribute_ptsz");
    cuStreamGetAttribute_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuStreamGetId_ptsz() {
    let cuStreamGetId_ptsz: extern "C" fn() = std::mem::transmute(get_sym("cuStreamGetId_ptsz"));
    log::debug!("[CALL] {}", "cuStreamGetId_ptsz");
    cuStreamGetId_ptsz()
}
#[no_mangle]
pub unsafe extern "C" fn cuWaitExternalSemaphoresAsync_ptsz() {
    let cuWaitExternalSemaphoresAsync_ptsz: extern "C" fn() =
        std::mem::transmute(get_sym("cuWaitExternalSemaphoresAsync_ptsz"));
    log::debug!("[CALL] {}", "cuWaitExternalSemaphoresAsync_ptsz");
    cuWaitExternalSemaphoresAsync_ptsz()
}
