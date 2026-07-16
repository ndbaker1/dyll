// Bindgen-generated bindings
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type __darwin_nl_item = ::std::os::raw::c_int;
pub type __darwin_wctrans_t = ::std::os::raw::c_int;
pub type __darwin_wctype_t = __uint32_t;
pub type idtype_t = ::std::os::raw::c_uint;
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type sig_atomic_t = ::std::os::raw::c_int;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = i64;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = i64;
pub type user_long_t = i64;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = i64;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int64_t;
pub type mcontext_t = *mut __darwin_mcontext64;
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type stack_t = __darwin_sigaltstack;
pub type ucontext_t = __darwin_ucontext;
pub type sigset_t = __darwin_sigset_t;
pub type uid_t = __darwin_uid_t;
pub type siginfo_t = __siginfo;
pub type sig_t = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub type rlim_t = __uint64_t;
pub type rusage_info_t = *mut ::std::os::raw::c_void;
pub type rusage_info_current = rusage_info_v6;
pub type ct_rune_t = __darwin_ct_rune_t;
pub type rune_t = __darwin_rune_t;
pub type wchar_t = __darwin_wchar_t;
pub type malloc_type_id_t = ::std::os::raw::c_ulonglong;
pub type malloc_zone_t = _malloc_zone_t;
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
pub type cuuint32_t = u32;
pub type cuuint64_t = u64;
pub type CUdeviceptr_v2 = ::std::os::raw::c_ulonglong;
pub type CUdeviceptr = CUdeviceptr_v2;
pub type CUdevice_v1 = ::std::os::raw::c_int;
pub type CUdevice = CUdevice_v1;
pub type CUcontext = *mut CUctx_st;
pub type CUmodule = *mut CUmod_st;
pub type CUfunction = *mut CUfunc_st;
pub type CUlibrary = *mut CUlib_st;
pub type CUkernel = *mut CUkern_st;
pub type CUarray = *mut CUarray_st;
pub type CUmipmappedArray = *mut CUmipmappedArray_st;
pub type CUtexref = *mut CUtexref_st;
pub type CUsurfref = *mut CUsurfref_st;
pub type CUevent = *mut CUevent_st;
pub type CUstream = *mut CUstream_st;
pub type CUgraphicsResource = *mut CUgraphicsResource_st;
pub type CUtexObject_v1 = ::std::os::raw::c_ulonglong;
pub type CUtexObject = CUtexObject_v1;
pub type CUsurfObject_v1 = ::std::os::raw::c_ulonglong;
pub type CUsurfObject = CUsurfObject_v1;
pub type CUexternalMemory = *mut CUextMemory_st;
pub type CUexternalSemaphore = *mut CUextSemaphore_st;
pub type CUgraph = *mut CUgraph_st;
pub type CUgraphNode = *mut CUgraphNode_st;
pub type CUgraphExec = *mut CUgraphExec_st;
pub type CUmemoryPool = *mut CUmemPoolHandle_st;
pub type CUuserObject = *mut CUuserObject_st;
pub type CUgraphConditionalHandle = cuuint64_t;
pub type CUgraphDeviceNode = *mut CUgraphDeviceUpdatableNode_st;
pub type CUasyncCallbackHandle = *mut CUasyncCallbackEntry_st;
pub type CUgreenCtx = *mut CUgreenCtx_st;
pub type CUuuid = CUuuid_st;
pub type CUmemFabricHandle_v1 = CUmemFabricHandle_st;
pub type CUmemFabricHandle = CUmemFabricHandle_v1;
pub type CUipcEventHandle_v1 = CUipcEventHandle_st;
pub type CUipcEventHandle = CUipcEventHandle_v1;
pub type CUipcMemHandle_v1 = CUipcMemHandle_st;
pub type CUipcMemHandle = CUipcMemHandle_v1;
pub type CUipcMem_flags_enum = ::std::os::raw::c_uint;
pub type CUmemAttach_flags_enum = ::std::os::raw::c_uint;
pub type CUctx_flags_enum = ::std::os::raw::c_uint;
pub type CUevent_sched_flags_enum = ::std::os::raw::c_uint;
pub type cl_event_flags_enum = ::std::os::raw::c_uint;
pub type cl_context_flags_enum = ::std::os::raw::c_uint;
pub type CUhostTaskSyncMode_enum = ::std::os::raw::c_uint;
pub type CUstream_flags_enum = ::std::os::raw::c_uint;
pub type CUevent_flags_enum = ::std::os::raw::c_uint;
pub type CUevent_record_flags_enum = ::std::os::raw::c_uint;
pub type CUevent_wait_flags_enum = ::std::os::raw::c_uint;
pub type CUatomicOperation_enum = ::std::os::raw::c_uint;
pub type CUatomicOperationCapability_enum = ::std::os::raw::c_uint;
pub type CUstreamWaitValue_flags_enum = ::std::os::raw::c_uint;
pub type CUstreamWriteValue_flags_enum = ::std::os::raw::c_uint;
pub type CUstreamBatchMemOpType_enum = ::std::os::raw::c_uint;
pub type CUstreamMemoryBarrier_flags_enum = ::std::os::raw::c_uint;
pub type CUstreamAtomicReductionOpType_enum = ::std::os::raw::c_uint;
pub type CUstreamAtomicReductionDataType_enum = ::std::os::raw::c_uint;
pub type CUstreamBatchMemOpParams_v1 = CUstreamBatchMemOpParams_union;
pub type CUstreamBatchMemOpParams = CUstreamBatchMemOpParams_v1;
pub type CUDA_BATCH_MEM_OP_NODE_PARAMS_v1 = CUDA_BATCH_MEM_OP_NODE_PARAMS_v1_st;
pub type CUDA_BATCH_MEM_OP_NODE_PARAMS = CUDA_BATCH_MEM_OP_NODE_PARAMS_v1;
pub type CUDA_BATCH_MEM_OP_NODE_PARAMS_v2 = CUDA_BATCH_MEM_OP_NODE_PARAMS_v2_st;
pub type CUoccupancy_flags_enum = ::std::os::raw::c_uint;
pub type CUstreamUpdateCaptureDependencies_flags_enum = ::std::os::raw::c_uint;
pub type CUasyncNotificationType_enum = ::std::os::raw::c_uint;
pub type CUasyncNotificationInfo = CUasyncNotificationInfo_st;
pub type CUasyncCallback = ::std::option::Option<
    unsafe extern "C" fn(
        info: *mut CUasyncNotificationInfo,
        userData: *mut ::std::os::raw::c_void,
        callback: CUasyncCallbackHandle,
    ),
>;
pub type CUarray_format_enum = ::std::os::raw::c_uint;
pub type CUaddress_mode_enum = ::std::os::raw::c_uint;
pub type CUfilter_mode_enum = ::std::os::raw::c_uint;
pub type CUdevice_attribute_enum = ::std::os::raw::c_uint;
pub type CUdevprop_v1 = CUdevprop_st;
pub type CUdevprop = CUdevprop_v1;
pub type CUpointer_attribute_enum = ::std::os::raw::c_uint;
pub type CUfunction_attribute_enum = ::std::os::raw::c_uint;
pub type CUfunc_cache_enum = ::std::os::raw::c_uint;
pub type CUsharedconfig_enum = ::std::os::raw::c_uint;
pub type CUshared_carveout_enum = ::std::os::raw::c_int;
pub type CUmemorytype_enum = ::std::os::raw::c_uint;
pub type CUcomputemode_enum = ::std::os::raw::c_uint;
pub type CUmem_advise_enum = ::std::os::raw::c_uint;
pub type CUmem_range_attribute_enum = ::std::os::raw::c_uint;
pub type CUjit_option_enum = ::std::os::raw::c_uint;
pub type CUjit_target_enum = ::std::os::raw::c_uint;
pub type CUjit_fallback_enum = ::std::os::raw::c_uint;
pub type CUjit_cacheMode_enum = ::std::os::raw::c_uint;
pub type CUjitInputType_enum = ::std::os::raw::c_uint;
pub type CUlinkState = *mut CUlinkState_st;
pub type CUgraphicsRegisterFlags_enum = ::std::os::raw::c_uint;
pub type CUgraphicsMapResourceFlags_enum = ::std::os::raw::c_uint;
pub type CUarray_cubemap_face_enum = ::std::os::raw::c_uint;
pub type CUlimit_enum = ::std::os::raw::c_uint;
pub type CUresourcetype_enum = ::std::os::raw::c_uint;
pub type CUhostFn =
    ::std::option::Option<unsafe extern "C" fn(userData: *mut ::std::os::raw::c_void)>;
pub type CUaccessProperty_enum = ::std::os::raw::c_uint;
pub type CUaccessPolicyWindow_v1 = CUaccessPolicyWindow_st;
pub type CUaccessPolicyWindow = CUaccessPolicyWindow_v1;
pub type CUDA_KERNEL_NODE_PARAMS_v1 = CUDA_KERNEL_NODE_PARAMS_st;
pub type CUDA_KERNEL_NODE_PARAMS_v2 = CUDA_KERNEL_NODE_PARAMS_v2_st;
pub type CUDA_KERNEL_NODE_PARAMS = CUDA_KERNEL_NODE_PARAMS_v2;
pub type CUDA_KERNEL_NODE_PARAMS_v3 = CUDA_KERNEL_NODE_PARAMS_v3_st;
pub type CUDA_MEMSET_NODE_PARAMS_v1 = CUDA_MEMSET_NODE_PARAMS_st;
pub type CUDA_MEMSET_NODE_PARAMS = CUDA_MEMSET_NODE_PARAMS_v1;
pub type CUDA_MEMSET_NODE_PARAMS_v2 = CUDA_MEMSET_NODE_PARAMS_v2_st;
pub type CUDA_HOST_NODE_PARAMS_v1 = CUDA_HOST_NODE_PARAMS_st;
pub type CUDA_HOST_NODE_PARAMS = CUDA_HOST_NODE_PARAMS_v1;
pub type CUDA_HOST_NODE_PARAMS_v2 = CUDA_HOST_NODE_PARAMS_v2_st;
pub type CUgraphConditionalNodeType_enum = ::std::os::raw::c_uint;
pub type CUgraphNodeType_enum = ::std::os::raw::c_uint;
pub type CUgraphDependencyType_enum = ::std::os::raw::c_uint;
pub type CUgraphEdgeData = CUgraphEdgeData_st;
pub type CUgraphInstantiateResult_enum = ::std::os::raw::c_uint;
pub type CUDA_GRAPH_INSTANTIATE_PARAMS = CUDA_GRAPH_INSTANTIATE_PARAMS_st;
pub type CUsynchronizationPolicy_enum = ::std::os::raw::c_uint;
pub type CUclusterSchedulingPolicy_enum = ::std::os::raw::c_uint;
pub type CUlaunchMemSyncDomain_enum = ::std::os::raw::c_uint;
pub type CUlaunchMemSyncDomainMap = CUlaunchMemSyncDomainMap_st;
pub type CUlaunchAttributePortableClusterMode_enum = ::std::os::raw::c_uint;
pub type CUsharedMemoryMode_enum = ::std::os::raw::c_uint;
pub type CUlaunchAttributeID_enum = ::std::os::raw::c_uint;
pub type CUlaunchAttributeValue = CUlaunchAttributeValue_union;
pub type CUlaunchAttribute = CUlaunchAttribute_st;
pub type CUlaunchConfig = CUlaunchConfig_st;
pub type CUkernelNodeAttrValue_v1 = CUlaunchAttributeValue;
pub type CUkernelNodeAttrValue = CUkernelNodeAttrValue_v1;
pub type CUstreamCaptureStatus_enum = ::std::os::raw::c_uint;
pub type CUstreamCaptureMode_enum = ::std::os::raw::c_uint;
pub type CUstreamAttrValue_v1 = CUlaunchAttributeValue;
pub type CUstreamAttrValue = CUstreamAttrValue_v1;
pub type CUdriverProcAddress_flags_enum = ::std::os::raw::c_uint;
pub type CUdriverProcAddressQueryResult_enum = ::std::os::raw::c_uint;
pub type CUexecAffinityType_enum = ::std::os::raw::c_uint;
pub type CUexecAffinitySmCount_v1 = CUexecAffinitySmCount_st;
pub type CUexecAffinitySmCount = CUexecAffinitySmCount_v1;
pub type CUexecAffinityParam_v1 = CUexecAffinityParam_st;
pub type CUexecAffinityParam = CUexecAffinityParam_v1;
pub type CUcigDataType_enum = ::std::os::raw::c_uint;
pub type CUctxCigParam = CUctxCigParam_st;
pub type CUctxCreateParams = CUctxCreateParams_st;
pub type CUstreamCigDataType_enum = ::std::os::raw::c_uint;
pub type CUstreamCigParam = CUstreamCigParam_st;
pub type CUstreamCigCaptureParams = CUstreamCigCaptureParams_st;
pub type CUlibraryOption_enum = ::std::os::raw::c_uint;
pub type CUlibraryHostUniversalFunctionAndDataTable = CUlibraryHostUniversalFunctionAndDataTable_st;
pub type cudaError_enum = ::std::os::raw::c_uint;
pub type CUdevice_P2PAttribute_enum = ::std::os::raw::c_uint;
pub type CUstreamCallback = ::std::option::Option<
    unsafe extern "C" fn(
        hStream: CUstream,
        status: CUresult,
        userData: *mut ::std::os::raw::c_void,
    ),
>;
pub type CUoccupancyB2DSize =
    ::std::option::Option<unsafe extern "C" fn(blockSize: ::std::os::raw::c_int) -> usize>;
pub type CUDA_MEMCPY2D_v2 = CUDA_MEMCPY2D_st;
pub type CUDA_MEMCPY2D = CUDA_MEMCPY2D_v2;
pub type CUDA_MEMCPY3D_v2 = CUDA_MEMCPY3D_st;
pub type CUDA_MEMCPY3D = CUDA_MEMCPY3D_v2;
pub type CUDA_MEMCPY3D_PEER_v1 = CUDA_MEMCPY3D_PEER_st;
pub type CUDA_MEMCPY3D_PEER = CUDA_MEMCPY3D_PEER_v1;
pub type CUDA_MEMCPY_NODE_PARAMS = CUDA_MEMCPY_NODE_PARAMS_st;
pub type CUDA_ARRAY_DESCRIPTOR_v2 = CUDA_ARRAY_DESCRIPTOR_st;
pub type CUDA_ARRAY_DESCRIPTOR = CUDA_ARRAY_DESCRIPTOR_v2;
pub type CUDA_ARRAY3D_DESCRIPTOR_v2 = CUDA_ARRAY3D_DESCRIPTOR_st;
pub type CUDA_ARRAY3D_DESCRIPTOR = CUDA_ARRAY3D_DESCRIPTOR_v2;
pub type CUDA_ARRAY_SPARSE_PROPERTIES_v1 = CUDA_ARRAY_SPARSE_PROPERTIES_st;
pub type CUDA_ARRAY_SPARSE_PROPERTIES = CUDA_ARRAY_SPARSE_PROPERTIES_v1;
pub type CUDA_ARRAY_MEMORY_REQUIREMENTS_v1 = CUDA_ARRAY_MEMORY_REQUIREMENTS_st;
pub type CUDA_ARRAY_MEMORY_REQUIREMENTS = CUDA_ARRAY_MEMORY_REQUIREMENTS_v1;
pub type CUDA_RESOURCE_DESC_v1 = CUDA_RESOURCE_DESC_st;
pub type CUDA_RESOURCE_DESC = CUDA_RESOURCE_DESC_v1;
pub type CUDA_TEXTURE_DESC_v1 = CUDA_TEXTURE_DESC_st;
pub type CUDA_TEXTURE_DESC = CUDA_TEXTURE_DESC_v1;
pub type CUresourceViewFormat_enum = ::std::os::raw::c_uint;
pub type CUDA_RESOURCE_VIEW_DESC_v1 = CUDA_RESOURCE_VIEW_DESC_st;
pub type CUDA_RESOURCE_VIEW_DESC = CUDA_RESOURCE_VIEW_DESC_v1;
pub type CUtensorMap = CUtensorMap_st;
pub type CUtensorMapDataType_enum = ::std::os::raw::c_uint;
pub type CUtensorMapInterleave_enum = ::std::os::raw::c_uint;
pub type CUtensorMapSwizzle_enum = ::std::os::raw::c_uint;
pub type CUtensorMapL2promotion_enum = ::std::os::raw::c_uint;
pub type CUtensorMapFloatOOBfill_enum = ::std::os::raw::c_uint;
pub type CUtensorMapIm2ColWideMode_enum = ::std::os::raw::c_uint;
pub type CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_v1 = CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_st;
pub type CUDA_POINTER_ATTRIBUTE_P2P_TOKENS = CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_v1;
pub type CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum = ::std::os::raw::c_uint;
pub type CUDA_LAUNCH_PARAMS_v1 = CUDA_LAUNCH_PARAMS_st;
pub type CUDA_LAUNCH_PARAMS = CUDA_LAUNCH_PARAMS_v1;
pub type CUexternalMemoryHandleType_enum = ::std::os::raw::c_uint;
pub type CUDA_EXTERNAL_MEMORY_HANDLE_DESC_v1 = CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st;
pub type CUDA_EXTERNAL_MEMORY_HANDLE_DESC = CUDA_EXTERNAL_MEMORY_HANDLE_DESC_v1;
pub type CUDA_EXTERNAL_MEMORY_BUFFER_DESC_v1 = CUDA_EXTERNAL_MEMORY_BUFFER_DESC_st;
pub type CUDA_EXTERNAL_MEMORY_BUFFER_DESC = CUDA_EXTERNAL_MEMORY_BUFFER_DESC_v1;
pub type CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_v1 =
    CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_st;
pub type CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC = CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_v1;
pub type CUexternalSemaphoreHandleType_enum = ::std::os::raw::c_uint;
pub type CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_v1 = CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st;
pub type CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC = CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_v1;
pub type CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_v1 = CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st;
pub type CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS = CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_v1;
pub type CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_v1 = CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st;
pub type CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS = CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_v1;
pub type CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v1 = CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_st;
pub type CUDA_EXT_SEM_SIGNAL_NODE_PARAMS = CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v1;
pub type CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2 = CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2_st;
pub type CUDA_EXT_SEM_WAIT_NODE_PARAMS_v1 = CUDA_EXT_SEM_WAIT_NODE_PARAMS_st;
pub type CUDA_EXT_SEM_WAIT_NODE_PARAMS = CUDA_EXT_SEM_WAIT_NODE_PARAMS_v1;
pub type CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2 = CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2_st;
pub type CUmemGenericAllocationHandle_v1 = ::std::os::raw::c_ulonglong;
pub type CUmemGenericAllocationHandle = CUmemGenericAllocationHandle_v1;
pub type CUmemAllocationHandleType_enum = ::std::os::raw::c_uint;
pub type CUmemAccess_flags_enum = ::std::os::raw::c_uint;
pub type CUmemLocationType_enum = ::std::os::raw::c_uint;
pub type CUmemAllocationType_enum = ::std::os::raw::c_uint;
pub type CUmemAllocationGranularity_flags_enum = ::std::os::raw::c_uint;
pub type CUmemRangeHandleType_enum = ::std::os::raw::c_uint;
pub type CUmemRangeFlags_enum = ::std::os::raw::c_uint;
pub type CUarraySparseSubresourceType_enum = ::std::os::raw::c_uint;
pub type CUmemOperationType_enum = ::std::os::raw::c_uint;
pub type CUmemHandleType_enum = ::std::os::raw::c_uint;
pub type CUarrayMapInfo_v1 = CUarrayMapInfo_st;
pub type CUarrayMapInfo = CUarrayMapInfo_v1;
pub type CUmemLocation_v1 = CUmemLocation_st;
pub type CUmemLocation = CUmemLocation_v1;
pub type CUmemAllocationCompType_enum = ::std::os::raw::c_uint;
pub type CUmemAllocationProp_v1 = CUmemAllocationProp_st;
pub type CUmemAllocationProp = CUmemAllocationProp_v1;
pub type CUmulticastGranularity_flags_enum = ::std::os::raw::c_uint;
pub type CUmulticastObjectProp_v1 = CUmulticastObjectProp_st;
pub type CUmulticastObjectProp = CUmulticastObjectProp_v1;
pub type CUmemAccessDesc_v1 = CUmemAccessDesc_st;
pub type CUmemAccessDesc = CUmemAccessDesc_v1;
pub type CUgraphExecUpdateResult_enum = ::std::os::raw::c_uint;
pub type CUgraphExecUpdateResultInfo_v1 = CUgraphExecUpdateResultInfo_st;
pub type CUgraphExecUpdateResultInfo = CUgraphExecUpdateResultInfo_v1;
pub type CUmemPool_attribute_enum = ::std::os::raw::c_uint;
pub type CUmemPoolProps_v1 = CUmemPoolProps_st;
pub type CUmemPoolProps = CUmemPoolProps_v1;
pub type CUmemPoolPtrExportData_v1 = CUmemPoolPtrExportData_st;
pub type CUmemPoolPtrExportData = CUmemPoolPtrExportData_v1;
pub type CUmemcpyFlags_enum = ::std::os::raw::c_uint;
pub type CUmemcpySrcAccessOrder_enum = ::std::os::raw::c_uint;
pub type CUmemcpyAttributes_v1 = CUmemcpyAttributes_st;
pub type CUmemcpyAttributes = CUmemcpyAttributes_v1;
pub type CUmemcpy3DOperandType_enum = ::std::os::raw::c_uint;
pub type CUoffset3D_v1 = CUoffset3D_st;
pub type CUoffset3D = CUoffset3D_v1;
pub type CUextent3D_v1 = CUextent3D_st;
pub type CUextent3D = CUextent3D_v1;
pub type CUmemcpy3DOperand_v1 = CUmemcpy3DOperand_st;
pub type CUmemcpy3DOperand = CUmemcpy3DOperand_v1;
pub type CUDA_MEMCPY3D_BATCH_OP_v1 = CUDA_MEMCPY3D_BATCH_OP_st;
pub type CUDA_MEMCPY3D_BATCH_OP = CUDA_MEMCPY3D_BATCH_OP_v1;
pub type CUDA_MEM_ALLOC_NODE_PARAMS_v1 = CUDA_MEM_ALLOC_NODE_PARAMS_v1_st;
pub type CUDA_MEM_ALLOC_NODE_PARAMS = CUDA_MEM_ALLOC_NODE_PARAMS_v1;
pub type CUDA_MEM_ALLOC_NODE_PARAMS_v2 = CUDA_MEM_ALLOC_NODE_PARAMS_v2_st;
pub type CUDA_MEM_FREE_NODE_PARAMS = CUDA_MEM_FREE_NODE_PARAMS_st;
pub type CUgraphMem_attribute_enum = ::std::os::raw::c_uint;
pub type CUgraphChildGraphNodeOwnership_enum = ::std::os::raw::c_int;
pub type CUDA_CHILD_GRAPH_NODE_PARAMS = CUDA_CHILD_GRAPH_NODE_PARAMS_st;
pub type CUDA_EVENT_RECORD_NODE_PARAMS = CUDA_EVENT_RECORD_NODE_PARAMS_st;
pub type CUDA_EVENT_WAIT_NODE_PARAMS = CUDA_EVENT_WAIT_NODE_PARAMS_st;
pub type CUgraphNodeParams = CUgraphNodeParams_st;
pub type CUflushGPUDirectRDMAWritesOptions_enum = ::std::os::raw::c_uint;
pub type CUGPUDirectRDMAWritesOrdering_enum = ::std::os::raw::c_uint;
pub type CUflushGPUDirectRDMAWritesScope_enum = ::std::os::raw::c_uint;
pub type CUflushGPUDirectRDMAWritesTarget_enum = ::std::os::raw::c_uint;
pub type CUgraphDebugDot_flags_enum = ::std::os::raw::c_uint;
pub type CUuserObject_flags_enum = ::std::os::raw::c_uint;
pub type CUuserObjectRetain_flags_enum = ::std::os::raw::c_uint;
pub type CUgraphInstantiate_flags_enum = ::std::os::raw::c_uint;
pub type CUdeviceNumaConfig_enum = ::std::os::raw::c_uint;
pub type CUprocessState_enum = ::std::os::raw::c_uint;
pub type CUcheckpointLockArgs = CUcheckpointLockArgs_st;
pub type CUcheckpointCheckpointArgs = CUcheckpointCheckpointArgs_st;
pub type CUcheckpointGpuPair = CUcheckpointGpuPair_st;
pub type CUcheckpointRestoreArgs = CUcheckpointRestoreArgs_st;
pub type CUcheckpointUnlockArgs = CUcheckpointUnlockArgs_st;
pub type CUmoduleLoadingMode_enum = ::std::os::raw::c_uint;
pub type CUmemDecompressAlgorithm_enum = ::std::os::raw::c_uint;
pub type CUmemDecompressParams = CUmemDecompressParams_st;
pub type CUlogicalEndpointId = cuuint32_t;
pub type CUlogicalEndpointIpcHandleType_enum = ::std::os::raw::c_uint;
pub type CUlogicalEndpointFabricHandle = CUlogicalEndpointFabricHandle_st;
pub type CUlogicalEndpointType_enum = ::std::os::raw::c_uint;
pub type CUlogicalEndpointFlag_enum = ::std::os::raw::c_uint;
pub type CUlogicalEndpointProp = CUlogicalEndpointProp_struct;
pub type CUgraphRecaptureStatus_enum = ::std::os::raw::c_uint;
pub type CUgraphRecaptureCallback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut ::std::os::raw::c_void,
        node: CUgraphNode,
        originalParams: *const CUgraphNodeParams,
        recaptureParams: *const CUgraphNodeParams,
        status: CUgraphRecaptureStatus,
    ) -> CUresult,
>;
pub type CUfunctionLoadingState_enum = ::std::os::raw::c_uint;
pub type CUcoredumpSettings_enum = ::std::os::raw::c_uint;
pub type CUCoredumpGenerationFlags = ::std::os::raw::c_int;
pub type CUcoredumpCallbackHandle = *mut CUcoredumpCallbackEntry_st;
pub type CUcoredumpStatusCallback = ::std::option::Option<
    unsafe extern "C" fn(
        userData: *mut ::std::os::raw::c_void,
        pid: ::std::os::raw::c_int,
        dev: CUdevice,
    ),
>;
pub type CUdevResourceDesc = *mut CUdevResourceDesc_st;
pub type CUgreenCtxCreate_flags = ::std::os::raw::c_uint;
pub type CUdevSmResourceGroup_flags = ::std::os::raw::c_uint;
pub type CUdevSmResourceSplitByCount_flags = ::std::os::raw::c_uint;
pub type CUdevResourceType = ::std::os::raw::c_uint;
pub type CUdevSmResource = CUdevSmResource_st;
pub type CUdevWorkqueueConfigScope = ::std::os::raw::c_uint;
pub type CUdevWorkqueueConfigResource = CUdevWorkqueueConfigResource_st;
pub type CUdevWorkqueueResource = CUdevWorkqueueResource_st;
pub type CU_DEV_SM_RESOURCE_GROUP_PARAMS = CU_DEV_SM_RESOURCE_GROUP_PARAMS_st;
pub type CUdevResource_v1 = CUdevResource_st;
pub type CUdevResource = CUdevResource_v1;
pub type CUlogLevel_enum = ::std::os::raw::c_uint;
pub type CUlogsCallbackHandle = *mut CUlogsCallbackEntry_st;
pub type CUlogsCallback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut ::std::os::raw::c_void,
        logLevel: CUlogLevel,
        message: *mut ::std::os::raw::c_char,
        length: usize,
    ),
>;
pub type CUlogIterator = ::std::os::raw::c_uint;
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
pub type __uint128_t = u128;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state {
    pub __exception: __uint32_t,
    pub __fsr: __uint32_t,
    pub __far: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state64 {
    pub __far: __uint64_t,
    pub __esr: __uint32_t,
    pub __exception: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state64_v2 {
    pub __far: __uint64_t,
    pub __esr: __uint64_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state {
    pub __r: [__uint32_t; 13usize],
    pub __sp: __uint32_t,
    pub __lr: __uint32_t,
    pub __pc: __uint32_t,
    pub __cpsr: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state64 {
    pub __x: [__uint64_t; 29usize],
    pub __fp: __uint64_t,
    pub __lr: __uint64_t,
    pub __sp: __uint64_t,
    pub __pc: __uint64_t,
    pub __cpsr: __uint32_t,
    pub __pad: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_vfp_state {
    pub __r: [__uint32_t; 64usize],
    pub __fpscr: __uint32_t,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_neon_state64 {
    pub __v: [__uint128_t; 32usize],
    pub __fpsr: __uint32_t,
    pub __fpcr: __uint32_t,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_neon_state {
    pub __v: [__uint128_t; 16usize],
    pub __fpsr: __uint32_t,
    pub __fpcr: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __arm_pagein_state {
    pub __pagein_error: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_sme_state {
    pub __svcr: __uint64_t,
    pub __tpidr2_el0: __uint64_t,
    pub __svl_b: __uint16_t,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_sve_z_state {
    pub __z: [[::std::os::raw::c_char; 256usize]; 16usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_sve_p_state {
    pub __p: [[::std::os::raw::c_char; 32usize]; 16usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_sme_za_state {
    pub __za: [::std::os::raw::c_char; 4096usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_sme2_state {
    pub __zt0: [::std::os::raw::c_char; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __arm_legacy_debug_state {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_debug_state32 {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_debug_state64 {
    pub __bvr: [__uint64_t; 16usize],
    pub __bcr: [__uint64_t; 16usize],
    pub __wvr: [__uint64_t; 16usize],
    pub __wcr: [__uint64_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_cpmu_state64 {
    pub __ctrs: [__uint64_t; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mcontext32 {
    pub __es: __darwin_arm_exception_state,
    pub __ss: __darwin_arm_thread_state,
    pub __fs: __darwin_arm_vfp_state,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mcontext64 {
    pub __es: __darwin_arm_exception_state64,
    pub __ss: __darwin_arm_thread_state64,
    pub __ns: __darwin_arm_neon_state64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_sigaltstack {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_size: __darwin_size_t,
    pub ss_flags: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_ucontext {
    pub uc_onstack: ::std::os::raw::c_int,
    pub uc_sigmask: __darwin_sigset_t,
    pub uc_stack: __darwin_sigaltstack,
    pub uc_link: *mut __darwin_ucontext,
    pub uc_mcsize: __darwin_size_t,
    pub uc_mcontext: *mut __darwin_mcontext64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_notify: ::std::os::raw::c_int,
    pub sigev_signo: ::std::os::raw::c_int,
    pub sigev_value: sigval,
    pub sigev_notify_function: ::std::option::Option<unsafe extern "C" fn(arg1: sigval)>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __siginfo {
    pub si_signo: ::std::os::raw::c_int,
    pub si_errno: ::std::os::raw::c_int,
    pub si_code: ::std::os::raw::c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: ::std::os::raw::c_int,
    pub si_addr: *mut ::std::os::raw::c_void,
    pub si_value: sigval,
    pub si_band: ::std::os::raw::c_long,
    pub __pad: [::std::os::raw::c_ulong; 7usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_tramp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *mut siginfo_t,
            arg5: *mut ::std::os::raw::c_void,
        ),
    >,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigvec {
    pub sv_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub sv_mask: ::std::os::raw::c_int,
    pub sv_flags: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigstack {
    pub ss_sp: *mut ::std::os::raw::c_char,
    pub ss_onstack: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: ::std::os::raw::c_long,
    pub ru_ixrss: ::std::os::raw::c_long,
    pub ru_idrss: ::std::os::raw::c_long,
    pub ru_isrss: ::std::os::raw::c_long,
    pub ru_minflt: ::std::os::raw::c_long,
    pub ru_majflt: ::std::os::raw::c_long,
    pub ru_nswap: ::std::os::raw::c_long,
    pub ru_inblock: ::std::os::raw::c_long,
    pub ru_oublock: ::std::os::raw::c_long,
    pub ru_msgsnd: ::std::os::raw::c_long,
    pub ru_msgrcv: ::std::os::raw::c_long,
    pub ru_nsignals: ::std::os::raw::c_long,
    pub ru_nvcsw: ::std::os::raw::c_long,
    pub ru_nivcsw: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v0 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v1 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v2 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v3 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v4 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v5 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
    pub ri_flags: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v6 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
    pub ri_flags: u64,
    pub ri_user_ptime: u64,
    pub ri_system_ptime: u64,
    pub ri_pinstructions: u64,
    pub ri_pcycles: u64,
    pub ri_energy_nj: u64,
    pub ri_penergy_nj: u64,
    pub ri_secure_time_in_system: u64,
    pub ri_secure_ptime_in_system: u64,
    pub ri_neural_footprint: u64,
    pub ri_lifetime_max_neural_footprint: u64,
    pub ri_interval_max_neural_footprint: u64,
    pub ri_reserved: [u64; 9usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct proc_rlimit_control_wakeupmon {
    pub wm_flags: u32,
    pub wm_rate: i32,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_2 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _malloc_zone_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUctx_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmod_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUfunc_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlib_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUkern_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUarray_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmipmappedArray_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUtexref_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUsurfref_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUevent_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphicsResource_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUextMemory_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUextSemaphore_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraph_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphNode_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphExec_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemPoolHandle_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUuserObject_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphDeviceUpdatableNode_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUasyncCallbackEntry_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgreenCtx_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUuuid_st {
    pub bytes: [::std::os::raw::c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemFabricHandle_st {
    pub data: [::std::os::raw::c_uchar; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUipcEventHandle_st {
    pub reserved: [::std::os::raw::c_char; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUipcMemHandle_st {
    pub reserved: [::std::os::raw::c_char; 64usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub address: CUdeviceptr,
    pub __bindgen_anon_1:
        CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st__bindgen_ty_1,
    pub flags: ::std::os::raw::c_uint,
    pub alias: CUdeviceptr,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub address: CUdeviceptr,
    pub __bindgen_anon_1:
        CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st__bindgen_ty_1,
    pub flags: ::std::os::raw::c_uint,
    pub alias: CUdeviceptr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpFlushRemoteWritesParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpMemoryBarrierParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpAtomicReductionParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub flags: ::std::os::raw::c_uint,
    pub reductionOp: CUstreamAtomicReductionOpType,
    pub dataType: CUstreamAtomicReductionDataType,
    pub address: CUdeviceptr,
    pub value: cuuint64_t,
    pub alias: CUdeviceptr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_BATCH_MEM_OP_NODE_PARAMS_v1_st {
    pub ctx: CUcontext,
    pub count: ::std::os::raw::c_uint,
    pub paramArray: *mut CUstreamBatchMemOpParams,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_BATCH_MEM_OP_NODE_PARAMS_v2_st {
    pub ctx: CUcontext,
    pub count: ::std::os::raw::c_uint,
    pub paramArray: *mut CUstreamBatchMemOpParams,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUasyncNotificationInfo_st {
    pub type_: CUasyncNotificationType,
    pub info: CUasyncNotificationInfo_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUasyncNotificationInfo_st__bindgen_ty_1__bindgen_ty_1 {
    pub bytesOverBudget: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUdevprop_st {
    pub maxThreadsPerBlock: ::std::os::raw::c_int,
    pub maxThreadsDim: [::std::os::raw::c_int; 3usize],
    pub maxGridSize: [::std::os::raw::c_int; 3usize],
    pub sharedMemPerBlock: ::std::os::raw::c_int,
    pub totalConstantMemory: ::std::os::raw::c_int,
    pub SIMDWidth: ::std::os::raw::c_int,
    pub memPitch: ::std::os::raw::c_int,
    pub regsPerBlock: ::std::os::raw::c_int,
    pub clockRate: ::std::os::raw::c_int,
    pub textureAlign: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlinkState_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUaccessPolicyWindow_st {
    pub base_ptr: *mut ::std::os::raw::c_void,
    pub num_bytes: usize,
    pub hitRatio: f32,
    pub hitProp: CUaccessProperty,
    pub missProp: CUaccessProperty,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_KERNEL_NODE_PARAMS_st {
    pub func: CUfunction,
    pub gridDimX: ::std::os::raw::c_uint,
    pub gridDimY: ::std::os::raw::c_uint,
    pub gridDimZ: ::std::os::raw::c_uint,
    pub blockDimX: ::std::os::raw::c_uint,
    pub blockDimY: ::std::os::raw::c_uint,
    pub blockDimZ: ::std::os::raw::c_uint,
    pub sharedMemBytes: ::std::os::raw::c_uint,
    pub kernelParams: *mut *mut ::std::os::raw::c_void,
    pub extra: *mut *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_KERNEL_NODE_PARAMS_v2_st {
    pub func: CUfunction,
    pub gridDimX: ::std::os::raw::c_uint,
    pub gridDimY: ::std::os::raw::c_uint,
    pub gridDimZ: ::std::os::raw::c_uint,
    pub blockDimX: ::std::os::raw::c_uint,
    pub blockDimY: ::std::os::raw::c_uint,
    pub blockDimZ: ::std::os::raw::c_uint,
    pub sharedMemBytes: ::std::os::raw::c_uint,
    pub kernelParams: *mut *mut ::std::os::raw::c_void,
    pub extra: *mut *mut ::std::os::raw::c_void,
    pub kern: CUkernel,
    pub ctx: CUcontext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_KERNEL_NODE_PARAMS_v3_st {
    pub func: CUfunction,
    pub gridDimX: ::std::os::raw::c_uint,
    pub gridDimY: ::std::os::raw::c_uint,
    pub gridDimZ: ::std::os::raw::c_uint,
    pub blockDimX: ::std::os::raw::c_uint,
    pub blockDimY: ::std::os::raw::c_uint,
    pub blockDimZ: ::std::os::raw::c_uint,
    pub sharedMemBytes: ::std::os::raw::c_uint,
    pub kernelParams: *mut *mut ::std::os::raw::c_void,
    pub extra: *mut *mut ::std::os::raw::c_void,
    pub kern: CUkernel,
    pub ctx: CUcontext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_MEMSET_NODE_PARAMS_st {
    pub dst: CUdeviceptr,
    pub pitch: usize,
    pub value: ::std::os::raw::c_uint,
    pub elementSize: ::std::os::raw::c_uint,
    pub width: usize,
    pub height: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_MEMSET_NODE_PARAMS_v2_st {
    pub dst: CUdeviceptr,
    pub pitch: usize,
    pub value: ::std::os::raw::c_uint,
    pub elementSize: ::std::os::raw::c_uint,
    pub width: usize,
    pub height: usize,
    pub ctx: CUcontext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_HOST_NODE_PARAMS_st {
    pub fn_: CUhostFn,
    pub userData: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_HOST_NODE_PARAMS_v2_st {
    pub fn_: CUhostFn,
    pub userData: *mut ::std::os::raw::c_void,
    pub syncMode: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_CONDITIONAL_NODE_PARAMS {
    pub handle: CUgraphConditionalHandle,
    pub type_: CUgraphConditionalNodeType,
    pub size: ::std::os::raw::c_uint,
    pub phGraph_out: *mut CUgraph,
    pub ctx: CUcontext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphEdgeData_st {
    pub from_port: ::std::os::raw::c_uchar,
    pub to_port: ::std::os::raw::c_uchar,
    pub type_: ::std::os::raw::c_uchar,
    pub reserved: [::std::os::raw::c_uchar; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_GRAPH_INSTANTIATE_PARAMS_st {
    pub flags: cuuint64_t,
    pub hUploadStream: CUstream,
    pub hErrNode_out: CUgraphNode,
    pub result_out: CUgraphInstantiateResult,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlaunchMemSyncDomainMap_st {
    pub default_: ::std::os::raw::c_uchar,
    pub remote: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_1 {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
    pub z: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_2 {
    pub event: CUevent,
    pub flags: ::std::os::raw::c_int,
    pub triggerAtBlockStart: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_3 {
    pub event: CUevent,
    pub flags: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_4 {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
    pub z: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_5 {
    pub deviceUpdatable: ::std::os::raw::c_int,
    pub devNode: CUgraphDeviceNode,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUlaunchAttribute_st {
    pub id: CUlaunchAttributeID,
    pub pad: [::std::os::raw::c_char; 4usize],
    pub value: CUlaunchAttributeValue,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlaunchConfig_st {
    pub gridDimX: ::std::os::raw::c_uint,
    pub gridDimY: ::std::os::raw::c_uint,
    pub gridDimZ: ::std::os::raw::c_uint,
    pub blockDimX: ::std::os::raw::c_uint,
    pub blockDimY: ::std::os::raw::c_uint,
    pub blockDimZ: ::std::os::raw::c_uint,
    pub sharedMemBytes: ::std::os::raw::c_uint,
    pub hStream: CUstream,
    pub attrs: *mut CUlaunchAttribute,
    pub numAttrs: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUexecAffinitySmCount_st {
    pub val: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUexecAffinityParam_st {
    pub type_: CUexecAffinityType,
    pub param: CUexecAffinityParam_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUctxCigParam_st {
    pub sharedDataType: CUcigDataType,
    pub sharedData: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUctxCreateParams_st {
    pub execAffinityParams: *mut CUexecAffinityParam,
    pub numExecAffinityParams: ::std::os::raw::c_int,
    pub cigParams: *mut CUctxCigParam,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstreamCigParam_st {
    pub streamSharedDataType: CUstreamCigDataType,
    pub streamSharedData: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstreamCigCaptureParams_st {
    pub streamCigParams: *mut CUstreamCigParam,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlibraryHostUniversalFunctionAndDataTable_st {
    pub functionTable: *mut ::std::os::raw::c_void,
    pub functionWindowSize: usize,
    pub dataTable: *mut ::std::os::raw::c_void,
    pub dataWindowSize: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_MEMCPY2D_st {
    pub srcXInBytes: usize,
    pub srcY: usize,
    pub srcMemoryType: CUmemorytype,
    pub srcHost: *const ::std::os::raw::c_void,
    pub srcDevice: CUdeviceptr,
    pub srcArray: CUarray,
    pub srcPitch: usize,
    pub dstXInBytes: usize,
    pub dstY: usize,
    pub dstMemoryType: CUmemorytype,
    pub dstHost: *mut ::std::os::raw::c_void,
    pub dstDevice: CUdeviceptr,
    pub dstArray: CUarray,
    pub dstPitch: usize,
    pub WidthInBytes: usize,
    pub Height: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_MEMCPY3D_st {
    pub srcXInBytes: usize,
    pub srcY: usize,
    pub srcZ: usize,
    pub srcLOD: usize,
    pub srcMemoryType: CUmemorytype,
    pub srcHost: *const ::std::os::raw::c_void,
    pub srcDevice: CUdeviceptr,
    pub srcArray: CUarray,
    pub reserved0: *mut ::std::os::raw::c_void,
    pub srcPitch: usize,
    pub srcHeight: usize,
    pub dstXInBytes: usize,
    pub dstY: usize,
    pub dstZ: usize,
    pub dstLOD: usize,
    pub dstMemoryType: CUmemorytype,
    pub dstHost: *mut ::std::os::raw::c_void,
    pub dstDevice: CUdeviceptr,
    pub dstArray: CUarray,
    pub reserved1: *mut ::std::os::raw::c_void,
    pub dstPitch: usize,
    pub dstHeight: usize,
    pub WidthInBytes: usize,
    pub Height: usize,
    pub Depth: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_MEMCPY3D_PEER_st {
    pub srcXInBytes: usize,
    pub srcY: usize,
    pub srcZ: usize,
    pub srcLOD: usize,
    pub srcMemoryType: CUmemorytype,
    pub srcHost: *const ::std::os::raw::c_void,
    pub srcDevice: CUdeviceptr,
    pub srcArray: CUarray,
    pub srcContext: CUcontext,
    pub srcPitch: usize,
    pub srcHeight: usize,
    pub dstXInBytes: usize,
    pub dstY: usize,
    pub dstZ: usize,
    pub dstLOD: usize,
    pub dstMemoryType: CUmemorytype,
    pub dstHost: *mut ::std::os::raw::c_void,
    pub dstDevice: CUdeviceptr,
    pub dstArray: CUarray,
    pub dstContext: CUcontext,
    pub dstPitch: usize,
    pub dstHeight: usize,
    pub WidthInBytes: usize,
    pub Height: usize,
    pub Depth: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_MEMCPY_NODE_PARAMS_st {
    pub flags: ::std::os::raw::c_int,
    pub reserved: ::std::os::raw::c_int,
    pub copyCtx: CUcontext,
    pub copyParams: CUDA_MEMCPY3D,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_ARRAY_DESCRIPTOR_st {
    pub Width: usize,
    pub Height: usize,
    pub Format: CUarray_format,
    pub NumChannels: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_ARRAY3D_DESCRIPTOR_st {
    pub Width: usize,
    pub Height: usize,
    pub Depth: usize,
    pub Format: CUarray_format,
    pub NumChannels: ::std::os::raw::c_uint,
    pub Flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_ARRAY_SPARSE_PROPERTIES_st {
    pub tileExtent: CUDA_ARRAY_SPARSE_PROPERTIES_st__bindgen_ty_1,
    pub miptailFirstLevel: ::std::os::raw::c_uint,
    pub miptailSize: ::std::os::raw::c_ulonglong,
    pub flags: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_ARRAY_SPARSE_PROPERTIES_st__bindgen_ty_1 {
    pub width: ::std::os::raw::c_uint,
    pub height: ::std::os::raw::c_uint,
    pub depth: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_ARRAY_MEMORY_REQUIREMENTS_st {
    pub size: usize,
    pub alignment: usize,
    pub reserved: [::std::os::raw::c_uint; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_RESOURCE_DESC_st {
    pub resType: CUresourcetype,
    pub res: CUDA_RESOURCE_DESC_st__bindgen_ty_1,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    pub hArray: CUarray,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2 {
    pub hMipmappedArray: CUmipmappedArray,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3 {
    pub devPtr: CUdeviceptr,
    pub format: CUarray_format,
    pub numChannels: ::std::os::raw::c_uint,
    pub sizeInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4 {
    pub devPtr: CUdeviceptr,
    pub format: CUarray_format,
    pub numChannels: ::std::os::raw::c_uint,
    pub width: usize,
    pub height: usize,
    pub pitchInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_5 {
    pub reserved: [::std::os::raw::c_int; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_TEXTURE_DESC_st {
    pub addressMode: [CUaddress_mode; 3usize],
    pub filterMode: CUfilter_mode,
    pub flags: ::std::os::raw::c_uint,
    pub maxAnisotropy: ::std::os::raw::c_uint,
    pub mipmapFilterMode: CUfilter_mode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub borderColor: [f32; 4usize],
    pub reserved: [::std::os::raw::c_int; 12usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_RESOURCE_VIEW_DESC_st {
    pub format: CUresourceViewFormat,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub firstMipmapLevel: ::std::os::raw::c_uint,
    pub lastMipmapLevel: ::std::os::raw::c_uint,
    pub firstLayer: ::std::os::raw::c_uint,
    pub lastLayer: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 16usize],
}
#[repr(C)]
#[repr(align(128))]
#[derive(Debug, Copy, Clone)]
pub struct CUtensorMap_st {
    pub opaque: [cuuint64_t; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_st {
    pub p2pToken: ::std::os::raw::c_ulonglong,
    pub vaSpaceToken: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_LAUNCH_PARAMS_st {
    pub function: CUfunction,
    pub gridDimX: ::std::os::raw::c_uint,
    pub gridDimY: ::std::os::raw::c_uint,
    pub gridDimZ: ::std::os::raw::c_uint,
    pub blockDimX: ::std::os::raw::c_uint,
    pub blockDimY: ::std::os::raw::c_uint,
    pub blockDimZ: ::std::os::raw::c_uint,
    pub sharedMemBytes: ::std::os::raw::c_uint,
    pub hStream: CUstream,
    pub kernelParams: *mut *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st {
    pub type_: CUexternalMemoryHandleType,
    pub handle: CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1,
    pub size: ::std::os::raw::c_ulonglong,
    pub flags: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    pub handle: *mut ::std::os::raw::c_void,
    pub name: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXTERNAL_MEMORY_BUFFER_DESC_st {
    pub offset: ::std::os::raw::c_ulonglong,
    pub size: ::std::os::raw::c_ulonglong,
    pub flags: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_st {
    pub offset: ::std::os::raw::c_ulonglong,
    pub arrayDesc: CUDA_ARRAY3D_DESCRIPTOR,
    pub numLevels: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st {
    pub type_: CUexternalSemaphoreHandleType,
    pub handle: CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1,
    pub flags: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    pub handle: *mut ::std::os::raw::c_void,
    pub name: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st {
    pub params: CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1,
    pub flags: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1 {
    pub fence: CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::std::os::raw::c_uint; 12usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st {
    pub params: CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1,
    pub flags: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1 {
    pub fence: CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::std::os::raw::c_uint; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::std::os::raw::c_ulonglong,
    pub timeoutMs: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_st {
    pub extSemArray: *mut CUexternalSemaphore,
    pub paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    pub numExtSems: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2_st {
    pub extSemArray: *mut CUexternalSemaphore,
    pub paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    pub numExtSems: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXT_SEM_WAIT_NODE_PARAMS_st {
    pub extSemArray: *mut CUexternalSemaphore,
    pub paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    pub numExtSems: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2_st {
    pub extSemArray: *mut CUexternalSemaphore,
    pub paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    pub numExtSems: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUarrayMapInfo_st {
    pub resourceType: CUresourcetype,
    pub resource: CUarrayMapInfo_st__bindgen_ty_1,
    pub subresourceType: CUarraySparseSubresourceType,
    pub subresource: CUarrayMapInfo_st__bindgen_ty_2,
    pub memOperationType: CUmemOperationType,
    pub memHandleType: CUmemHandleType,
    pub memHandle: CUarrayMapInfo_st__bindgen_ty_3,
    pub offset: ::std::os::raw::c_ulonglong,
    pub deviceBitMask: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_1 {
    pub level: ::std::os::raw::c_uint,
    pub layer: ::std::os::raw::c_uint,
    pub offsetX: ::std::os::raw::c_uint,
    pub offsetY: ::std::os::raw::c_uint,
    pub offsetZ: ::std::os::raw::c_uint,
    pub extentWidth: ::std::os::raw::c_uint,
    pub extentHeight: ::std::os::raw::c_uint,
    pub extentDepth: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_2 {
    pub layer: ::std::os::raw::c_uint,
    pub offset: ::std::os::raw::c_ulonglong,
    pub size: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUmemLocation_st {
    pub type_: CUmemLocationType,
    pub __bindgen_anon_1: CUmemLocation_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUmemAllocationProp_st {
    pub type_: CUmemAllocationType,
    pub requestedHandleTypes: CUmemAllocationHandleType,
    pub location: CUmemLocation,
    pub win32HandleMetaData: *mut ::std::os::raw::c_void,
    pub allocFlags: CUmemAllocationProp_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemAllocationProp_st__bindgen_ty_1 {
    pub compressionType: ::std::os::raw::c_uchar,
    pub gpuDirectRDMACapable: ::std::os::raw::c_uchar,
    pub usage: ::std::os::raw::c_ushort,
    pub reserved: [::std::os::raw::c_uchar; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmulticastObjectProp_st {
    pub numDevices: ::std::os::raw::c_uint,
    pub size: usize,
    pub handleTypes: ::std::os::raw::c_ulonglong,
    pub flags: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUmemAccessDesc_st {
    pub location: CUmemLocation,
    pub flags: CUmemAccess_flags,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphExecUpdateResultInfo_st {
    pub result: CUgraphExecUpdateResult,
    pub errorNode: CUgraphNode,
    pub errorFromNode: CUgraphNode,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUmemPoolProps_st {
    pub allocType: CUmemAllocationType,
    pub handleTypes: CUmemAllocationHandleType,
    pub location: CUmemLocation,
    pub win32SecurityAttributes: *mut ::std::os::raw::c_void,
    pub maxSize: usize,
    pub usage: ::std::os::raw::c_ushort,
    pub reserved: [::std::os::raw::c_uchar; 54usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemPoolPtrExportData_st {
    pub reserved: [::std::os::raw::c_uchar; 64usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUmemcpyAttributes_st {
    pub srcAccessOrder: CUmemcpySrcAccessOrder,
    pub srcLocHint: CUmemLocation,
    pub dstLocHint: CUmemLocation,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUoffset3D_st {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUextent3D_st {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUmemcpy3DOperand_st {
    pub type_: CUmemcpy3DOperandType,
    pub op: CUmemcpy3DOperand_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUmemcpy3DOperand_st__bindgen_ty_1__bindgen_ty_1 {
    pub ptr: CUdeviceptr,
    pub rowLength: usize,
    pub layerHeight: usize,
    pub locHint: CUmemLocation,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemcpy3DOperand_st__bindgen_ty_1__bindgen_ty_2 {
    pub array: CUarray,
    pub offset: CUoffset3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_MEMCPY3D_BATCH_OP_st {
    pub src: CUmemcpy3DOperand,
    pub dst: CUmemcpy3DOperand,
    pub extent: CUextent3D,
    pub srcAccessOrder: CUmemcpySrcAccessOrder,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_MEM_ALLOC_NODE_PARAMS_v1_st {
    pub poolProps: CUmemPoolProps,
    pub accessDescs: *const CUmemAccessDesc,
    pub accessDescCount: usize,
    pub bytesize: usize,
    pub dptr: CUdeviceptr,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_MEM_ALLOC_NODE_PARAMS_v2_st {
    pub poolProps: CUmemPoolProps,
    pub accessDescs: *const CUmemAccessDesc,
    pub accessDescCount: usize,
    pub bytesize: usize,
    pub dptr: CUdeviceptr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_MEM_FREE_NODE_PARAMS_st {
    pub dptr: CUdeviceptr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_CHILD_GRAPH_NODE_PARAMS_st {
    pub graph: CUgraph,
    pub ownership: CUgraphChildGraphNodeOwnership,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EVENT_RECORD_NODE_PARAMS_st {
    pub event: CUevent,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_EVENT_WAIT_NODE_PARAMS_st {
    pub event: CUevent,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUgraphNodeParams_st {
    pub type_: CUgraphNodeType,
    pub reserved0: [::std::os::raw::c_int; 3usize],
    pub __bindgen_anon_1: CUgraphNodeParams_st__bindgen_ty_1,
    pub reserved2: ::std::os::raw::c_longlong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUcheckpointLockArgs_st {
    pub timeoutMs: ::std::os::raw::c_uint,
    pub reserved0: ::std::os::raw::c_uint,
    pub reserved1: [cuuint64_t; 7usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUcheckpointCheckpointArgs_st {
    pub reserved: [cuuint64_t; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUcheckpointGpuPair_st {
    pub oldUuid: CUuuid,
    pub newUuid: CUuuid,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUcheckpointRestoreArgs_st {
    pub gpuPairs: *mut CUcheckpointGpuPair,
    pub gpuPairsCount: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_char; 52usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUcheckpointUnlockArgs_st {
    pub reserved: [cuuint64_t; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemDecompressParams_st {
    pub srcNumBytes: usize,
    pub dstNumBytes: usize,
    pub dstActBytes: *mut cuuint32_t,
    pub src: *const ::std::os::raw::c_void,
    pub dst: *mut ::std::os::raw::c_void,
    pub algo: CUmemDecompressAlgorithm,
    pub padding: [::std::os::raw::c_uchar; 20usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlogicalEndpointFabricHandle_st {
    pub data: [::std::os::raw::c_uchar; 64usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUlogicalEndpointProp_struct {
    pub type_: CUlogicalEndpointType,
    pub __bindgen_anon_1: CUlogicalEndpointProp_struct__bindgen_ty_1,
    pub size: ::std::os::raw::c_ulonglong,
    pub ipcHandleTypes: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlogicalEndpointProp_struct__bindgen_ty_1__bindgen_ty_1 {
    pub device: CUdevice,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlogicalEndpointProp_struct__bindgen_ty_1__bindgen_ty_2 {
    pub numDevices: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUcoredumpCallbackEntry_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUdevResourceDesc_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUdevSmResource_st {
    pub smCount: ::std::os::raw::c_uint,
    pub minSmPartitionSize: ::std::os::raw::c_uint,
    pub smCoscheduledAlignment: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUdevWorkqueueConfigResource_st {
    pub device: CUdevice,
    pub wqConcurrencyLimit: ::std::os::raw::c_uint,
    pub sharingScope: CUdevWorkqueueConfigScope,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUdevWorkqueueResource_st {
    pub reserved: [::std::os::raw::c_uchar; 40usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CU_DEV_SM_RESOURCE_GROUP_PARAMS_st {
    pub smCount: ::std::os::raw::c_uint,
    pub coscheduledSmCount: ::std::os::raw::c_uint,
    pub preferredCoscheduledSmCount: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 12usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUdevResource_st {
    pub type_: CUdevResourceType,
    pub _internal_padding: [::std::os::raw::c_uchar; 92usize],
    pub __bindgen_anon_1: CUdevResource_st__bindgen_ty_1,
    pub nextResource: *mut CUdevResource_st,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlogsCallbackEntry_st {
    _unused: [u8; 0],
}
pub const __API_TO_BE_DEPRECATED: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_IOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_IOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACCATALYST: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACCATALYSTAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_WATCHOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_WATCHOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_TVOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_TVOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_DRIVERKIT: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_VISIONOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_VISIONOSAPPLICATIONEXTENSION: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_KERNELKIT: u32 = 100000;
pub const __MAC_10_0: u32 = 1000;
pub const __MAC_10_1: u32 = 1010;
pub const __MAC_10_2: u32 = 1020;
pub const __MAC_10_3: u32 = 1030;
pub const __MAC_10_4: u32 = 1040;
pub const __MAC_10_5: u32 = 1050;
pub const __MAC_10_6: u32 = 1060;
pub const __MAC_10_7: u32 = 1070;
pub const __MAC_10_8: u32 = 1080;
pub const __MAC_10_9: u32 = 1090;
pub const __MAC_10_10: u32 = 101000;
pub const __MAC_10_10_2: u32 = 101002;
pub const __MAC_10_10_3: u32 = 101003;
pub const __MAC_10_11: u32 = 101100;
pub const __MAC_10_11_2: u32 = 101102;
pub const __MAC_10_11_3: u32 = 101103;
pub const __MAC_10_11_4: u32 = 101104;
pub const __MAC_10_12: u32 = 101200;
pub const __MAC_10_12_1: u32 = 101201;
pub const __MAC_10_12_2: u32 = 101202;
pub const __MAC_10_12_4: u32 = 101204;
pub const __MAC_10_13: u32 = 101300;
pub const __MAC_10_13_1: u32 = 101301;
pub const __MAC_10_13_2: u32 = 101302;
pub const __MAC_10_13_4: u32 = 101304;
pub const __MAC_10_14: u32 = 101400;
pub const __MAC_10_14_1: u32 = 101401;
pub const __MAC_10_14_4: u32 = 101404;
pub const __MAC_10_14_5: u32 = 101405;
pub const __MAC_10_14_6: u32 = 101406;
pub const __MAC_10_15: u32 = 101500;
pub const __MAC_10_15_1: u32 = 101501;
pub const __MAC_10_15_4: u32 = 101504;
pub const __MAC_10_16: u32 = 101600;
pub const __MAC_11_0: u32 = 110000;
pub const __MAC_11_1: u32 = 110100;
pub const __MAC_11_3: u32 = 110300;
pub const __MAC_11_4: u32 = 110400;
pub const __MAC_11_5: u32 = 110500;
pub const __MAC_11_6: u32 = 110600;
pub const __MAC_12_0: u32 = 120000;
pub const __MAC_12_1: u32 = 120100;
pub const __MAC_12_2: u32 = 120200;
pub const __MAC_12_3: u32 = 120300;
pub const __MAC_12_4: u32 = 120400;
pub const __MAC_12_5: u32 = 120500;
pub const __MAC_12_6: u32 = 120600;
pub const __MAC_12_7: u32 = 120700;
pub const __MAC_13_0: u32 = 130000;
pub const __MAC_13_1: u32 = 130100;
pub const __MAC_13_2: u32 = 130200;
pub const __MAC_13_3: u32 = 130300;
pub const __MAC_13_4: u32 = 130400;
pub const __MAC_13_5: u32 = 130500;
pub const __MAC_13_6: u32 = 130600;
pub const __MAC_13_7: u32 = 130700;
pub const __MAC_14_0: u32 = 140000;
pub const __MAC_14_1: u32 = 140100;
pub const __MAC_14_2: u32 = 140200;
pub const __MAC_14_3: u32 = 140300;
pub const __MAC_14_4: u32 = 140400;
pub const __MAC_14_5: u32 = 140500;
pub const __MAC_14_6: u32 = 140600;
pub const __MAC_14_7: u32 = 140700;
pub const __MAC_15_0: u32 = 150000;
pub const __MAC_15_1: u32 = 150100;
pub const __MAC_15_2: u32 = 150200;
pub const __MAC_15_3: u32 = 150300;
pub const __MAC_15_4: u32 = 150400;
pub const __MAC_15_5: u32 = 150500;
pub const __MAC_15_6: u32 = 150600;
pub const __MAC_16_0: u32 = 160000;
pub const __MAC_26_0: u32 = 260000;
pub const __MAC_26_1: u32 = 260100;
pub const __MAC_26_2: u32 = 260200;
pub const __MAC_26_3: u32 = 260300;
pub const __MAC_26_4: u32 = 260400;
pub const __IPHONE_2_0: u32 = 20000;
pub const __IPHONE_2_1: u32 = 20100;
pub const __IPHONE_2_2: u32 = 20200;
pub const __IPHONE_3_0: u32 = 30000;
pub const __IPHONE_3_1: u32 = 30100;
pub const __IPHONE_3_2: u32 = 30200;
pub const __IPHONE_4_0: u32 = 40000;
pub const __IPHONE_4_1: u32 = 40100;
pub const __IPHONE_4_2: u32 = 40200;
pub const __IPHONE_4_3: u32 = 40300;
pub const __IPHONE_5_0: u32 = 50000;
pub const __IPHONE_5_1: u32 = 50100;
pub const __IPHONE_6_0: u32 = 60000;
pub const __IPHONE_6_1: u32 = 60100;
pub const __IPHONE_7_0: u32 = 70000;
pub const __IPHONE_7_1: u32 = 70100;
pub const __IPHONE_8_0: u32 = 80000;
pub const __IPHONE_8_1: u32 = 80100;
pub const __IPHONE_8_2: u32 = 80200;
pub const __IPHONE_8_3: u32 = 80300;
pub const __IPHONE_8_4: u32 = 80400;
pub const __IPHONE_9_0: u32 = 90000;
pub const __IPHONE_9_1: u32 = 90100;
pub const __IPHONE_9_2: u32 = 90200;
pub const __IPHONE_9_3: u32 = 90300;
pub const __IPHONE_10_0: u32 = 100000;
pub const __IPHONE_10_1: u32 = 100100;
pub const __IPHONE_10_2: u32 = 100200;
pub const __IPHONE_10_3: u32 = 100300;
pub const __IPHONE_11_0: u32 = 110000;
pub const __IPHONE_11_1: u32 = 110100;
pub const __IPHONE_11_2: u32 = 110200;
pub const __IPHONE_11_3: u32 = 110300;
pub const __IPHONE_11_4: u32 = 110400;
pub const __IPHONE_12_0: u32 = 120000;
pub const __IPHONE_12_1: u32 = 120100;
pub const __IPHONE_12_2: u32 = 120200;
pub const __IPHONE_12_3: u32 = 120300;
pub const __IPHONE_12_4: u32 = 120400;
pub const __IPHONE_13_0: u32 = 130000;
pub const __IPHONE_13_1: u32 = 130100;
pub const __IPHONE_13_2: u32 = 130200;
pub const __IPHONE_13_3: u32 = 130300;
pub const __IPHONE_13_4: u32 = 130400;
pub const __IPHONE_13_5: u32 = 130500;
pub const __IPHONE_13_6: u32 = 130600;
pub const __IPHONE_13_7: u32 = 130700;
pub const __IPHONE_14_0: u32 = 140000;
pub const __IPHONE_14_1: u32 = 140100;
pub const __IPHONE_14_2: u32 = 140200;
pub const __IPHONE_14_3: u32 = 140300;
pub const __IPHONE_14_5: u32 = 140500;
pub const __IPHONE_14_6: u32 = 140600;
pub const __IPHONE_14_7: u32 = 140700;
pub const __IPHONE_14_8: u32 = 140800;
pub const __IPHONE_15_0: u32 = 150000;
pub const __IPHONE_15_1: u32 = 150100;
pub const __IPHONE_15_2: u32 = 150200;
pub const __IPHONE_15_3: u32 = 150300;
pub const __IPHONE_15_4: u32 = 150400;
pub const __IPHONE_15_5: u32 = 150500;
pub const __IPHONE_15_6: u32 = 150600;
pub const __IPHONE_15_7: u32 = 150700;
pub const __IPHONE_15_8: u32 = 150800;
pub const __IPHONE_16_0: u32 = 160000;
pub const __IPHONE_16_1: u32 = 160100;
pub const __IPHONE_16_2: u32 = 160200;
pub const __IPHONE_16_3: u32 = 160300;
pub const __IPHONE_16_4: u32 = 160400;
pub const __IPHONE_16_5: u32 = 160500;
pub const __IPHONE_16_6: u32 = 160600;
pub const __IPHONE_16_7: u32 = 160700;
pub const __IPHONE_17_0: u32 = 170000;
pub const __IPHONE_17_1: u32 = 170100;
pub const __IPHONE_17_2: u32 = 170200;
pub const __IPHONE_17_3: u32 = 170300;
pub const __IPHONE_17_4: u32 = 170400;
pub const __IPHONE_17_5: u32 = 170500;
pub const __IPHONE_17_6: u32 = 170600;
pub const __IPHONE_17_7: u32 = 170700;
pub const __IPHONE_18_0: u32 = 180000;
pub const __IPHONE_18_1: u32 = 180100;
pub const __IPHONE_18_2: u32 = 180200;
pub const __IPHONE_18_3: u32 = 180300;
pub const __IPHONE_18_4: u32 = 180400;
pub const __IPHONE_18_5: u32 = 180500;
pub const __IPHONE_18_6: u32 = 180600;
pub const __IPHONE_19_0: u32 = 190000;
pub const __IPHONE_26_0: u32 = 260000;
pub const __IPHONE_26_1: u32 = 260100;
pub const __IPHONE_26_2: u32 = 260200;
pub const __IPHONE_26_3: u32 = 260300;
pub const __IPHONE_26_4: u32 = 260400;
pub const __WATCHOS_1_0: u32 = 10000;
pub const __WATCHOS_2_0: u32 = 20000;
pub const __WATCHOS_2_1: u32 = 20100;
pub const __WATCHOS_2_2: u32 = 20200;
pub const __WATCHOS_3_0: u32 = 30000;
pub const __WATCHOS_3_1: u32 = 30100;
pub const __WATCHOS_3_1_1: u32 = 30101;
pub const __WATCHOS_3_2: u32 = 30200;
pub const __WATCHOS_4_0: u32 = 40000;
pub const __WATCHOS_4_1: u32 = 40100;
pub const __WATCHOS_4_2: u32 = 40200;
pub const __WATCHOS_4_3: u32 = 40300;
pub const __WATCHOS_5_0: u32 = 50000;
pub const __WATCHOS_5_1: u32 = 50100;
pub const __WATCHOS_5_2: u32 = 50200;
pub const __WATCHOS_5_3: u32 = 50300;
pub const __WATCHOS_6_0: u32 = 60000;
pub const __WATCHOS_6_1: u32 = 60100;
pub const __WATCHOS_6_2: u32 = 60200;
pub const __WATCHOS_7_0: u32 = 70000;
pub const __WATCHOS_7_1: u32 = 70100;
pub const __WATCHOS_7_2: u32 = 70200;
pub const __WATCHOS_7_3: u32 = 70300;
pub const __WATCHOS_7_4: u32 = 70400;
pub const __WATCHOS_7_5: u32 = 70500;
pub const __WATCHOS_7_6: u32 = 70600;
pub const __WATCHOS_8_0: u32 = 80000;
pub const __WATCHOS_8_1: u32 = 80100;
pub const __WATCHOS_8_3: u32 = 80300;
pub const __WATCHOS_8_4: u32 = 80400;
pub const __WATCHOS_8_5: u32 = 80500;
pub const __WATCHOS_8_6: u32 = 80600;
pub const __WATCHOS_8_7: u32 = 80700;
pub const __WATCHOS_8_8: u32 = 80800;
pub const __WATCHOS_9_0: u32 = 90000;
pub const __WATCHOS_9_1: u32 = 90100;
pub const __WATCHOS_9_2: u32 = 90200;
pub const __WATCHOS_9_3: u32 = 90300;
pub const __WATCHOS_9_4: u32 = 90400;
pub const __WATCHOS_9_5: u32 = 90500;
pub const __WATCHOS_9_6: u32 = 90600;
pub const __WATCHOS_10_0: u32 = 100000;
pub const __WATCHOS_10_1: u32 = 100100;
pub const __WATCHOS_10_2: u32 = 100200;
pub const __WATCHOS_10_3: u32 = 100300;
pub const __WATCHOS_10_4: u32 = 100400;
pub const __WATCHOS_10_5: u32 = 100500;
pub const __WATCHOS_10_6: u32 = 100600;
pub const __WATCHOS_10_7: u32 = 100700;
pub const __WATCHOS_11_0: u32 = 110000;
pub const __WATCHOS_11_1: u32 = 110100;
pub const __WATCHOS_11_2: u32 = 110200;
pub const __WATCHOS_11_3: u32 = 110300;
pub const __WATCHOS_11_4: u32 = 110400;
pub const __WATCHOS_11_5: u32 = 110500;
pub const __WATCHOS_11_6: u32 = 110600;
pub const __WATCHOS_12_0: u32 = 120000;
pub const __WATCHOS_26_0: u32 = 260000;
pub const __WATCHOS_26_1: u32 = 260100;
pub const __WATCHOS_26_2: u32 = 260200;
pub const __WATCHOS_26_3: u32 = 260300;
pub const __WATCHOS_26_4: u32 = 260400;
pub const __TVOS_9_0: u32 = 90000;
pub const __TVOS_9_1: u32 = 90100;
pub const __TVOS_9_2: u32 = 90200;
pub const __TVOS_10_0: u32 = 100000;
pub const __TVOS_10_0_1: u32 = 100001;
pub const __TVOS_10_1: u32 = 100100;
pub const __TVOS_10_2: u32 = 100200;
pub const __TVOS_11_0: u32 = 110000;
pub const __TVOS_11_1: u32 = 110100;
pub const __TVOS_11_2: u32 = 110200;
pub const __TVOS_11_3: u32 = 110300;
pub const __TVOS_11_4: u32 = 110400;
pub const __TVOS_12_0: u32 = 120000;
pub const __TVOS_12_1: u32 = 120100;
pub const __TVOS_12_2: u32 = 120200;
pub const __TVOS_12_3: u32 = 120300;
pub const __TVOS_12_4: u32 = 120400;
pub const __TVOS_13_0: u32 = 130000;
pub const __TVOS_13_2: u32 = 130200;
pub const __TVOS_13_3: u32 = 130300;
pub const __TVOS_13_4: u32 = 130400;
pub const __TVOS_14_0: u32 = 140000;
pub const __TVOS_14_1: u32 = 140100;
pub const __TVOS_14_2: u32 = 140200;
pub const __TVOS_14_3: u32 = 140300;
pub const __TVOS_14_5: u32 = 140500;
pub const __TVOS_14_6: u32 = 140600;
pub const __TVOS_14_7: u32 = 140700;
pub const __TVOS_15_0: u32 = 150000;
pub const __TVOS_15_1: u32 = 150100;
pub const __TVOS_15_2: u32 = 150200;
pub const __TVOS_15_3: u32 = 150300;
pub const __TVOS_15_4: u32 = 150400;
pub const __TVOS_15_5: u32 = 150500;
pub const __TVOS_15_6: u32 = 150600;
pub const __TVOS_16_0: u32 = 160000;
pub const __TVOS_16_1: u32 = 160100;
pub const __TVOS_16_2: u32 = 160200;
pub const __TVOS_16_3: u32 = 160300;
pub const __TVOS_16_4: u32 = 160400;
pub const __TVOS_16_5: u32 = 160500;
pub const __TVOS_16_6: u32 = 160600;
pub const __TVOS_17_0: u32 = 170000;
pub const __TVOS_17_1: u32 = 170100;
pub const __TVOS_17_2: u32 = 170200;
pub const __TVOS_17_3: u32 = 170300;
pub const __TVOS_17_4: u32 = 170400;
pub const __TVOS_17_5: u32 = 170500;
pub const __TVOS_17_6: u32 = 170600;
pub const __TVOS_18_0: u32 = 180000;
pub const __TVOS_18_1: u32 = 180100;
pub const __TVOS_18_2: u32 = 180200;
pub const __TVOS_18_3: u32 = 180300;
pub const __TVOS_18_4: u32 = 180400;
pub const __TVOS_18_5: u32 = 180500;
pub const __TVOS_18_6: u32 = 180600;
pub const __TVOS_19_0: u32 = 190000;
pub const __TVOS_26_0: u32 = 260000;
pub const __TVOS_26_1: u32 = 260100;
pub const __TVOS_26_2: u32 = 260200;
pub const __TVOS_26_3: u32 = 260300;
pub const __TVOS_26_4: u32 = 260400;
pub const __BRIDGEOS_2_0: u32 = 20000;
pub const __BRIDGEOS_3_0: u32 = 30000;
pub const __BRIDGEOS_3_1: u32 = 30100;
pub const __BRIDGEOS_3_4: u32 = 30400;
pub const __BRIDGEOS_4_0: u32 = 40000;
pub const __BRIDGEOS_4_1: u32 = 40100;
pub const __BRIDGEOS_5_0: u32 = 50000;
pub const __BRIDGEOS_5_1: u32 = 50100;
pub const __BRIDGEOS_5_3: u32 = 50300;
pub const __BRIDGEOS_6_0: u32 = 60000;
pub const __BRIDGEOS_6_2: u32 = 60200;
pub const __BRIDGEOS_6_4: u32 = 60400;
pub const __BRIDGEOS_6_5: u32 = 60500;
pub const __BRIDGEOS_6_6: u32 = 60600;
pub const __BRIDGEOS_7_0: u32 = 70000;
pub const __BRIDGEOS_7_1: u32 = 70100;
pub const __BRIDGEOS_7_2: u32 = 70200;
pub const __BRIDGEOS_7_3: u32 = 70300;
pub const __BRIDGEOS_7_4: u32 = 70400;
pub const __BRIDGEOS_7_6: u32 = 70600;
pub const __BRIDGEOS_8_0: u32 = 80000;
pub const __BRIDGEOS_8_1: u32 = 80100;
pub const __BRIDGEOS_8_2: u32 = 80200;
pub const __BRIDGEOS_8_3: u32 = 80300;
pub const __BRIDGEOS_8_4: u32 = 80400;
pub const __BRIDGEOS_8_5: u32 = 80500;
pub const __BRIDGEOS_8_6: u32 = 80600;
pub const __BRIDGEOS_9_0: u32 = 90000;
pub const __BRIDGEOS_9_1: u32 = 90100;
pub const __BRIDGEOS_9_2: u32 = 90200;
pub const __BRIDGEOS_9_3: u32 = 90300;
pub const __BRIDGEOS_9_4: u32 = 90400;
pub const __BRIDGEOS_9_5: u32 = 90500;
pub const __BRIDGEOS_9_6: u32 = 90600;
pub const __BRIDGEOS_10_0: u32 = 100000;
pub const __BRIDGEOS_10_1: u32 = 100100;
pub const __BRIDGEOS_10_2: u32 = 100200;
pub const __BRIDGEOS_10_3: u32 = 100300;
pub const __BRIDGEOS_10_4: u32 = 100400;
pub const __DRIVERKIT_19_0: u32 = 190000;
pub const __DRIVERKIT_20_0: u32 = 200000;
pub const __DRIVERKIT_21_0: u32 = 210000;
pub const __DRIVERKIT_22_0: u32 = 220000;
pub const __DRIVERKIT_22_4: u32 = 220400;
pub const __DRIVERKIT_22_5: u32 = 220500;
pub const __DRIVERKIT_22_6: u32 = 220600;
pub const __DRIVERKIT_23_0: u32 = 230000;
pub const __DRIVERKIT_23_1: u32 = 230100;
pub const __DRIVERKIT_23_2: u32 = 230200;
pub const __DRIVERKIT_23_3: u32 = 230300;
pub const __DRIVERKIT_23_4: u32 = 230400;
pub const __DRIVERKIT_23_5: u32 = 230500;
pub const __DRIVERKIT_23_6: u32 = 230600;
pub const __DRIVERKIT_24_0: u32 = 240000;
pub const __DRIVERKIT_24_1: u32 = 240100;
pub const __DRIVERKIT_24_2: u32 = 240200;
pub const __DRIVERKIT_24_3: u32 = 240300;
pub const __DRIVERKIT_24_4: u32 = 240400;
pub const __DRIVERKIT_24_5: u32 = 240500;
pub const __DRIVERKIT_24_6: u32 = 240600;
pub const __DRIVERKIT_25_0: u32 = 250000;
pub const __DRIVERKIT_25_1: u32 = 250100;
pub const __DRIVERKIT_25_2: u32 = 250200;
pub const __DRIVERKIT_25_3: u32 = 250300;
pub const __DRIVERKIT_25_4: u32 = 250400;
pub const __VISIONOS_1_0: u32 = 10000;
pub const __VISIONOS_1_1: u32 = 10100;
pub const __VISIONOS_1_2: u32 = 10200;
pub const __VISIONOS_1_3: u32 = 10300;
pub const __VISIONOS_2_0: u32 = 20000;
pub const __VISIONOS_2_1: u32 = 20100;
pub const __VISIONOS_2_2: u32 = 20200;
pub const __VISIONOS_2_3: u32 = 20300;
pub const __VISIONOS_2_4: u32 = 20400;
pub const __VISIONOS_2_5: u32 = 20500;
pub const __VISIONOS_2_6: u32 = 20600;
pub const __VISIONOS_3_0: u32 = 30000;
pub const __VISIONOS_26_0: u32 = 260000;
pub const __VISIONOS_26_1: u32 = 260100;
pub const __VISIONOS_26_2: u32 = 260200;
pub const __VISIONOS_26_3: u32 = 260300;
pub const __VISIONOS_26_4: u32 = 260400;
pub const MAC_OS_X_VERSION_10_0: u32 = 1000;
pub const MAC_OS_X_VERSION_10_1: u32 = 1010;
pub const MAC_OS_X_VERSION_10_2: u32 = 1020;
pub const MAC_OS_X_VERSION_10_3: u32 = 1030;
pub const MAC_OS_X_VERSION_10_4: u32 = 1040;
pub const MAC_OS_X_VERSION_10_5: u32 = 1050;
pub const MAC_OS_X_VERSION_10_6: u32 = 1060;
pub const MAC_OS_X_VERSION_10_7: u32 = 1070;
pub const MAC_OS_X_VERSION_10_8: u32 = 1080;
pub const MAC_OS_X_VERSION_10_9: u32 = 1090;
pub const MAC_OS_X_VERSION_10_10: u32 = 101000;
pub const MAC_OS_X_VERSION_10_10_2: u32 = 101002;
pub const MAC_OS_X_VERSION_10_10_3: u32 = 101003;
pub const MAC_OS_X_VERSION_10_11: u32 = 101100;
pub const MAC_OS_X_VERSION_10_11_2: u32 = 101102;
pub const MAC_OS_X_VERSION_10_11_3: u32 = 101103;
pub const MAC_OS_X_VERSION_10_11_4: u32 = 101104;
pub const MAC_OS_X_VERSION_10_12: u32 = 101200;
pub const MAC_OS_X_VERSION_10_12_1: u32 = 101201;
pub const MAC_OS_X_VERSION_10_12_2: u32 = 101202;
pub const MAC_OS_X_VERSION_10_12_4: u32 = 101204;
pub const MAC_OS_X_VERSION_10_13: u32 = 101300;
pub const MAC_OS_X_VERSION_10_13_1: u32 = 101301;
pub const MAC_OS_X_VERSION_10_13_2: u32 = 101302;
pub const MAC_OS_X_VERSION_10_13_4: u32 = 101304;
pub const MAC_OS_X_VERSION_10_14: u32 = 101400;
pub const MAC_OS_X_VERSION_10_14_1: u32 = 101401;
pub const MAC_OS_X_VERSION_10_14_4: u32 = 101404;
pub const MAC_OS_X_VERSION_10_14_5: u32 = 101405;
pub const MAC_OS_X_VERSION_10_14_6: u32 = 101406;
pub const MAC_OS_X_VERSION_10_15: u32 = 101500;
pub const MAC_OS_X_VERSION_10_15_1: u32 = 101501;
pub const MAC_OS_X_VERSION_10_15_4: u32 = 101504;
pub const MAC_OS_X_VERSION_10_16: u32 = 101600;
pub const MAC_OS_VERSION_11_0: u32 = 110000;
pub const MAC_OS_VERSION_11_1: u32 = 110100;
pub const MAC_OS_VERSION_11_3: u32 = 110300;
pub const MAC_OS_VERSION_11_4: u32 = 110400;
pub const MAC_OS_VERSION_11_5: u32 = 110500;
pub const MAC_OS_VERSION_11_6: u32 = 110600;
pub const MAC_OS_VERSION_12_0: u32 = 120000;
pub const MAC_OS_VERSION_12_1: u32 = 120100;
pub const MAC_OS_VERSION_12_2: u32 = 120200;
pub const MAC_OS_VERSION_12_3: u32 = 120300;
pub const MAC_OS_VERSION_12_4: u32 = 120400;
pub const MAC_OS_VERSION_12_5: u32 = 120500;
pub const MAC_OS_VERSION_12_6: u32 = 120600;
pub const MAC_OS_VERSION_12_7: u32 = 120700;
pub const MAC_OS_VERSION_13_0: u32 = 130000;
pub const MAC_OS_VERSION_13_1: u32 = 130100;
pub const MAC_OS_VERSION_13_2: u32 = 130200;
pub const MAC_OS_VERSION_13_3: u32 = 130300;
pub const MAC_OS_VERSION_13_4: u32 = 130400;
pub const MAC_OS_VERSION_13_5: u32 = 130500;
pub const MAC_OS_VERSION_13_6: u32 = 130600;
pub const MAC_OS_VERSION_13_7: u32 = 130700;
pub const MAC_OS_VERSION_14_0: u32 = 140000;
pub const MAC_OS_VERSION_14_1: u32 = 140100;
pub const MAC_OS_VERSION_14_2: u32 = 140200;
pub const MAC_OS_VERSION_14_3: u32 = 140300;
pub const MAC_OS_VERSION_14_4: u32 = 140400;
pub const MAC_OS_VERSION_14_5: u32 = 140500;
pub const MAC_OS_VERSION_14_6: u32 = 140600;
pub const MAC_OS_VERSION_14_7: u32 = 140700;
pub const MAC_OS_VERSION_15_0: u32 = 150000;
pub const MAC_OS_VERSION_15_1: u32 = 150100;
pub const MAC_OS_VERSION_15_2: u32 = 150200;
pub const MAC_OS_VERSION_15_3: u32 = 150300;
pub const MAC_OS_VERSION_15_4: u32 = 150400;
pub const MAC_OS_VERSION_15_5: u32 = 150500;
pub const MAC_OS_VERSION_15_6: u32 = 150600;
pub const MAC_OS_VERSION_16_0: u32 = 160000;
pub const MAC_OS_VERSION_26_0: u32 = 260000;
pub const MAC_OS_VERSION_26_1: u32 = 260100;
pub const MAC_OS_VERSION_26_2: u32 = 260200;
pub const MAC_OS_VERSION_26_3: u32 = 260300;
pub const MAC_OS_VERSION_26_4: u32 = 260400;
pub const __AVAILABILITY_VERSIONS_VERSION_HASH: u32 = 93585900;
pub const __AVAILABILITY_VERSIONS_VERSION_STRING: &[u8; 6] = b"Local\0";
pub const __AVAILABILITY_FILE: &[u8; 23] = b"AvailabilityVersions.h\0";
pub const __MAC_OS_X_VERSION_MAX_ALLOWED: u32 = 260400;
pub const __ENABLE_LEGACY_MAC_AVAILABILITY: u32 = 1;
pub const __has_safe_buffers: u32 = 1;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_EXTSN: &[u8; 14] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __has_ptrcheck: u32 = 0;
pub const __has_bounds_safety_attributes: u32 = 0;
pub const USE_CLANG_TYPES: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const __DARWIN_WCHAR_MIN: i32 = -2147483648;
pub const _FORTIFY_SOURCE: u32 = 2;
pub const __DARWIN_NSIG: u32 = 32;
pub const NSIG: u32 = 32;
pub const _ARM_SIGNAL_: u32 = 1;
pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;
pub const SIGEMT: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGBUS: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGSYS: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGURG: u32 = 16;
pub const SIGSTOP: u32 = 17;
pub const SIGTSTP: u32 = 18;
pub const SIGCONT: u32 = 19;
pub const SIGCHLD: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGIO: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGINFO: u32 = 29;
pub const SIGUSR1: u32 = 30;
pub const SIGUSR2: u32 = 31;
pub const __DARWIN_OPAQUE_ARM_THREAD_STATE64: u32 = 0;
pub const USE_CLANG_STDDEF: u32 = 0;
pub const SIGEV_NONE: u32 = 0;
pub const SIGEV_SIGNAL: u32 = 1;
pub const SIGEV_THREAD: u32 = 3;
pub const SIGEV_KEVENT: u32 = 4;
pub const ILL_NOOP: u32 = 0;
pub const ILL_ILLOPC: u32 = 1;
pub const ILL_ILLTRP: u32 = 2;
pub const ILL_PRVOPC: u32 = 3;
pub const ILL_ILLOPN: u32 = 4;
pub const ILL_ILLADR: u32 = 5;
pub const ILL_PRVREG: u32 = 6;
pub const ILL_COPROC: u32 = 7;
pub const ILL_BADSTK: u32 = 8;
pub const FPE_NOOP: u32 = 0;
pub const FPE_FLTDIV: u32 = 1;
pub const FPE_FLTOVF: u32 = 2;
pub const FPE_FLTUND: u32 = 3;
pub const FPE_FLTRES: u32 = 4;
pub const FPE_FLTINV: u32 = 5;
pub const FPE_FLTSUB: u32 = 6;
pub const FPE_INTDIV: u32 = 7;
pub const FPE_INTOVF: u32 = 8;
pub const SEGV_NOOP: u32 = 0;
pub const SEGV_MAPERR: u32 = 1;
pub const SEGV_ACCERR: u32 = 2;
pub const BUS_NOOP: u32 = 0;
pub const BUS_ADRALN: u32 = 1;
pub const BUS_ADRERR: u32 = 2;
pub const BUS_OBJERR: u32 = 3;
pub const TRAP_BRKPT: u32 = 1;
pub const TRAP_TRACE: u32 = 2;
pub const CLD_NOOP: u32 = 0;
pub const CLD_EXITED: u32 = 1;
pub const CLD_KILLED: u32 = 2;
pub const CLD_DUMPED: u32 = 3;
pub const CLD_TRAPPED: u32 = 4;
pub const CLD_STOPPED: u32 = 5;
pub const CLD_CONTINUED: u32 = 6;
pub const POLL_IN: u32 = 1;
pub const POLL_OUT: u32 = 2;
pub const POLL_MSG: u32 = 3;
pub const POLL_ERR: u32 = 4;
pub const POLL_PRI: u32 = 5;
pub const POLL_HUP: u32 = 6;
pub const SA_ONSTACK: u32 = 1;
pub const SA_RESTART: u32 = 2;
pub const SA_RESETHAND: u32 = 4;
pub const SA_NOCLDSTOP: u32 = 8;
pub const SA_NODEFER: u32 = 16;
pub const SA_NOCLDWAIT: u32 = 32;
pub const SA_SIGINFO: u32 = 64;
pub const SA_USERTRAMP: u32 = 256;
pub const SA_64REGSET: u32 = 512;
pub const SA_USERSPACE_MASK: u32 = 127;
pub const SIG_BLOCK: u32 = 1;
pub const SIG_UNBLOCK: u32 = 2;
pub const SIG_SETMASK: u32 = 3;
pub const SI_USER: u32 = 65537;
pub const SI_QUEUE: u32 = 65538;
pub const SI_TIMER: u32 = 65539;
pub const SI_ASYNCIO: u32 = 65540;
pub const SI_MESGQ: u32 = 65541;
pub const SS_ONSTACK: u32 = 1;
pub const SS_DISABLE: u32 = 4;
pub const MINSIGSTKSZ: u32 = 32768;
pub const SIGSTKSZ: u32 = 131072;
pub const SV_ONSTACK: u32 = 1;
pub const SV_INTERRUPT: u32 = 2;
pub const SV_RESETHAND: u32 = 4;
pub const SV_NODEFER: u32 = 16;
pub const SV_NOCLDSTOP: u32 = 8;
pub const SV_SIGINFO: u32 = 64;
pub const __WORDSIZE: u32 = 64;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const PRIO_PROCESS: u32 = 0;
pub const PRIO_PGRP: u32 = 1;
pub const PRIO_USER: u32 = 2;
pub const PRIO_DARWIN_THREAD: u32 = 3;
pub const PRIO_DARWIN_PROCESS: u32 = 4;
pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: u32 = 20;
pub const PRIO_DARWIN_BG: u32 = 4096;
pub const PRIO_DARWIN_NONUI: u32 = 4097;
pub const RUSAGE_SELF: u32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
pub const RUSAGE_INFO_V0: u32 = 0;
pub const RUSAGE_INFO_V1: u32 = 1;
pub const RUSAGE_INFO_V2: u32 = 2;
pub const RUSAGE_INFO_V3: u32 = 3;
pub const RUSAGE_INFO_V4: u32 = 4;
pub const RUSAGE_INFO_V5: u32 = 5;
pub const RUSAGE_INFO_V6: u32 = 6;
pub const RUSAGE_INFO_CURRENT: u32 = 6;
pub const RU_PROC_RUNS_RESLIDE: u32 = 1;
pub const RLIMIT_CPU: u32 = 0;
pub const RLIMIT_FSIZE: u32 = 1;
pub const RLIMIT_DATA: u32 = 2;
pub const RLIMIT_STACK: u32 = 3;
pub const RLIMIT_CORE: u32 = 4;
pub const RLIMIT_AS: u32 = 5;
pub const RLIMIT_RSS: u32 = 5;
pub const RLIMIT_MEMLOCK: u32 = 6;
pub const RLIMIT_NPROC: u32 = 7;
pub const RLIMIT_NOFILE: u32 = 8;
pub const RLIM_NLIMITS: u32 = 9;
pub const _RLIMIT_POSIX_FLAG: u32 = 4096;
pub const RLIMIT_WAKEUPS_MONITOR: u32 = 1;
pub const RLIMIT_CPU_USAGE_MONITOR: u32 = 2;
pub const RLIMIT_THREAD_CPULIMITS: u32 = 3;
pub const RLIMIT_FOOTPRINT_INTERVAL: u32 = 4;
pub const WAKEMON_ENABLE: u32 = 1;
pub const WAKEMON_DISABLE: u32 = 2;
pub const WAKEMON_GET_PARAMS: u32 = 4;
pub const WAKEMON_SET_DEFAULTS: u32 = 8;
pub const WAKEMON_MAKE_FATAL: u32 = 16;
pub const CPUMON_MAKE_FATAL: u32 = 4096;
pub const FOOTPRINT_INTERVAL_RESET: u32 = 1;
pub const IOPOL_TYPE_DISK: u32 = 0;
pub const IOPOL_TYPE_VFS_ATIME_UPDATES: u32 = 2;
pub const IOPOL_TYPE_VFS_MATERIALIZE_DATALESS_FILES: u32 = 3;
pub const IOPOL_TYPE_VFS_STATFS_NO_DATA_VOLUME: u32 = 4;
pub const IOPOL_TYPE_VFS_TRIGGER_RESOLVE: u32 = 5;
pub const IOPOL_TYPE_VFS_IGNORE_CONTENT_PROTECTION: u32 = 6;
pub const IOPOL_TYPE_VFS_IGNORE_PERMISSIONS: u32 = 7;
pub const IOPOL_TYPE_VFS_SKIP_MTIME_UPDATE: u32 = 8;
pub const IOPOL_TYPE_VFS_ALLOW_LOW_SPACE_WRITES: u32 = 9;
pub const IOPOL_TYPE_VFS_DISALLOW_RW_FOR_O_EVTONLY: u32 = 10;
pub const IOPOL_TYPE_VFS_ENTITLED_RESERVE_ACCESS: u32 = 14;
pub const IOPOL_SCOPE_PROCESS: u32 = 0;
pub const IOPOL_SCOPE_THREAD: u32 = 1;
pub const IOPOL_SCOPE_DARWIN_BG: u32 = 2;
pub const IOPOL_DEFAULT: u32 = 0;
pub const IOPOL_IMPORTANT: u32 = 1;
pub const IOPOL_PASSIVE: u32 = 2;
pub const IOPOL_THROTTLE: u32 = 3;
pub const IOPOL_UTILITY: u32 = 4;
pub const IOPOL_STANDARD: u32 = 5;
pub const IOPOL_APPLICATION: u32 = 5;
pub const IOPOL_NORMAL: u32 = 1;
pub const IOPOL_ATIME_UPDATES_DEFAULT: u32 = 0;
pub const IOPOL_ATIME_UPDATES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_DEFAULT: u32 = 0;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ON: u32 = 2;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ORIG: u32 = 4;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_BASIC_MASK: u32 = 3;
pub const IOPOL_VFS_STATFS_NO_DATA_VOLUME_DEFAULT: u32 = 0;
pub const IOPOL_VFS_STATFS_FORCE_NO_DATA_VOLUME: u32 = 1;
pub const IOPOL_VFS_TRIGGER_RESOLVE_DEFAULT: u32 = 0;
pub const IOPOL_VFS_TRIGGER_RESOLVE_OFF: u32 = 1;
pub const IOPOL_VFS_CONTENT_PROTECTION_DEFAULT: u32 = 0;
pub const IOPOL_VFS_CONTENT_PROTECTION_IGNORE: u32 = 1;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_OFF: u32 = 0;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_ON: u32 = 1;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_OFF: u32 = 0;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_ON: u32 = 1;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_IGNORE: u32 = 2;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_OFF: u32 = 0;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_ON: u32 = 1;
pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_DEFAULT: u32 = 0;
pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_ON: u32 = 1;
pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_DEFAULT: u32 = 0;
pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_ON: u32 = 1;
pub const IOPOL_VFS_ENTITLED_RESERVE_ACCESS_OFF: u32 = 0;
pub const IOPOL_VFS_ENTITLED_RESERVE_ACCESS_ON: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WCOREFLAG: u32 = 128;
pub const _WSTOPPED: u32 = 127;
pub const WEXITED: u32 = 4;
pub const WSTOPPED: u32 = 8;
pub const WCONTINUED: u32 = 16;
pub const WNOWAIT: u32 = 32;
pub const WAIT_ANY: i32 = -1;
pub const WAIT_MYPGRP: u32 = 0;
pub const _QUAD_HIGHWORD: u32 = 1;
pub const _QUAD_LOWWORD: u32 = 0;
pub const __DARWIN_LITTLE_ENDIAN: u32 = 1234;
pub const __DARWIN_BIG_ENDIAN: u32 = 4321;
pub const __DARWIN_PDP_ENDIAN: u32 = 3412;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const __DARWIN_BYTE_ORDER: u32 = 1234;
pub const BYTE_ORDER: u32 = 1234;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const RAND_MAX: u32 = 2147483647;
pub const _MALLOC_TYPE_MALLOC_BACKDEPLOY_PUBLIC: u32 = 1;
pub const CUDA_VERSION: u32 = 13030;
pub const CU_IPC_HANDLE_SIZE: u32 = 64;
pub const CU_COMPUTE_ACCELERATED_TARGET_BASE: u32 = 65536;
pub const CU_COMPUTE_FAMILY_TARGET_BASE: u32 = 131072;
pub const CU_GRAPH_COND_ASSIGN_DEFAULT: u32 = 1;
pub const CU_GRAPH_KERNEL_NODE_PORT_DEFAULT: u32 = 0;
pub const CU_GRAPH_KERNEL_NODE_PORT_PROGRAMMATIC: u32 = 1;
pub const CU_GRAPH_KERNEL_NODE_PORT_LAUNCH_ORDER: u32 = 2;
pub const CU_MEMHOSTALLOC_PORTABLE: u32 = 1;
pub const CU_MEMHOSTALLOC_DEVICEMAP: u32 = 2;
pub const CU_MEMHOSTALLOC_WRITECOMBINED: u32 = 4;
pub const CU_MEMHOSTREGISTER_PORTABLE: u32 = 1;
pub const CU_MEMHOSTREGISTER_DEVICEMAP: u32 = 2;
pub const CU_MEMHOSTREGISTER_IOMEMORY: u32 = 4;
pub const CU_MEMHOSTREGISTER_READ_ONLY: u32 = 8;
pub const CU_ARRAY_SPARSE_PROPERTIES_SINGLE_MIPTAIL: u32 = 1;
pub const CU_TENSOR_MAP_NUM_QWORDS: u32 = 16;
pub const CUDA_EXTERNAL_MEMORY_DEDICATED: u32 = 1;
pub const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_SKIP_NVSCIBUF_MEMSYNC: u32 = 1;
pub const CUDA_EXTERNAL_SEMAPHORE_WAIT_SKIP_NVSCIBUF_MEMSYNC: u32 = 2;
pub const CUDA_NVSCISYNC_ATTR_SIGNAL: u32 = 1;
pub const CUDA_NVSCISYNC_ATTR_WAIT: u32 = 2;
pub const CU_MEM_CREATE_USAGE_TILE_POOL: u32 = 1;
pub const CU_MEM_CREATE_USAGE_HW_DECOMPRESS: u32 = 2;
pub const CU_MEM_POOL_CREATE_USAGE_HW_DECOMPRESS: u32 = 2;
pub const CUDA_COOPERATIVE_LAUNCH_MULTI_DEVICE_NO_PRE_LAUNCH_SYNC: u32 = 1;
pub const CUDA_COOPERATIVE_LAUNCH_MULTI_DEVICE_NO_POST_LAUNCH_SYNC: u32 = 2;
pub const CUDA_ARRAY3D_LAYERED: u32 = 1;
pub const CUDA_ARRAY3D_2DARRAY: u32 = 1;
pub const CUDA_ARRAY3D_SURFACE_LDST: u32 = 2;
pub const CUDA_ARRAY3D_CUBEMAP: u32 = 4;
pub const CUDA_ARRAY3D_TEXTURE_GATHER: u32 = 8;
pub const CUDA_ARRAY3D_DEPTH_TEXTURE: u32 = 16;
pub const CUDA_ARRAY3D_COLOR_ATTACHMENT: u32 = 32;
pub const CUDA_ARRAY3D_SPARSE: u32 = 64;
pub const CUDA_ARRAY3D_DEFERRED_MAPPING: u32 = 128;
pub const CUDA_ARRAY3D_VIDEO_ENCODE_DECODE: u32 = 256;
pub const CU_TRSA_OVERRIDE_FORMAT: u32 = 1;
pub const CU_TRSF_READ_AS_INTEGER: u32 = 1;
pub const CU_TRSF_NORMALIZED_COORDINATES: u32 = 2;
pub const CU_TRSF_SRGB: u32 = 16;
pub const CU_TRSF_DISABLE_TRILINEAR_OPTIMIZATION: u32 = 32;
pub const CU_TRSF_SEAMLESS_CUBEMAP: u32 = 64;
pub const CU_LAUNCH_KERNEL_REQUIRED_BLOCK_DIM: u32 = 1;
pub const CU_LAUNCH_PARAM_END_AS_INT: u32 = 0;
pub const CU_LAUNCH_PARAM_BUFFER_POINTER_AS_INT: u32 = 1;
pub const CU_LAUNCH_PARAM_BUFFER_SIZE_AS_INT: u32 = 2;
pub const CU_PARAM_TR_DEFAULT: i32 = -1;
pub const RESOURCE_ABI_VERSION: u32 = 1;
pub const RESOURCE_ABI_BYTES: u32 = 40;
pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 2;
pub const CU_IPC_MEM_LAZY_ENABLE_PEER_ACCESS: CUipcMem_flags_enum = 1;
pub const CU_MEM_ATTACH_GLOBAL: CUmemAttach_flags_enum = 1;
pub const CU_MEM_ATTACH_HOST: CUmemAttach_flags_enum = 2;
pub const CU_MEM_ATTACH_SINGLE: CUmemAttach_flags_enum = 4;
pub const CU_CTX_SCHED_AUTO: CUctx_flags_enum = 0;
pub const CU_CTX_SCHED_SPIN: CUctx_flags_enum = 1;
pub const CU_CTX_SCHED_YIELD: CUctx_flags_enum = 2;
pub const CU_CTX_SCHED_BLOCKING_SYNC: CUctx_flags_enum = 4;
pub const CU_CTX_BLOCKING_SYNC: CUctx_flags_enum = 4;
pub const CU_CTX_SCHED_MASK: CUctx_flags_enum = 7;
pub const CU_CTX_MAP_HOST: CUctx_flags_enum = 8;
pub const CU_CTX_LMEM_RESIZE_TO_MAX: CUctx_flags_enum = 16;
pub const CU_CTX_COREDUMP_ENABLE: CUctx_flags_enum = 32;
pub const CU_CTX_USER_COREDUMP_ENABLE: CUctx_flags_enum = 64;
pub const CU_CTX_SYNC_MEMOPS: CUctx_flags_enum = 128;
pub const CU_CTX_FLAGS_MASK: CUctx_flags_enum = 255;
pub const CU_EVENT_SCHED_AUTO: CUevent_sched_flags_enum = 0;
pub const CU_EVENT_SCHED_SPIN: CUevent_sched_flags_enum = 1;
pub const CU_EVENT_SCHED_YIELD: CUevent_sched_flags_enum = 2;
pub const CU_EVENT_SCHED_BLOCKING_SYNC: CUevent_sched_flags_enum = 4;
pub const NVCL_EVENT_SCHED_AUTO: cl_event_flags_enum = 0;
pub const NVCL_EVENT_SCHED_SPIN: cl_event_flags_enum = 1;
pub const NVCL_EVENT_SCHED_YIELD: cl_event_flags_enum = 2;
pub const NVCL_EVENT_SCHED_BLOCKING_SYNC: cl_event_flags_enum = 4;
pub const NVCL_CTX_SCHED_AUTO: cl_context_flags_enum = 0;
pub const NVCL_CTX_SCHED_SPIN: cl_context_flags_enum = 1;
pub const NVCL_CTX_SCHED_YIELD: cl_context_flags_enum = 2;
pub const NVCL_CTX_SCHED_BLOCKING_SYNC: cl_context_flags_enum = 4;
pub const CU_HOST_TASK_BLOCKING: CUhostTaskSyncMode_enum = 0;
pub const CU_HOST_TASK_SPINWAIT: CUhostTaskSyncMode_enum = 1;
pub const CU_STREAM_DEFAULT: CUstream_flags_enum = 0;
pub const CU_STREAM_NON_BLOCKING: CUstream_flags_enum = 1;
pub const CU_EVENT_DEFAULT: CUevent_flags_enum = 0;
pub const CU_EVENT_BLOCKING_SYNC: CUevent_flags_enum = 1;
pub const CU_EVENT_DISABLE_TIMING: CUevent_flags_enum = 2;
pub const CU_EVENT_INTERPROCESS: CUevent_flags_enum = 4;
pub const CU_EVENT_RECORD_DEFAULT: CUevent_record_flags_enum = 0;
pub const CU_EVENT_RECORD_EXTERNAL: CUevent_record_flags_enum = 1;
pub const CU_EVENT_WAIT_DEFAULT: CUevent_wait_flags_enum = 0;
pub const CU_EVENT_WAIT_EXTERNAL: CUevent_wait_flags_enum = 1;
pub const CU_ATOMIC_OPERATION_INTEGER_ADD: CUatomicOperation_enum = 0;
pub const CU_ATOMIC_OPERATION_INTEGER_MIN: CUatomicOperation_enum = 1;
pub const CU_ATOMIC_OPERATION_INTEGER_MAX: CUatomicOperation_enum = 2;
pub const CU_ATOMIC_OPERATION_INTEGER_INCREMENT: CUatomicOperation_enum = 3;
pub const CU_ATOMIC_OPERATION_INTEGER_DECREMENT: CUatomicOperation_enum = 4;
pub const CU_ATOMIC_OPERATION_AND: CUatomicOperation_enum = 5;
pub const CU_ATOMIC_OPERATION_OR: CUatomicOperation_enum = 6;
pub const CU_ATOMIC_OPERATION_XOR: CUatomicOperation_enum = 7;
pub const CU_ATOMIC_OPERATION_EXCHANGE: CUatomicOperation_enum = 8;
pub const CU_ATOMIC_OPERATION_CAS: CUatomicOperation_enum = 9;
pub const CU_ATOMIC_OPERATION_FLOAT_ADD: CUatomicOperation_enum = 10;
pub const CU_ATOMIC_OPERATION_FLOAT_MIN: CUatomicOperation_enum = 11;
pub const CU_ATOMIC_OPERATION_FLOAT_MAX: CUatomicOperation_enum = 12;
pub const CU_ATOMIC_OPERATION_MAX: CUatomicOperation_enum = 13;
pub const CU_ATOMIC_CAPABILITY_SIGNED: CUatomicOperationCapability_enum = 1;
pub const CU_ATOMIC_CAPABILITY_UNSIGNED: CUatomicOperationCapability_enum = 2;
pub const CU_ATOMIC_CAPABILITY_REDUCTION: CUatomicOperationCapability_enum = 4;
pub const CU_ATOMIC_CAPABILITY_SCALAR_32: CUatomicOperationCapability_enum = 8;
pub const CU_ATOMIC_CAPABILITY_SCALAR_64: CUatomicOperationCapability_enum = 16;
pub const CU_ATOMIC_CAPABILITY_SCALAR_128: CUatomicOperationCapability_enum = 32;
pub const CU_ATOMIC_CAPABILITY_VECTOR_32x4: CUatomicOperationCapability_enum = 64;
pub const CU_STREAM_WAIT_VALUE_GEQ: CUstreamWaitValue_flags_enum = 0;
pub const CU_STREAM_WAIT_VALUE_EQ: CUstreamWaitValue_flags_enum = 1;
pub const CU_STREAM_WAIT_VALUE_AND: CUstreamWaitValue_flags_enum = 2;
pub const CU_STREAM_WAIT_VALUE_NOR: CUstreamWaitValue_flags_enum = 3;
pub const CU_STREAM_WAIT_VALUE_FLUSH: CUstreamWaitValue_flags_enum = 1073741824;
pub const CU_STREAM_WRITE_VALUE_DEFAULT: CUstreamWriteValue_flags_enum = 0;
pub const CU_STREAM_WRITE_VALUE_NO_MEMORY_BARRIER: CUstreamWriteValue_flags_enum = 1;
pub const CU_STREAM_MEM_OP_WAIT_VALUE_32: CUstreamBatchMemOpType_enum = 1;
pub const CU_STREAM_MEM_OP_WRITE_VALUE_32: CUstreamBatchMemOpType_enum = 2;
pub const CU_STREAM_MEM_OP_WAIT_VALUE_64: CUstreamBatchMemOpType_enum = 4;
pub const CU_STREAM_MEM_OP_WRITE_VALUE_64: CUstreamBatchMemOpType_enum = 5;
pub const CU_STREAM_MEM_OP_BARRIER: CUstreamBatchMemOpType_enum = 6;
pub const CU_STREAM_MEM_OP_ATOMIC_REDUCTION: CUstreamBatchMemOpType_enum = 8;
pub const CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES: CUstreamBatchMemOpType_enum = 3;
pub const CU_STREAM_MEMORY_BARRIER_TYPE_SYS: CUstreamMemoryBarrier_flags_enum = 0;
pub const CU_STREAM_MEMORY_BARRIER_TYPE_GPU: CUstreamMemoryBarrier_flags_enum = 1;
pub const CU_STREAM_ATOMIC_REDUCTION_OP_OR: CUstreamAtomicReductionOpType_enum = 6;
pub const CU_STREAM_ATOMIC_REDUCTION_OP_AND: CUstreamAtomicReductionOpType_enum = 5;
pub const CU_STREAM_ATOMIC_REDUCTION_OP_ADD: CUstreamAtomicReductionOpType_enum = 0;
pub const CU_STREAM_ATOMIC_REDUCTION_UNSIGNED_32: CUstreamAtomicReductionDataType_enum = 14;
pub const CU_STREAM_ATOMIC_REDUCTION_UNSIGNED_64: CUstreamAtomicReductionDataType_enum = 22;
pub const CU_OCCUPANCY_DEFAULT: CUoccupancy_flags_enum = 0;
pub const CU_OCCUPANCY_DISABLE_CACHING_OVERRIDE: CUoccupancy_flags_enum = 1;
pub const CU_STREAM_ADD_CAPTURE_DEPENDENCIES: CUstreamUpdateCaptureDependencies_flags_enum = 0;
pub const CU_STREAM_SET_CAPTURE_DEPENDENCIES: CUstreamUpdateCaptureDependencies_flags_enum = 1;
pub const CU_ASYNC_NOTIFICATION_TYPE_OVER_BUDGET: CUasyncNotificationType_enum = 1;
pub const CU_AD_FORMAT_UNSIGNED_INT8: CUarray_format_enum = 1;
pub const CU_AD_FORMAT_UNSIGNED_INT16: CUarray_format_enum = 2;
pub const CU_AD_FORMAT_UNSIGNED_INT32: CUarray_format_enum = 3;
pub const CU_AD_FORMAT_SIGNED_INT8: CUarray_format_enum = 8;
pub const CU_AD_FORMAT_SIGNED_INT16: CUarray_format_enum = 9;
pub const CU_AD_FORMAT_SIGNED_INT32: CUarray_format_enum = 10;
pub const CU_AD_FORMAT_HALF: CUarray_format_enum = 16;
pub const CU_AD_FORMAT_FLOAT: CUarray_format_enum = 32;
pub const CU_AD_FORMAT_NV12: CUarray_format_enum = 176;
pub const CU_AD_FORMAT_UNORM_INT8X1: CUarray_format_enum = 192;
pub const CU_AD_FORMAT_UNORM_INT8X2: CUarray_format_enum = 193;
pub const CU_AD_FORMAT_UNORM_INT8X4: CUarray_format_enum = 194;
pub const CU_AD_FORMAT_UNORM_INT16X1: CUarray_format_enum = 195;
pub const CU_AD_FORMAT_UNORM_INT16X2: CUarray_format_enum = 196;
pub const CU_AD_FORMAT_UNORM_INT16X4: CUarray_format_enum = 197;
pub const CU_AD_FORMAT_SNORM_INT8X1: CUarray_format_enum = 198;
pub const CU_AD_FORMAT_SNORM_INT8X2: CUarray_format_enum = 199;
pub const CU_AD_FORMAT_SNORM_INT8X4: CUarray_format_enum = 200;
pub const CU_AD_FORMAT_SNORM_INT16X1: CUarray_format_enum = 201;
pub const CU_AD_FORMAT_SNORM_INT16X2: CUarray_format_enum = 202;
pub const CU_AD_FORMAT_SNORM_INT16X4: CUarray_format_enum = 203;
pub const CU_AD_FORMAT_BC1_UNORM: CUarray_format_enum = 145;
pub const CU_AD_FORMAT_BC1_UNORM_SRGB: CUarray_format_enum = 146;
pub const CU_AD_FORMAT_BC2_UNORM: CUarray_format_enum = 147;
pub const CU_AD_FORMAT_BC2_UNORM_SRGB: CUarray_format_enum = 148;
pub const CU_AD_FORMAT_BC3_UNORM: CUarray_format_enum = 149;
pub const CU_AD_FORMAT_BC3_UNORM_SRGB: CUarray_format_enum = 150;
pub const CU_AD_FORMAT_BC4_UNORM: CUarray_format_enum = 151;
pub const CU_AD_FORMAT_BC4_SNORM: CUarray_format_enum = 152;
pub const CU_AD_FORMAT_BC5_UNORM: CUarray_format_enum = 153;
pub const CU_AD_FORMAT_BC5_SNORM: CUarray_format_enum = 154;
pub const CU_AD_FORMAT_BC6H_UF16: CUarray_format_enum = 155;
pub const CU_AD_FORMAT_BC6H_SF16: CUarray_format_enum = 156;
pub const CU_AD_FORMAT_BC7_UNORM: CUarray_format_enum = 157;
pub const CU_AD_FORMAT_BC7_UNORM_SRGB: CUarray_format_enum = 158;
pub const CU_AD_FORMAT_P010: CUarray_format_enum = 159;
pub const CU_AD_FORMAT_P016: CUarray_format_enum = 161;
pub const CU_AD_FORMAT_NV16: CUarray_format_enum = 162;
pub const CU_AD_FORMAT_P210: CUarray_format_enum = 163;
pub const CU_AD_FORMAT_P216: CUarray_format_enum = 164;
pub const CU_AD_FORMAT_YUY2: CUarray_format_enum = 165;
pub const CU_AD_FORMAT_Y210: CUarray_format_enum = 166;
pub const CU_AD_FORMAT_Y216: CUarray_format_enum = 167;
pub const CU_AD_FORMAT_AYUV: CUarray_format_enum = 168;
pub const CU_AD_FORMAT_Y410: CUarray_format_enum = 169;
pub const CU_AD_FORMAT_Y416: CUarray_format_enum = 177;
pub const CU_AD_FORMAT_Y444_PLANAR8: CUarray_format_enum = 178;
pub const CU_AD_FORMAT_Y444_PLANAR10: CUarray_format_enum = 179;
pub const CU_AD_FORMAT_YUV444_8bit_SemiPlanar: CUarray_format_enum = 180;
pub const CU_AD_FORMAT_YUV444_16bit_SemiPlanar: CUarray_format_enum = 181;
pub const CU_AD_FORMAT_UNORM_INT_101010_2: CUarray_format_enum = 80;
pub const CU_AD_FORMAT_UINT8_PACKED_422: CUarray_format_enum = 81;
pub const CU_AD_FORMAT_UINT8_PACKED_444: CUarray_format_enum = 82;
pub const CU_AD_FORMAT_UINT8_SEMIPLANAR_420: CUarray_format_enum = 83;
pub const CU_AD_FORMAT_UINT16_SEMIPLANAR_420: CUarray_format_enum = 84;
pub const CU_AD_FORMAT_UINT8_SEMIPLANAR_422: CUarray_format_enum = 85;
pub const CU_AD_FORMAT_UINT16_SEMIPLANAR_422: CUarray_format_enum = 86;
pub const CU_AD_FORMAT_UINT8_SEMIPLANAR_444: CUarray_format_enum = 87;
pub const CU_AD_FORMAT_UINT16_SEMIPLANAR_444: CUarray_format_enum = 88;
pub const CU_AD_FORMAT_UINT8_PLANAR_420: CUarray_format_enum = 89;
pub const CU_AD_FORMAT_UINT16_PLANAR_420: CUarray_format_enum = 90;
pub const CU_AD_FORMAT_UINT8_PLANAR_422: CUarray_format_enum = 91;
pub const CU_AD_FORMAT_UINT16_PLANAR_422: CUarray_format_enum = 92;
pub const CU_AD_FORMAT_UINT8_PLANAR_444: CUarray_format_enum = 93;
pub const CU_AD_FORMAT_UINT16_PLANAR_444: CUarray_format_enum = 94;
pub const CU_AD_FORMAT_MAX: CUarray_format_enum = 2147483647;
pub const CU_TR_ADDRESS_MODE_WRAP: CUaddress_mode_enum = 0;
pub const CU_TR_ADDRESS_MODE_CLAMP: CUaddress_mode_enum = 1;
pub const CU_TR_ADDRESS_MODE_MIRROR: CUaddress_mode_enum = 2;
pub const CU_TR_ADDRESS_MODE_BORDER: CUaddress_mode_enum = 3;
pub const CU_TR_FILTER_MODE_POINT: CUfilter_mode_enum = 0;
pub const CU_TR_FILTER_MODE_LINEAR: CUfilter_mode_enum = 1;
pub const CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK: CUdevice_attribute_enum = 1;
pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X: CUdevice_attribute_enum = 2;
pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y: CUdevice_attribute_enum = 3;
pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z: CUdevice_attribute_enum = 4;
pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X: CUdevice_attribute_enum = 5;
pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y: CUdevice_attribute_enum = 6;
pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z: CUdevice_attribute_enum = 7;
pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK: CUdevice_attribute_enum = 8;
pub const CU_DEVICE_ATTRIBUTE_SHARED_MEMORY_PER_BLOCK: CUdevice_attribute_enum = 8;
pub const CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY: CUdevice_attribute_enum = 9;
pub const CU_DEVICE_ATTRIBUTE_WARP_SIZE: CUdevice_attribute_enum = 10;
pub const CU_DEVICE_ATTRIBUTE_MAX_PITCH: CUdevice_attribute_enum = 11;
pub const CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK: CUdevice_attribute_enum = 12;
pub const CU_DEVICE_ATTRIBUTE_REGISTERS_PER_BLOCK: CUdevice_attribute_enum = 12;
pub const CU_DEVICE_ATTRIBUTE_CLOCK_RATE: CUdevice_attribute_enum = 13;
pub const CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT: CUdevice_attribute_enum = 14;
pub const CU_DEVICE_ATTRIBUTE_GPU_OVERLAP: CUdevice_attribute_enum = 15;
pub const CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT: CUdevice_attribute_enum = 16;
pub const CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT: CUdevice_attribute_enum = 17;
pub const CU_DEVICE_ATTRIBUTE_INTEGRATED: CUdevice_attribute_enum = 18;
pub const CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY: CUdevice_attribute_enum = 19;
pub const CU_DEVICE_ATTRIBUTE_COMPUTE_MODE: CUdevice_attribute_enum = 20;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH: CUdevice_attribute_enum = 21;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_WIDTH: CUdevice_attribute_enum = 22;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_HEIGHT: CUdevice_attribute_enum = 23;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH: CUdevice_attribute_enum = 24;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT: CUdevice_attribute_enum = 25;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH: CUdevice_attribute_enum = 26;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH: CUdevice_attribute_enum = 27;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT: CUdevice_attribute_enum = 28;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS: CUdevice_attribute_enum = 29;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_WIDTH: CUdevice_attribute_enum = 27;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_HEIGHT: CUdevice_attribute_enum = 28;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_NUMSLICES: CUdevice_attribute_enum = 29;
pub const CU_DEVICE_ATTRIBUTE_SURFACE_ALIGNMENT: CUdevice_attribute_enum = 30;
pub const CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS: CUdevice_attribute_enum = 31;
pub const CU_DEVICE_ATTRIBUTE_ECC_ENABLED: CUdevice_attribute_enum = 32;
pub const CU_DEVICE_ATTRIBUTE_PCI_BUS_ID: CUdevice_attribute_enum = 33;
pub const CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID: CUdevice_attribute_enum = 34;
pub const CU_DEVICE_ATTRIBUTE_TCC_DRIVER: CUdevice_attribute_enum = 35;
pub const CU_DEVICE_ATTRIBUTE_MEMORY_CLOCK_RATE: CUdevice_attribute_enum = 36;
pub const CU_DEVICE_ATTRIBUTE_GLOBAL_MEMORY_BUS_WIDTH: CUdevice_attribute_enum = 37;
pub const CU_DEVICE_ATTRIBUTE_L2_CACHE_SIZE: CUdevice_attribute_enum = 38;
pub const CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR: CUdevice_attribute_enum = 39;
pub const CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT: CUdevice_attribute_enum = 40;
pub const CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING: CUdevice_attribute_enum = 41;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH: CUdevice_attribute_enum = 42;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS: CUdevice_attribute_enum = 43;
pub const CU_DEVICE_ATTRIBUTE_CAN_TEX2D_GATHER: CUdevice_attribute_enum = 44;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH: CUdevice_attribute_enum = 45;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT: CUdevice_attribute_enum = 46;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE: CUdevice_attribute_enum = 47;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE: CUdevice_attribute_enum = 48;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE: CUdevice_attribute_enum = 49;
pub const CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID: CUdevice_attribute_enum = 50;
pub const CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT: CUdevice_attribute_enum = 51;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH: CUdevice_attribute_enum = 52;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH: CUdevice_attribute_enum = 53;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS: CUdevice_attribute_enum = 54;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH: CUdevice_attribute_enum = 55;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH: CUdevice_attribute_enum = 56;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT: CUdevice_attribute_enum = 57;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH: CUdevice_attribute_enum = 58;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT: CUdevice_attribute_enum = 59;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH: CUdevice_attribute_enum = 60;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH: CUdevice_attribute_enum = 61;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS: CUdevice_attribute_enum = 62;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH: CUdevice_attribute_enum = 63;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT: CUdevice_attribute_enum = 64;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS: CUdevice_attribute_enum = 65;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH: CUdevice_attribute_enum = 66;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH: CUdevice_attribute_enum = 67;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS: CUdevice_attribute_enum = 68;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH: CUdevice_attribute_enum = 69;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH: CUdevice_attribute_enum = 70;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT: CUdevice_attribute_enum = 71;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH: CUdevice_attribute_enum = 72;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH: CUdevice_attribute_enum = 73;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT: CUdevice_attribute_enum = 74;
pub const CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR: CUdevice_attribute_enum = 75;
pub const CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR: CUdevice_attribute_enum = 76;
pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH: CUdevice_attribute_enum = 77;
pub const CU_DEVICE_ATTRIBUTE_STREAM_PRIORITIES_SUPPORTED: CUdevice_attribute_enum = 78;
pub const CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED: CUdevice_attribute_enum = 79;
pub const CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED: CUdevice_attribute_enum = 80;
pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR: CUdevice_attribute_enum = 81;
pub const CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR: CUdevice_attribute_enum = 82;
pub const CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY: CUdevice_attribute_enum = 83;
pub const CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD: CUdevice_attribute_enum = 84;
pub const CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID: CUdevice_attribute_enum = 85;
pub const CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED: CUdevice_attribute_enum = 86;
pub const CU_DEVICE_ATTRIBUTE_SINGLE_TO_DOUBLE_PRECISION_PERF_RATIO: CUdevice_attribute_enum = 87;
pub const CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS: CUdevice_attribute_enum = 88;
pub const CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS: CUdevice_attribute_enum = 89;
pub const CU_DEVICE_ATTRIBUTE_COMPUTE_PREEMPTION_SUPPORTED: CUdevice_attribute_enum = 90;
pub const CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM: CUdevice_attribute_enum = 91;
pub const CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_MEM_OPS_V1: CUdevice_attribute_enum = 92;
pub const CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS_V1: CUdevice_attribute_enum = 93;
pub const CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR_V1: CUdevice_attribute_enum = 94;
pub const CU_DEVICE_ATTRIBUTE_COOPERATIVE_LAUNCH: CUdevice_attribute_enum = 95;
pub const CU_DEVICE_ATTRIBUTE_COOPERATIVE_MULTI_DEVICE_LAUNCH: CUdevice_attribute_enum = 96;
pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN: CUdevice_attribute_enum = 97;
pub const CU_DEVICE_ATTRIBUTE_CAN_FLUSH_REMOTE_WRITES: CUdevice_attribute_enum = 98;
pub const CU_DEVICE_ATTRIBUTE_HOST_REGISTER_SUPPORTED: CUdevice_attribute_enum = 99;
pub const CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES:
    CUdevice_attribute_enum = 100;
pub const CU_DEVICE_ATTRIBUTE_DIRECT_MANAGED_MEM_ACCESS_FROM_HOST: CUdevice_attribute_enum = 101;
pub const CU_DEVICE_ATTRIBUTE_VIRTUAL_ADDRESS_MANAGEMENT_SUPPORTED: CUdevice_attribute_enum = 102;
pub const CU_DEVICE_ATTRIBUTE_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED: CUdevice_attribute_enum = 102;
pub const CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR_SUPPORTED: CUdevice_attribute_enum =
    103;
pub const CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_HANDLE_SUPPORTED: CUdevice_attribute_enum = 104;
pub const CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_KMT_HANDLE_SUPPORTED: CUdevice_attribute_enum = 105;
pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR: CUdevice_attribute_enum = 106;
pub const CU_DEVICE_ATTRIBUTE_GENERIC_COMPRESSION_SUPPORTED: CUdevice_attribute_enum = 107;
pub const CU_DEVICE_ATTRIBUTE_MAX_PERSISTING_L2_CACHE_SIZE: CUdevice_attribute_enum = 108;
pub const CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE: CUdevice_attribute_enum = 109;
pub const CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WITH_CUDA_VMM_SUPPORTED: CUdevice_attribute_enum =
    110;
pub const CU_DEVICE_ATTRIBUTE_RESERVED_SHARED_MEMORY_PER_BLOCK: CUdevice_attribute_enum = 111;
pub const CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED: CUdevice_attribute_enum = 112;
pub const CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED: CUdevice_attribute_enum = 113;
pub const CU_DEVICE_ATTRIBUTE_TIMELINE_SEMAPHORE_INTEROP_SUPPORTED: CUdevice_attribute_enum = 114;
pub const CU_DEVICE_ATTRIBUTE_MEMORY_POOLS_SUPPORTED: CUdevice_attribute_enum = 115;
pub const CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_SUPPORTED: CUdevice_attribute_enum = 116;
pub const CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS: CUdevice_attribute_enum = 117;
pub const CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WRITES_ORDERING: CUdevice_attribute_enum = 118;
pub const CU_DEVICE_ATTRIBUTE_MEMPOOL_SUPPORTED_HANDLE_TYPES: CUdevice_attribute_enum = 119;
pub const CU_DEVICE_ATTRIBUTE_CLUSTER_LAUNCH: CUdevice_attribute_enum = 120;
pub const CU_DEVICE_ATTRIBUTE_DEFERRED_MAPPING_CUDA_ARRAY_SUPPORTED: CUdevice_attribute_enum = 121;
pub const CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS: CUdevice_attribute_enum = 122;
pub const CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR: CUdevice_attribute_enum = 123;
pub const CU_DEVICE_ATTRIBUTE_DMA_BUF_SUPPORTED: CUdevice_attribute_enum = 124;
pub const CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED: CUdevice_attribute_enum = 125;
pub const CU_DEVICE_ATTRIBUTE_MEM_SYNC_DOMAIN_COUNT: CUdevice_attribute_enum = 126;
pub const CU_DEVICE_ATTRIBUTE_TENSOR_MAP_ACCESS_SUPPORTED: CUdevice_attribute_enum = 127;
pub const CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_FABRIC_SUPPORTED: CUdevice_attribute_enum = 128;
pub const CU_DEVICE_ATTRIBUTE_UNIFIED_FUNCTION_POINTERS: CUdevice_attribute_enum = 129;
pub const CU_DEVICE_ATTRIBUTE_NUMA_CONFIG: CUdevice_attribute_enum = 130;
pub const CU_DEVICE_ATTRIBUTE_NUMA_ID: CUdevice_attribute_enum = 131;
pub const CU_DEVICE_ATTRIBUTE_MULTICAST_SUPPORTED: CUdevice_attribute_enum = 132;
pub const CU_DEVICE_ATTRIBUTE_MPS_ENABLED: CUdevice_attribute_enum = 133;
pub const CU_DEVICE_ATTRIBUTE_HOST_NUMA_ID: CUdevice_attribute_enum = 134;
pub const CU_DEVICE_ATTRIBUTE_D3D12_CIG_SUPPORTED: CUdevice_attribute_enum = 135;
pub const CU_DEVICE_ATTRIBUTE_MEM_DECOMPRESS_ALGORITHM_MASK: CUdevice_attribute_enum = 136;
pub const CU_DEVICE_ATTRIBUTE_MEM_DECOMPRESS_MAXIMUM_LENGTH: CUdevice_attribute_enum = 137;
pub const CU_DEVICE_ATTRIBUTE_VULKAN_CIG_SUPPORTED: CUdevice_attribute_enum = 138;
pub const CU_DEVICE_ATTRIBUTE_GPU_PCI_DEVICE_ID: CUdevice_attribute_enum = 139;
pub const CU_DEVICE_ATTRIBUTE_GPU_PCI_SUBSYSTEM_ID: CUdevice_attribute_enum = 140;
pub const CU_DEVICE_ATTRIBUTE_HOST_NUMA_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED:
    CUdevice_attribute_enum = 141;
pub const CU_DEVICE_ATTRIBUTE_HOST_NUMA_MEMORY_POOLS_SUPPORTED: CUdevice_attribute_enum = 142;
pub const CU_DEVICE_ATTRIBUTE_HOST_NUMA_MULTINODE_IPC_SUPPORTED: CUdevice_attribute_enum = 143;
pub const CU_DEVICE_ATTRIBUTE_HOST_MEMORY_POOLS_SUPPORTED: CUdevice_attribute_enum = 144;
pub const CU_DEVICE_ATTRIBUTE_HOST_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED: CUdevice_attribute_enum =
    145;
pub const CU_DEVICE_ATTRIBUTE_HOST_ALLOC_DMA_BUF_SUPPORTED: CUdevice_attribute_enum = 146;
pub const CU_DEVICE_ATTRIBUTE_ONLY_PARTIAL_HOST_NATIVE_ATOMIC_SUPPORTED: CUdevice_attribute_enum =
    147;
pub const CU_DEVICE_ATTRIBUTE_ATOMIC_REDUCTION_SUPPORTED: CUdevice_attribute_enum = 148;
pub const CU_DEVICE_ATTRIBUTE_D3D12_CIG_STREAMS_SUPPORTED: CUdevice_attribute_enum = 151;
pub const CU_DEVICE_ATTRIBUTE_DMA_BUF_MMAP_SUPPORTED: CUdevice_attribute_enum = 152;
pub const CU_DEVICE_ATTRIBUTE_LOGICAL_ENDPOINT_UNICAST_SUPPORTED: CUdevice_attribute_enum = 153;
pub const CU_DEVICE_ATTRIBUTE_LOGICAL_ENDPOINT_MULTICAST_SUPPORTED: CUdevice_attribute_enum = 154;
pub const CU_DEVICE_ATTRIBUTE_LOGICAL_ENDPOINT_COUNTED_OPS_SUPPORTED: CUdevice_attribute_enum = 155;
pub const CU_DEVICE_ATTRIBUTE_LOGICAL_ENDPOINT_UNICAST_ACCESS_ON_OWNER_DEVICE_SUPPORTED:
    CUdevice_attribute_enum = 156;
pub const CU_DEVICE_ATTRIBUTE_MAX: CUdevice_attribute_enum = 157;
pub const CU_POINTER_ATTRIBUTE_CONTEXT: CUpointer_attribute_enum = 1;
pub const CU_POINTER_ATTRIBUTE_MEMORY_TYPE: CUpointer_attribute_enum = 2;
pub const CU_POINTER_ATTRIBUTE_DEVICE_POINTER: CUpointer_attribute_enum = 3;
pub const CU_POINTER_ATTRIBUTE_HOST_POINTER: CUpointer_attribute_enum = 4;
pub const CU_POINTER_ATTRIBUTE_P2P_TOKENS: CUpointer_attribute_enum = 5;
pub const CU_POINTER_ATTRIBUTE_SYNC_MEMOPS: CUpointer_attribute_enum = 6;
pub const CU_POINTER_ATTRIBUTE_BUFFER_ID: CUpointer_attribute_enum = 7;
pub const CU_POINTER_ATTRIBUTE_IS_MANAGED: CUpointer_attribute_enum = 8;
pub const CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL: CUpointer_attribute_enum = 9;
pub const CU_POINTER_ATTRIBUTE_IS_LEGACY_CUDA_IPC_CAPABLE: CUpointer_attribute_enum = 10;
pub const CU_POINTER_ATTRIBUTE_RANGE_START_ADDR: CUpointer_attribute_enum = 11;
pub const CU_POINTER_ATTRIBUTE_RANGE_SIZE: CUpointer_attribute_enum = 12;
pub const CU_POINTER_ATTRIBUTE_MAPPED: CUpointer_attribute_enum = 13;
pub const CU_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES: CUpointer_attribute_enum = 14;
pub const CU_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE: CUpointer_attribute_enum = 15;
pub const CU_POINTER_ATTRIBUTE_ACCESS_FLAGS: CUpointer_attribute_enum = 16;
pub const CU_POINTER_ATTRIBUTE_MEMPOOL_HANDLE: CUpointer_attribute_enum = 17;
pub const CU_POINTER_ATTRIBUTE_MAPPING_SIZE: CUpointer_attribute_enum = 18;
pub const CU_POINTER_ATTRIBUTE_MAPPING_BASE_ADDR: CUpointer_attribute_enum = 19;
pub const CU_POINTER_ATTRIBUTE_MEMORY_BLOCK_ID: CUpointer_attribute_enum = 20;
pub const CU_POINTER_ATTRIBUTE_IS_HW_DECOMPRESS_CAPABLE: CUpointer_attribute_enum = 21;
pub const CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK: CUfunction_attribute_enum = 0;
pub const CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES: CUfunction_attribute_enum = 1;
pub const CU_FUNC_ATTRIBUTE_CONST_SIZE_BYTES: CUfunction_attribute_enum = 2;
pub const CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES: CUfunction_attribute_enum = 3;
pub const CU_FUNC_ATTRIBUTE_NUM_REGS: CUfunction_attribute_enum = 4;
pub const CU_FUNC_ATTRIBUTE_PTX_VERSION: CUfunction_attribute_enum = 5;
pub const CU_FUNC_ATTRIBUTE_BINARY_VERSION: CUfunction_attribute_enum = 6;
pub const CU_FUNC_ATTRIBUTE_CACHE_MODE_CA: CUfunction_attribute_enum = 7;
pub const CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES: CUfunction_attribute_enum = 8;
pub const CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT: CUfunction_attribute_enum = 9;
pub const CU_FUNC_ATTRIBUTE_CLUSTER_SIZE_MUST_BE_SET: CUfunction_attribute_enum = 10;
pub const CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH: CUfunction_attribute_enum = 11;
pub const CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT: CUfunction_attribute_enum = 12;
pub const CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH: CUfunction_attribute_enum = 13;
pub const CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED: CUfunction_attribute_enum = 14;
pub const CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE: CUfunction_attribute_enum = 15;
pub const CU_FUNC_ATTRIBUTE_DEVICE_NODE_UPDATE_SUPPORTED: CUfunction_attribute_enum = 16;
pub const CU_FUNC_ATTRIBUTE_MAX: CUfunction_attribute_enum = 17;
pub const CU_FUNC_CACHE_PREFER_NONE: CUfunc_cache_enum = 0;
pub const CU_FUNC_CACHE_PREFER_SHARED: CUfunc_cache_enum = 1;
pub const CU_FUNC_CACHE_PREFER_L1: CUfunc_cache_enum = 2;
pub const CU_FUNC_CACHE_PREFER_EQUAL: CUfunc_cache_enum = 3;
pub const CU_SHARED_MEM_CONFIG_DEFAULT_BANK_SIZE: CUsharedconfig_enum = 0;
pub const CU_SHARED_MEM_CONFIG_FOUR_BYTE_BANK_SIZE: CUsharedconfig_enum = 1;
pub const CU_SHARED_MEM_CONFIG_EIGHT_BYTE_BANK_SIZE: CUsharedconfig_enum = 2;
pub const CU_SHAREDMEM_CARVEOUT_DEFAULT: CUshared_carveout_enum = -1;
pub const CU_SHAREDMEM_CARVEOUT_MAX_SHARED: CUshared_carveout_enum = 100;
pub const CU_SHAREDMEM_CARVEOUT_MAX_L1: CUshared_carveout_enum = 0;
pub const CU_MEMORYTYPE_HOST: CUmemorytype_enum = 1;
pub const CU_MEMORYTYPE_DEVICE: CUmemorytype_enum = 2;
pub const CU_MEMORYTYPE_ARRAY: CUmemorytype_enum = 3;
pub const CU_MEMORYTYPE_UNIFIED: CUmemorytype_enum = 4;
pub const CU_COMPUTEMODE_DEFAULT: CUcomputemode_enum = 0;
pub const CU_COMPUTEMODE_PROHIBITED: CUcomputemode_enum = 2;
pub const CU_COMPUTEMODE_EXCLUSIVE_PROCESS: CUcomputemode_enum = 3;
pub const CU_MEM_ADVISE_SET_READ_MOSTLY: CUmem_advise_enum = 1;
pub const CU_MEM_ADVISE_UNSET_READ_MOSTLY: CUmem_advise_enum = 2;
pub const CU_MEM_ADVISE_SET_PREFERRED_LOCATION: CUmem_advise_enum = 3;
pub const CU_MEM_ADVISE_UNSET_PREFERRED_LOCATION: CUmem_advise_enum = 4;
pub const CU_MEM_ADVISE_SET_ACCESSED_BY: CUmem_advise_enum = 5;
pub const CU_MEM_ADVISE_UNSET_ACCESSED_BY: CUmem_advise_enum = 6;
pub const CU_MEM_RANGE_ATTRIBUTE_READ_MOSTLY: CUmem_range_attribute_enum = 1;
pub const CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION: CUmem_range_attribute_enum = 2;
pub const CU_MEM_RANGE_ATTRIBUTE_ACCESSED_BY: CUmem_range_attribute_enum = 3;
pub const CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION: CUmem_range_attribute_enum = 4;
pub const CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_TYPE: CUmem_range_attribute_enum = 5;
pub const CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_ID: CUmem_range_attribute_enum = 6;
pub const CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_TYPE: CUmem_range_attribute_enum = 7;
pub const CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_ID: CUmem_range_attribute_enum = 8;
pub const CU_JIT_MAX_REGISTERS: CUjit_option_enum = 0;
pub const CU_JIT_THREADS_PER_BLOCK: CUjit_option_enum = 1;
pub const CU_JIT_WALL_TIME: CUjit_option_enum = 2;
pub const CU_JIT_INFO_LOG_BUFFER: CUjit_option_enum = 3;
pub const CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES: CUjit_option_enum = 4;
pub const CU_JIT_ERROR_LOG_BUFFER: CUjit_option_enum = 5;
pub const CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES: CUjit_option_enum = 6;
pub const CU_JIT_OPTIMIZATION_LEVEL: CUjit_option_enum = 7;
pub const CU_JIT_TARGET_FROM_CUCONTEXT: CUjit_option_enum = 8;
pub const CU_JIT_TARGET: CUjit_option_enum = 9;
pub const CU_JIT_FALLBACK_STRATEGY: CUjit_option_enum = 10;
pub const CU_JIT_GENERATE_DEBUG_INFO: CUjit_option_enum = 11;
pub const CU_JIT_LOG_VERBOSE: CUjit_option_enum = 12;
pub const CU_JIT_GENERATE_LINE_INFO: CUjit_option_enum = 13;
pub const CU_JIT_CACHE_MODE: CUjit_option_enum = 14;
pub const CU_JIT_NEW_SM3X_OPT: CUjit_option_enum = 15;
pub const CU_JIT_FAST_COMPILE: CUjit_option_enum = 16;
pub const CU_JIT_GLOBAL_SYMBOL_NAMES: CUjit_option_enum = 17;
pub const CU_JIT_GLOBAL_SYMBOL_ADDRESSES: CUjit_option_enum = 18;
pub const CU_JIT_GLOBAL_SYMBOL_COUNT: CUjit_option_enum = 19;
pub const CU_JIT_LTO: CUjit_option_enum = 20;
pub const CU_JIT_FTZ: CUjit_option_enum = 21;
pub const CU_JIT_PREC_DIV: CUjit_option_enum = 22;
pub const CU_JIT_PREC_SQRT: CUjit_option_enum = 23;
pub const CU_JIT_FMA: CUjit_option_enum = 24;
pub const CU_JIT_REFERENCED_KERNEL_NAMES: CUjit_option_enum = 25;
pub const CU_JIT_REFERENCED_KERNEL_COUNT: CUjit_option_enum = 26;
pub const CU_JIT_REFERENCED_VARIABLE_NAMES: CUjit_option_enum = 27;
pub const CU_JIT_REFERENCED_VARIABLE_COUNT: CUjit_option_enum = 28;
pub const CU_JIT_OPTIMIZE_UNUSED_DEVICE_VARIABLES: CUjit_option_enum = 29;
pub const CU_JIT_POSITION_INDEPENDENT_CODE: CUjit_option_enum = 30;
pub const CU_JIT_MIN_CTA_PER_SM: CUjit_option_enum = 31;
pub const CU_JIT_MAX_THREADS_PER_BLOCK: CUjit_option_enum = 32;
pub const CU_JIT_OVERRIDE_DIRECTIVE_VALUES: CUjit_option_enum = 33;
pub const CU_JIT_SPLIT_COMPILE: CUjit_option_enum = 34;
pub const CU_JIT_BINARY_LOADER_THREAD_COUNT: CUjit_option_enum = 35;
pub const CU_JIT_NUM_OPTIONS: CUjit_option_enum = 36;
pub const CU_TARGET_COMPUTE_30: CUjit_target_enum = 30;
pub const CU_TARGET_COMPUTE_32: CUjit_target_enum = 32;
pub const CU_TARGET_COMPUTE_35: CUjit_target_enum = 35;
pub const CU_TARGET_COMPUTE_37: CUjit_target_enum = 37;
pub const CU_TARGET_COMPUTE_50: CUjit_target_enum = 50;
pub const CU_TARGET_COMPUTE_52: CUjit_target_enum = 52;
pub const CU_TARGET_COMPUTE_53: CUjit_target_enum = 53;
pub const CU_TARGET_COMPUTE_60: CUjit_target_enum = 60;
pub const CU_TARGET_COMPUTE_61: CUjit_target_enum = 61;
pub const CU_TARGET_COMPUTE_62: CUjit_target_enum = 62;
pub const CU_TARGET_COMPUTE_70: CUjit_target_enum = 70;
pub const CU_TARGET_COMPUTE_72: CUjit_target_enum = 72;
pub const CU_TARGET_COMPUTE_75: CUjit_target_enum = 75;
pub const CU_TARGET_COMPUTE_80: CUjit_target_enum = 80;
pub const CU_TARGET_COMPUTE_86: CUjit_target_enum = 86;
pub const CU_TARGET_COMPUTE_87: CUjit_target_enum = 87;
pub const CU_TARGET_COMPUTE_89: CUjit_target_enum = 89;
pub const CU_TARGET_COMPUTE_90: CUjit_target_enum = 90;
pub const CU_TARGET_COMPUTE_100: CUjit_target_enum = 100;
pub const CU_TARGET_COMPUTE_110: CUjit_target_enum = 110;
pub const CU_TARGET_COMPUTE_103: CUjit_target_enum = 103;
pub const CU_TARGET_COMPUTE_120: CUjit_target_enum = 120;
pub const CU_TARGET_COMPUTE_121: CUjit_target_enum = 121;
pub const CU_TARGET_COMPUTE_90A: CUjit_target_enum = 65626;
pub const CU_TARGET_COMPUTE_100A: CUjit_target_enum = 65636;
pub const CU_TARGET_COMPUTE_110A: CUjit_target_enum = 65646;
pub const CU_TARGET_COMPUTE_103A: CUjit_target_enum = 65639;
pub const CU_TARGET_COMPUTE_120A: CUjit_target_enum = 65656;
pub const CU_TARGET_COMPUTE_121A: CUjit_target_enum = 65657;
pub const CU_TARGET_COMPUTE_100F: CUjit_target_enum = 131172;
pub const CU_TARGET_COMPUTE_110F: CUjit_target_enum = 131182;
pub const CU_TARGET_COMPUTE_103F: CUjit_target_enum = 131175;
pub const CU_TARGET_COMPUTE_120F: CUjit_target_enum = 131192;
pub const CU_TARGET_COMPUTE_121F: CUjit_target_enum = 131193;
pub const CU_PREFER_PTX: CUjit_fallback_enum = 0;
pub const CU_PREFER_BINARY: CUjit_fallback_enum = 1;
pub const CU_JIT_CACHE_OPTION_NONE: CUjit_cacheMode_enum = 0;
pub const CU_JIT_CACHE_OPTION_CG: CUjit_cacheMode_enum = 1;
pub const CU_JIT_CACHE_OPTION_CA: CUjit_cacheMode_enum = 2;
pub const CU_JIT_INPUT_CUBIN: CUjitInputType_enum = 0;
pub const CU_JIT_INPUT_PTX: CUjitInputType_enum = 1;
pub const CU_JIT_INPUT_FATBINARY: CUjitInputType_enum = 2;
pub const CU_JIT_INPUT_OBJECT: CUjitInputType_enum = 3;
pub const CU_JIT_INPUT_LIBRARY: CUjitInputType_enum = 4;
pub const CU_JIT_INPUT_NVVM: CUjitInputType_enum = 5;
pub const CU_JIT_NUM_INPUT_TYPES: CUjitInputType_enum = 6;
pub const CU_GRAPHICS_REGISTER_FLAGS_NONE: CUgraphicsRegisterFlags_enum = 0;
pub const CU_GRAPHICS_REGISTER_FLAGS_READ_ONLY: CUgraphicsRegisterFlags_enum = 1;
pub const CU_GRAPHICS_REGISTER_FLAGS_WRITE_DISCARD: CUgraphicsRegisterFlags_enum = 2;
pub const CU_GRAPHICS_REGISTER_FLAGS_SURFACE_LDST: CUgraphicsRegisterFlags_enum = 4;
pub const CU_GRAPHICS_REGISTER_FLAGS_TEXTURE_GATHER: CUgraphicsRegisterFlags_enum = 8;
pub const CU_GRAPHICS_MAP_RESOURCE_FLAGS_NONE: CUgraphicsMapResourceFlags_enum = 0;
pub const CU_GRAPHICS_MAP_RESOURCE_FLAGS_READ_ONLY: CUgraphicsMapResourceFlags_enum = 1;
pub const CU_GRAPHICS_MAP_RESOURCE_FLAGS_WRITE_DISCARD: CUgraphicsMapResourceFlags_enum = 2;
pub const CU_CUBEMAP_FACE_POSITIVE_X: CUarray_cubemap_face_enum = 0;
pub const CU_CUBEMAP_FACE_NEGATIVE_X: CUarray_cubemap_face_enum = 1;
pub const CU_CUBEMAP_FACE_POSITIVE_Y: CUarray_cubemap_face_enum = 2;
pub const CU_CUBEMAP_FACE_NEGATIVE_Y: CUarray_cubemap_face_enum = 3;
pub const CU_CUBEMAP_FACE_POSITIVE_Z: CUarray_cubemap_face_enum = 4;
pub const CU_CUBEMAP_FACE_NEGATIVE_Z: CUarray_cubemap_face_enum = 5;
pub const CU_LIMIT_STACK_SIZE: CUlimit_enum = 0;
pub const CU_LIMIT_PRINTF_FIFO_SIZE: CUlimit_enum = 1;
pub const CU_LIMIT_MALLOC_HEAP_SIZE: CUlimit_enum = 2;
pub const CU_LIMIT_DEV_RUNTIME_SYNC_DEPTH: CUlimit_enum = 3;
pub const CU_LIMIT_DEV_RUNTIME_PENDING_LAUNCH_COUNT: CUlimit_enum = 4;
pub const CU_LIMIT_MAX_L2_FETCH_GRANULARITY: CUlimit_enum = 5;
pub const CU_LIMIT_PERSISTING_L2_CACHE_SIZE: CUlimit_enum = 6;
pub const CU_LIMIT_SHMEM_SIZE: CUlimit_enum = 7;
pub const CU_LIMIT_CIG_ENABLED: CUlimit_enum = 8;
pub const CU_LIMIT_CIG_SHMEM_FALLBACK_ENABLED: CUlimit_enum = 9;
pub const CU_LIMIT_MAX: CUlimit_enum = 10;
pub const CU_RESOURCE_TYPE_ARRAY: CUresourcetype_enum = 0;
pub const CU_RESOURCE_TYPE_MIPMAPPED_ARRAY: CUresourcetype_enum = 1;
pub const CU_RESOURCE_TYPE_LINEAR: CUresourcetype_enum = 2;
pub const CU_RESOURCE_TYPE_PITCH2D: CUresourcetype_enum = 3;
pub const CU_ACCESS_PROPERTY_NORMAL: CUaccessProperty_enum = 0;
pub const CU_ACCESS_PROPERTY_STREAMING: CUaccessProperty_enum = 1;
pub const CU_ACCESS_PROPERTY_PERSISTING: CUaccessProperty_enum = 2;
pub const CU_GRAPH_COND_TYPE_IF: CUgraphConditionalNodeType_enum = 0;
pub const CU_GRAPH_COND_TYPE_WHILE: CUgraphConditionalNodeType_enum = 1;
pub const CU_GRAPH_COND_TYPE_SWITCH: CUgraphConditionalNodeType_enum = 2;
pub const CU_GRAPH_NODE_TYPE_KERNEL: CUgraphNodeType_enum = 0;
pub const CU_GRAPH_NODE_TYPE_MEMCPY: CUgraphNodeType_enum = 1;
pub const CU_GRAPH_NODE_TYPE_MEMSET: CUgraphNodeType_enum = 2;
pub const CU_GRAPH_NODE_TYPE_HOST: CUgraphNodeType_enum = 3;
pub const CU_GRAPH_NODE_TYPE_GRAPH: CUgraphNodeType_enum = 4;
pub const CU_GRAPH_NODE_TYPE_EMPTY: CUgraphNodeType_enum = 5;
pub const CU_GRAPH_NODE_TYPE_WAIT_EVENT: CUgraphNodeType_enum = 6;
pub const CU_GRAPH_NODE_TYPE_EVENT_RECORD: CUgraphNodeType_enum = 7;
pub const CU_GRAPH_NODE_TYPE_EXT_SEMAS_SIGNAL: CUgraphNodeType_enum = 8;
pub const CU_GRAPH_NODE_TYPE_EXT_SEMAS_WAIT: CUgraphNodeType_enum = 9;
pub const CU_GRAPH_NODE_TYPE_MEM_ALLOC: CUgraphNodeType_enum = 10;
pub const CU_GRAPH_NODE_TYPE_MEM_FREE: CUgraphNodeType_enum = 11;
pub const CU_GRAPH_NODE_TYPE_BATCH_MEM_OP: CUgraphNodeType_enum = 12;
pub const CU_GRAPH_NODE_TYPE_CONDITIONAL: CUgraphNodeType_enum = 13;
pub const CU_GRAPH_NODE_TYPE_RESERVED_16: CUgraphNodeType_enum = 16;
pub const CU_GRAPH_DEPENDENCY_TYPE_DEFAULT: CUgraphDependencyType_enum = 0;
pub const CU_GRAPH_DEPENDENCY_TYPE_PROGRAMMATIC: CUgraphDependencyType_enum = 1;
pub const CUDA_GRAPH_INSTANTIATE_SUCCESS: CUgraphInstantiateResult_enum = 0;
pub const CUDA_GRAPH_INSTANTIATE_ERROR: CUgraphInstantiateResult_enum = 1;
pub const CUDA_GRAPH_INSTANTIATE_INVALID_STRUCTURE: CUgraphInstantiateResult_enum = 2;
pub const CUDA_GRAPH_INSTANTIATE_NODE_OPERATION_NOT_SUPPORTED: CUgraphInstantiateResult_enum = 3;
pub const CUDA_GRAPH_INSTANTIATE_MULTIPLE_CTXS_NOT_SUPPORTED: CUgraphInstantiateResult_enum = 4;
pub const CUDA_GRAPH_INSTANTIATE_CONDITIONAL_HANDLE_UNUSED: CUgraphInstantiateResult_enum = 5;
pub const CU_SYNC_POLICY_AUTO: CUsynchronizationPolicy_enum = 1;
pub const CU_SYNC_POLICY_SPIN: CUsynchronizationPolicy_enum = 2;
pub const CU_SYNC_POLICY_YIELD: CUsynchronizationPolicy_enum = 3;
pub const CU_SYNC_POLICY_BLOCKING_SYNC: CUsynchronizationPolicy_enum = 4;
pub const CU_CLUSTER_SCHEDULING_POLICY_DEFAULT: CUclusterSchedulingPolicy_enum = 0;
pub const CU_CLUSTER_SCHEDULING_POLICY_SPREAD: CUclusterSchedulingPolicy_enum = 1;
pub const CU_CLUSTER_SCHEDULING_POLICY_LOAD_BALANCING: CUclusterSchedulingPolicy_enum = 2;
pub const CU_LAUNCH_MEM_SYNC_DOMAIN_DEFAULT: CUlaunchMemSyncDomain_enum = 0;
pub const CU_LAUNCH_MEM_SYNC_DOMAIN_REMOTE: CUlaunchMemSyncDomain_enum = 1;
pub const CU_LAUNCH_PORTABLE_CLUSTER_MODE_DEFAULT: CUlaunchAttributePortableClusterMode_enum = 0;
pub const CU_LAUNCH_PORTABLE_CLUSTER_MODE_REQUIRE_PORTABLE:
    CUlaunchAttributePortableClusterMode_enum = 1;
pub const CU_LAUNCH_PORTABLE_CLUSTER_MODE_ALLOW_NON_PORTABLE:
    CUlaunchAttributePortableClusterMode_enum = 2;
pub const CU_SHARED_MEMORY_MODE_DEFAULT: CUsharedMemoryMode_enum = 0;
pub const CU_SHARED_MEMORY_MODE_REQUIRE_PORTABLE: CUsharedMemoryMode_enum = 1;
pub const CU_SHARED_MEMORY_MODE_ALLOW_NON_PORTABLE: CUsharedMemoryMode_enum = 2;
pub const CU_LAUNCH_ATTRIBUTE_IGNORE: CUlaunchAttributeID_enum = 0;
pub const CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW: CUlaunchAttributeID_enum = 1;
pub const CU_LAUNCH_ATTRIBUTE_COOPERATIVE: CUlaunchAttributeID_enum = 2;
pub const CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY: CUlaunchAttributeID_enum = 3;
pub const CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION: CUlaunchAttributeID_enum = 4;
pub const CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE: CUlaunchAttributeID_enum = 5;
pub const CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION: CUlaunchAttributeID_enum = 6;
pub const CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT: CUlaunchAttributeID_enum = 7;
pub const CU_LAUNCH_ATTRIBUTE_PRIORITY: CUlaunchAttributeID_enum = 8;
pub const CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP: CUlaunchAttributeID_enum = 9;
pub const CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN: CUlaunchAttributeID_enum = 10;
pub const CU_LAUNCH_ATTRIBUTE_PREFERRED_CLUSTER_DIMENSION: CUlaunchAttributeID_enum = 11;
pub const CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT: CUlaunchAttributeID_enum = 12;
pub const CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE: CUlaunchAttributeID_enum = 13;
pub const CU_LAUNCH_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT: CUlaunchAttributeID_enum = 14;
pub const CU_LAUNCH_ATTRIBUTE_NVLINK_UTIL_CENTRIC_SCHEDULING: CUlaunchAttributeID_enum = 16;
pub const CU_LAUNCH_ATTRIBUTE_PORTABLE_CLUSTER_SIZE_MODE: CUlaunchAttributeID_enum = 17;
pub const CU_LAUNCH_ATTRIBUTE_SHARED_MEMORY_MODE: CUlaunchAttributeID_enum = 18;
pub const CU_STREAM_CAPTURE_STATUS_NONE: CUstreamCaptureStatus_enum = 0;
pub const CU_STREAM_CAPTURE_STATUS_ACTIVE: CUstreamCaptureStatus_enum = 1;
pub const CU_STREAM_CAPTURE_STATUS_INVALIDATED: CUstreamCaptureStatus_enum = 2;
pub const CU_STREAM_CAPTURE_MODE_GLOBAL: CUstreamCaptureMode_enum = 0;
pub const CU_STREAM_CAPTURE_MODE_THREAD_LOCAL: CUstreamCaptureMode_enum = 1;
pub const CU_STREAM_CAPTURE_MODE_RELAXED: CUstreamCaptureMode_enum = 2;
pub const CU_GET_PROC_ADDRESS_DEFAULT: CUdriverProcAddress_flags_enum = 0;
pub const CU_GET_PROC_ADDRESS_LEGACY_STREAM: CUdriverProcAddress_flags_enum = 1;
pub const CU_GET_PROC_ADDRESS_PER_THREAD_DEFAULT_STREAM: CUdriverProcAddress_flags_enum = 2;
pub const CU_GET_PROC_ADDRESS_SUCCESS: CUdriverProcAddressQueryResult_enum = 0;
pub const CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND: CUdriverProcAddressQueryResult_enum = 1;
pub const CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT: CUdriverProcAddressQueryResult_enum = 2;
pub const CU_EXEC_AFFINITY_TYPE_SM_COUNT: CUexecAffinityType_enum = 0;
pub const CU_EXEC_AFFINITY_TYPE_MAX: CUexecAffinityType_enum = 1;
pub const CIG_DATA_TYPE_D3D12_COMMAND_QUEUE: CUcigDataType_enum = 1;
pub const CIG_DATA_TYPE_NV_BLOB: CUcigDataType_enum = 2;
pub const STREAM_CIG_DATA_TYPE_D3D12_COMMAND_LIST: CUstreamCigDataType_enum = 1;
pub const CU_LIBRARY_HOST_UNIVERSAL_FUNCTION_AND_DATA_TABLE: CUlibraryOption_enum = 0;
pub const CU_LIBRARY_BINARY_IS_PRESERVED: CUlibraryOption_enum = 1;
pub const CU_LIBRARY_NUM_OPTIONS: CUlibraryOption_enum = 2;
pub const CUDA_SUCCESS: cudaError_enum = 0;
pub const CUDA_ERROR_INVALID_VALUE: cudaError_enum = 1;
pub const CUDA_ERROR_OUT_OF_MEMORY: cudaError_enum = 2;
pub const CUDA_ERROR_NOT_INITIALIZED: cudaError_enum = 3;
pub const CUDA_ERROR_DEINITIALIZED: cudaError_enum = 4;
pub const CUDA_ERROR_PROFILER_DISABLED: cudaError_enum = 5;
pub const CUDA_ERROR_PROFILER_NOT_INITIALIZED: cudaError_enum = 6;
pub const CUDA_ERROR_PROFILER_ALREADY_STARTED: cudaError_enum = 7;
pub const CUDA_ERROR_PROFILER_ALREADY_STOPPED: cudaError_enum = 8;
pub const CUDA_ERROR_STUB_LIBRARY: cudaError_enum = 34;
pub const CUDA_ERROR_CALL_REQUIRES_NEWER_DRIVER: cudaError_enum = 36;
pub const CUDA_ERROR_DEVICE_UNAVAILABLE: cudaError_enum = 46;
pub const CUDA_ERROR_NO_DEVICE: cudaError_enum = 100;
pub const CUDA_ERROR_INVALID_DEVICE: cudaError_enum = 101;
pub const CUDA_ERROR_DEVICE_NOT_LICENSED: cudaError_enum = 102;
pub const CUDA_ERROR_INVALID_IMAGE: cudaError_enum = 200;
pub const CUDA_ERROR_INVALID_CONTEXT: cudaError_enum = 201;
pub const CUDA_ERROR_CONTEXT_ALREADY_CURRENT: cudaError_enum = 202;
pub const CUDA_ERROR_MAP_FAILED: cudaError_enum = 205;
pub const CUDA_ERROR_UNMAP_FAILED: cudaError_enum = 206;
pub const CUDA_ERROR_ARRAY_IS_MAPPED: cudaError_enum = 207;
pub const CUDA_ERROR_ALREADY_MAPPED: cudaError_enum = 208;
pub const CUDA_ERROR_NO_BINARY_FOR_GPU: cudaError_enum = 209;
pub const CUDA_ERROR_ALREADY_ACQUIRED: cudaError_enum = 210;
pub const CUDA_ERROR_NOT_MAPPED: cudaError_enum = 211;
pub const CUDA_ERROR_NOT_MAPPED_AS_ARRAY: cudaError_enum = 212;
pub const CUDA_ERROR_NOT_MAPPED_AS_POINTER: cudaError_enum = 213;
pub const CUDA_ERROR_ECC_UNCORRECTABLE: cudaError_enum = 214;
pub const CUDA_ERROR_UNSUPPORTED_LIMIT: cudaError_enum = 215;
pub const CUDA_ERROR_CONTEXT_ALREADY_IN_USE: cudaError_enum = 216;
pub const CUDA_ERROR_PEER_ACCESS_UNSUPPORTED: cudaError_enum = 217;
pub const CUDA_ERROR_INVALID_PTX: cudaError_enum = 218;
pub const CUDA_ERROR_INVALID_GRAPHICS_CONTEXT: cudaError_enum = 219;
pub const CUDA_ERROR_NVLINK_UNCORRECTABLE: cudaError_enum = 220;
pub const CUDA_ERROR_JIT_COMPILER_NOT_FOUND: cudaError_enum = 221;
pub const CUDA_ERROR_UNSUPPORTED_PTX_VERSION: cudaError_enum = 222;
pub const CUDA_ERROR_JIT_COMPILATION_DISABLED: cudaError_enum = 223;
pub const CUDA_ERROR_UNSUPPORTED_EXEC_AFFINITY: cudaError_enum = 224;
pub const CUDA_ERROR_UNSUPPORTED_DEVSIDE_SYNC: cudaError_enum = 225;
pub const CUDA_ERROR_CONTAINED: cudaError_enum = 226;
pub const CUDA_ERROR_INVALID_SOURCE: cudaError_enum = 300;
pub const CUDA_ERROR_FILE_NOT_FOUND: cudaError_enum = 301;
pub const CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND: cudaError_enum = 302;
pub const CUDA_ERROR_SHARED_OBJECT_INIT_FAILED: cudaError_enum = 303;
pub const CUDA_ERROR_OPERATING_SYSTEM: cudaError_enum = 304;
pub const CUDA_ERROR_INVALID_HANDLE: cudaError_enum = 400;
pub const CUDA_ERROR_ILLEGAL_STATE: cudaError_enum = 401;
pub const CUDA_ERROR_LOSSY_QUERY: cudaError_enum = 402;
pub const CUDA_ERROR_NOT_FOUND: cudaError_enum = 500;
pub const CUDA_ERROR_NOT_READY: cudaError_enum = 600;
pub const CUDA_ERROR_ILLEGAL_ADDRESS: cudaError_enum = 700;
pub const CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES: cudaError_enum = 701;
pub const CUDA_ERROR_LAUNCH_TIMEOUT: cudaError_enum = 702;
pub const CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING: cudaError_enum = 703;
pub const CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED: cudaError_enum = 704;
pub const CUDA_ERROR_PEER_ACCESS_NOT_ENABLED: cudaError_enum = 705;
pub const CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE: cudaError_enum = 708;
pub const CUDA_ERROR_CONTEXT_IS_DESTROYED: cudaError_enum = 709;
pub const CUDA_ERROR_ASSERT: cudaError_enum = 710;
pub const CUDA_ERROR_TOO_MANY_PEERS: cudaError_enum = 711;
pub const CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED: cudaError_enum = 712;
pub const CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED: cudaError_enum = 713;
pub const CUDA_ERROR_HARDWARE_STACK_ERROR: cudaError_enum = 714;
pub const CUDA_ERROR_ILLEGAL_INSTRUCTION: cudaError_enum = 715;
pub const CUDA_ERROR_MISALIGNED_ADDRESS: cudaError_enum = 716;
pub const CUDA_ERROR_INVALID_ADDRESS_SPACE: cudaError_enum = 717;
pub const CUDA_ERROR_INVALID_PC: cudaError_enum = 718;
pub const CUDA_ERROR_LAUNCH_FAILED: cudaError_enum = 719;
pub const CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE: cudaError_enum = 720;
pub const CUDA_ERROR_TENSOR_MEMORY_LEAK: cudaError_enum = 721;
pub const CUDA_ERROR_NOT_PERMITTED: cudaError_enum = 800;
pub const CUDA_ERROR_NOT_SUPPORTED: cudaError_enum = 801;
pub const CUDA_ERROR_SYSTEM_NOT_READY: cudaError_enum = 802;
pub const CUDA_ERROR_SYSTEM_DRIVER_MISMATCH: cudaError_enum = 803;
pub const CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE: cudaError_enum = 804;
pub const CUDA_ERROR_MPS_CONNECTION_FAILED: cudaError_enum = 805;
pub const CUDA_ERROR_MPS_RPC_FAILURE: cudaError_enum = 806;
pub const CUDA_ERROR_MPS_SERVER_NOT_READY: cudaError_enum = 807;
pub const CUDA_ERROR_MPS_MAX_CLIENTS_REACHED: cudaError_enum = 808;
pub const CUDA_ERROR_MPS_MAX_CONNECTIONS_REACHED: cudaError_enum = 809;
pub const CUDA_ERROR_MPS_CLIENT_TERMINATED: cudaError_enum = 810;
pub const CUDA_ERROR_CDP_NOT_SUPPORTED: cudaError_enum = 811;
pub const CUDA_ERROR_CDP_VERSION_MISMATCH: cudaError_enum = 812;
pub const CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED: cudaError_enum = 900;
pub const CUDA_ERROR_STREAM_CAPTURE_INVALIDATED: cudaError_enum = 901;
pub const CUDA_ERROR_STREAM_CAPTURE_MERGE: cudaError_enum = 902;
pub const CUDA_ERROR_STREAM_CAPTURE_UNMATCHED: cudaError_enum = 903;
pub const CUDA_ERROR_STREAM_CAPTURE_UNJOINED: cudaError_enum = 904;
pub const CUDA_ERROR_STREAM_CAPTURE_ISOLATION: cudaError_enum = 905;
pub const CUDA_ERROR_STREAM_CAPTURE_IMPLICIT: cudaError_enum = 906;
pub const CUDA_ERROR_CAPTURED_EVENT: cudaError_enum = 907;
pub const CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD: cudaError_enum = 908;
pub const CUDA_ERROR_TIMEOUT: cudaError_enum = 909;
pub const CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE: cudaError_enum = 910;
pub const CUDA_ERROR_EXTERNAL_DEVICE: cudaError_enum = 911;
pub const CUDA_ERROR_INVALID_CLUSTER_SIZE: cudaError_enum = 912;
pub const CUDA_ERROR_FUNCTION_NOT_LOADED: cudaError_enum = 913;
pub const CUDA_ERROR_INVALID_RESOURCE_TYPE: cudaError_enum = 914;
pub const CUDA_ERROR_INVALID_RESOURCE_CONFIGURATION: cudaError_enum = 915;
pub const CUDA_ERROR_KEY_ROTATION: cudaError_enum = 916;
pub const CUDA_ERROR_STREAM_DETACHED: cudaError_enum = 917;
pub const CUDA_ERROR_GRAPH_RECAPTURE_FAILURE: cudaError_enum = 918;
pub const CUDA_ERROR_UNKNOWN: cudaError_enum = 999;
pub const CU_DEVICE_P2P_ATTRIBUTE_PERFORMANCE_RANK: CUdevice_P2PAttribute_enum = 1;
pub const CU_DEVICE_P2P_ATTRIBUTE_ACCESS_SUPPORTED: CUdevice_P2PAttribute_enum = 2;
pub const CU_DEVICE_P2P_ATTRIBUTE_NATIVE_ATOMIC_SUPPORTED: CUdevice_P2PAttribute_enum = 3;
pub const CU_DEVICE_P2P_ATTRIBUTE_ACCESS_ACCESS_SUPPORTED: CUdevice_P2PAttribute_enum = 4;
pub const CU_DEVICE_P2P_ATTRIBUTE_CUDA_ARRAY_ACCESS_SUPPORTED: CUdevice_P2PAttribute_enum = 4;
pub const CU_DEVICE_P2P_ATTRIBUTE_ONLY_PARTIAL_NATIVE_ATOMIC_SUPPORTED: CUdevice_P2PAttribute_enum =
    5;
pub const CU_RES_VIEW_FORMAT_NONE: CUresourceViewFormat_enum = 0;
pub const CU_RES_VIEW_FORMAT_UINT_1X8: CUresourceViewFormat_enum = 1;
pub const CU_RES_VIEW_FORMAT_UINT_2X8: CUresourceViewFormat_enum = 2;
pub const CU_RES_VIEW_FORMAT_UINT_4X8: CUresourceViewFormat_enum = 3;
pub const CU_RES_VIEW_FORMAT_SINT_1X8: CUresourceViewFormat_enum = 4;
pub const CU_RES_VIEW_FORMAT_SINT_2X8: CUresourceViewFormat_enum = 5;
pub const CU_RES_VIEW_FORMAT_SINT_4X8: CUresourceViewFormat_enum = 6;
pub const CU_RES_VIEW_FORMAT_UINT_1X16: CUresourceViewFormat_enum = 7;
pub const CU_RES_VIEW_FORMAT_UINT_2X16: CUresourceViewFormat_enum = 8;
pub const CU_RES_VIEW_FORMAT_UINT_4X16: CUresourceViewFormat_enum = 9;
pub const CU_RES_VIEW_FORMAT_SINT_1X16: CUresourceViewFormat_enum = 10;
pub const CU_RES_VIEW_FORMAT_SINT_2X16: CUresourceViewFormat_enum = 11;
pub const CU_RES_VIEW_FORMAT_SINT_4X16: CUresourceViewFormat_enum = 12;
pub const CU_RES_VIEW_FORMAT_UINT_1X32: CUresourceViewFormat_enum = 13;
pub const CU_RES_VIEW_FORMAT_UINT_2X32: CUresourceViewFormat_enum = 14;
pub const CU_RES_VIEW_FORMAT_UINT_4X32: CUresourceViewFormat_enum = 15;
pub const CU_RES_VIEW_FORMAT_SINT_1X32: CUresourceViewFormat_enum = 16;
pub const CU_RES_VIEW_FORMAT_SINT_2X32: CUresourceViewFormat_enum = 17;
pub const CU_RES_VIEW_FORMAT_SINT_4X32: CUresourceViewFormat_enum = 18;
pub const CU_RES_VIEW_FORMAT_FLOAT_1X16: CUresourceViewFormat_enum = 19;
pub const CU_RES_VIEW_FORMAT_FLOAT_2X16: CUresourceViewFormat_enum = 20;
pub const CU_RES_VIEW_FORMAT_FLOAT_4X16: CUresourceViewFormat_enum = 21;
pub const CU_RES_VIEW_FORMAT_FLOAT_1X32: CUresourceViewFormat_enum = 22;
pub const CU_RES_VIEW_FORMAT_FLOAT_2X32: CUresourceViewFormat_enum = 23;
pub const CU_RES_VIEW_FORMAT_FLOAT_4X32: CUresourceViewFormat_enum = 24;
pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC1: CUresourceViewFormat_enum = 25;
pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC2: CUresourceViewFormat_enum = 26;
pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC3: CUresourceViewFormat_enum = 27;
pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC4: CUresourceViewFormat_enum = 28;
pub const CU_RES_VIEW_FORMAT_SIGNED_BC4: CUresourceViewFormat_enum = 29;
pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC5: CUresourceViewFormat_enum = 30;
pub const CU_RES_VIEW_FORMAT_SIGNED_BC5: CUresourceViewFormat_enum = 31;
pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC6H: CUresourceViewFormat_enum = 32;
pub const CU_RES_VIEW_FORMAT_SIGNED_BC6H: CUresourceViewFormat_enum = 33;
pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC7: CUresourceViewFormat_enum = 34;
pub const CU_TENSOR_MAP_DATA_TYPE_UINT8: CUtensorMapDataType_enum = 0;
pub const CU_TENSOR_MAP_DATA_TYPE_UINT16: CUtensorMapDataType_enum = 1;
pub const CU_TENSOR_MAP_DATA_TYPE_UINT32: CUtensorMapDataType_enum = 2;
pub const CU_TENSOR_MAP_DATA_TYPE_INT32: CUtensorMapDataType_enum = 3;
pub const CU_TENSOR_MAP_DATA_TYPE_UINT64: CUtensorMapDataType_enum = 4;
pub const CU_TENSOR_MAP_DATA_TYPE_INT64: CUtensorMapDataType_enum = 5;
pub const CU_TENSOR_MAP_DATA_TYPE_FLOAT16: CUtensorMapDataType_enum = 6;
pub const CU_TENSOR_MAP_DATA_TYPE_FLOAT32: CUtensorMapDataType_enum = 7;
pub const CU_TENSOR_MAP_DATA_TYPE_FLOAT64: CUtensorMapDataType_enum = 8;
pub const CU_TENSOR_MAP_DATA_TYPE_BFLOAT16: CUtensorMapDataType_enum = 9;
pub const CU_TENSOR_MAP_DATA_TYPE_FLOAT32_FTZ: CUtensorMapDataType_enum = 10;
pub const CU_TENSOR_MAP_DATA_TYPE_TFLOAT32: CUtensorMapDataType_enum = 11;
pub const CU_TENSOR_MAP_DATA_TYPE_TFLOAT32_FTZ: CUtensorMapDataType_enum = 12;
pub const CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B: CUtensorMapDataType_enum = 13;
pub const CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B: CUtensorMapDataType_enum = 14;
pub const CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B: CUtensorMapDataType_enum = 15;
pub const CU_TENSOR_MAP_INTERLEAVE_NONE: CUtensorMapInterleave_enum = 0;
pub const CU_TENSOR_MAP_INTERLEAVE_16B: CUtensorMapInterleave_enum = 1;
pub const CU_TENSOR_MAP_INTERLEAVE_32B: CUtensorMapInterleave_enum = 2;
pub const CU_TENSOR_MAP_SWIZZLE_NONE: CUtensorMapSwizzle_enum = 0;
pub const CU_TENSOR_MAP_SWIZZLE_32B: CUtensorMapSwizzle_enum = 1;
pub const CU_TENSOR_MAP_SWIZZLE_64B: CUtensorMapSwizzle_enum = 2;
pub const CU_TENSOR_MAP_SWIZZLE_128B: CUtensorMapSwizzle_enum = 3;
pub const CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B: CUtensorMapSwizzle_enum = 4;
pub const CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B_FLIP_8B: CUtensorMapSwizzle_enum = 5;
pub const CU_TENSOR_MAP_SWIZZLE_128B_ATOM_64B: CUtensorMapSwizzle_enum = 6;
pub const CU_TENSOR_MAP_L2_PROMOTION_NONE: CUtensorMapL2promotion_enum = 0;
pub const CU_TENSOR_MAP_L2_PROMOTION_L2_64B: CUtensorMapL2promotion_enum = 1;
pub const CU_TENSOR_MAP_L2_PROMOTION_L2_128B: CUtensorMapL2promotion_enum = 2;
pub const CU_TENSOR_MAP_L2_PROMOTION_L2_256B: CUtensorMapL2promotion_enum = 3;
pub const CU_TENSOR_MAP_FLOAT_OOB_FILL_NONE: CUtensorMapFloatOOBfill_enum = 0;
pub const CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA: CUtensorMapFloatOOBfill_enum = 1;
pub const CU_TENSOR_MAP_IM2COL_WIDE_MODE_W: CUtensorMapIm2ColWideMode_enum = 0;
pub const CU_TENSOR_MAP_IM2COL_WIDE_MODE_W128: CUtensorMapIm2ColWideMode_enum = 1;
pub const CU_POINTER_ATTRIBUTE_ACCESS_FLAG_NONE: CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum = 0;
pub const CU_POINTER_ATTRIBUTE_ACCESS_FLAG_READ: CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum = 1;
pub const CU_POINTER_ATTRIBUTE_ACCESS_FLAG_READWRITE: CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum = 3;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD: CUexternalMemoryHandleType_enum = 1;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32: CUexternalMemoryHandleType_enum = 2;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT: CUexternalMemoryHandleType_enum = 3;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP: CUexternalMemoryHandleType_enum = 4;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE: CUexternalMemoryHandleType_enum = 5;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE: CUexternalMemoryHandleType_enum = 6;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT: CUexternalMemoryHandleType_enum = 7;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF: CUexternalMemoryHandleType_enum = 8;
pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_DMABUF_FD: CUexternalMemoryHandleType_enum = 9;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD: CUexternalSemaphoreHandleType_enum = 1;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32: CUexternalSemaphoreHandleType_enum = 2;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT: CUexternalSemaphoreHandleType_enum =
    3;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE: CUexternalSemaphoreHandleType_enum = 4;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE: CUexternalSemaphoreHandleType_enum = 5;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC: CUexternalSemaphoreHandleType_enum = 6;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX: CUexternalSemaphoreHandleType_enum =
    7;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT:
    CUexternalSemaphoreHandleType_enum = 8;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD:
    CUexternalSemaphoreHandleType_enum = 9;
pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32:
    CUexternalSemaphoreHandleType_enum = 10;
pub const CU_MEM_HANDLE_TYPE_NONE: CUmemAllocationHandleType_enum = 0;
pub const CU_MEM_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR: CUmemAllocationHandleType_enum = 1;
pub const CU_MEM_HANDLE_TYPE_WIN32: CUmemAllocationHandleType_enum = 2;
pub const CU_MEM_HANDLE_TYPE_WIN32_KMT: CUmemAllocationHandleType_enum = 4;
pub const CU_MEM_HANDLE_TYPE_FABRIC: CUmemAllocationHandleType_enum = 8;
pub const CU_MEM_HANDLE_TYPE_MAX: CUmemAllocationHandleType_enum = 2147483647;
pub const CU_MEM_ACCESS_FLAGS_PROT_NONE: CUmemAccess_flags_enum = 0;
pub const CU_MEM_ACCESS_FLAGS_PROT_READ: CUmemAccess_flags_enum = 1;
pub const CU_MEM_ACCESS_FLAGS_PROT_READWRITE: CUmemAccess_flags_enum = 3;
pub const CU_MEM_ACCESS_FLAGS_PROT_MAX: CUmemAccess_flags_enum = 2147483647;
pub const CU_MEM_LOCATION_TYPE_INVALID: CUmemLocationType_enum = 0;
pub const CU_MEM_LOCATION_TYPE_NONE: CUmemLocationType_enum = 0;
pub const CU_MEM_LOCATION_TYPE_DEVICE: CUmemLocationType_enum = 1;
pub const CU_MEM_LOCATION_TYPE_HOST: CUmemLocationType_enum = 2;
pub const CU_MEM_LOCATION_TYPE_HOST_NUMA: CUmemLocationType_enum = 3;
pub const CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT: CUmemLocationType_enum = 4;
pub const CU_MEM_LOCATION_TYPE_INVISIBLE: CUmemLocationType_enum = 5;
pub const CU_MEM_LOCATION_TYPE_MAX: CUmemLocationType_enum = 2147483647;
pub const CU_MEM_ALLOCATION_TYPE_INVALID: CUmemAllocationType_enum = 0;
pub const CU_MEM_ALLOCATION_TYPE_PINNED: CUmemAllocationType_enum = 1;
pub const CU_MEM_ALLOCATION_TYPE_MANAGED: CUmemAllocationType_enum = 2;
pub const CU_MEM_ALLOCATION_TYPE_MAX: CUmemAllocationType_enum = 2147483647;
pub const CU_MEM_ALLOC_GRANULARITY_MINIMUM: CUmemAllocationGranularity_flags_enum = 0;
pub const CU_MEM_ALLOC_GRANULARITY_RECOMMENDED: CUmemAllocationGranularity_flags_enum = 1;
pub const CU_MEM_RANGE_HANDLE_TYPE_DMA_BUF_FD: CUmemRangeHandleType_enum = 1;
pub const CU_MEM_RANGE_HANDLE_TYPE_MAX: CUmemRangeHandleType_enum = 2147483647;
pub const CU_MEM_RANGE_FLAG_DMA_BUF_MAPPING_TYPE_PCIE: CUmemRangeFlags_enum = 1;
pub const CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_SPARSE_LEVEL: CUarraySparseSubresourceType_enum = 0;
pub const CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_MIPTAIL: CUarraySparseSubresourceType_enum = 1;
pub const CU_MEM_OPERATION_TYPE_MAP: CUmemOperationType_enum = 1;
pub const CU_MEM_OPERATION_TYPE_UNMAP: CUmemOperationType_enum = 2;
pub const CU_MEM_HANDLE_TYPE_GENERIC: CUmemHandleType_enum = 0;
pub const CU_MEM_ALLOCATION_COMP_NONE: CUmemAllocationCompType_enum = 0;
pub const CU_MEM_ALLOCATION_COMP_GENERIC: CUmemAllocationCompType_enum = 1;
pub const CU_MULTICAST_GRANULARITY_MINIMUM: CUmulticastGranularity_flags_enum = 0;
pub const CU_MULTICAST_GRANULARITY_RECOMMENDED: CUmulticastGranularity_flags_enum = 1;
pub const CU_GRAPH_EXEC_UPDATE_SUCCESS: CUgraphExecUpdateResult_enum = 0;
pub const CU_GRAPH_EXEC_UPDATE_ERROR: CUgraphExecUpdateResult_enum = 1;
pub const CU_GRAPH_EXEC_UPDATE_ERROR_TOPOLOGY_CHANGED: CUgraphExecUpdateResult_enum = 2;
pub const CU_GRAPH_EXEC_UPDATE_ERROR_NODE_TYPE_CHANGED: CUgraphExecUpdateResult_enum = 3;
pub const CU_GRAPH_EXEC_UPDATE_ERROR_FUNCTION_CHANGED: CUgraphExecUpdateResult_enum = 4;
pub const CU_GRAPH_EXEC_UPDATE_ERROR_PARAMETERS_CHANGED: CUgraphExecUpdateResult_enum = 5;
pub const CU_GRAPH_EXEC_UPDATE_ERROR_NOT_SUPPORTED: CUgraphExecUpdateResult_enum = 6;
pub const CU_GRAPH_EXEC_UPDATE_ERROR_UNSUPPORTED_FUNCTION_CHANGE: CUgraphExecUpdateResult_enum = 7;
pub const CU_GRAPH_EXEC_UPDATE_ERROR_ATTRIBUTES_CHANGED: CUgraphExecUpdateResult_enum = 8;
pub const CU_MEMPOOL_ATTR_REUSE_FOLLOW_EVENT_DEPENDENCIES: CUmemPool_attribute_enum = 1;
pub const CU_MEMPOOL_ATTR_REUSE_ALLOW_OPPORTUNISTIC: CUmemPool_attribute_enum = 2;
pub const CU_MEMPOOL_ATTR_REUSE_ALLOW_INTERNAL_DEPENDENCIES: CUmemPool_attribute_enum = 3;
pub const CU_MEMPOOL_ATTR_RELEASE_THRESHOLD: CUmemPool_attribute_enum = 4;
pub const CU_MEMPOOL_ATTR_RESERVED_MEM_CURRENT: CUmemPool_attribute_enum = 5;
pub const CU_MEMPOOL_ATTR_RESERVED_MEM_HIGH: CUmemPool_attribute_enum = 6;
pub const CU_MEMPOOL_ATTR_USED_MEM_CURRENT: CUmemPool_attribute_enum = 7;
pub const CU_MEMPOOL_ATTR_USED_MEM_HIGH: CUmemPool_attribute_enum = 8;
pub const CU_MEMPOOL_ATTR_ALLOCATION_TYPE: CUmemPool_attribute_enum = 9;
pub const CU_MEMPOOL_ATTR_EXPORT_HANDLE_TYPES: CUmemPool_attribute_enum = 10;
pub const CU_MEMPOOL_ATTR_LOCATION_ID: CUmemPool_attribute_enum = 11;
pub const CU_MEMPOOL_ATTR_LOCATION_TYPE: CUmemPool_attribute_enum = 12;
pub const CU_MEMPOOL_ATTR_MAX_POOL_SIZE: CUmemPool_attribute_enum = 13;
pub const CU_MEMPOOL_ATTR_HW_DECOMPRESS_ENABLED: CUmemPool_attribute_enum = 14;
pub const CU_MEMCPY_FLAG_DEFAULT: CUmemcpyFlags_enum = 0;
pub const CU_MEMCPY_FLAG_PREFER_OVERLAP_WITH_COMPUTE: CUmemcpyFlags_enum = 1;
pub const CU_MEMCPY_SRC_ACCESS_ORDER_INVALID: CUmemcpySrcAccessOrder_enum = 0;
pub const CU_MEMCPY_SRC_ACCESS_ORDER_STREAM: CUmemcpySrcAccessOrder_enum = 1;
pub const CU_MEMCPY_SRC_ACCESS_ORDER_DURING_API_CALL: CUmemcpySrcAccessOrder_enum = 2;
pub const CU_MEMCPY_SRC_ACCESS_ORDER_ANY: CUmemcpySrcAccessOrder_enum = 3;
pub const CU_MEMCPY_SRC_ACCESS_ORDER_MAX: CUmemcpySrcAccessOrder_enum = 2147483647;
pub const CU_MEMCPY_OPERAND_TYPE_POINTER: CUmemcpy3DOperandType_enum = 1;
pub const CU_MEMCPY_OPERAND_TYPE_ARRAY: CUmemcpy3DOperandType_enum = 2;
pub const CU_MEMCPY_OPERAND_TYPE_MAX: CUmemcpy3DOperandType_enum = 2147483647;
pub const CU_GRAPH_MEM_ATTR_USED_MEM_CURRENT: CUgraphMem_attribute_enum = 0;
pub const CU_GRAPH_MEM_ATTR_USED_MEM_HIGH: CUgraphMem_attribute_enum = 1;
pub const CU_GRAPH_MEM_ATTR_RESERVED_MEM_CURRENT: CUgraphMem_attribute_enum = 2;
pub const CU_GRAPH_MEM_ATTR_RESERVED_MEM_HIGH: CUgraphMem_attribute_enum = 3;
pub const CU_GRAPH_CHILD_GRAPH_OWNERSHIP_CLONE: CUgraphChildGraphNodeOwnership_enum = 0;
pub const CU_GRAPH_CHILD_GRAPH_OWNERSHIP_MOVE: CUgraphChildGraphNodeOwnership_enum = 1;
pub const CU_GRAPH_CHILD_GRAPH_OWNERSHIP_INVALID: CUgraphChildGraphNodeOwnership_enum = -1;
pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_OPTION_HOST: CUflushGPUDirectRDMAWritesOptions_enum = 1;
pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_OPTION_MEMOPS: CUflushGPUDirectRDMAWritesOptions_enum = 2;
pub const CU_GPU_DIRECT_RDMA_WRITES_ORDERING_NONE: CUGPUDirectRDMAWritesOrdering_enum = 0;
pub const CU_GPU_DIRECT_RDMA_WRITES_ORDERING_OWNER: CUGPUDirectRDMAWritesOrdering_enum = 100;
pub const CU_GPU_DIRECT_RDMA_WRITES_ORDERING_ALL_DEVICES: CUGPUDirectRDMAWritesOrdering_enum = 200;
pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TO_OWNER: CUflushGPUDirectRDMAWritesScope_enum = 100;
pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TO_ALL_DEVICES: CUflushGPUDirectRDMAWritesScope_enum =
    200;
pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TARGET_CURRENT_CTX:
    CUflushGPUDirectRDMAWritesTarget_enum = 0;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_VERBOSE: CUgraphDebugDot_flags_enum = 1;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_RUNTIME_TYPES: CUgraphDebugDot_flags_enum = 2;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_KERNEL_NODE_PARAMS: CUgraphDebugDot_flags_enum = 4;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_MEMCPY_NODE_PARAMS: CUgraphDebugDot_flags_enum = 8;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_MEMSET_NODE_PARAMS: CUgraphDebugDot_flags_enum = 16;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_HOST_NODE_PARAMS: CUgraphDebugDot_flags_enum = 32;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_EVENT_NODE_PARAMS: CUgraphDebugDot_flags_enum = 64;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_EXT_SEMAS_SIGNAL_NODE_PARAMS: CUgraphDebugDot_flags_enum = 128;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_EXT_SEMAS_WAIT_NODE_PARAMS: CUgraphDebugDot_flags_enum = 256;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_KERNEL_NODE_ATTRIBUTES: CUgraphDebugDot_flags_enum = 512;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_HANDLES: CUgraphDebugDot_flags_enum = 1024;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_MEM_ALLOC_NODE_PARAMS: CUgraphDebugDot_flags_enum = 2048;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_MEM_FREE_NODE_PARAMS: CUgraphDebugDot_flags_enum = 4096;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_BATCH_MEM_OP_NODE_PARAMS: CUgraphDebugDot_flags_enum = 8192;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_EXTRA_TOPO_INFO: CUgraphDebugDot_flags_enum = 16384;
pub const CU_GRAPH_DEBUG_DOT_FLAGS_CONDITIONAL_NODE_PARAMS: CUgraphDebugDot_flags_enum = 32768;
pub const CU_USER_OBJECT_NO_DESTRUCTOR_SYNC: CUuserObject_flags_enum = 1;
pub const CU_GRAPH_USER_OBJECT_MOVE: CUuserObjectRetain_flags_enum = 1;
pub const CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH: CUgraphInstantiate_flags_enum = 1;
pub const CUDA_GRAPH_INSTANTIATE_FLAG_UPLOAD: CUgraphInstantiate_flags_enum = 2;
pub const CUDA_GRAPH_INSTANTIATE_FLAG_DEVICE_LAUNCH: CUgraphInstantiate_flags_enum = 4;
pub const CUDA_GRAPH_INSTANTIATE_FLAG_USE_NODE_PRIORITY: CUgraphInstantiate_flags_enum = 8;
pub const CU_DEVICE_NUMA_CONFIG_NONE: CUdeviceNumaConfig_enum = 0;
pub const CU_DEVICE_NUMA_CONFIG_NUMA_NODE: CUdeviceNumaConfig_enum = 1;
pub const CU_PROCESS_STATE_RUNNING: CUprocessState_enum = 0;
pub const CU_PROCESS_STATE_LOCKED: CUprocessState_enum = 1;
pub const CU_PROCESS_STATE_CHECKPOINTED: CUprocessState_enum = 2;
pub const CU_PROCESS_STATE_FAILED: CUprocessState_enum = 3;
pub const CU_MODULE_EAGER_LOADING: CUmoduleLoadingMode_enum = 1;
pub const CU_MODULE_LAZY_LOADING: CUmoduleLoadingMode_enum = 2;
pub const CU_MEM_DECOMPRESS_UNSUPPORTED: CUmemDecompressAlgorithm_enum = 0;
pub const CU_MEM_DECOMPRESS_ALGORITHM_DEFLATE: CUmemDecompressAlgorithm_enum = 1;
pub const CU_MEM_DECOMPRESS_ALGORITHM_SNAPPY: CUmemDecompressAlgorithm_enum = 2;
pub const CU_MEM_DECOMPRESS_ALGORITHM_LZ4: CUmemDecompressAlgorithm_enum = 4;
pub const CU_LOGICAL_ENDPOINT_IPC_HANDLE_TYPE_NONE: CUlogicalEndpointIpcHandleType_enum = 0;
pub const CU_LOGICAL_ENDPOINT_IPC_HANDLE_TYPE_FABRIC: CUlogicalEndpointIpcHandleType_enum = 1;
pub const CU_LOGICAL_ENDPOINT_TYPE_INVALID: CUlogicalEndpointType_enum = 0;
pub const CU_LOGICAL_ENDPOINT_TYPE_UNICAST: CUlogicalEndpointType_enum = 1;
pub const CU_LOGICAL_ENDPOINT_TYPE_MULTICAST: CUlogicalEndpointType_enum = 2;
pub const CU_LOGICAL_ENDPOINT_FLAG_NONE: CUlogicalEndpointFlag_enum = 0;
pub const CU_LOGICAL_ENDPOINT_FLAG_COUNTED_OPS: CUlogicalEndpointFlag_enum = 1;
pub const CU_GRAPH_RECAPTURE_ELIGIBLE_FOR_UPDATE: CUgraphRecaptureStatus_enum = 0;
pub const CU_GRAPH_RECAPTURE_INELIGIBLE_FOR_UPDATE: CUgraphRecaptureStatus_enum = 1;
pub const CU_GRAPH_RECAPTURE_ERROR: CUgraphRecaptureStatus_enum = 2;
pub const CU_FUNCTION_LOADING_STATE_UNLOADED: CUfunctionLoadingState_enum = 0;
pub const CU_FUNCTION_LOADING_STATE_LOADED: CUfunctionLoadingState_enum = 1;
pub const CU_FUNCTION_LOADING_STATE_MAX: CUfunctionLoadingState_enum = 2;
pub const CU_COREDUMP_ENABLE_ON_EXCEPTION: CUcoredumpSettings_enum = 1;
pub const CU_COREDUMP_TRIGGER_HOST: CUcoredumpSettings_enum = 2;
pub const CU_COREDUMP_LIGHTWEIGHT: CUcoredumpSettings_enum = 3;
pub const CU_COREDUMP_ENABLE_USER_TRIGGER: CUcoredumpSettings_enum = 4;
pub const CU_COREDUMP_FILE: CUcoredumpSettings_enum = 5;
pub const CU_COREDUMP_PIPE: CUcoredumpSettings_enum = 6;
pub const CU_COREDUMP_GENERATION_FLAGS: CUcoredumpSettings_enum = 7;
pub const CU_COREDUMP_MAX: CUcoredumpSettings_enum = 8;
pub const CU_COREDUMP_DEFAULT_FLAGS: CUCoredumpGenerationFlags = 0;
pub const CU_COREDUMP_SKIP_NONRELOCATED_ELF_IMAGES: CUCoredumpGenerationFlags = 1;
pub const CU_COREDUMP_SKIP_GLOBAL_MEMORY: CUCoredumpGenerationFlags = 2;
pub const CU_COREDUMP_SKIP_SHARED_MEMORY: CUCoredumpGenerationFlags = 4;
pub const CU_COREDUMP_SKIP_LOCAL_MEMORY: CUCoredumpGenerationFlags = 8;
pub const CU_COREDUMP_SKIP_ABORT: CUCoredumpGenerationFlags = 16;
pub const CU_COREDUMP_SKIP_CONSTBANK_MEMORY: CUCoredumpGenerationFlags = 32;
pub const CU_COREDUMP_GZIP_COMPRESS: CUCoredumpGenerationFlags = 64;
pub const CU_COREDUMP_FAULTED_CONTEXTS_ONLY: CUCoredumpGenerationFlags = 128;
pub const CU_COREDUMP_NO_ERRBAR_AT_EXIT: CUCoredumpGenerationFlags = 1073741824;
pub const CU_COREDUMP_LOG_ONLY: CUCoredumpGenerationFlags = -2147483648;
pub const CU_COREDUMP_LIGHTWEIGHT_FLAGS: CUCoredumpGenerationFlags = 47;
pub const CU_GREEN_CTX_NONE: CUgreenCtxCreate_flags = 0;
pub const CU_GREEN_CTX_DEFAULT_STREAM: CUgreenCtxCreate_flags = 1;
pub const CU_DEV_SM_RESOURCE_GROUP_DEFAULT: CUdevSmResourceGroup_flags = 0;
pub const CU_DEV_SM_RESOURCE_GROUP_BACKFILL: CUdevSmResourceGroup_flags = 1;
pub const CU_DEV_SM_RESOURCE_SPLIT_IGNORE_SM_COSCHEDULING: CUdevSmResourceSplitByCount_flags = 1;
pub const CU_DEV_SM_RESOURCE_SPLIT_MAX_POTENTIAL_CLUSTER_SIZE: CUdevSmResourceSplitByCount_flags =
    2;
pub const CU_DEV_RESOURCE_TYPE_INVALID: CUdevResourceType = 0;
pub const CU_DEV_RESOURCE_TYPE_SM: CUdevResourceType = 1;
pub const CU_DEV_RESOURCE_TYPE_WORKQUEUE_CONFIG: CUdevResourceType = 1000;
pub const CU_DEV_RESOURCE_TYPE_WORKQUEUE: CUdevResourceType = 10000;
pub const CU_WORKQUEUE_SCOPE_DEVICE_CTX: CUdevWorkqueueConfigScope = 0;
pub const CU_WORKQUEUE_SCOPE_GREEN_CTX_BALANCED: CUdevWorkqueueConfigScope = 1;
pub const CU_LOG_LEVEL_ERROR: CUlogLevel_enum = 0;
pub const CU_LOG_LEVEL_WARNING: CUlogLevel_enum = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::std::os::raw::c_int,
    pub sival_ptr: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sigaction_u {
    pub __sa_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub __sa_sigaction: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut __siginfo,
            arg3: *mut ::std::os::raw::c_void,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wait {
    pub w_status: ::std::os::raw::c_int,
    pub w_T: wait__bindgen_ty_1,
    pub w_S: wait__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUstreamBatchMemOpParams_union {
    pub operation: CUstreamBatchMemOpType,
    pub waitValue: CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st,
    pub writeValue: CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st,
    pub flushRemoteWrites: CUstreamBatchMemOpParams_union_CUstreamMemOpFlushRemoteWritesParams_st,
    pub memoryBarrier: CUstreamBatchMemOpParams_union_CUstreamMemOpMemoryBarrierParams_st,
    pub atomicReduction: CUstreamBatchMemOpParams_union_CUstreamMemOpAtomicReductionParams_st,
    pub pad: [cuuint64_t; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st__bindgen_ty_1 {
    pub value: cuuint32_t,
    pub value64: cuuint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st__bindgen_ty_1 {
    pub value: cuuint32_t,
    pub value64: cuuint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUasyncNotificationInfo_st__bindgen_ty_1 {
    pub overBudget: CUasyncNotificationInfo_st__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUlaunchAttributeValue_union {
    pub pad: [::std::os::raw::c_char; 64usize],
    pub accessPolicyWindow: CUaccessPolicyWindow,
    pub cooperative: ::std::os::raw::c_int,
    pub syncPolicy: CUsynchronizationPolicy,
    pub clusterDim: CUlaunchAttributeValue_union__bindgen_ty_1,
    pub clusterSchedulingPolicyPreference: CUclusterSchedulingPolicy,
    pub programmaticStreamSerializationAllowed: ::std::os::raw::c_int,
    pub programmaticEvent: CUlaunchAttributeValue_union__bindgen_ty_2,
    pub launchCompletionEvent: CUlaunchAttributeValue_union__bindgen_ty_3,
    pub priority: ::std::os::raw::c_int,
    pub memSyncDomainMap: CUlaunchMemSyncDomainMap,
    pub memSyncDomain: CUlaunchMemSyncDomain,
    pub preferredClusterDim: CUlaunchAttributeValue_union__bindgen_ty_4,
    pub deviceUpdatableKernelNode: CUlaunchAttributeValue_union__bindgen_ty_5,
    pub sharedMemCarveout: ::std::os::raw::c_uint,
    pub nvlinkUtilCentricScheduling: ::std::os::raw::c_uint,
    pub portableClusterSizeMode: CUlaunchAttributePortableClusterMode,
    pub sharedMemoryMode: CUsharedMemoryMode,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUexecAffinityParam_st__bindgen_ty_1 {
    pub smCount: CUexecAffinitySmCount,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_RESOURCE_DESC_st__bindgen_ty_1 {
    pub array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    pub mipmap: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2,
    pub linear: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3,
    pub pitch2D: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4,
    pub reserved: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_5,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1 {
    pub fd: ::std::os::raw::c_int,
    pub win32: CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciBufObject: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1 {
    pub fd: ::std::os::raw::c_int,
    pub win32: CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSyncObj: *const ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::std::os::raw::c_void,
    pub reserved: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::std::os::raw::c_void,
    pub reserved: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUarrayMapInfo_st__bindgen_ty_1 {
    pub mipmap: CUmipmappedArray,
    pub array: CUarray,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUarrayMapInfo_st__bindgen_ty_2 {
    pub sparseLevel: CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_1,
    pub miptail: CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUarrayMapInfo_st__bindgen_ty_3 {
    pub memHandle: CUmemGenericAllocationHandle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUmemLocation_st__bindgen_ty_1 {
    pub id: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUmemcpy3DOperand_st__bindgen_ty_1 {
    pub ptr: CUmemcpy3DOperand_st__bindgen_ty_1__bindgen_ty_1,
    pub array: CUmemcpy3DOperand_st__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUgraphNodeParams_st__bindgen_ty_1 {
    pub reserved1: [::std::os::raw::c_longlong; 29usize],
    pub kernel: CUDA_KERNEL_NODE_PARAMS_v3,
    pub memcpy: CUDA_MEMCPY_NODE_PARAMS,
    pub memset: CUDA_MEMSET_NODE_PARAMS_v2,
    pub host: CUDA_HOST_NODE_PARAMS_v2,
    pub graph: CUDA_CHILD_GRAPH_NODE_PARAMS,
    pub eventWait: CUDA_EVENT_WAIT_NODE_PARAMS,
    pub eventRecord: CUDA_EVENT_RECORD_NODE_PARAMS,
    pub extSemSignal: CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2,
    pub extSemWait: CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2,
    pub alloc: CUDA_MEM_ALLOC_NODE_PARAMS_v2,
    pub free: CUDA_MEM_FREE_NODE_PARAMS,
    pub memOp: CUDA_BATCH_MEM_OP_NODE_PARAMS_v2,
    pub conditional: CUDA_CONDITIONAL_NODE_PARAMS,
    pub asBytes: [::std::os::raw::c_char; 232usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUlogicalEndpointProp_struct__bindgen_ty_1 {
    pub unicast: CUlogicalEndpointProp_struct__bindgen_ty_1__bindgen_ty_1,
    pub multicast: CUlogicalEndpointProp_struct__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUdevResource_st__bindgen_ty_1 {
    pub sm: CUdevSmResource,
    pub wqConfig: CUdevWorkqueueConfigResource,
    pub wq: CUdevWorkqueueResource,
    pub _oversize: [::std::os::raw::c_uchar; 40usize],
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
impl wait__bindgen_ty_1 {
    #[inline]
    pub fn w_Termsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_w_Termsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Coredump(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_w_Coredump(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Retcode(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Retcode(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Termsig: ::std::os::raw::c_uint,
        w_Coredump: ::std::os::raw::c_uint,
        w_Retcode: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let w_Termsig: u32 = unsafe { ::std::mem::transmute(w_Termsig) };
            w_Termsig as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let w_Coredump: u32 = unsafe { ::std::mem::transmute(w_Coredump) };
            w_Coredump as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Retcode: u32 = unsafe { ::std::mem::transmute(w_Retcode) };
            w_Retcode as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
impl wait__bindgen_ty_2 {
    #[inline]
    pub fn w_Stopval(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopval(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Stopsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Stopval: ::std::os::raw::c_uint,
        w_Stopsig: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let w_Stopval: u32 = unsafe { ::std::mem::transmute(w_Stopval) };
            w_Stopval as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Stopsig: u32 = unsafe { ::std::mem::transmute(w_Stopsig) };
            w_Stopsig as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
pub use self::cl_context_flags_enum as cl_context_flags;
pub use self::cl_event_flags_enum as cl_event_flags;
pub use self::cudaError_enum as CUresult;
pub use self::CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum as CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS;
pub use self::CUGPUDirectRDMAWritesOrdering_enum as CUGPUDirectRDMAWritesOrdering;
pub use self::CUaccessProperty_enum as CUaccessProperty;
pub use self::CUaddress_mode_enum as CUaddress_mode;
pub use self::CUarraySparseSubresourceType_enum as CUarraySparseSubresourceType;
pub use self::CUarray_cubemap_face_enum as CUarray_cubemap_face;
pub use self::CUarray_format_enum as CUarray_format;
pub use self::CUasyncNotificationType_enum as CUasyncNotificationType;
pub use self::CUatomicOperationCapability_enum as CUatomicOperationCapability;
pub use self::CUatomicOperation_enum as CUatomicOperation;
pub use self::CUcigDataType_enum as CUcigDataType;
pub use self::CUclusterSchedulingPolicy_enum as CUclusterSchedulingPolicy;
pub use self::CUcomputemode_enum as CUcomputemode;
pub use self::CUcoredumpSettings_enum as CUcoredumpSettings;
pub use self::CUctx_flags_enum as CUctx_flags;
pub use self::CUdeviceNumaConfig_enum as CUdeviceNumaConfig;
pub use self::CUdevice_P2PAttribute_enum as CUdevice_P2PAttribute;
pub use self::CUdevice_attribute_enum as CUdevice_attribute;
pub use self::CUdriverProcAddressQueryResult_enum as CUdriverProcAddressQueryResult;
pub use self::CUdriverProcAddress_flags_enum as CUdriverProcAddress_flags;
pub use self::CUevent_flags_enum as CUevent_flags;
pub use self::CUevent_record_flags_enum as CUevent_record_flags;
pub use self::CUevent_sched_flags_enum as CUevent_sched_flags;
pub use self::CUevent_wait_flags_enum as CUevent_wait_flags;
pub use self::CUexecAffinityType_enum as CUexecAffinityType;
pub use self::CUexternalMemoryHandleType_enum as CUexternalMemoryHandleType;
pub use self::CUexternalSemaphoreHandleType_enum as CUexternalSemaphoreHandleType;
pub use self::CUfilter_mode_enum as CUfilter_mode;
pub use self::CUflushGPUDirectRDMAWritesOptions_enum as CUflushGPUDirectRDMAWritesOptions;
pub use self::CUflushGPUDirectRDMAWritesScope_enum as CUflushGPUDirectRDMAWritesScope;
pub use self::CUflushGPUDirectRDMAWritesTarget_enum as CUflushGPUDirectRDMAWritesTarget;
pub use self::CUfunc_cache_enum as CUfunc_cache;
pub use self::CUfunctionLoadingState_enum as CUfunctionLoadingState;
pub use self::CUfunction_attribute_enum as CUfunction_attribute;
pub use self::CUgraphChildGraphNodeOwnership_enum as CUgraphChildGraphNodeOwnership;
pub use self::CUgraphConditionalNodeType_enum as CUgraphConditionalNodeType;
pub use self::CUgraphDebugDot_flags_enum as CUgraphDebugDot_flags;
pub use self::CUgraphDependencyType_enum as CUgraphDependencyType;
pub use self::CUgraphExecUpdateResult_enum as CUgraphExecUpdateResult;
pub use self::CUgraphInstantiateResult_enum as CUgraphInstantiateResult;
pub use self::CUgraphInstantiate_flags_enum as CUgraphInstantiate_flags;
pub use self::CUgraphMem_attribute_enum as CUgraphMem_attribute;
pub use self::CUgraphNodeType_enum as CUgraphNodeType;
pub use self::CUgraphRecaptureStatus_enum as CUgraphRecaptureStatus;
pub use self::CUgraphicsMapResourceFlags_enum as CUgraphicsMapResourceFlags;
pub use self::CUgraphicsRegisterFlags_enum as CUgraphicsRegisterFlags;
pub use self::CUhostTaskSyncMode_enum as CUhostTaskSyncMode;
pub use self::CUipcMem_flags_enum as CUipcMem_flags;
pub use self::CUjitInputType_enum as CUjitInputType;
pub use self::CUjit_cacheMode_enum as CUjit_cacheMode;
pub use self::CUjit_fallback_enum as CUjit_fallback;
pub use self::CUjit_option_enum as CUjit_option;
pub use self::CUjit_target_enum as CUjit_target;
pub use self::CUlaunchAttributeID as CUkernelNodeAttrID;
pub use self::CUlaunchAttributeID as CUstreamAttrID;
pub use self::CUlaunchAttributeID_enum as CUlaunchAttributeID;
pub use self::CUlaunchAttributePortableClusterMode_enum as CUlaunchAttributePortableClusterMode;
pub use self::CUlaunchMemSyncDomain_enum as CUlaunchMemSyncDomain;
pub use self::CUlibraryOption_enum as CUlibraryOption;
pub use self::CUlimit_enum as CUlimit;
pub use self::CUlogLevel_enum as CUlogLevel;
pub use self::CUlogicalEndpointFlag_enum as CUlogicalEndpointFlag;
pub use self::CUlogicalEndpointIpcHandleType_enum as CUlogicalEndpointIpcHandleType;
pub use self::CUlogicalEndpointType_enum as CUlogicalEndpointType;
pub use self::CUmemAccess_flags_enum as CUmemAccess_flags;
pub use self::CUmemAllocationCompType_enum as CUmemAllocationCompType;
pub use self::CUmemAllocationGranularity_flags_enum as CUmemAllocationGranularity_flags;
pub use self::CUmemAllocationHandleType_enum as CUmemAllocationHandleType;
pub use self::CUmemAllocationType_enum as CUmemAllocationType;
pub use self::CUmemAttach_flags_enum as CUmemAttach_flags;
pub use self::CUmemDecompressAlgorithm_enum as CUmemDecompressAlgorithm;
pub use self::CUmemHandleType_enum as CUmemHandleType;
pub use self::CUmemLocationType_enum as CUmemLocationType;
pub use self::CUmemOperationType_enum as CUmemOperationType;
pub use self::CUmemPool_attribute_enum as CUmemPool_attribute;
pub use self::CUmemRangeFlags_enum as CUmemRangeFlags;
pub use self::CUmemRangeHandleType_enum as CUmemRangeHandleType;
pub use self::CUmem_advise_enum as CUmem_advise;
pub use self::CUmem_range_attribute_enum as CUmem_range_attribute;
pub use self::CUmemcpy3DOperandType_enum as CUmemcpy3DOperandType;
pub use self::CUmemcpyFlags_enum as CUmemcpyFlags;
pub use self::CUmemcpySrcAccessOrder_enum as CUmemcpySrcAccessOrder;
pub use self::CUmemorytype_enum as CUmemorytype;
pub use self::CUmoduleLoadingMode_enum as CUmoduleLoadingMode;
pub use self::CUmulticastGranularity_flags_enum as CUmulticastGranularity_flags;
pub use self::CUoccupancy_flags_enum as CUoccupancy_flags;
pub use self::CUpointer_attribute_enum as CUpointer_attribute;
pub use self::CUprocessState_enum as CUprocessState;
pub use self::CUresourceViewFormat_enum as CUresourceViewFormat;
pub use self::CUresourcetype_enum as CUresourcetype;
pub use self::CUsharedMemoryMode_enum as CUsharedMemoryMode;
pub use self::CUshared_carveout_enum as CUshared_carveout;
pub use self::CUsharedconfig_enum as CUsharedconfig;
pub use self::CUstreamAtomicReductionDataType_enum as CUstreamAtomicReductionDataType;
pub use self::CUstreamAtomicReductionOpType_enum as CUstreamAtomicReductionOpType;
pub use self::CUstreamBatchMemOpType_enum as CUstreamBatchMemOpType;
pub use self::CUstreamCaptureMode_enum as CUstreamCaptureMode;
pub use self::CUstreamCaptureStatus_enum as CUstreamCaptureStatus;
pub use self::CUstreamCigDataType_enum as CUstreamCigDataType;
pub use self::CUstreamMemoryBarrier_flags_enum as CUstreamMemoryBarrier_flags;
pub use self::CUstreamUpdateCaptureDependencies_flags_enum as CUstreamUpdateCaptureDependencies_flags;
pub use self::CUstreamWaitValue_flags_enum as CUstreamWaitValue_flags;
pub use self::CUstreamWriteValue_flags_enum as CUstreamWriteValue_flags;
pub use self::CUstream_flags_enum as CUstream_flags;
pub use self::CUsynchronizationPolicy_enum as CUsynchronizationPolicy;
pub use self::CUtensorMapDataType_enum as CUtensorMapDataType;
pub use self::CUtensorMapFloatOOBfill_enum as CUtensorMapFloatOOBfill;
pub use self::CUtensorMapIm2ColWideMode_enum as CUtensorMapIm2ColWideMode;
pub use self::CUtensorMapInterleave_enum as CUtensorMapInterleave;
pub use self::CUtensorMapL2promotion_enum as CUtensorMapL2promotion;
pub use self::CUtensorMapSwizzle_enum as CUtensorMapSwizzle;
pub use self::CUuserObjectRetain_flags_enum as CUuserObjectRetain_flags;
pub use self::CUuserObject_flags_enum as CUuserObject_flags;
