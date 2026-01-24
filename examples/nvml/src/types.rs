// Bindgen-generated bindings
pub type nvmlDevice_t = *mut nvmlDevice_st;
pub type nvmlGpuInstance_t = *mut nvmlGpuInstance_st;
pub type nvmlPciInfoExt_t = nvmlPciInfoExt_v1_t;
pub type nvmlPciInfo_t = nvmlPciInfo_st;
pub type nvmlEccErrorCounts_t = nvmlEccErrorCounts_st;
pub type nvmlUtilization_t = nvmlUtilization_st;
pub type nvmlMemory_t = nvmlMemory_st;
pub type nvmlMemory_v2_t = nvmlMemory_v2_st;
pub type nvmlBAR1Memory_t = nvmlBAR1Memory_st;
pub type nvmlProcessInfo_v1_t = nvmlProcessInfo_v1_st;
pub type nvmlProcessInfo_v2_t = nvmlProcessInfo_v2_st;
pub type nvmlProcessInfo_t = nvmlProcessInfo_v2_st;
pub type nvmlProcessDetailList_t = nvmlProcessDetailList_v1_t;
pub type nvmlDeviceAttributes_t = nvmlDeviceAttributes_st;
pub type nvmlDeviceAddressingModeType_t = ::std::os::raw::c_uint;
pub type nvmlDeviceAddressingMode_t = nvmlDeviceAddressingMode_v1_t;
pub type nvmlRepairStatus_t = nvmlRepairStatus_v1_t;
pub type nvmlRowRemapperHistogramValues_t = nvmlRowRemapperHistogramValues_st;
pub type nvmlBridgeChipType_enum = ::std::os::raw::c_uint;
pub type nvmlNvLinkUtilizationCountUnits_enum = ::std::os::raw::c_uint;
pub type nvmlNvLinkUtilizationCountPktTypes_enum = ::std::os::raw::c_uint;
pub type nvmlNvLinkUtilizationControl_t = nvmlNvLinkUtilizationControl_st;
pub type nvmlNvLinkCapability_enum = ::std::os::raw::c_uint;
pub type nvmlNvLinkErrorCounter_enum = ::std::os::raw::c_uint;
pub type nvmlIntNvLinkDeviceType_enum = ::std::os::raw::c_uint;
pub type nvmlGpuLevel_enum = ::std::os::raw::c_uint;
pub type nvmlGpuP2PStatus_enum = ::std::os::raw::c_uint;
pub type nvmlGpuP2PCapsIndex_enum = ::std::os::raw::c_uint;
pub type nvmlBridgeChipInfo_t = nvmlBridgeChipInfo_st;
pub type nvmlBridgeChipHierarchy_t = nvmlBridgeChipHierarchy_st;
pub type nvmlSamplingType_enum = ::std::os::raw::c_uint;
pub type nvmlPcieUtilCounter_enum = ::std::os::raw::c_uint;
pub type nvmlValueType_enum = ::std::os::raw::c_uint;
pub type nvmlValue_t = nvmlValue_st;
pub type nvmlSample_t = nvmlSample_st;
pub type nvmlPerfPolicyType_enum = ::std::os::raw::c_uint;
pub type nvmlViolationTime_t = nvmlViolationTime_st;
pub type nvmlThermalTarget_t = ::std::os::raw::c_int;
pub type nvmlThermalController_t = ::std::os::raw::c_int;
pub type nvmlCoolerControl_enum = ::std::os::raw::c_uint;
pub type nvmlCoolerTarget_enum = ::std::os::raw::c_uint;
pub type nvmlCoolerInfo_t = nvmlCoolerInfo_v1_t;
pub type nvmlUUIDType_t = ::std::os::raw::c_uint;
pub type nvmlUUID_t = nvmlUUID_v1_t;
pub type nvmlPdi_t = nvmlPdi_v1_t;
pub type nvmlEnableState_enum = ::std::os::raw::c_uint;
pub type nvmlDramEncryptionInfo_t = nvmlDramEncryptionInfo_v1_t;
pub type nvmlBrandType_enum = ::std::os::raw::c_uint;
pub type nvmlTemperatureThresholds_enum = ::std::os::raw::c_uint;
pub type nvmlTemperatureSensors_enum = ::std::os::raw::c_uint;
pub type nvmlMarginTemperature_t = nvmlMarginTemperature_v1_t;
pub type nvmlComputeMode_enum = ::std::os::raw::c_uint;
pub type nvmlClkMonFaultInfo_t = nvmlClkMonFaultInfo_struct;
pub type nvmlClkMonStatus_t = nvmlClkMonStatus_status;
pub type nvmlMemoryErrorType_enum = ::std::os::raw::c_uint;
pub type nvmlNvlinkVersion_enum = ::std::os::raw::c_uint;
pub type nvmlEccCounterType_enum = ::std::os::raw::c_uint;
pub type nvmlClockType_enum = ::std::os::raw::c_uint;
pub type nvmlClockId_enum = ::std::os::raw::c_uint;
pub type nvmlDriverModel_enum = ::std::os::raw::c_uint;
pub type nvmlPStates_enum = ::std::os::raw::c_uint;
pub type nvmlClockOffset_t = nvmlClockOffset_v1_t;
pub type nvmlFanSpeedInfo_t = nvmlFanSpeedInfo_v1_t;
pub type nvmlDevicePerfModes_t = nvmlDevicePerfModes_v1_t;
pub type nvmlDeviceCurrentClockFreqs_t = nvmlDeviceCurrentClockFreqs_v1_t;
pub type nvmlGom_enum = ::std::os::raw::c_uint;
pub type nvmlInforomObject_enum = ::std::os::raw::c_uint;
pub type nvmlReturn_enum = ::std::os::raw::c_uint;
pub type nvmlMemoryLocation_enum = ::std::os::raw::c_uint;
pub type nvmlPageRetirementCause_enum = ::std::os::raw::c_uint;
pub type nvmlRestrictedAPI_enum = ::std::os::raw::c_uint;
pub type nvmlProcessUtilizationSample_t = nvmlProcessUtilizationSample_st;
pub type nvmlProcessesUtilizationInfo_t = nvmlProcessesUtilizationInfo_v1_t;
pub type nvmlEccSramErrorStatus_t = nvmlEccSramErrorStatus_v1_t;
pub type nvmlPlatformInfo_t = nvmlPlatformInfo_v2_t;
pub type nvmlEccSramUniqueUncorrectedErrorCounts_t = nvmlEccSramUniqueUncorrectedErrorCounts_v1_t;
pub type nvmlDeviceArchitecture_t = ::std::os::raw::c_uint;
pub type nvmlBusType_t = ::std::os::raw::c_uint;
pub type nvmlFanControlPolicy_t = ::std::os::raw::c_uint;
pub type nvmlPowerSource_t = ::std::os::raw::c_uint;
pub type nvmlGpuUtilizationDomainId_t = ::std::os::raw::c_uint;
pub type nvmlGpuDynamicPstatesInfo_t = nvmlGpuDynamicPstatesInfo_st;
pub type nvmlPowerScopeType_t = ::std::os::raw::c_uchar;
pub type nvmlGpuVirtualizationMode = ::std::os::raw::c_uint;
pub type nvmlHostVgpuMode_enum = ::std::os::raw::c_uint;
pub type nvmlVgpuVmIdType = ::std::os::raw::c_uint;
pub type nvmlVgpuGuestInfoState_enum = ::std::os::raw::c_uint;
pub type nvmlGridLicenseFeatureCode_t = ::std::os::raw::c_uint;
pub type nvmlVgpuCapability_enum = ::std::os::raw::c_uint;
pub type nvmlVgpuDriverCapability_enum = ::std::os::raw::c_uint;
pub type nvmlDeviceVgpuCapability_enum = ::std::os::raw::c_uint;
pub type nvmlVgpuTypeId_t = ::std::os::raw::c_uint;
pub type nvmlVgpuInstance_t = ::std::os::raw::c_uint;
pub type nvmlVgpuHeterogeneousMode_t = nvmlVgpuHeterogeneousMode_v1_t;
pub type nvmlVgpuPlacementId_t = nvmlVgpuPlacementId_v1_t;
pub type nvmlVgpuPlacementList_t = nvmlVgpuPlacementList_v2_t;
pub type nvmlVgpuTypeBar1Info_t = nvmlVgpuTypeBar1Info_v1_t;
pub type nvmlVgpuInstanceUtilizationSample_t = nvmlVgpuInstanceUtilizationSample_st;
pub type nvmlVgpuInstancesUtilizationInfo_t = nvmlVgpuInstancesUtilizationInfo_v1_t;
pub type nvmlVgpuProcessUtilizationSample_t = nvmlVgpuProcessUtilizationSample_st;
pub type nvmlVgpuProcessesUtilizationInfo_t = nvmlVgpuProcessesUtilizationInfo_v1_t;
pub type nvmlVgpuRuntimeState_t = nvmlVgpuRuntimeState_v1_t;
pub type nvmlVgpuSchedulerLogEntry_t = nvmlVgpuSchedulerLogEntries_st;
pub type nvmlVgpuSchedulerLog_t = nvmlVgpuSchedulerLog_st;
pub type nvmlVgpuSchedulerGetState_t = nvmlVgpuSchedulerGetState_st;
pub type nvmlVgpuSchedulerSetState_t = nvmlVgpuSchedulerSetState_st;
pub type nvmlVgpuSchedulerCapabilities_t = nvmlVgpuSchedulerCapabilities_st;
pub type nvmlVgpuLicenseExpiry_t = nvmlVgpuLicenseExpiry_st;
pub type nvmlVgpuLicenseInfo_t = nvmlVgpuLicenseInfo_st;
pub type nvmlGridLicenseExpiry_t = nvmlGridLicenseExpiry_st;
pub type nvmlGridLicensableFeature_t = nvmlGridLicensableFeature_st;
pub type nvmlGridLicensableFeatures_t = nvmlGridLicensableFeatures_st;
pub type nvmlDeviceGpuRecoveryAction_s = ::std::os::raw::c_uint;
pub type nvmlVgpuTypeIdInfo_t = nvmlVgpuTypeIdInfo_v1_t;
pub type nvmlVgpuTypeMaxInstance_t = nvmlVgpuTypeMaxInstance_v1_t;
pub type nvmlActiveVgpuInstanceInfo_t = nvmlActiveVgpuInstanceInfo_v1_t;
pub type nvmlVgpuSchedulerState_t = nvmlVgpuSchedulerState_v1_t;
pub type nvmlVgpuSchedulerStateInfo_t = nvmlVgpuSchedulerStateInfo_v1_t;
pub type nvmlVgpuSchedulerLogInfo_t = nvmlVgpuSchedulerLogInfo_v1_t;
pub type nvmlVgpuCreatablePlacementInfo_t = nvmlVgpuCreatablePlacementInfo_v1_t;
pub type nvmlNvLinkPowerThres_t = nvmlNvLinkPowerThres_st;
pub type nvmlFieldValue_t = nvmlFieldValue_st;
pub type nvmlUnit_t = *mut nvmlUnit_st;
pub type nvmlHwbcEntry_t = nvmlHwbcEntry_st;
pub type nvmlFanState_enum = ::std::os::raw::c_uint;
pub type nvmlLedColor_enum = ::std::os::raw::c_uint;
pub type nvmlLedState_t = nvmlLedState_st;
pub type nvmlUnitInfo_t = nvmlUnitInfo_st;
pub type nvmlPSUInfo_t = nvmlPSUInfo_st;
pub type nvmlUnitFanInfo_t = nvmlUnitFanInfo_st;
pub type nvmlUnitFanSpeeds_t = nvmlUnitFanSpeeds_st;
pub type nvmlEventSet_t = *mut nvmlEventSet_st;
pub type nvmlEventData_t = nvmlEventData_st;
pub type nvmlSystemEventSet_t = *mut nvmlSystemEventSet_st;
pub type nvmlSystemEventSetCreateRequest_t = nvmlSystemEventSetCreateRequest_v1_t;
pub type nvmlSystemEventSetFreeRequest_t = nvmlSystemEventSetFreeRequest_v1_t;
pub type nvmlSystemRegisterEventRequest_t = nvmlSystemRegisterEventRequest_v1_t;
pub type nvmlSystemEventSetWaitRequest_t = nvmlSystemEventSetWaitRequest_v1_t;
pub type nvmlAccountingStats_t = nvmlAccountingStats_st;
pub type nvmlEncoderQueryType_enum = ::std::os::raw::c_uint;
pub type nvmlEncoderSessionInfo_t = nvmlEncoderSessionInfo_st;
pub type nvmlFBCSessionType_enum = ::std::os::raw::c_uint;
pub type nvmlFBCStats_t = nvmlFBCStats_st;
pub type nvmlFBCSessionInfo_t = nvmlFBCSessionInfo_st;
pub type nvmlDetachGpuState_enum = ::std::os::raw::c_uint;
pub type nvmlPcieLinkState_enum = ::std::os::raw::c_uint;
pub type nvmlConfComputeSystemCaps_t = nvmlConfComputeSystemCaps_st;
pub type nvmlConfComputeSystemState_t = nvmlConfComputeSystemState_st;
pub type nvmlSystemConfComputeSettings_t = nvmlSystemConfComputeSettings_v1_t;
pub type nvmlConfComputeMemSizeInfo_t = nvmlConfComputeMemSizeInfo_st;
pub type nvmlConfComputeGpuCertificate_t = nvmlConfComputeGpuCertificate_st;
pub type nvmlConfComputeGpuAttestationReport_t = nvmlConfComputeGpuAttestationReport_st;
pub type nvmlConfComputeSetKeyRotationThresholdInfo_v1_t =
    nvmlConfComputeSetKeyRotationThresholdInfo_st;
pub type nvmlConfComputeSetKeyRotationThresholdInfo_t =
    nvmlConfComputeSetKeyRotationThresholdInfo_v1_t;
pub type nvmlConfComputeGetKeyRotationThresholdInfo_v1_t =
    nvmlConfComputeGetKeyRotationThresholdInfo_st;
pub type nvmlConfComputeGetKeyRotationThresholdInfo_t =
    nvmlConfComputeGetKeyRotationThresholdInfo_v1_t;
pub type nvmlGpuFabricState_t = ::std::os::raw::c_uchar;
pub type nvmlGpuFabricInfoV_t = nvmlGpuFabricInfo_v3_t;
pub type nvmlSystemDriverBranchInfo_t = nvmlSystemDriverBranchInfo_v1_t;
pub type nvmlAffinityScope_t = ::std::os::raw::c_uint;
pub type nvmlTemperature_t = nvmlTemperature_v1_t;
pub type nvmlClockLimitId_enum = ::std::os::raw::c_uint;
pub type nvmlNvlinkSupportedBwModes_t = nvmlNvlinkSupportedBwModes_v1_t;
pub type nvmlNvlinkGetBwMode_t = nvmlNvlinkGetBwMode_v1_t;
pub type nvmlNvlinkSetBwMode_t = nvmlNvlinkSetBwMode_v1_t;
pub type nvmlNvLinkInfo_t = nvmlNvLinkInfo_v2_t;
pub type nvmlVgpuVersion_t = nvmlVgpuVersion_st;
pub type nvmlVgpuMetadata_t = nvmlVgpuMetadata_st;
pub type nvmlVgpuPgpuMetadata_t = nvmlVgpuPgpuMetadata_st;
pub type nvmlVgpuVmCompatibility_enum = ::std::os::raw::c_uint;
pub type nvmlVgpuPgpuCompatibilityLimitCode_enum = ::std::os::raw::c_uint;
pub type nvmlVgpuPgpuCompatibility_t = nvmlVgpuPgpuCompatibility_st;
pub type nvmlExcludedDeviceInfo_t = nvmlExcludedDeviceInfo_st;
pub type nvmlGpuInstancePlacement_t = nvmlGpuInstancePlacement_st;
pub type nvmlGpuInstanceProfileInfo_t = nvmlGpuInstanceProfileInfo_st;
pub type nvmlGpuInstanceProfileInfo_v2_t = nvmlGpuInstanceProfileInfo_v2_st;
pub type nvmlGpuInstanceProfileInfo_v3_t = nvmlGpuInstanceProfileInfo_v3_st;
pub type nvmlGpuInstanceInfo_t = nvmlGpuInstanceInfo_st;
pub type nvmlComputeInstancePlacement_t = nvmlComputeInstancePlacement_st;
pub type nvmlComputeInstanceProfileInfo_t = nvmlComputeInstanceProfileInfo_st;
pub type nvmlComputeInstanceProfileInfo_v2_t = nvmlComputeInstanceProfileInfo_v2_st;
pub type nvmlComputeInstanceProfileInfo_v3_t = nvmlComputeInstanceProfileInfo_v3_st;
pub type nvmlComputeInstanceInfo_t = nvmlComputeInstanceInfo_st;
pub type nvmlComputeInstance_t = *mut nvmlComputeInstance_st;
pub type nvmlGpmMetricId_t = ::std::os::raw::c_uint;
pub type nvmlGpmSample_t = *mut nvmlGpmSample_st;
pub type nvmlDeviceCapabilities_t = nvmlDeviceCapabilities_v1_t;
pub type nvmlPowerProfileType_t = ::std::os::raw::c_uint;
pub type nvmlWorkloadPowerProfileInfo_t = nvmlWorkloadPowerProfileInfo_v1_t;
pub type nvmlWorkloadPowerProfileProfilesInfo_t = nvmlWorkloadPowerProfileProfilesInfo_v1_t;
pub type nvmlWorkloadPowerProfileCurrentProfiles_t = nvmlWorkloadPowerProfileCurrentProfiles_v1_t;
pub type nvmlWorkloadPowerProfileRequestedProfiles_t =
    nvmlWorkloadPowerProfileRequestedProfiles_v1_t;
pub type nvmlPowerSmoothingProfile_t = nvmlPowerSmoothingProfile_v1_t;
pub type nvmlPowerSmoothingState_t = nvmlPowerSmoothingState_v1_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDevice_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuInstance_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPciInfoExt_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub domain: ::std::os::raw::c_uint,
    pub bus: ::std::os::raw::c_uint,
    pub device: ::std::os::raw::c_uint,
    pub pciDeviceId: ::std::os::raw::c_uint,
    pub pciSubSystemId: ::std::os::raw::c_uint,
    pub baseClass: ::std::os::raw::c_uint,
    pub subClass: ::std::os::raw::c_uint,
    pub busId: [::std::os::raw::c_char; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPciInfo_st {
    pub busIdLegacy: [::std::os::raw::c_char; 16usize],
    pub domain: ::std::os::raw::c_uint,
    pub bus: ::std::os::raw::c_uint,
    pub device: ::std::os::raw::c_uint,
    pub pciDeviceId: ::std::os::raw::c_uint,
    pub pciSubSystemId: ::std::os::raw::c_uint,
    pub busId: [::std::os::raw::c_char; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlEccErrorCounts_st {
    pub l1Cache: ::std::os::raw::c_ulonglong,
    pub l2Cache: ::std::os::raw::c_ulonglong,
    pub deviceMemory: ::std::os::raw::c_ulonglong,
    pub registerFile: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlUtilization_st {
    pub gpu: ::std::os::raw::c_uint,
    pub memory: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlMemory_st {
    pub total: ::std::os::raw::c_ulonglong,
    pub free: ::std::os::raw::c_ulonglong,
    pub used: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlMemory_v2_st {
    pub version: ::std::os::raw::c_uint,
    pub total: ::std::os::raw::c_ulonglong,
    pub reserved: ::std::os::raw::c_ulonglong,
    pub free: ::std::os::raw::c_ulonglong,
    pub used: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlBAR1Memory_st {
    pub bar1Total: ::std::os::raw::c_ulonglong,
    pub bar1Free: ::std::os::raw::c_ulonglong,
    pub bar1Used: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlProcessInfo_v1_st {
    pub pid: ::std::os::raw::c_uint,
    pub usedGpuMemory: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlProcessInfo_v2_st {
    pub pid: ::std::os::raw::c_uint,
    pub usedGpuMemory: ::std::os::raw::c_ulonglong,
    pub gpuInstanceId: ::std::os::raw::c_uint,
    pub computeInstanceId: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlProcessDetail_v1_t {
    pub pid: ::std::os::raw::c_uint,
    pub usedGpuMemory: ::std::os::raw::c_ulonglong,
    pub gpuInstanceId: ::std::os::raw::c_uint,
    pub computeInstanceId: ::std::os::raw::c_uint,
    pub usedGpuCcProtectedMemory: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlProcessDetailList_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub mode: ::std::os::raw::c_uint,
    pub numProcArrayEntries: ::std::os::raw::c_uint,
    pub procArray: *mut nvmlProcessDetail_v1_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDeviceAttributes_st {
    pub multiprocessorCount: ::std::os::raw::c_uint,
    pub sharedCopyEngineCount: ::std::os::raw::c_uint,
    pub sharedDecoderCount: ::std::os::raw::c_uint,
    pub sharedEncoderCount: ::std::os::raw::c_uint,
    pub sharedJpegCount: ::std::os::raw::c_uint,
    pub sharedOfaCount: ::std::os::raw::c_uint,
    pub gpuInstanceSliceCount: ::std::os::raw::c_uint,
    pub computeInstanceSliceCount: ::std::os::raw::c_uint,
    pub memorySizeMB: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlC2cModeInfo_v1_t {
    pub isC2cEnabled: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDeviceAddressingMode_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub value: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlRepairStatus_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub bChannelRepairPending: ::std::os::raw::c_uint,
    pub bTpcRepairPending: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlRowRemapperHistogramValues_st {
    pub max: ::std::os::raw::c_uint,
    pub high: ::std::os::raw::c_uint,
    pub partial: ::std::os::raw::c_uint,
    pub low: ::std::os::raw::c_uint,
    pub none: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvLinkUtilizationControl_st {
    pub units: nvmlNvLinkUtilizationCountUnits_t,
    pub pktfilter: nvmlNvLinkUtilizationCountPktTypes_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlBridgeChipInfo_st {
    pub type_: nvmlBridgeChipType_t,
    pub fwVersion: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlBridgeChipHierarchy_st {
    pub bridgeCount: ::std::os::raw::c_uchar,
    pub bridgeChipInfo: [nvmlBridgeChipInfo_t; 128usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlSample_st {
    pub timeStamp: ::std::os::raw::c_ulonglong,
    pub sampleValue: nvmlValue_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlViolationTime_st {
    pub referenceTime: ::std::os::raw::c_ulonglong,
    pub violationTime: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuThermalSettings_t {
    pub count: ::std::os::raw::c_uint,
    pub sensor: [nvmlGpuThermalSettings_t__bindgen_ty_1; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuThermalSettings_t__bindgen_ty_1 {
    pub controller: nvmlThermalController_t,
    pub defaultMinTemp: ::std::os::raw::c_int,
    pub defaultMaxTemp: ::std::os::raw::c_int,
    pub currentTemp: ::std::os::raw::c_int,
    pub target: nvmlThermalTarget_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlCoolerInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub index: ::std::os::raw::c_uint,
    pub signalType: nvmlCoolerControl_t,
    pub target: nvmlCoolerTarget_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlUUID_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub type_: ::std::os::raw::c_uint,
    pub value: nvmlUUIDValue_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPdi_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub value: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDramEncryptionInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub encryptionState: nvmlEnableState_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlMarginTemperature_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub marginTemperature: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlClkMonFaultInfo_struct {
    pub clkApiDomain: ::std::os::raw::c_uint,
    pub clkDomainFaultMask: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlClkMonStatus_status {
    pub bGlobalStatus: ::std::os::raw::c_uint,
    pub clkMonListSize: ::std::os::raw::c_uint,
    pub clkMonList: [nvmlClkMonFaultInfo_t; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlClockOffset_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub type_: nvmlClockType_t,
    pub pstate: nvmlPstates_t,
    pub clockOffsetMHz: ::std::os::raw::c_int,
    pub minClockOffsetMHz: ::std::os::raw::c_int,
    pub maxClockOffsetMHz: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlFanSpeedInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub fan: ::std::os::raw::c_uint,
    pub speed: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDevicePerfModes_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub str_: [::std::os::raw::c_char; 2048usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDeviceCurrentClockFreqs_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub str_: [::std::os::raw::c_char; 2048usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDevicePowerMizerModes_v1_t {
    pub currentMode: ::std::os::raw::c_uint,
    pub mode: ::std::os::raw::c_uint,
    pub supportedPowerMizerModes: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlProcessUtilizationSample_st {
    pub pid: ::std::os::raw::c_uint,
    pub timeStamp: ::std::os::raw::c_ulonglong,
    pub smUtil: ::std::os::raw::c_uint,
    pub memUtil: ::std::os::raw::c_uint,
    pub encUtil: ::std::os::raw::c_uint,
    pub decUtil: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlProcessUtilizationInfo_v1_t {
    pub timeStamp: ::std::os::raw::c_ulonglong,
    pub pid: ::std::os::raw::c_uint,
    pub smUtil: ::std::os::raw::c_uint,
    pub memUtil: ::std::os::raw::c_uint,
    pub encUtil: ::std::os::raw::c_uint,
    pub decUtil: ::std::os::raw::c_uint,
    pub jpgUtil: ::std::os::raw::c_uint,
    pub ofaUtil: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlProcessesUtilizationInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub processSamplesCount: ::std::os::raw::c_uint,
    pub lastSeenTimeStamp: ::std::os::raw::c_ulonglong,
    pub procUtilArray: *mut nvmlProcessUtilizationInfo_v1_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlEccSramErrorStatus_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub aggregateUncParity: ::std::os::raw::c_ulonglong,
    pub aggregateUncSecDed: ::std::os::raw::c_ulonglong,
    pub aggregateCor: ::std::os::raw::c_ulonglong,
    pub volatileUncParity: ::std::os::raw::c_ulonglong,
    pub volatileUncSecDed: ::std::os::raw::c_ulonglong,
    pub volatileCor: ::std::os::raw::c_ulonglong,
    pub aggregateUncBucketL2: ::std::os::raw::c_ulonglong,
    pub aggregateUncBucketSm: ::std::os::raw::c_ulonglong,
    pub aggregateUncBucketPcie: ::std::os::raw::c_ulonglong,
    pub aggregateUncBucketMcu: ::std::os::raw::c_ulonglong,
    pub aggregateUncBucketOther: ::std::os::raw::c_ulonglong,
    pub bThresholdExceeded: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPlatformInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub ibGuid: [::std::os::raw::c_uchar; 16usize],
    pub rackGuid: [::std::os::raw::c_uchar; 16usize],
    pub chassisPhysicalSlotNumber: ::std::os::raw::c_uchar,
    pub computeSlotIndex: ::std::os::raw::c_uchar,
    pub nodeIndex: ::std::os::raw::c_uchar,
    pub peerType: ::std::os::raw::c_uchar,
    pub moduleId: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPlatformInfo_v2_t {
    pub version: ::std::os::raw::c_uint,
    pub ibGuid: [::std::os::raw::c_uchar; 16usize],
    pub chassisSerialNumber: [::std::os::raw::c_uchar; 16usize],
    pub slotNumber: ::std::os::raw::c_uchar,
    pub trayIndex: ::std::os::raw::c_uchar,
    pub hostId: ::std::os::raw::c_uchar,
    pub peerType: ::std::os::raw::c_uchar,
    pub moduleId: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlEccSramUniqueUncorrectedErrorEntry_v1_t {
    pub unit: ::std::os::raw::c_uint,
    pub location: ::std::os::raw::c_uint,
    pub sublocation: ::std::os::raw::c_uint,
    pub extlocation: ::std::os::raw::c_uint,
    pub address: ::std::os::raw::c_uint,
    pub isParity: ::std::os::raw::c_uint,
    pub count: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlEccSramUniqueUncorrectedErrorCounts_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub entryCount: ::std::os::raw::c_uint,
    pub entries: *mut nvmlEccSramUniqueUncorrectedErrorEntry_v1_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuDynamicPstatesInfo_st {
    pub flags: ::std::os::raw::c_uint,
    pub utilization: [nvmlGpuDynamicPstatesInfo_st__bindgen_ty_1; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuDynamicPstatesInfo_st__bindgen_ty_1 {
    pub bIsPresent: ::std::os::raw::c_uint,
    pub percentage: ::std::os::raw::c_uint,
    pub incThreshold: ::std::os::raw::c_uint,
    pub decThreshold: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPowerValue_v2_t {
    pub version: ::std::os::raw::c_uint,
    pub powerScope: nvmlPowerScopeType_t,
    pub powerValueMw: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuHeterogeneousMode_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub mode: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuPlacementId_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub placementId: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuPlacementList_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub placementSize: ::std::os::raw::c_uint,
    pub count: ::std::os::raw::c_uint,
    pub placementIds: *mut ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuPlacementList_v2_t {
    pub version: ::std::os::raw::c_uint,
    pub placementSize: ::std::os::raw::c_uint,
    pub count: ::std::os::raw::c_uint,
    pub placementIds: *mut ::std::os::raw::c_uint,
    pub mode: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuTypeBar1Info_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub bar1Size: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuInstanceUtilizationSample_st {
    pub vgpuInstance: nvmlVgpuInstance_t,
    pub timeStamp: ::std::os::raw::c_ulonglong,
    pub smUtil: nvmlValue_t,
    pub memUtil: nvmlValue_t,
    pub encUtil: nvmlValue_t,
    pub decUtil: nvmlValue_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuInstanceUtilizationInfo_v1_t {
    pub timeStamp: ::std::os::raw::c_ulonglong,
    pub vgpuInstance: nvmlVgpuInstance_t,
    pub smUtil: nvmlValue_t,
    pub memUtil: nvmlValue_t,
    pub encUtil: nvmlValue_t,
    pub decUtil: nvmlValue_t,
    pub jpgUtil: nvmlValue_t,
    pub ofaUtil: nvmlValue_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuInstancesUtilizationInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub sampleValType: nvmlValueType_t,
    pub vgpuInstanceCount: ::std::os::raw::c_uint,
    pub lastSeenTimeStamp: ::std::os::raw::c_ulonglong,
    pub vgpuUtilArray: *mut nvmlVgpuInstanceUtilizationInfo_v1_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuProcessUtilizationSample_st {
    pub vgpuInstance: nvmlVgpuInstance_t,
    pub pid: ::std::os::raw::c_uint,
    pub processName: [::std::os::raw::c_char; 64usize],
    pub timeStamp: ::std::os::raw::c_ulonglong,
    pub smUtil: ::std::os::raw::c_uint,
    pub memUtil: ::std::os::raw::c_uint,
    pub encUtil: ::std::os::raw::c_uint,
    pub decUtil: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuProcessUtilizationInfo_v1_t {
    pub processName: [::std::os::raw::c_char; 64usize],
    pub timeStamp: ::std::os::raw::c_ulonglong,
    pub vgpuInstance: nvmlVgpuInstance_t,
    pub pid: ::std::os::raw::c_uint,
    pub smUtil: ::std::os::raw::c_uint,
    pub memUtil: ::std::os::raw::c_uint,
    pub encUtil: ::std::os::raw::c_uint,
    pub decUtil: ::std::os::raw::c_uint,
    pub jpgUtil: ::std::os::raw::c_uint,
    pub ofaUtil: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuProcessesUtilizationInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub vgpuProcessCount: ::std::os::raw::c_uint,
    pub lastSeenTimeStamp: ::std::os::raw::c_ulonglong,
    pub vgpuProcUtilArray: *mut nvmlVgpuProcessUtilizationInfo_v1_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuRuntimeState_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub size: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuSchedulerParams_t__bindgen_ty_1 {
    pub avgFactor: ::std::os::raw::c_uint,
    pub timeslice: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuSchedulerParams_t__bindgen_ty_2 {
    pub timeslice: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuSchedulerLogEntries_st {
    pub timestamp: ::std::os::raw::c_ulonglong,
    pub timeRunTotal: ::std::os::raw::c_ulonglong,
    pub timeRun: ::std::os::raw::c_ulonglong,
    pub swRunlistId: ::std::os::raw::c_uint,
    pub targetTimeSlice: ::std::os::raw::c_ulonglong,
    pub cumulativePreemptionTime: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerLog_st {
    pub engineId: ::std::os::raw::c_uint,
    pub schedulerPolicy: ::std::os::raw::c_uint,
    pub arrMode: ::std::os::raw::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerParams_t,
    pub entriesCount: ::std::os::raw::c_uint,
    pub logEntries: [nvmlVgpuSchedulerLogEntry_t; 200usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerGetState_st {
    pub schedulerPolicy: ::std::os::raw::c_uint,
    pub arrMode: ::std::os::raw::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerParams_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuSchedulerSetParams_t__bindgen_ty_1 {
    pub avgFactor: ::std::os::raw::c_uint,
    pub frequency: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuSchedulerSetParams_t__bindgen_ty_2 {
    pub timeslice: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerSetState_st {
    pub schedulerPolicy: ::std::os::raw::c_uint,
    pub enableARRMode: ::std::os::raw::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerSetParams_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuSchedulerCapabilities_st {
    pub supportedSchedulers: [::std::os::raw::c_uint; 3usize],
    pub maxTimeslice: ::std::os::raw::c_uint,
    pub minTimeslice: ::std::os::raw::c_uint,
    pub isArrModeSupported: ::std::os::raw::c_uint,
    pub maxFrequencyForARR: ::std::os::raw::c_uint,
    pub minFrequencyForARR: ::std::os::raw::c_uint,
    pub maxAvgFactorForARR: ::std::os::raw::c_uint,
    pub minAvgFactorForARR: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuLicenseExpiry_st {
    pub year: ::std::os::raw::c_uint,
    pub month: ::std::os::raw::c_ushort,
    pub day: ::std::os::raw::c_ushort,
    pub hour: ::std::os::raw::c_ushort,
    pub min: ::std::os::raw::c_ushort,
    pub sec: ::std::os::raw::c_ushort,
    pub status: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuLicenseInfo_st {
    pub isLicensed: ::std::os::raw::c_uchar,
    pub licenseExpiry: nvmlVgpuLicenseExpiry_t,
    pub currentState: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGridLicenseExpiry_st {
    pub year: ::std::os::raw::c_uint,
    pub month: ::std::os::raw::c_ushort,
    pub day: ::std::os::raw::c_ushort,
    pub hour: ::std::os::raw::c_ushort,
    pub min: ::std::os::raw::c_ushort,
    pub sec: ::std::os::raw::c_ushort,
    pub status: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGridLicensableFeature_st {
    pub featureCode: nvmlGridLicenseFeatureCode_t,
    pub featureState: ::std::os::raw::c_uint,
    pub licenseInfo: [::std::os::raw::c_char; 128usize],
    pub productName: [::std::os::raw::c_char; 128usize],
    pub featureEnabled: ::std::os::raw::c_uint,
    pub licenseExpiry: nvmlGridLicenseExpiry_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGridLicensableFeatures_st {
    pub isGridLicenseSupported: ::std::os::raw::c_int,
    pub licensableFeaturesCount: ::std::os::raw::c_uint,
    pub gridLicensableFeatures: [nvmlGridLicensableFeature_t; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuTypeIdInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub vgpuCount: ::std::os::raw::c_uint,
    pub vgpuTypeIds: *mut nvmlVgpuTypeId_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuTypeMaxInstance_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub vgpuTypeId: nvmlVgpuTypeId_t,
    pub maxInstancePerGI: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlActiveVgpuInstanceInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub vgpuCount: ::std::os::raw::c_uint,
    pub vgpuInstances: *mut nvmlVgpuInstance_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerState_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub engineId: ::std::os::raw::c_uint,
    pub schedulerPolicy: ::std::os::raw::c_uint,
    pub enableARRMode: ::std::os::raw::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerSetParams_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerStateInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub engineId: ::std::os::raw::c_uint,
    pub schedulerPolicy: ::std::os::raw::c_uint,
    pub arrMode: ::std::os::raw::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerParams_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerLogInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub engineId: ::std::os::raw::c_uint,
    pub schedulerPolicy: ::std::os::raw::c_uint,
    pub arrMode: ::std::os::raw::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerParams_t,
    pub entriesCount: ::std::os::raw::c_uint,
    pub logEntries: [nvmlVgpuSchedulerLogEntry_t; 200usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuCreatablePlacementInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub vgpuTypeId: nvmlVgpuTypeId_t,
    pub count: ::std::os::raw::c_uint,
    pub placementIds: *mut ::std::os::raw::c_uint,
    pub placementSize: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvLinkPowerThres_st {
    pub lowPwrThreshold: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlFieldValue_st {
    pub fieldId: ::std::os::raw::c_uint,
    pub scopeId: ::std::os::raw::c_uint,
    pub timestamp: ::std::os::raw::c_longlong,
    pub latencyUsec: ::std::os::raw::c_longlong,
    pub valueType: nvmlValueType_t,
    pub nvmlReturn: nvmlReturn_t,
    pub value: nvmlValue_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlUnit_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlHwbcEntry_st {
    pub hwbcId: ::std::os::raw::c_uint,
    pub firmwareVersion: [::std::os::raw::c_char; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlLedState_st {
    pub cause: [::std::os::raw::c_char; 256usize],
    pub color: nvmlLedColor_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlUnitInfo_st {
    pub name: [::std::os::raw::c_char; 96usize],
    pub id: [::std::os::raw::c_char; 96usize],
    pub serial: [::std::os::raw::c_char; 96usize],
    pub firmwareVersion: [::std::os::raw::c_char; 96usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPSUInfo_st {
    pub state: [::std::os::raw::c_char; 256usize],
    pub current: ::std::os::raw::c_uint,
    pub voltage: ::std::os::raw::c_uint,
    pub power: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlUnitFanInfo_st {
    pub speed: ::std::os::raw::c_uint,
    pub state: nvmlFanState_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlUnitFanSpeeds_st {
    pub fans: [nvmlUnitFanInfo_t; 24usize],
    pub count: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlEventSet_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlEventData_st {
    pub device: nvmlDevice_t,
    pub eventType: ::std::os::raw::c_ulonglong,
    pub eventData: ::std::os::raw::c_ulonglong,
    pub gpuInstanceId: ::std::os::raw::c_uint,
    pub computeInstanceId: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlSystemEventSet_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlSystemEventSetCreateRequest_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub set: nvmlSystemEventSet_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlSystemEventSetFreeRequest_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub set: nvmlSystemEventSet_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlSystemRegisterEventRequest_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub eventTypes: ::std::os::raw::c_ulonglong,
    pub set: nvmlSystemEventSet_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlSystemEventData_v1_t {
    pub eventType: ::std::os::raw::c_ulonglong,
    pub gpuId: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlSystemEventSetWaitRequest_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub timeoutms: ::std::os::raw::c_uint,
    pub set: nvmlSystemEventSet_t,
    pub data: *mut nvmlSystemEventData_v1_t,
    pub dataSize: ::std::os::raw::c_uint,
    pub numEvent: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlAccountingStats_st {
    pub gpuUtilization: ::std::os::raw::c_uint,
    pub memoryUtilization: ::std::os::raw::c_uint,
    pub maxMemoryUsage: ::std::os::raw::c_ulonglong,
    pub time: ::std::os::raw::c_ulonglong,
    pub startTime: ::std::os::raw::c_ulonglong,
    pub isRunning: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 5usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlEncoderSessionInfo_st {
    pub sessionId: ::std::os::raw::c_uint,
    pub pid: ::std::os::raw::c_uint,
    pub vgpuInstance: nvmlVgpuInstance_t,
    pub codecType: nvmlEncoderType_t,
    pub hResolution: ::std::os::raw::c_uint,
    pub vResolution: ::std::os::raw::c_uint,
    pub averageFps: ::std::os::raw::c_uint,
    pub averageLatency: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlFBCStats_st {
    pub sessionsCount: ::std::os::raw::c_uint,
    pub averageFPS: ::std::os::raw::c_uint,
    pub averageLatency: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlFBCSessionInfo_st {
    pub sessionId: ::std::os::raw::c_uint,
    pub pid: ::std::os::raw::c_uint,
    pub vgpuInstance: nvmlVgpuInstance_t,
    pub displayOrdinal: ::std::os::raw::c_uint,
    pub sessionType: nvmlFBCSessionType_t,
    pub sessionFlags: ::std::os::raw::c_uint,
    pub hMaxResolution: ::std::os::raw::c_uint,
    pub vMaxResolution: ::std::os::raw::c_uint,
    pub hResolution: ::std::os::raw::c_uint,
    pub vResolution: ::std::os::raw::c_uint,
    pub averageFPS: ::std::os::raw::c_uint,
    pub averageLatency: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlConfComputeSystemCaps_st {
    pub cpuCaps: ::std::os::raw::c_uint,
    pub gpusCaps: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlConfComputeSystemState_st {
    pub environment: ::std::os::raw::c_uint,
    pub ccFeature: ::std::os::raw::c_uint,
    pub devToolsMode: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlSystemConfComputeSettings_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub environment: ::std::os::raw::c_uint,
    pub ccFeature: ::std::os::raw::c_uint,
    pub devToolsMode: ::std::os::raw::c_uint,
    pub multiGpuMode: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlConfComputeMemSizeInfo_st {
    pub protectedMemSizeKib: ::std::os::raw::c_ulonglong,
    pub unprotectedMemSizeKib: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlConfComputeGpuCertificate_st {
    pub certChainSize: ::std::os::raw::c_uint,
    pub attestationCertChainSize: ::std::os::raw::c_uint,
    pub certChain: [::std::os::raw::c_uchar; 4096usize],
    pub attestationCertChain: [::std::os::raw::c_uchar; 5120usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlConfComputeGpuAttestationReport_st {
    pub isCecAttestationReportPresent: ::std::os::raw::c_uint,
    pub attestationReportSize: ::std::os::raw::c_uint,
    pub cecAttestationReportSize: ::std::os::raw::c_uint,
    pub nonce: [::std::os::raw::c_uchar; 32usize],
    pub attestationReport: [::std::os::raw::c_uchar; 8192usize],
    pub cecAttestationReport: [::std::os::raw::c_uchar; 4096usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlConfComputeSetKeyRotationThresholdInfo_st {
    pub version: ::std::os::raw::c_uint,
    pub maxAttackerAdvantage: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlConfComputeGetKeyRotationThresholdInfo_st {
    pub version: ::std::os::raw::c_uint,
    pub attackerAdvantage: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuFabricInfo_t {
    pub clusterUuid: [::std::os::raw::c_uchar; 16usize],
    pub status: nvmlReturn_t,
    pub cliqueId: ::std::os::raw::c_uint,
    pub state: nvmlGpuFabricState_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuFabricInfo_v2_t {
    pub version: ::std::os::raw::c_uint,
    pub clusterUuid: [::std::os::raw::c_uchar; 16usize],
    pub status: nvmlReturn_t,
    pub cliqueId: ::std::os::raw::c_uint,
    pub state: nvmlGpuFabricState_t,
    pub healthMask: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuFabricInfo_v3_t {
    pub version: ::std::os::raw::c_uint,
    pub clusterUuid: [::std::os::raw::c_uchar; 16usize],
    pub status: nvmlReturn_t,
    pub cliqueId: ::std::os::raw::c_uint,
    pub state: nvmlGpuFabricState_t,
    pub healthMask: ::std::os::raw::c_uint,
    pub healthSummary: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlSystemDriverBranchInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub branch: [::std::os::raw::c_char; 80usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlTemperature_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub sensorType: nvmlTemperatureSensors_t,
    pub temperature: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvlinkSupportedBwModes_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub bwModes: [::std::os::raw::c_uchar; 23usize],
    pub totalBwModes: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvlinkGetBwMode_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub bIsBest: ::std::os::raw::c_uint,
    pub bwMode: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvlinkSetBwMode_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub bSetBest: ::std::os::raw::c_uint,
    pub bwMode: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvLinkInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub isNvleEnabled: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvlinkFirmwareVersion_t {
    pub ucodeType: ::std::os::raw::c_uchar,
    pub major: ::std::os::raw::c_uint,
    pub minor: ::std::os::raw::c_uint,
    pub subMinor: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvlinkFirmwareInfo_t {
    pub firmwareVersion: [nvmlNvlinkFirmwareVersion_t; 100usize],
    pub numValidEntries: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlNvLinkInfo_v2_t {
    pub version: ::std::os::raw::c_uint,
    pub isNvleEnabled: ::std::os::raw::c_uint,
    pub firmwareInfo: nvmlNvlinkFirmwareInfo_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuVersion_st {
    pub minVersion: ::std::os::raw::c_uint,
    pub maxVersion: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuMetadata_st {
    pub version: ::std::os::raw::c_uint,
    pub revision: ::std::os::raw::c_uint,
    pub guestInfoState: nvmlVgpuGuestInfoState_t,
    pub guestDriverVersion: [::std::os::raw::c_char; 80usize],
    pub hostDriverVersion: [::std::os::raw::c_char; 80usize],
    pub reserved: [::std::os::raw::c_uint; 6usize],
    pub vgpuVirtualizationCaps: ::std::os::raw::c_uint,
    pub guestVgpuVersion: ::std::os::raw::c_uint,
    pub opaqueDataSize: ::std::os::raw::c_uint,
    pub opaqueData: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuPgpuMetadata_st {
    pub version: ::std::os::raw::c_uint,
    pub revision: ::std::os::raw::c_uint,
    pub hostDriverVersion: [::std::os::raw::c_char; 80usize],
    pub pgpuVirtualizationCaps: ::std::os::raw::c_uint,
    pub reserved: [::std::os::raw::c_uint; 5usize],
    pub hostSupportedVgpuRange: nvmlVgpuVersion_t,
    pub opaqueDataSize: ::std::os::raw::c_uint,
    pub opaqueData: [::std::os::raw::c_char; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlVgpuPgpuCompatibility_st {
    pub vgpuVmCompatibility: nvmlVgpuVmCompatibility_t,
    pub compatibilityLimitCode: nvmlVgpuPgpuCompatibilityLimitCode_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlExcludedDeviceInfo_st {
    pub pciInfo: nvmlPciInfo_t,
    pub uuid: [::std::os::raw::c_char; 80usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlPRMTLV_v1_t {
    pub dataSize: ::std::os::raw::c_uint,
    pub status: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: nvmlPRMTLV_v1_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuInstancePlacement_st {
    pub start: ::std::os::raw::c_uint,
    pub size: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuInstanceProfileInfo_st {
    pub id: ::std::os::raw::c_uint,
    pub isP2pSupported: ::std::os::raw::c_uint,
    pub sliceCount: ::std::os::raw::c_uint,
    pub instanceCount: ::std::os::raw::c_uint,
    pub multiprocessorCount: ::std::os::raw::c_uint,
    pub copyEngineCount: ::std::os::raw::c_uint,
    pub decoderCount: ::std::os::raw::c_uint,
    pub encoderCount: ::std::os::raw::c_uint,
    pub jpegCount: ::std::os::raw::c_uint,
    pub ofaCount: ::std::os::raw::c_uint,
    pub memorySizeMB: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuInstanceProfileInfo_v2_st {
    pub version: ::std::os::raw::c_uint,
    pub id: ::std::os::raw::c_uint,
    pub isP2pSupported: ::std::os::raw::c_uint,
    pub sliceCount: ::std::os::raw::c_uint,
    pub instanceCount: ::std::os::raw::c_uint,
    pub multiprocessorCount: ::std::os::raw::c_uint,
    pub copyEngineCount: ::std::os::raw::c_uint,
    pub decoderCount: ::std::os::raw::c_uint,
    pub encoderCount: ::std::os::raw::c_uint,
    pub jpegCount: ::std::os::raw::c_uint,
    pub ofaCount: ::std::os::raw::c_uint,
    pub memorySizeMB: ::std::os::raw::c_ulonglong,
    pub name: [::std::os::raw::c_char; 96usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuInstanceProfileInfo_v3_st {
    pub version: ::std::os::raw::c_uint,
    pub id: ::std::os::raw::c_uint,
    pub sliceCount: ::std::os::raw::c_uint,
    pub instanceCount: ::std::os::raw::c_uint,
    pub multiprocessorCount: ::std::os::raw::c_uint,
    pub copyEngineCount: ::std::os::raw::c_uint,
    pub decoderCount: ::std::os::raw::c_uint,
    pub encoderCount: ::std::os::raw::c_uint,
    pub jpegCount: ::std::os::raw::c_uint,
    pub ofaCount: ::std::os::raw::c_uint,
    pub memorySizeMB: ::std::os::raw::c_ulonglong,
    pub name: [::std::os::raw::c_char; 96usize],
    pub capabilities: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuInstanceInfo_st {
    pub device: nvmlDevice_t,
    pub id: ::std::os::raw::c_uint,
    pub profileId: ::std::os::raw::c_uint,
    pub placement: nvmlGpuInstancePlacement_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlComputeInstancePlacement_st {
    pub start: ::std::os::raw::c_uint,
    pub size: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlComputeInstanceProfileInfo_st {
    pub id: ::std::os::raw::c_uint,
    pub sliceCount: ::std::os::raw::c_uint,
    pub instanceCount: ::std::os::raw::c_uint,
    pub multiprocessorCount: ::std::os::raw::c_uint,
    pub sharedCopyEngineCount: ::std::os::raw::c_uint,
    pub sharedDecoderCount: ::std::os::raw::c_uint,
    pub sharedEncoderCount: ::std::os::raw::c_uint,
    pub sharedJpegCount: ::std::os::raw::c_uint,
    pub sharedOfaCount: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlComputeInstanceProfileInfo_v2_st {
    pub version: ::std::os::raw::c_uint,
    pub id: ::std::os::raw::c_uint,
    pub sliceCount: ::std::os::raw::c_uint,
    pub instanceCount: ::std::os::raw::c_uint,
    pub multiprocessorCount: ::std::os::raw::c_uint,
    pub sharedCopyEngineCount: ::std::os::raw::c_uint,
    pub sharedDecoderCount: ::std::os::raw::c_uint,
    pub sharedEncoderCount: ::std::os::raw::c_uint,
    pub sharedJpegCount: ::std::os::raw::c_uint,
    pub sharedOfaCount: ::std::os::raw::c_uint,
    pub name: [::std::os::raw::c_char; 96usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlComputeInstanceProfileInfo_v3_st {
    pub version: ::std::os::raw::c_uint,
    pub id: ::std::os::raw::c_uint,
    pub sliceCount: ::std::os::raw::c_uint,
    pub instanceCount: ::std::os::raw::c_uint,
    pub multiprocessorCount: ::std::os::raw::c_uint,
    pub sharedCopyEngineCount: ::std::os::raw::c_uint,
    pub sharedDecoderCount: ::std::os::raw::c_uint,
    pub sharedEncoderCount: ::std::os::raw::c_uint,
    pub sharedJpegCount: ::std::os::raw::c_uint,
    pub sharedOfaCount: ::std::os::raw::c_uint,
    pub name: [::std::os::raw::c_char; 96usize],
    pub capabilities: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlComputeInstanceInfo_st {
    pub device: nvmlDevice_t,
    pub gpuInstance: nvmlGpuInstance_t,
    pub id: ::std::os::raw::c_uint,
    pub profileId: ::std::os::raw::c_uint,
    pub placement: nvmlComputeInstancePlacement_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlComputeInstance_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpmSample_st {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpmMetric_t {
    pub metricId: ::std::os::raw::c_uint,
    pub nvmlReturn: nvmlReturn_t,
    pub value: f64,
    pub metricInfo: nvmlGpmMetric_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpmMetric_t__bindgen_ty_1 {
    pub shortName: *mut ::std::os::raw::c_char,
    pub longName: *mut ::std::os::raw::c_char,
    pub unit: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpmMetricsGet_t {
    pub version: ::std::os::raw::c_uint,
    pub numMetrics: ::std::os::raw::c_uint,
    pub sample1: nvmlGpmSample_t,
    pub sample2: nvmlGpmSample_t,
    pub metrics: [nvmlGpmMetric_t; 210usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpmSupport_t {
    pub version: ::std::os::raw::c_uint,
    pub isSupportedDevice: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDeviceCapabilities_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub capMask: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlMask255_t {
    pub mask: [::std::os::raw::c_uint; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlWorkloadPowerProfileInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub profileId: ::std::os::raw::c_uint,
    pub priority: ::std::os::raw::c_uint,
    pub conflictingMask: nvmlMask255_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlWorkloadPowerProfileProfilesInfo_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub perfProfilesMask: nvmlMask255_t,
    pub perfProfile: [nvmlWorkloadPowerProfileInfo_t; 255usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlWorkloadPowerProfileCurrentProfiles_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub perfProfilesMask: nvmlMask255_t,
    pub requestedProfilesMask: nvmlMask255_t,
    pub enforcedProfilesMask: nvmlMask255_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlWorkloadPowerProfileRequestedProfiles_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub requestedProfilesMask: nvmlMask255_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPowerSmoothingProfile_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub profileId: ::std::os::raw::c_uint,
    pub paramId: ::std::os::raw::c_uint,
    pub value: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlPowerSmoothingState_v1_t {
    pub version: ::std::os::raw::c_uint,
    pub state: nvmlEnableState_t,
}
pub const NVML_API_VERSION: u32 = 13;
pub const NVML_API_VERSION_STR: &[u8; 3] = b"13\0";
pub const NVML_VALUE_NOT_AVAILABLE: i32 = -1;
pub const NVML_DEVICE_PCI_BUS_ID_BUFFER_SIZE: u32 = 32;
pub const NVML_DEVICE_PCI_BUS_ID_BUFFER_V2_SIZE: u32 = 16;
pub const NVML_DEVICE_PCI_BUS_ID_LEGACY_FMT: &[u8; 17] = b"%04X:%02X:%02X.0\0";
pub const NVML_DEVICE_PCI_BUS_ID_FMT: &[u8; 17] = b"%08X:%02X:%02X.0\0";
pub const NVML_NVLINK_MAX_LINKS: u32 = 18;
pub const NVML_MAX_PHYSICAL_BRIDGE: u32 = 128;
pub const NVML_MAX_THERMAL_SENSORS_PER_GPU: u32 = 3;
pub const NVML_DEVICE_UUID_ASCII_LEN: u32 = 41;
pub const NVML_DEVICE_UUID_BINARY_LEN: u32 = 16;
pub const nvmlFlagDefault: u32 = 0;
pub const nvmlFlagForce: u32 = 1;
pub const MAX_CLK_DOMAINS: u32 = 32;
pub const NVML_MAX_GPU_PERF_PSTATES: u32 = 16;
pub const NVML_PERF_MODES_BUFFER_SIZE: u32 = 2048;
pub const NVML_POWER_MIZER_MODE_ADAPTIVE: u32 = 0;
pub const NVML_POWER_MIZER_MODE_PREFER_MAXIMUM_PERFORMANCE: u32 = 1;
pub const NVML_POWER_MIZER_MODE_AUTO: u32 = 2;
pub const NVML_POWER_MIZER_MODE_PREFER_CONSISTENT_PERFORMANCE: u32 = 3;
pub const NVML_GSP_FIRMWARE_VERSION_BUF_SIZE: u32 = 64;
pub const NVML_DEVICE_ARCH_KEPLER: u32 = 2;
pub const NVML_DEVICE_ARCH_MAXWELL: u32 = 3;
pub const NVML_DEVICE_ARCH_PASCAL: u32 = 4;
pub const NVML_DEVICE_ARCH_VOLTA: u32 = 5;
pub const NVML_DEVICE_ARCH_TURING: u32 = 6;
pub const NVML_DEVICE_ARCH_AMPERE: u32 = 7;
pub const NVML_DEVICE_ARCH_ADA: u32 = 8;
pub const NVML_DEVICE_ARCH_HOPPER: u32 = 9;
pub const NVML_DEVICE_ARCH_BLACKWELL: u32 = 10;
pub const NVML_DEVICE_ARCH_UNKNOWN: u32 = 4294967295;
pub const NVML_BUS_TYPE_UNKNOWN: u32 = 0;
pub const NVML_BUS_TYPE_PCI: u32 = 1;
pub const NVML_BUS_TYPE_PCIE: u32 = 2;
pub const NVML_BUS_TYPE_FPCI: u32 = 3;
pub const NVML_BUS_TYPE_AGP: u32 = 4;
pub const NVML_FAN_POLICY_TEMPERATURE_CONTINOUS_SW: u32 = 0;
pub const NVML_FAN_POLICY_MANUAL: u32 = 1;
pub const NVML_POWER_SOURCE_AC: u32 = 0;
pub const NVML_POWER_SOURCE_BATTERY: u32 = 1;
pub const NVML_POWER_SOURCE_UNDERSIZED: u32 = 2;
pub const NVML_PCIE_LINK_MAX_SPEED_INVALID: u32 = 0;
pub const NVML_PCIE_LINK_MAX_SPEED_2500MBPS: u32 = 1;
pub const NVML_PCIE_LINK_MAX_SPEED_5000MBPS: u32 = 2;
pub const NVML_PCIE_LINK_MAX_SPEED_8000MBPS: u32 = 3;
pub const NVML_PCIE_LINK_MAX_SPEED_16000MBPS: u32 = 4;
pub const NVML_PCIE_LINK_MAX_SPEED_32000MBPS: u32 = 5;
pub const NVML_PCIE_LINK_MAX_SPEED_64000MBPS: u32 = 6;
pub const NVML_ADAPTIVE_CLOCKING_INFO_STATUS_DISABLED: u32 = 0;
pub const NVML_ADAPTIVE_CLOCKING_INFO_STATUS_ENABLED: u32 = 1;
pub const NVML_MAX_GPU_UTILIZATIONS: u32 = 8;
pub const NVML_PCIE_ATOMICS_CAP_FETCHADD32: u32 = 1;
pub const NVML_PCIE_ATOMICS_CAP_FETCHADD64: u32 = 2;
pub const NVML_PCIE_ATOMICS_CAP_SWAP32: u32 = 4;
pub const NVML_PCIE_ATOMICS_CAP_SWAP64: u32 = 8;
pub const NVML_PCIE_ATOMICS_CAP_CAS32: u32 = 16;
pub const NVML_PCIE_ATOMICS_CAP_CAS64: u32 = 32;
pub const NVML_PCIE_ATOMICS_CAP_CAS128: u32 = 64;
pub const NVML_PCIE_ATOMICS_OPS_MAX: u32 = 7;
pub const NVML_POWER_SCOPE_GPU: u32 = 0;
pub const NVML_POWER_SCOPE_MODULE: u32 = 1;
pub const NVML_POWER_SCOPE_MEMORY: u32 = 2;
pub const NVML_GRID_LICENSE_EXPIRY_NOT_AVAILABLE: u32 = 0;
pub const NVML_GRID_LICENSE_EXPIRY_INVALID: u32 = 1;
pub const NVML_GRID_LICENSE_EXPIRY_VALID: u32 = 2;
pub const NVML_GRID_LICENSE_EXPIRY_NOT_APPLICABLE: u32 = 3;
pub const NVML_GRID_LICENSE_EXPIRY_PERMANENT: u32 = 4;
pub const NVML_GRID_LICENSE_BUFFER_SIZE: u32 = 128;
pub const NVML_VGPU_NAME_BUFFER_SIZE: u32 = 64;
pub const NVML_GRID_LICENSE_FEATURE_MAX_COUNT: u32 = 3;
pub const INVALID_GPU_INSTANCE_PROFILE_ID: u32 = 4294967295;
pub const INVALID_GPU_INSTANCE_ID: u32 = 4294967295;
pub const NVML_INVALID_VGPU_PLACEMENT_ID: u32 = 65535;
pub const NVML_VGPU_VIRTUALIZATION_CAP_MIGRATION_NO: u32 = 0;
pub const NVML_VGPU_VIRTUALIZATION_CAP_MIGRATION_YES: u32 = 1;
pub const NVML_VGPU_PGPU_VIRTUALIZATION_CAP_MIGRATION_NO: u32 = 0;
pub const NVML_VGPU_PGPU_VIRTUALIZATION_CAP_MIGRATION_YES: u32 = 1;
pub const NVML_VGPU_PGPU_HETEROGENEOUS_MODE: u32 = 0;
pub const NVML_VGPU_PGPU_HOMOGENEOUS_MODE: u32 = 1;
pub const NVML_VGPU_SCHEDULER_POLICY_UNKNOWN: u32 = 0;
pub const NVML_VGPU_SCHEDULER_POLICY_BEST_EFFORT: u32 = 1;
pub const NVML_VGPU_SCHEDULER_POLICY_EQUAL_SHARE: u32 = 2;
pub const NVML_VGPU_SCHEDULER_POLICY_FIXED_SHARE: u32 = 3;
pub const NVML_SUPPORTED_VGPU_SCHEDULER_POLICY_COUNT: u32 = 3;
pub const NVML_SCHEDULER_SW_MAX_LOG_ENTRIES: u32 = 200;
pub const NVML_VGPU_SCHEDULER_ARR_DEFAULT: u32 = 0;
pub const NVML_VGPU_SCHEDULER_ARR_DISABLE: u32 = 1;
pub const NVML_VGPU_SCHEDULER_ARR_ENABLE: u32 = 2;
pub const NVML_VGPU_SCHEDULER_ENGINE_TYPE_GRAPHICS: u32 = 1;
pub const NVML_GRID_LICENSE_STATE_UNKNOWN: u32 = 0;
pub const NVML_GRID_LICENSE_STATE_UNINITIALIZED: u32 = 1;
pub const NVML_GRID_LICENSE_STATE_UNLICENSED_UNRESTRICTED: u32 = 2;
pub const NVML_GRID_LICENSE_STATE_UNLICENSED_RESTRICTED: u32 = 3;
pub const NVML_GRID_LICENSE_STATE_UNLICENSED: u32 = 4;
pub const NVML_GRID_LICENSE_STATE_LICENSED: u32 = 5;
pub const NVML_FI_DEV_ECC_CURRENT: u32 = 1;
pub const NVML_FI_DEV_ECC_PENDING: u32 = 2;
pub const NVML_FI_DEV_ECC_SBE_VOL_TOTAL: u32 = 3;
pub const NVML_FI_DEV_ECC_DBE_VOL_TOTAL: u32 = 4;
pub const NVML_FI_DEV_ECC_SBE_AGG_TOTAL: u32 = 5;
pub const NVML_FI_DEV_ECC_DBE_AGG_TOTAL: u32 = 6;
pub const NVML_FI_DEV_ECC_SBE_VOL_L1: u32 = 7;
pub const NVML_FI_DEV_ECC_DBE_VOL_L1: u32 = 8;
pub const NVML_FI_DEV_ECC_SBE_VOL_L2: u32 = 9;
pub const NVML_FI_DEV_ECC_DBE_VOL_L2: u32 = 10;
pub const NVML_FI_DEV_ECC_SBE_VOL_DEV: u32 = 11;
pub const NVML_FI_DEV_ECC_DBE_VOL_DEV: u32 = 12;
pub const NVML_FI_DEV_ECC_SBE_VOL_REG: u32 = 13;
pub const NVML_FI_DEV_ECC_DBE_VOL_REG: u32 = 14;
pub const NVML_FI_DEV_ECC_SBE_VOL_TEX: u32 = 15;
pub const NVML_FI_DEV_ECC_DBE_VOL_TEX: u32 = 16;
pub const NVML_FI_DEV_ECC_DBE_VOL_CBU: u32 = 17;
pub const NVML_FI_DEV_ECC_SBE_AGG_L1: u32 = 18;
pub const NVML_FI_DEV_ECC_DBE_AGG_L1: u32 = 19;
pub const NVML_FI_DEV_ECC_SBE_AGG_L2: u32 = 20;
pub const NVML_FI_DEV_ECC_DBE_AGG_L2: u32 = 21;
pub const NVML_FI_DEV_ECC_SBE_AGG_DEV: u32 = 22;
pub const NVML_FI_DEV_ECC_DBE_AGG_DEV: u32 = 23;
pub const NVML_FI_DEV_ECC_SBE_AGG_REG: u32 = 24;
pub const NVML_FI_DEV_ECC_DBE_AGG_REG: u32 = 25;
pub const NVML_FI_DEV_ECC_SBE_AGG_TEX: u32 = 26;
pub const NVML_FI_DEV_ECC_DBE_AGG_TEX: u32 = 27;
pub const NVML_FI_DEV_ECC_DBE_AGG_CBU: u32 = 28;
pub const NVML_FI_DEV_RETIRED_SBE: u32 = 29;
pub const NVML_FI_DEV_RETIRED_DBE: u32 = 30;
pub const NVML_FI_DEV_RETIRED_PENDING: u32 = 31;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L0: u32 = 32;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L1: u32 = 33;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L2: u32 = 34;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L3: u32 = 35;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L4: u32 = 36;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L5: u32 = 37;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_TOTAL: u32 = 38;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L0: u32 = 39;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L1: u32 = 40;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L2: u32 = 41;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L3: u32 = 42;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L4: u32 = 43;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L5: u32 = 44;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_TOTAL: u32 = 45;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L0: u32 = 46;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L1: u32 = 47;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L2: u32 = 48;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L3: u32 = 49;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L4: u32 = 50;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L5: u32 = 51;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_TOTAL: u32 = 52;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L0: u32 = 53;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L1: u32 = 54;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L2: u32 = 55;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L3: u32 = 56;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L4: u32 = 57;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L5: u32 = 58;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_TOTAL: u32 = 59;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L0: u32 = 60;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L1: u32 = 61;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L2: u32 = 62;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L3: u32 = 63;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L4: u32 = 64;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L5: u32 = 65;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_TOTAL: u32 = 66;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L0: u32 = 67;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L1: u32 = 68;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L2: u32 = 69;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L3: u32 = 70;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L4: u32 = 71;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L5: u32 = 72;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_TOTAL: u32 = 73;
pub const NVML_FI_DEV_PERF_POLICY_POWER: u32 = 74;
pub const NVML_FI_DEV_PERF_POLICY_THERMAL: u32 = 75;
pub const NVML_FI_DEV_PERF_POLICY_SYNC_BOOST: u32 = 76;
pub const NVML_FI_DEV_PERF_POLICY_BOARD_LIMIT: u32 = 77;
pub const NVML_FI_DEV_PERF_POLICY_LOW_UTILIZATION: u32 = 78;
pub const NVML_FI_DEV_PERF_POLICY_RELIABILITY: u32 = 79;
pub const NVML_FI_DEV_PERF_POLICY_TOTAL_APP_CLOCKS: u32 = 80;
pub const NVML_FI_DEV_PERF_POLICY_TOTAL_BASE_CLOCKS: u32 = 81;
pub const NVML_FI_DEV_MEMORY_TEMP: u32 = 82;
pub const NVML_FI_DEV_TOTAL_ENERGY_CONSUMPTION: u32 = 83;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L0: u32 = 84;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L1: u32 = 85;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L2: u32 = 86;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L3: u32 = 87;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L4: u32 = 88;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L5: u32 = 89;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_COMMON: u32 = 90;
pub const NVML_FI_DEV_NVLINK_LINK_COUNT: u32 = 91;
pub const NVML_FI_DEV_RETIRED_PENDING_SBE: u32 = 92;
pub const NVML_FI_DEV_RETIRED_PENDING_DBE: u32 = 93;
pub const NVML_FI_DEV_PCIE_REPLAY_COUNTER: u32 = 94;
pub const NVML_FI_DEV_PCIE_REPLAY_ROLLOVER_COUNTER: u32 = 95;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L6: u32 = 96;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L7: u32 = 97;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L8: u32 = 98;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L9: u32 = 99;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L10: u32 = 100;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L11: u32 = 101;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L6: u32 = 102;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L7: u32 = 103;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L8: u32 = 104;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L9: u32 = 105;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L10: u32 = 106;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L11: u32 = 107;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L6: u32 = 108;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L7: u32 = 109;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L8: u32 = 110;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L9: u32 = 111;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L10: u32 = 112;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L11: u32 = 113;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L6: u32 = 114;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L7: u32 = 115;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L8: u32 = 116;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L9: u32 = 117;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L10: u32 = 118;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L11: u32 = 119;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L6: u32 = 120;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L7: u32 = 121;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L8: u32 = 122;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L9: u32 = 123;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L10: u32 = 124;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L11: u32 = 125;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L6: u32 = 126;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L7: u32 = 127;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L8: u32 = 128;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L9: u32 = 129;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L10: u32 = 130;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L11: u32 = 131;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L6: u32 = 132;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L7: u32 = 133;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L8: u32 = 134;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L9: u32 = 135;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L10: u32 = 136;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L11: u32 = 137;
pub const NVML_FI_DEV_NVLINK_THROUGHPUT_DATA_TX: u32 = 138;
pub const NVML_FI_DEV_NVLINK_THROUGHPUT_DATA_RX: u32 = 139;
pub const NVML_FI_DEV_NVLINK_THROUGHPUT_RAW_TX: u32 = 140;
pub const NVML_FI_DEV_NVLINK_THROUGHPUT_RAW_RX: u32 = 141;
pub const NVML_FI_DEV_REMAPPED_COR: u32 = 142;
pub const NVML_FI_DEV_REMAPPED_UNC: u32 = 143;
pub const NVML_FI_DEV_REMAPPED_PENDING: u32 = 144;
pub const NVML_FI_DEV_REMAPPED_FAILURE: u32 = 145;
pub const NVML_FI_DEV_NVLINK_REMOTE_NVLINK_ID: u32 = 146;
pub const NVML_FI_DEV_NVSWITCH_CONNECTED_LINK_COUNT: u32 = 147;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L0: u32 = 148;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L1: u32 = 149;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L2: u32 = 150;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L3: u32 = 151;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L4: u32 = 152;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L5: u32 = 153;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L6: u32 = 154;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L7: u32 = 155;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L8: u32 = 156;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L9: u32 = 157;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L10: u32 = 158;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L11: u32 = 159;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_TOTAL: u32 = 160;
pub const NVML_FI_DEV_NVLINK_ERROR_DL_REPLAY: u32 = 161;
pub const NVML_FI_DEV_NVLINK_ERROR_DL_RECOVERY: u32 = 162;
pub const NVML_FI_DEV_NVLINK_ERROR_DL_CRC: u32 = 163;
pub const NVML_FI_DEV_NVLINK_GET_SPEED: u32 = 164;
pub const NVML_FI_DEV_NVLINK_GET_STATE: u32 = 165;
pub const NVML_FI_DEV_NVLINK_GET_VERSION: u32 = 166;
pub const NVML_FI_DEV_NVLINK_GET_POWER_STATE: u32 = 167;
pub const NVML_FI_DEV_NVLINK_GET_POWER_THRESHOLD: u32 = 168;
pub const NVML_FI_DEV_PCIE_L0_TO_RECOVERY_COUNTER: u32 = 169;
pub const NVML_FI_DEV_C2C_LINK_COUNT: u32 = 170;
pub const NVML_FI_DEV_C2C_LINK_GET_STATUS: u32 = 171;
pub const NVML_FI_DEV_C2C_LINK_GET_MAX_BW: u32 = 172;
pub const NVML_FI_DEV_PCIE_COUNT_CORRECTABLE_ERRORS: u32 = 173;
pub const NVML_FI_DEV_PCIE_COUNT_NAKS_RECEIVED: u32 = 174;
pub const NVML_FI_DEV_PCIE_COUNT_RECEIVER_ERROR: u32 = 175;
pub const NVML_FI_DEV_PCIE_COUNT_BAD_TLP: u32 = 176;
pub const NVML_FI_DEV_PCIE_COUNT_NAKS_SENT: u32 = 177;
pub const NVML_FI_DEV_PCIE_COUNT_BAD_DLLP: u32 = 178;
pub const NVML_FI_DEV_PCIE_COUNT_NON_FATAL_ERROR: u32 = 179;
pub const NVML_FI_DEV_PCIE_COUNT_FATAL_ERROR: u32 = 180;
pub const NVML_FI_DEV_PCIE_COUNT_UNSUPPORTED_REQ: u32 = 181;
pub const NVML_FI_DEV_PCIE_COUNT_LCRC_ERROR: u32 = 182;
pub const NVML_FI_DEV_PCIE_COUNT_LANE_ERROR: u32 = 183;
pub const NVML_FI_DEV_IS_RESETLESS_MIG_SUPPORTED: u32 = 184;
pub const NVML_FI_DEV_POWER_AVERAGE: u32 = 185;
pub const NVML_FI_DEV_POWER_INSTANT: u32 = 186;
pub const NVML_FI_DEV_POWER_MIN_LIMIT: u32 = 187;
pub const NVML_FI_DEV_POWER_MAX_LIMIT: u32 = 188;
pub const NVML_FI_DEV_POWER_DEFAULT_LIMIT: u32 = 189;
pub const NVML_FI_DEV_POWER_CURRENT_LIMIT: u32 = 190;
pub const NVML_FI_DEV_ENERGY: u32 = 191;
pub const NVML_FI_DEV_POWER_REQUESTED_LIMIT: u32 = 192;
pub const NVML_FI_DEV_TEMPERATURE_SHUTDOWN_TLIMIT: u32 = 193;
pub const NVML_FI_DEV_TEMPERATURE_SLOWDOWN_TLIMIT: u32 = 194;
pub const NVML_FI_DEV_TEMPERATURE_MEM_MAX_TLIMIT: u32 = 195;
pub const NVML_FI_DEV_TEMPERATURE_GPU_MAX_TLIMIT: u32 = 196;
pub const NVML_FI_DEV_PCIE_COUNT_TX_BYTES: u32 = 197;
pub const NVML_FI_DEV_PCIE_COUNT_RX_BYTES: u32 = 198;
pub const NVML_FI_DEV_IS_MIG_MODE_INDEPENDENT_MIG_QUERY_CAPABLE: u32 = 199;
pub const NVML_FI_DEV_NVLINK_GET_POWER_THRESHOLD_MAX: u32 = 200;
pub const NVML_FI_DEV_NVLINK_COUNT_XMIT_PACKETS: u32 = 201;
pub const NVML_FI_DEV_NVLINK_COUNT_XMIT_BYTES: u32 = 202;
pub const NVML_FI_DEV_NVLINK_COUNT_RCV_PACKETS: u32 = 203;
pub const NVML_FI_DEV_NVLINK_COUNT_RCV_BYTES: u32 = 204;
pub const NVML_FI_DEV_NVLINK_COUNT_VL15_DROPPED: u32 = 205;
pub const NVML_FI_DEV_NVLINK_COUNT_MALFORMED_PACKET_ERRORS: u32 = 206;
pub const NVML_FI_DEV_NVLINK_COUNT_BUFFER_OVERRUN_ERRORS: u32 = 207;
pub const NVML_FI_DEV_NVLINK_COUNT_RCV_ERRORS: u32 = 208;
pub const NVML_FI_DEV_NVLINK_COUNT_RCV_REMOTE_ERRORS: u32 = 209;
pub const NVML_FI_DEV_NVLINK_COUNT_RCV_GENERAL_ERRORS: u32 = 210;
pub const NVML_FI_DEV_NVLINK_COUNT_LOCAL_LINK_INTEGRITY_ERRORS: u32 = 211;
pub const NVML_FI_DEV_NVLINK_COUNT_XMIT_DISCARDS: u32 = 212;
pub const NVML_FI_DEV_NVLINK_COUNT_LINK_RECOVERY_SUCCESSFUL_EVENTS: u32 = 213;
pub const NVML_FI_DEV_NVLINK_COUNT_LINK_RECOVERY_FAILED_EVENTS: u32 = 214;
pub const NVML_FI_DEV_NVLINK_COUNT_LINK_RECOVERY_EVENTS: u32 = 215;
pub const NVML_FI_DEV_NVLINK_COUNT_RAW_BER_LANE0: u32 = 216;
pub const NVML_FI_DEV_NVLINK_COUNT_RAW_BER_LANE1: u32 = 217;
pub const NVML_FI_DEV_NVLINK_COUNT_RAW_BER: u32 = 218;
pub const NVML_FI_DEV_NVLINK_COUNT_EFFECTIVE_ERRORS: u32 = 219;
pub const NVML_FI_DEV_NVLINK_COUNT_EFFECTIVE_BER: u32 = 220;
pub const NVML_FI_DEV_NVLINK_COUNT_SYMBOL_ERRORS: u32 = 221;
pub const NVML_FI_DEV_NVLINK_COUNT_SYMBOL_BER: u32 = 222;
pub const NVML_FI_DEV_NVLINK_GET_POWER_THRESHOLD_MIN: u32 = 223;
pub const NVML_FI_DEV_NVLINK_GET_POWER_THRESHOLD_UNITS: u32 = 224;
pub const NVML_FI_DEV_NVLINK_GET_POWER_THRESHOLD_SUPPORTED: u32 = 225;
pub const NVML_FI_DEV_RESET_STATUS: u32 = 226;
pub const NVML_FI_DEV_DRAIN_AND_RESET_STATUS: u32 = 227;
pub const NVML_FI_DEV_PCIE_OUTBOUND_ATOMICS_MASK: u32 = 228;
pub const NVML_FI_DEV_PCIE_INBOUND_ATOMICS_MASK: u32 = 229;
pub const NVML_FI_DEV_GET_GPU_RECOVERY_ACTION: u32 = 230;
pub const NVML_FI_DEV_C2C_LINK_ERROR_INTR: u32 = 231;
pub const NVML_FI_DEV_C2C_LINK_ERROR_REPLAY: u32 = 232;
pub const NVML_FI_DEV_C2C_LINK_ERROR_REPLAY_B2B: u32 = 233;
pub const NVML_FI_DEV_C2C_LINK_POWER_STATE: u32 = 234;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_0: u32 = 235;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_1: u32 = 236;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_2: u32 = 237;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_3: u32 = 238;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_4: u32 = 239;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_5: u32 = 240;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_6: u32 = 241;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_7: u32 = 242;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_8: u32 = 243;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_9: u32 = 244;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_10: u32 = 245;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_11: u32 = 246;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_12: u32 = 247;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_13: u32 = 248;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_14: u32 = 249;
pub const NVML_FI_DEV_NVLINK_COUNT_FEC_HISTORY_15: u32 = 250;
pub const NVML_FI_DEV_CLOCKS_EVENT_REASON_SW_POWER_CAP: u32 = 74;
pub const NVML_FI_DEV_CLOCKS_EVENT_REASON_SYNC_BOOST: u32 = 76;
pub const NVML_FI_DEV_CLOCKS_EVENT_REASON_SW_THERM_SLOWDOWN: u32 = 251;
pub const NVML_FI_DEV_CLOCKS_EVENT_REASON_HW_THERM_SLOWDOWN: u32 = 252;
pub const NVML_FI_DEV_CLOCKS_EVENT_REASON_HW_POWER_BRAKE_SLOWDOWN: u32 = 253;
pub const NVML_FI_DEV_POWER_SYNC_BALANCING_FREQ: u32 = 254;
pub const NVML_FI_DEV_POWER_SYNC_BALANCING_AF: u32 = 255;
pub const NVML_FI_PWR_SMOOTHING_ENABLED: u32 = 256;
pub const NVML_FI_PWR_SMOOTHING_PRIV_LVL: u32 = 257;
pub const NVML_FI_PWR_SMOOTHING_IMM_RAMP_DOWN_ENABLED: u32 = 258;
pub const NVML_FI_PWR_SMOOTHING_APPLIED_TMP_CEIL: u32 = 259;
pub const NVML_FI_PWR_SMOOTHING_APPLIED_TMP_FLOOR: u32 = 260;
pub const NVML_FI_PWR_SMOOTHING_MAX_PERCENT_TMP_FLOOR_SETTING: u32 = 261;
pub const NVML_FI_PWR_SMOOTHING_MIN_PERCENT_TMP_FLOOR_SETTING: u32 = 262;
pub const NVML_FI_PWR_SMOOTHING_HW_CIRCUITRY_PERCENT_LIFETIME_REMAINING: u32 = 263;
pub const NVML_FI_PWR_SMOOTHING_MAX_NUM_PRESET_PROFILES: u32 = 264;
pub const NVML_FI_PWR_SMOOTHING_PROFILE_PERCENT_TMP_FLOOR: u32 = 265;
pub const NVML_FI_PWR_SMOOTHING_PROFILE_RAMP_UP_RATE: u32 = 266;
pub const NVML_FI_PWR_SMOOTHING_PROFILE_RAMP_DOWN_RATE: u32 = 267;
pub const NVML_FI_PWR_SMOOTHING_PROFILE_RAMP_DOWN_HYST_VAL: u32 = 268;
pub const NVML_FI_PWR_SMOOTHING_ACTIVE_PRESET_PROFILE: u32 = 269;
pub const NVML_FI_PWR_SMOOTHING_ADMIN_OVERRIDE_PERCENT_TMP_FLOOR: u32 = 270;
pub const NVML_FI_PWR_SMOOTHING_ADMIN_OVERRIDE_RAMP_UP_RATE: u32 = 271;
pub const NVML_FI_PWR_SMOOTHING_ADMIN_OVERRIDE_RAMP_DOWN_RATE: u32 = 272;
pub const NVML_FI_PWR_SMOOTHING_ADMIN_OVERRIDE_RAMP_DOWN_HYST_VAL: u32 = 273;
pub const NVML_FI_MAX: u32 = 274;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_UNIT_100US: u32 = 0;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_UNIT_50US: u32 = 1;
pub const NVML_NVLINK_POWER_STATE_HIGH_SPEED: u32 = 0;
pub const NVML_NVLINK_POWER_STATE_LOW: u32 = 1;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_MIN: u32 = 1;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_MAX: u32 = 8191;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_RESET: u32 = 4294967295;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_DEFAULT: u32 = 4294967295;
pub const NVML_C2C_POWER_STATE_FULL_POWER: u32 = 0;
pub const NVML_C2C_POWER_STATE_LOW_POWER: u32 = 1;
pub const nvmlEventTypeNone: u32 = 0;
pub const nvmlEventTypeSingleBitEccError: u32 = 1;
pub const nvmlEventTypeDoubleBitEccError: u32 = 2;
pub const nvmlEventTypePState: u32 = 4;
pub const nvmlEventTypeXidCriticalError: u32 = 8;
pub const nvmlEventTypeClock: u32 = 16;
pub const nvmlEventTypePowerSourceChange: u32 = 128;
pub const nvmlEventMigConfigChange: u32 = 256;
pub const nvmlEventTypeSingleBitEccErrorStorm: u32 = 512;
pub const nvmlEventTypeDramRetirementEvent: u32 = 1024;
pub const nvmlEventTypeDramRetirementFailure: u32 = 2048;
pub const nvmlEventTypeNonFatalPoisonError: u32 = 4096;
pub const nvmlEventTypeFatalPoisonError: u32 = 8192;
pub const nvmlEventTypeGpuUnavailableError: u32 = 16384;
pub const nvmlEventTypeGpuRecoveryAction: u32 = 32768;
pub const nvmlEventTypeAll: u32 = 65439;
pub const nvmlSystemEventTypeGpuDriverUnbind: u32 = 1;
pub const nvmlSystemEventTypeGpuDriverBind: u32 = 2;
pub const nvmlSystemEventTypeCount: u32 = 2;
pub const nvmlClocksEventReasonGpuIdle: u32 = 1;
pub const nvmlClocksEventReasonApplicationsClocksSetting: u32 = 2;
pub const nvmlClocksThrottleReasonUserDefinedClocks: u32 = 2;
pub const nvmlClocksEventReasonSwPowerCap: u32 = 4;
pub const nvmlClocksThrottleReasonHwSlowdown: u32 = 8;
pub const nvmlClocksEventReasonSyncBoost: u32 = 16;
pub const nvmlClocksEventReasonSwThermalSlowdown: u32 = 32;
pub const nvmlClocksThrottleReasonHwThermalSlowdown: u32 = 64;
pub const nvmlClocksThrottleReasonHwPowerBrakeSlowdown: u32 = 128;
pub const nvmlClocksEventReasonDisplayClockSetting: u32 = 256;
pub const nvmlClocksEventReasonNone: u32 = 0;
pub const nvmlClocksThrottleReasonGpuIdle: u32 = 1;
pub const nvmlClocksThrottleReasonApplicationsClocksSetting: u32 = 2;
pub const nvmlClocksThrottleReasonSyncBoost: u32 = 16;
pub const nvmlClocksThrottleReasonSwPowerCap: u32 = 4;
pub const nvmlClocksThrottleReasonSwThermalSlowdown: u32 = 32;
pub const nvmlClocksThrottleReasonDisplayClockSetting: u32 = 256;
pub const nvmlClocksThrottleReasonNone: u32 = 0;
pub const NVML_NVFBC_SESSION_FLAG_DIFFMAP_ENABLED: u32 = 1;
pub const NVML_NVFBC_SESSION_FLAG_CLASSIFICATIONMAP_ENABLED: u32 = 2;
pub const NVML_NVFBC_SESSION_FLAG_CAPTURE_WITH_WAIT_NO_WAIT: u32 = 4;
pub const NVML_NVFBC_SESSION_FLAG_CAPTURE_WITH_WAIT_INFINITE: u32 = 8;
pub const NVML_NVFBC_SESSION_FLAG_CAPTURE_WITH_WAIT_TIMEOUT: u32 = 16;
pub const NVML_CC_SYSTEM_CPU_CAPS_NONE: u32 = 0;
pub const NVML_CC_SYSTEM_CPU_CAPS_AMD_SEV: u32 = 1;
pub const NVML_CC_SYSTEM_CPU_CAPS_INTEL_TDX: u32 = 2;
pub const NVML_CC_SYSTEM_CPU_CAPS_AMD_SEV_SNP: u32 = 3;
pub const NVML_CC_SYSTEM_CPU_CAPS_AMD_SNP_VTOM: u32 = 4;
pub const NVML_CC_SYSTEM_GPUS_CC_NOT_CAPABLE: u32 = 0;
pub const NVML_CC_SYSTEM_GPUS_CC_CAPABLE: u32 = 1;
pub const NVML_CC_SYSTEM_DEVTOOLS_MODE_OFF: u32 = 0;
pub const NVML_CC_SYSTEM_DEVTOOLS_MODE_ON: u32 = 1;
pub const NVML_CC_SYSTEM_ENVIRONMENT_UNAVAILABLE: u32 = 0;
pub const NVML_CC_SYSTEM_ENVIRONMENT_SIM: u32 = 1;
pub const NVML_CC_SYSTEM_ENVIRONMENT_PROD: u32 = 2;
pub const NVML_CC_SYSTEM_FEATURE_DISABLED: u32 = 0;
pub const NVML_CC_SYSTEM_FEATURE_ENABLED: u32 = 1;
pub const NVML_CC_SYSTEM_MULTIGPU_NONE: u32 = 0;
pub const NVML_CC_SYSTEM_MULTIGPU_PROTECTED_PCIE: u32 = 1;
pub const NVML_CC_SYSTEM_MULTIGPU_NVLE: u32 = 2;
pub const NVML_CC_ACCEPTING_CLIENT_REQUESTS_FALSE: u32 = 0;
pub const NVML_CC_ACCEPTING_CLIENT_REQUESTS_TRUE: u32 = 1;
pub const NVML_GPU_CERT_CHAIN_SIZE: u32 = 4096;
pub const NVML_GPU_ATTESTATION_CERT_CHAIN_SIZE: u32 = 5120;
pub const NVML_CC_GPU_CEC_NONCE_SIZE: u32 = 32;
pub const NVML_CC_GPU_ATTESTATION_REPORT_SIZE: u32 = 8192;
pub const NVML_CC_GPU_CEC_ATTESTATION_REPORT_SIZE: u32 = 4096;
pub const NVML_CC_CEC_ATTESTATION_REPORT_NOT_PRESENT: u32 = 0;
pub const NVML_CC_CEC_ATTESTATION_REPORT_PRESENT: u32 = 1;
pub const NVML_CC_KEY_ROTATION_THRESHOLD_ATTACKER_ADVANTAGE_MIN: u32 = 50;
pub const NVML_CC_KEY_ROTATION_THRESHOLD_ATTACKER_ADVANTAGE_MAX: u32 = 65;
pub const NVML_GPU_FABRIC_UUID_LEN: u32 = 16;
pub const NVML_GPU_FABRIC_STATE_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_STATE_NOT_STARTED: u32 = 1;
pub const NVML_GPU_FABRIC_STATE_IN_PROGRESS: u32 = 2;
pub const NVML_GPU_FABRIC_STATE_COMPLETED: u32 = 3;
pub const NVML_GPU_FABRIC_HEALTH_MASK_DEGRADED_BW_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_MASK_DEGRADED_BW_TRUE: u32 = 1;
pub const NVML_GPU_FABRIC_HEALTH_MASK_DEGRADED_BW_FALSE: u32 = 2;
pub const NVML_GPU_FABRIC_HEALTH_MASK_SHIFT_DEGRADED_BW: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_MASK_WIDTH_DEGRADED_BW: u32 = 3;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ROUTE_RECOVERY_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ROUTE_RECOVERY_TRUE: u32 = 1;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ROUTE_RECOVERY_FALSE: u32 = 2;
pub const NVML_GPU_FABRIC_HEALTH_MASK_SHIFT_ROUTE_RECOVERY: u32 = 2;
pub const NVML_GPU_FABRIC_HEALTH_MASK_WIDTH_ROUTE_RECOVERY: u32 = 3;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ROUTE_UNHEALTHY_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ROUTE_UNHEALTHY_TRUE: u32 = 1;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ROUTE_UNHEALTHY_FALSE: u32 = 2;
pub const NVML_GPU_FABRIC_HEALTH_MASK_SHIFT_ROUTE_UNHEALTHY: u32 = 4;
pub const NVML_GPU_FABRIC_HEALTH_MASK_WIDTH_ROUTE_UNHEALTHY: u32 = 3;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ACCESS_TIMEOUT_RECOVERY_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ACCESS_TIMEOUT_RECOVERY_TRUE: u32 = 1;
pub const NVML_GPU_FABRIC_HEALTH_MASK_ACCESS_TIMEOUT_RECOVERY_FALSE: u32 = 2;
pub const NVML_GPU_FABRIC_HEALTH_MASK_SHIFT_ACCESS_TIMEOUT_RECOVERY: u32 = 6;
pub const NVML_GPU_FABRIC_HEALTH_MASK_WIDTH_ACCESS_TIMEOUT_RECOVERY: u32 = 3;
pub const NVML_GPU_FABRIC_HEALTH_MASK_INCORRECT_CONFIGURATION_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_MASK_INCORRECT_CONFIGURATION_NONE: u32 = 1;
pub const NVML_GPU_FABRIC_HEALTH_MASK_INCORRECT_CONFIGURATION_INCORRECT_SYSGUID: u32 = 2;
pub const NVML_GPU_FABRIC_HEALTH_MASK_INCORRECT_CONFIGURATION_INCORRECT_CHASSIS_SN: u32 = 3;
pub const NVML_GPU_FABRIC_HEALTH_MASK_INCORRECT_CONFIGURATION_NO_PARTITION: u32 = 4;
pub const NVML_GPU_FABRIC_HEALTH_MASK_INCORRECT_CONFIGURATION_INSUFFICIENT_NVLINKS: u32 = 5;
pub const NVML_GPU_FABRIC_HEALTH_MASK_SHIFT_INCORRECT_CONFIGURATION: u32 = 8;
pub const NVML_GPU_FABRIC_HEALTH_MASK_WIDTH_INCORRECT_CONFIGURATION: u32 = 15;
pub const NVML_GPU_FABRIC_HEALTH_SUMMARY_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_SUMMARY_HEALTHY: u32 = 1;
pub const NVML_GPU_FABRIC_HEALTH_SUMMARY_UNHEALTHY: u32 = 2;
pub const NVML_GPU_FABRIC_HEALTH_SUMMARY_LIMITED_CAPACITY: u32 = 3;
pub const NVML_INIT_FLAG_NO_GPUS: u32 = 1;
pub const NVML_INIT_FLAG_NO_ATTACH: u32 = 2;
pub const NVML_DEVICE_INFOROM_VERSION_BUFFER_SIZE: u32 = 16;
pub const NVML_DEVICE_UUID_BUFFER_SIZE: u32 = 80;
pub const NVML_DEVICE_UUID_V2_BUFFER_SIZE: u32 = 96;
pub const NVML_DEVICE_PART_NUMBER_BUFFER_SIZE: u32 = 80;
pub const NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE: u32 = 80;
pub const NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE: u32 = 80;
pub const NVML_DEVICE_NAME_BUFFER_SIZE: u32 = 64;
pub const NVML_DEVICE_NAME_V2_BUFFER_SIZE: u32 = 96;
pub const NVML_DEVICE_SERIAL_BUFFER_SIZE: u32 = 30;
pub const NVML_DEVICE_VBIOS_VERSION_BUFFER_SIZE: u32 = 32;
pub const NVML_AFFINITY_SCOPE_NODE: u32 = 0;
pub const NVML_AFFINITY_SCOPE_SOCKET: u32 = 1;
pub const NVML_NVLINK_BER_MANTISSA_SHIFT: u32 = 8;
pub const NVML_NVLINK_BER_MANTISSA_WIDTH: u32 = 15;
pub const NVML_NVLINK_BER_EXP_SHIFT: u32 = 0;
pub const NVML_NVLINK_BER_EXP_WIDTH: u32 = 255;
pub const NVML_NVLINK_STATE_INACTIVE: u32 = 0;
pub const NVML_NVLINK_STATE_ACTIVE: u32 = 1;
pub const NVML_NVLINK_STATE_SLEEP: u32 = 2;
pub const NVML_NVLINK_TOTAL_SUPPORTED_BW_MODES: u32 = 23;
pub const NVML_NVLINK_FIRMWARE_UCODE_TYPE_MSE: u32 = 1;
pub const NVML_NVLINK_FIRMWARE_UCODE_TYPE_NETIR: u32 = 2;
pub const NVML_NVLINK_FIRMWARE_UCODE_TYPE_NETIR_UPHY: u32 = 3;
pub const NVML_NVLINK_FIRMWARE_UCODE_TYPE_NETIR_CLN: u32 = 4;
pub const NVML_NVLINK_FIRMWARE_UCODE_TYPE_NETIR_DLN: u32 = 5;
pub const NVML_NVLINK_FIRMWARE_VERSION_LENGTH: u32 = 100;
pub const NVML_PRM_DATA_MAX_SIZE: u32 = 496;
pub const NVML_DEVICE_MIG_DISABLE: u32 = 0;
pub const NVML_DEVICE_MIG_ENABLE: u32 = 1;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE: u32 = 0;
pub const NVML_GPU_INSTANCE_PROFILE_2_SLICE: u32 = 1;
pub const NVML_GPU_INSTANCE_PROFILE_3_SLICE: u32 = 2;
pub const NVML_GPU_INSTANCE_PROFILE_4_SLICE: u32 = 3;
pub const NVML_GPU_INSTANCE_PROFILE_7_SLICE: u32 = 4;
pub const NVML_GPU_INSTANCE_PROFILE_8_SLICE: u32 = 5;
pub const NVML_GPU_INSTANCE_PROFILE_6_SLICE: u32 = 6;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE_REV1: u32 = 7;
pub const NVML_GPU_INSTANCE_PROFILE_2_SLICE_REV1: u32 = 8;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE_REV2: u32 = 9;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE_GFX: u32 = 10;
pub const NVML_GPU_INSTANCE_PROFILE_2_SLICE_GFX: u32 = 11;
pub const NVML_GPU_INSTANCE_PROFILE_4_SLICE_GFX: u32 = 12;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE_NO_ME: u32 = 13;
pub const NVML_GPU_INSTANCE_PROFILE_2_SLICE_NO_ME: u32 = 14;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE_ALL_ME: u32 = 15;
pub const NVML_GPU_INSTANCE_PROFILE_2_SLICE_ALL_ME: u32 = 16;
pub const NVML_GPU_INSTANCE_PROFILE_COUNT: u32 = 17;
pub const NVML_GPU_INSTANCE_PROFILE_CAPS_P2P: u32 = 1;
pub const NVML_GPU_INTSTANCE_PROFILE_CAPS_P2P: u32 = 1;
pub const NVML_GPU_INSTANCE_PROFILE_CAPS_GFX: u32 = 2;
pub const NVML_COMPUTE_INSTANCE_PROFILE_CAPS_GFX: u32 = 1;
pub const NVML_COMPUTE_INSTANCE_PROFILE_1_SLICE: u32 = 0;
pub const NVML_COMPUTE_INSTANCE_PROFILE_2_SLICE: u32 = 1;
pub const NVML_COMPUTE_INSTANCE_PROFILE_3_SLICE: u32 = 2;
pub const NVML_COMPUTE_INSTANCE_PROFILE_4_SLICE: u32 = 3;
pub const NVML_COMPUTE_INSTANCE_PROFILE_7_SLICE: u32 = 4;
pub const NVML_COMPUTE_INSTANCE_PROFILE_8_SLICE: u32 = 5;
pub const NVML_COMPUTE_INSTANCE_PROFILE_6_SLICE: u32 = 6;
pub const NVML_COMPUTE_INSTANCE_PROFILE_1_SLICE_REV1: u32 = 7;
pub const NVML_COMPUTE_INSTANCE_PROFILE_COUNT: u32 = 8;
pub const NVML_COMPUTE_INSTANCE_ENGINE_PROFILE_SHARED: u32 = 0;
pub const NVML_COMPUTE_INSTANCE_ENGINE_PROFILE_COUNT: u32 = 1;
pub const NVML_GPM_METRICS_GET_VERSION: u32 = 1;
pub const NVML_GPM_SUPPORT_VERSION: u32 = 1;
pub const NVML_DEV_CAP_EGM: u32 = 1;
pub const NVML_255_MASK_BITS_PER_ELEM: u32 = 32;
pub const NVML_255_MASK_NUM_ELEMS: u32 = 8;
pub const NVML_WORKLOAD_POWER_MAX_PROFILES: u32 = 255;
pub const NVML_POWER_SMOOTHING_MAX_NUM_PROFILES: u32 = 5;
pub const NVML_POWER_SMOOTHING_NUM_PROFILE_PARAMS: u32 = 4;
pub const NVML_POWER_SMOOTHING_ADMIN_OVERRIDE_NOT_SET: u32 = 4294967295;
pub const NVML_POWER_SMOOTHING_PROFILE_PARAM_PERCENT_TMP_FLOOR: u32 = 0;
pub const NVML_POWER_SMOOTHING_PROFILE_PARAM_RAMP_UP_RATE: u32 = 1;
pub const NVML_POWER_SMOOTHING_PROFILE_PARAM_RAMP_DOWN_RATE: u32 = 2;
pub const NVML_POWER_SMOOTHING_PROFILE_PARAM_RAMP_DOWN_HYSTERESIS: u32 = 3;
pub const NVML_DEVICE_ADDRESSING_MODE_NONE: nvmlDeviceAddressingModeType_t = 0;
pub const NVML_DEVICE_ADDRESSING_MODE_HMM: nvmlDeviceAddressingModeType_t = 1;
pub const NVML_DEVICE_ADDRESSING_MODE_ATS: nvmlDeviceAddressingModeType_t = 2;
pub const NVML_BRIDGE_CHIP_PLX: nvmlBridgeChipType_enum = 0;
pub const NVML_BRIDGE_CHIP_BRO4: nvmlBridgeChipType_enum = 1;
pub const NVML_NVLINK_COUNTER_UNIT_CYCLES: nvmlNvLinkUtilizationCountUnits_enum = 0;
pub const NVML_NVLINK_COUNTER_UNIT_PACKETS: nvmlNvLinkUtilizationCountUnits_enum = 1;
pub const NVML_NVLINK_COUNTER_UNIT_BYTES: nvmlNvLinkUtilizationCountUnits_enum = 2;
pub const NVML_NVLINK_COUNTER_UNIT_RESERVED: nvmlNvLinkUtilizationCountUnits_enum = 3;
pub const NVML_NVLINK_COUNTER_UNIT_COUNT: nvmlNvLinkUtilizationCountUnits_enum = 4;
pub const NVML_NVLINK_COUNTER_PKTFILTER_NOP: nvmlNvLinkUtilizationCountPktTypes_enum = 1;
pub const NVML_NVLINK_COUNTER_PKTFILTER_READ: nvmlNvLinkUtilizationCountPktTypes_enum = 2;
pub const NVML_NVLINK_COUNTER_PKTFILTER_WRITE: nvmlNvLinkUtilizationCountPktTypes_enum = 4;
pub const NVML_NVLINK_COUNTER_PKTFILTER_RATOM: nvmlNvLinkUtilizationCountPktTypes_enum = 8;
pub const NVML_NVLINK_COUNTER_PKTFILTER_NRATOM: nvmlNvLinkUtilizationCountPktTypes_enum = 16;
pub const NVML_NVLINK_COUNTER_PKTFILTER_FLUSH: nvmlNvLinkUtilizationCountPktTypes_enum = 32;
pub const NVML_NVLINK_COUNTER_PKTFILTER_RESPDATA: nvmlNvLinkUtilizationCountPktTypes_enum = 64;
pub const NVML_NVLINK_COUNTER_PKTFILTER_RESPNODATA: nvmlNvLinkUtilizationCountPktTypes_enum = 128;
pub const NVML_NVLINK_COUNTER_PKTFILTER_ALL: nvmlNvLinkUtilizationCountPktTypes_enum = 255;
pub const NVML_NVLINK_CAP_P2P_SUPPORTED: nvmlNvLinkCapability_enum = 0;
pub const NVML_NVLINK_CAP_SYSMEM_ACCESS: nvmlNvLinkCapability_enum = 1;
pub const NVML_NVLINK_CAP_P2P_ATOMICS: nvmlNvLinkCapability_enum = 2;
pub const NVML_NVLINK_CAP_SYSMEM_ATOMICS: nvmlNvLinkCapability_enum = 3;
pub const NVML_NVLINK_CAP_SLI_BRIDGE: nvmlNvLinkCapability_enum = 4;
pub const NVML_NVLINK_CAP_VALID: nvmlNvLinkCapability_enum = 5;
pub const NVML_NVLINK_CAP_COUNT: nvmlNvLinkCapability_enum = 6;
pub const NVML_NVLINK_ERROR_DL_REPLAY: nvmlNvLinkErrorCounter_enum = 0;
pub const NVML_NVLINK_ERROR_DL_RECOVERY: nvmlNvLinkErrorCounter_enum = 1;
pub const NVML_NVLINK_ERROR_DL_CRC_FLIT: nvmlNvLinkErrorCounter_enum = 2;
pub const NVML_NVLINK_ERROR_DL_CRC_DATA: nvmlNvLinkErrorCounter_enum = 3;
pub const NVML_NVLINK_ERROR_DL_ECC_DATA: nvmlNvLinkErrorCounter_enum = 4;
pub const NVML_NVLINK_ERROR_COUNT: nvmlNvLinkErrorCounter_enum = 5;
pub const NVML_NVLINK_DEVICE_TYPE_GPU: nvmlIntNvLinkDeviceType_enum = 0;
pub const NVML_NVLINK_DEVICE_TYPE_IBMNPU: nvmlIntNvLinkDeviceType_enum = 1;
pub const NVML_NVLINK_DEVICE_TYPE_SWITCH: nvmlIntNvLinkDeviceType_enum = 2;
pub const NVML_NVLINK_DEVICE_TYPE_UNKNOWN: nvmlIntNvLinkDeviceType_enum = 255;
pub const NVML_TOPOLOGY_INTERNAL: nvmlGpuLevel_enum = 0;
pub const NVML_TOPOLOGY_SINGLE: nvmlGpuLevel_enum = 10;
pub const NVML_TOPOLOGY_MULTIPLE: nvmlGpuLevel_enum = 20;
pub const NVML_TOPOLOGY_HOSTBRIDGE: nvmlGpuLevel_enum = 30;
pub const NVML_TOPOLOGY_NODE: nvmlGpuLevel_enum = 40;
pub const NVML_TOPOLOGY_SYSTEM: nvmlGpuLevel_enum = 50;
pub const NVML_P2P_STATUS_OK: nvmlGpuP2PStatus_enum = 0;
pub const NVML_P2P_STATUS_CHIPSET_NOT_SUPPORED: nvmlGpuP2PStatus_enum = 1;
pub const NVML_P2P_STATUS_CHIPSET_NOT_SUPPORTED: nvmlGpuP2PStatus_enum = 1;
pub const NVML_P2P_STATUS_GPU_NOT_SUPPORTED: nvmlGpuP2PStatus_enum = 2;
pub const NVML_P2P_STATUS_IOH_TOPOLOGY_NOT_SUPPORTED: nvmlGpuP2PStatus_enum = 3;
pub const NVML_P2P_STATUS_DISABLED_BY_REGKEY: nvmlGpuP2PStatus_enum = 4;
pub const NVML_P2P_STATUS_NOT_SUPPORTED: nvmlGpuP2PStatus_enum = 5;
pub const NVML_P2P_STATUS_UNKNOWN: nvmlGpuP2PStatus_enum = 6;
pub const NVML_P2P_CAPS_INDEX_READ: nvmlGpuP2PCapsIndex_enum = 0;
pub const NVML_P2P_CAPS_INDEX_WRITE: nvmlGpuP2PCapsIndex_enum = 1;
pub const NVML_P2P_CAPS_INDEX_NVLINK: nvmlGpuP2PCapsIndex_enum = 2;
pub const NVML_P2P_CAPS_INDEX_ATOMICS: nvmlGpuP2PCapsIndex_enum = 3;
pub const NVML_P2P_CAPS_INDEX_PCI: nvmlGpuP2PCapsIndex_enum = 4;
pub const NVML_P2P_CAPS_INDEX_PROP: nvmlGpuP2PCapsIndex_enum = 4;
pub const NVML_P2P_CAPS_INDEX_UNKNOWN: nvmlGpuP2PCapsIndex_enum = 5;
pub const NVML_TOTAL_POWER_SAMPLES: nvmlSamplingType_enum = 0;
pub const NVML_GPU_UTILIZATION_SAMPLES: nvmlSamplingType_enum = 1;
pub const NVML_MEMORY_UTILIZATION_SAMPLES: nvmlSamplingType_enum = 2;
pub const NVML_ENC_UTILIZATION_SAMPLES: nvmlSamplingType_enum = 3;
pub const NVML_DEC_UTILIZATION_SAMPLES: nvmlSamplingType_enum = 4;
pub const NVML_PROCESSOR_CLK_SAMPLES: nvmlSamplingType_enum = 5;
pub const NVML_MEMORY_CLK_SAMPLES: nvmlSamplingType_enum = 6;
pub const NVML_MODULE_POWER_SAMPLES: nvmlSamplingType_enum = 7;
pub const NVML_JPG_UTILIZATION_SAMPLES: nvmlSamplingType_enum = 8;
pub const NVML_OFA_UTILIZATION_SAMPLES: nvmlSamplingType_enum = 9;
pub const NVML_SAMPLINGTYPE_COUNT: nvmlSamplingType_enum = 10;
pub const NVML_PCIE_UTIL_TX_BYTES: nvmlPcieUtilCounter_enum = 0;
pub const NVML_PCIE_UTIL_RX_BYTES: nvmlPcieUtilCounter_enum = 1;
pub const NVML_PCIE_UTIL_COUNT: nvmlPcieUtilCounter_enum = 2;
pub const NVML_VALUE_TYPE_DOUBLE: nvmlValueType_enum = 0;
pub const NVML_VALUE_TYPE_UNSIGNED_INT: nvmlValueType_enum = 1;
pub const NVML_VALUE_TYPE_UNSIGNED_LONG: nvmlValueType_enum = 2;
pub const NVML_VALUE_TYPE_UNSIGNED_LONG_LONG: nvmlValueType_enum = 3;
pub const NVML_VALUE_TYPE_SIGNED_LONG_LONG: nvmlValueType_enum = 4;
pub const NVML_VALUE_TYPE_SIGNED_INT: nvmlValueType_enum = 5;
pub const NVML_VALUE_TYPE_UNSIGNED_SHORT: nvmlValueType_enum = 6;
pub const NVML_VALUE_TYPE_COUNT: nvmlValueType_enum = 7;
pub const NVML_PERF_POLICY_POWER: nvmlPerfPolicyType_enum = 0;
pub const NVML_PERF_POLICY_THERMAL: nvmlPerfPolicyType_enum = 1;
pub const NVML_PERF_POLICY_SYNC_BOOST: nvmlPerfPolicyType_enum = 2;
pub const NVML_PERF_POLICY_BOARD_LIMIT: nvmlPerfPolicyType_enum = 3;
pub const NVML_PERF_POLICY_LOW_UTILIZATION: nvmlPerfPolicyType_enum = 4;
pub const NVML_PERF_POLICY_RELIABILITY: nvmlPerfPolicyType_enum = 5;
pub const NVML_PERF_POLICY_TOTAL_APP_CLOCKS: nvmlPerfPolicyType_enum = 10;
pub const NVML_PERF_POLICY_TOTAL_BASE_CLOCKS: nvmlPerfPolicyType_enum = 11;
pub const NVML_PERF_POLICY_COUNT: nvmlPerfPolicyType_enum = 12;
pub const NVML_THERMAL_TARGET_NONE: nvmlThermalTarget_t = 0;
pub const NVML_THERMAL_TARGET_GPU: nvmlThermalTarget_t = 1;
pub const NVML_THERMAL_TARGET_MEMORY: nvmlThermalTarget_t = 2;
pub const NVML_THERMAL_TARGET_POWER_SUPPLY: nvmlThermalTarget_t = 4;
pub const NVML_THERMAL_TARGET_BOARD: nvmlThermalTarget_t = 8;
pub const NVML_THERMAL_TARGET_VCD_BOARD: nvmlThermalTarget_t = 9;
pub const NVML_THERMAL_TARGET_VCD_INLET: nvmlThermalTarget_t = 10;
pub const NVML_THERMAL_TARGET_VCD_OUTLET: nvmlThermalTarget_t = 11;
pub const NVML_THERMAL_TARGET_ALL: nvmlThermalTarget_t = 15;
pub const NVML_THERMAL_TARGET_UNKNOWN: nvmlThermalTarget_t = -1;
pub const NVML_THERMAL_CONTROLLER_NONE: nvmlThermalController_t = 0;
pub const NVML_THERMAL_CONTROLLER_GPU_INTERNAL: nvmlThermalController_t = 1;
pub const NVML_THERMAL_CONTROLLER_ADM1032: nvmlThermalController_t = 2;
pub const NVML_THERMAL_CONTROLLER_ADT7461: nvmlThermalController_t = 3;
pub const NVML_THERMAL_CONTROLLER_MAX6649: nvmlThermalController_t = 4;
pub const NVML_THERMAL_CONTROLLER_MAX1617: nvmlThermalController_t = 5;
pub const NVML_THERMAL_CONTROLLER_LM99: nvmlThermalController_t = 6;
pub const NVML_THERMAL_CONTROLLER_LM89: nvmlThermalController_t = 7;
pub const NVML_THERMAL_CONTROLLER_LM64: nvmlThermalController_t = 8;
pub const NVML_THERMAL_CONTROLLER_G781: nvmlThermalController_t = 9;
pub const NVML_THERMAL_CONTROLLER_ADT7473: nvmlThermalController_t = 10;
pub const NVML_THERMAL_CONTROLLER_SBMAX6649: nvmlThermalController_t = 11;
pub const NVML_THERMAL_CONTROLLER_VBIOSEVT: nvmlThermalController_t = 12;
pub const NVML_THERMAL_CONTROLLER_OS: nvmlThermalController_t = 13;
pub const NVML_THERMAL_CONTROLLER_NVSYSCON_CANOAS: nvmlThermalController_t = 14;
pub const NVML_THERMAL_CONTROLLER_NVSYSCON_E551: nvmlThermalController_t = 15;
pub const NVML_THERMAL_CONTROLLER_MAX6649R: nvmlThermalController_t = 16;
pub const NVML_THERMAL_CONTROLLER_ADT7473S: nvmlThermalController_t = 17;
pub const NVML_THERMAL_CONTROLLER_UNKNOWN: nvmlThermalController_t = -1;
pub const NVML_THERMAL_COOLER_SIGNAL_NONE: nvmlCoolerControl_enum = 0;
pub const NVML_THERMAL_COOLER_SIGNAL_TOGGLE: nvmlCoolerControl_enum = 1;
pub const NVML_THERMAL_COOLER_SIGNAL_VARIABLE: nvmlCoolerControl_enum = 2;
pub const NVML_THERMAL_COOLER_SIGNAL_COUNT: nvmlCoolerControl_enum = 3;
pub const NVML_THERMAL_COOLER_TARGET_NONE: nvmlCoolerTarget_enum = 1;
pub const NVML_THERMAL_COOLER_TARGET_GPU: nvmlCoolerTarget_enum = 2;
pub const NVML_THERMAL_COOLER_TARGET_MEMORY: nvmlCoolerTarget_enum = 4;
pub const NVML_THERMAL_COOLER_TARGET_POWER_SUPPLY: nvmlCoolerTarget_enum = 8;
pub const NVML_THERMAL_COOLER_TARGET_GPU_RELATED: nvmlCoolerTarget_enum = 14;
pub const NVML_UUID_TYPE_NONE: nvmlUUIDType_t = 0;
pub const NVML_UUID_TYPE_ASCII: nvmlUUIDType_t = 1;
pub const NVML_UUID_TYPE_BINARY: nvmlUUIDType_t = 2;
pub const NVML_FEATURE_DISABLED: nvmlEnableState_enum = 0;
pub const NVML_FEATURE_ENABLED: nvmlEnableState_enum = 1;
pub const NVML_BRAND_UNKNOWN: nvmlBrandType_enum = 0;
pub const NVML_BRAND_QUADRO: nvmlBrandType_enum = 1;
pub const NVML_BRAND_TESLA: nvmlBrandType_enum = 2;
pub const NVML_BRAND_NVS: nvmlBrandType_enum = 3;
pub const NVML_BRAND_GRID: nvmlBrandType_enum = 4;
pub const NVML_BRAND_GEFORCE: nvmlBrandType_enum = 5;
pub const NVML_BRAND_TITAN: nvmlBrandType_enum = 6;
pub const NVML_BRAND_NVIDIA_VAPPS: nvmlBrandType_enum = 7;
pub const NVML_BRAND_NVIDIA_VPC: nvmlBrandType_enum = 8;
pub const NVML_BRAND_NVIDIA_VCS: nvmlBrandType_enum = 9;
pub const NVML_BRAND_NVIDIA_VWS: nvmlBrandType_enum = 10;
pub const NVML_BRAND_NVIDIA_CLOUD_GAMING: nvmlBrandType_enum = 11;
pub const NVML_BRAND_NVIDIA_VGAMING: nvmlBrandType_enum = 11;
pub const NVML_BRAND_QUADRO_RTX: nvmlBrandType_enum = 12;
pub const NVML_BRAND_NVIDIA_RTX: nvmlBrandType_enum = 13;
pub const NVML_BRAND_NVIDIA: nvmlBrandType_enum = 14;
pub const NVML_BRAND_GEFORCE_RTX: nvmlBrandType_enum = 15;
pub const NVML_BRAND_TITAN_RTX: nvmlBrandType_enum = 16;
pub const NVML_BRAND_COUNT: nvmlBrandType_enum = 18;
pub const NVML_TEMPERATURE_THRESHOLD_SHUTDOWN: nvmlTemperatureThresholds_enum = 0;
pub const NVML_TEMPERATURE_THRESHOLD_SLOWDOWN: nvmlTemperatureThresholds_enum = 1;
pub const NVML_TEMPERATURE_THRESHOLD_MEM_MAX: nvmlTemperatureThresholds_enum = 2;
pub const NVML_TEMPERATURE_THRESHOLD_GPU_MAX: nvmlTemperatureThresholds_enum = 3;
pub const NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_MIN: nvmlTemperatureThresholds_enum = 4;
pub const NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_CURR: nvmlTemperatureThresholds_enum = 5;
pub const NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_MAX: nvmlTemperatureThresholds_enum = 6;
pub const NVML_TEMPERATURE_THRESHOLD_GPS_CURR: nvmlTemperatureThresholds_enum = 7;
pub const NVML_TEMPERATURE_THRESHOLD_COUNT: nvmlTemperatureThresholds_enum = 8;
pub const NVML_TEMPERATURE_GPU: nvmlTemperatureSensors_enum = 0;
pub const NVML_TEMPERATURE_COUNT: nvmlTemperatureSensors_enum = 1;
pub const NVML_COMPUTEMODE_DEFAULT: nvmlComputeMode_enum = 0;
pub const NVML_COMPUTEMODE_EXCLUSIVE_THREAD: nvmlComputeMode_enum = 1;
pub const NVML_COMPUTEMODE_PROHIBITED: nvmlComputeMode_enum = 2;
pub const NVML_COMPUTEMODE_EXCLUSIVE_PROCESS: nvmlComputeMode_enum = 3;
pub const NVML_COMPUTEMODE_COUNT: nvmlComputeMode_enum = 4;
pub const NVML_MEMORY_ERROR_TYPE_CORRECTED: nvmlMemoryErrorType_enum = 0;
pub const NVML_MEMORY_ERROR_TYPE_UNCORRECTED: nvmlMemoryErrorType_enum = 1;
pub const NVML_MEMORY_ERROR_TYPE_COUNT: nvmlMemoryErrorType_enum = 2;
pub const NVML_NVLINK_VERSION_INVALID: nvmlNvlinkVersion_enum = 0;
pub const NVML_NVLINK_VERSION_1_0: nvmlNvlinkVersion_enum = 1;
pub const NVML_NVLINK_VERSION_2_0: nvmlNvlinkVersion_enum = 2;
pub const NVML_NVLINK_VERSION_2_2: nvmlNvlinkVersion_enum = 3;
pub const NVML_NVLINK_VERSION_3_0: nvmlNvlinkVersion_enum = 4;
pub const NVML_NVLINK_VERSION_3_1: nvmlNvlinkVersion_enum = 5;
pub const NVML_NVLINK_VERSION_4_0: nvmlNvlinkVersion_enum = 6;
pub const NVML_NVLINK_VERSION_5_0: nvmlNvlinkVersion_enum = 7;
pub const NVML_VOLATILE_ECC: nvmlEccCounterType_enum = 0;
pub const NVML_AGGREGATE_ECC: nvmlEccCounterType_enum = 1;
pub const NVML_ECC_COUNTER_TYPE_COUNT: nvmlEccCounterType_enum = 2;
pub const NVML_CLOCK_GRAPHICS: nvmlClockType_enum = 0;
pub const NVML_CLOCK_SM: nvmlClockType_enum = 1;
pub const NVML_CLOCK_MEM: nvmlClockType_enum = 2;
pub const NVML_CLOCK_VIDEO: nvmlClockType_enum = 3;
pub const NVML_CLOCK_COUNT: nvmlClockType_enum = 4;
pub const NVML_CLOCK_ID_CURRENT: nvmlClockId_enum = 0;
pub const NVML_CLOCK_ID_APP_CLOCK_TARGET: nvmlClockId_enum = 1;
pub const NVML_CLOCK_ID_APP_CLOCK_DEFAULT: nvmlClockId_enum = 2;
pub const NVML_CLOCK_ID_CUSTOMER_BOOST_MAX: nvmlClockId_enum = 3;
pub const NVML_CLOCK_ID_COUNT: nvmlClockId_enum = 4;
pub const NVML_DRIVER_WDDM: nvmlDriverModel_enum = 0;
pub const NVML_DRIVER_WDM: nvmlDriverModel_enum = 1;
pub const NVML_DRIVER_MCDM: nvmlDriverModel_enum = 2;
pub const NVML_PSTATE_0: nvmlPStates_enum = 0;
pub const NVML_PSTATE_1: nvmlPStates_enum = 1;
pub const NVML_PSTATE_2: nvmlPStates_enum = 2;
pub const NVML_PSTATE_3: nvmlPStates_enum = 3;
pub const NVML_PSTATE_4: nvmlPStates_enum = 4;
pub const NVML_PSTATE_5: nvmlPStates_enum = 5;
pub const NVML_PSTATE_6: nvmlPStates_enum = 6;
pub const NVML_PSTATE_7: nvmlPStates_enum = 7;
pub const NVML_PSTATE_8: nvmlPStates_enum = 8;
pub const NVML_PSTATE_9: nvmlPStates_enum = 9;
pub const NVML_PSTATE_10: nvmlPStates_enum = 10;
pub const NVML_PSTATE_11: nvmlPStates_enum = 11;
pub const NVML_PSTATE_12: nvmlPStates_enum = 12;
pub const NVML_PSTATE_13: nvmlPStates_enum = 13;
pub const NVML_PSTATE_14: nvmlPStates_enum = 14;
pub const NVML_PSTATE_15: nvmlPStates_enum = 15;
pub const NVML_PSTATE_UNKNOWN: nvmlPStates_enum = 32;
pub const NVML_GOM_ALL_ON: nvmlGom_enum = 0;
pub const NVML_GOM_COMPUTE: nvmlGom_enum = 1;
pub const NVML_GOM_LOW_DP: nvmlGom_enum = 2;
pub const NVML_INFOROM_OEM: nvmlInforomObject_enum = 0;
pub const NVML_INFOROM_ECC: nvmlInforomObject_enum = 1;
pub const NVML_INFOROM_POWER: nvmlInforomObject_enum = 2;
pub const NVML_INFOROM_DEN: nvmlInforomObject_enum = 3;
pub const NVML_INFOROM_COUNT: nvmlInforomObject_enum = 4;
pub const NVML_SUCCESS: nvmlReturn_enum = 0;
pub const NVML_ERROR_UNINITIALIZED: nvmlReturn_enum = 1;
pub const NVML_ERROR_INVALID_ARGUMENT: nvmlReturn_enum = 2;
pub const NVML_ERROR_NOT_SUPPORTED: nvmlReturn_enum = 3;
pub const NVML_ERROR_NO_PERMISSION: nvmlReturn_enum = 4;
pub const NVML_ERROR_ALREADY_INITIALIZED: nvmlReturn_enum = 5;
pub const NVML_ERROR_NOT_FOUND: nvmlReturn_enum = 6;
pub const NVML_ERROR_INSUFFICIENT_SIZE: nvmlReturn_enum = 7;
pub const NVML_ERROR_INSUFFICIENT_POWER: nvmlReturn_enum = 8;
pub const NVML_ERROR_DRIVER_NOT_LOADED: nvmlReturn_enum = 9;
pub const NVML_ERROR_TIMEOUT: nvmlReturn_enum = 10;
pub const NVML_ERROR_IRQ_ISSUE: nvmlReturn_enum = 11;
pub const NVML_ERROR_LIBRARY_NOT_FOUND: nvmlReturn_enum = 12;
pub const NVML_ERROR_FUNCTION_NOT_FOUND: nvmlReturn_enum = 13;
pub const NVML_ERROR_CORRUPTED_INFOROM: nvmlReturn_enum = 14;
pub const NVML_ERROR_GPU_IS_LOST: nvmlReturn_enum = 15;
pub const NVML_ERROR_RESET_REQUIRED: nvmlReturn_enum = 16;
pub const NVML_ERROR_OPERATING_SYSTEM: nvmlReturn_enum = 17;
pub const NVML_ERROR_LIB_RM_VERSION_MISMATCH: nvmlReturn_enum = 18;
pub const NVML_ERROR_IN_USE: nvmlReturn_enum = 19;
pub const NVML_ERROR_MEMORY: nvmlReturn_enum = 20;
pub const NVML_ERROR_NO_DATA: nvmlReturn_enum = 21;
pub const NVML_ERROR_VGPU_ECC_NOT_SUPPORTED: nvmlReturn_enum = 22;
pub const NVML_ERROR_INSUFFICIENT_RESOURCES: nvmlReturn_enum = 23;
pub const NVML_ERROR_FREQ_NOT_SUPPORTED: nvmlReturn_enum = 24;
pub const NVML_ERROR_ARGUMENT_VERSION_MISMATCH: nvmlReturn_enum = 25;
pub const NVML_ERROR_DEPRECATED: nvmlReturn_enum = 26;
pub const NVML_ERROR_NOT_READY: nvmlReturn_enum = 27;
pub const NVML_ERROR_GPU_NOT_FOUND: nvmlReturn_enum = 28;
pub const NVML_ERROR_INVALID_STATE: nvmlReturn_enum = 29;
pub const NVML_ERROR_RESET_TYPE_NOT_SUPPORTED: nvmlReturn_enum = 30;
pub const NVML_ERROR_UNKNOWN: nvmlReturn_enum = 999;
pub const NVML_MEMORY_LOCATION_L1_CACHE: nvmlMemoryLocation_enum = 0;
pub const NVML_MEMORY_LOCATION_L2_CACHE: nvmlMemoryLocation_enum = 1;
pub const NVML_MEMORY_LOCATION_DRAM: nvmlMemoryLocation_enum = 2;
pub const NVML_MEMORY_LOCATION_DEVICE_MEMORY: nvmlMemoryLocation_enum = 2;
pub const NVML_MEMORY_LOCATION_REGISTER_FILE: nvmlMemoryLocation_enum = 3;
pub const NVML_MEMORY_LOCATION_TEXTURE_MEMORY: nvmlMemoryLocation_enum = 4;
pub const NVML_MEMORY_LOCATION_TEXTURE_SHM: nvmlMemoryLocation_enum = 5;
pub const NVML_MEMORY_LOCATION_CBU: nvmlMemoryLocation_enum = 6;
pub const NVML_MEMORY_LOCATION_SRAM: nvmlMemoryLocation_enum = 7;
pub const NVML_MEMORY_LOCATION_COUNT: nvmlMemoryLocation_enum = 8;
pub const NVML_PAGE_RETIREMENT_CAUSE_MULTIPLE_SINGLE_BIT_ECC_ERRORS: nvmlPageRetirementCause_enum =
    0;
pub const NVML_PAGE_RETIREMENT_CAUSE_DOUBLE_BIT_ECC_ERROR: nvmlPageRetirementCause_enum = 1;
pub const NVML_PAGE_RETIREMENT_CAUSE_COUNT: nvmlPageRetirementCause_enum = 2;
pub const NVML_RESTRICTED_API_SET_APPLICATION_CLOCKS: nvmlRestrictedAPI_enum = 0;
pub const NVML_RESTRICTED_API_SET_AUTO_BOOSTED_CLOCKS: nvmlRestrictedAPI_enum = 1;
pub const NVML_RESTRICTED_API_COUNT: nvmlRestrictedAPI_enum = 2;
pub const NVML_GPU_UTILIZATION_DOMAIN_GPU: nvmlGpuUtilizationDomainId_t = 0;
pub const NVML_GPU_UTILIZATION_DOMAIN_FB: nvmlGpuUtilizationDomainId_t = 1;
pub const NVML_GPU_UTILIZATION_DOMAIN_VID: nvmlGpuUtilizationDomainId_t = 2;
pub const NVML_GPU_UTILIZATION_DOMAIN_BUS: nvmlGpuUtilizationDomainId_t = 3;
pub const NVML_GPU_VIRTUALIZATION_MODE_NONE: nvmlGpuVirtualizationMode = 0;
pub const NVML_GPU_VIRTUALIZATION_MODE_PASSTHROUGH: nvmlGpuVirtualizationMode = 1;
pub const NVML_GPU_VIRTUALIZATION_MODE_VGPU: nvmlGpuVirtualizationMode = 2;
pub const NVML_GPU_VIRTUALIZATION_MODE_HOST_VGPU: nvmlGpuVirtualizationMode = 3;
pub const NVML_GPU_VIRTUALIZATION_MODE_HOST_VSGA: nvmlGpuVirtualizationMode = 4;
pub const NVML_HOST_VGPU_MODE_NON_SRIOV: nvmlHostVgpuMode_enum = 0;
pub const NVML_HOST_VGPU_MODE_SRIOV: nvmlHostVgpuMode_enum = 1;
pub const NVML_VGPU_VM_ID_DOMAIN_ID: nvmlVgpuVmIdType = 0;
pub const NVML_VGPU_VM_ID_UUID: nvmlVgpuVmIdType = 1;
pub const NVML_VGPU_INSTANCE_GUEST_INFO_STATE_UNINITIALIZED: nvmlVgpuGuestInfoState_enum = 0;
pub const NVML_VGPU_INSTANCE_GUEST_INFO_STATE_INITIALIZED: nvmlVgpuGuestInfoState_enum = 1;
pub const NVML_GRID_LICENSE_FEATURE_CODE_UNKNOWN: nvmlGridLicenseFeatureCode_t = 0;
pub const NVML_GRID_LICENSE_FEATURE_CODE_VGPU: nvmlGridLicenseFeatureCode_t = 1;
pub const NVML_GRID_LICENSE_FEATURE_CODE_NVIDIA_RTX: nvmlGridLicenseFeatureCode_t = 2;
pub const NVML_GRID_LICENSE_FEATURE_CODE_VWORKSTATION: nvmlGridLicenseFeatureCode_t = 2;
pub const NVML_GRID_LICENSE_FEATURE_CODE_GAMING: nvmlGridLicenseFeatureCode_t = 3;
pub const NVML_GRID_LICENSE_FEATURE_CODE_COMPUTE: nvmlGridLicenseFeatureCode_t = 4;
pub const NVML_VGPU_CAP_NVLINK_P2P: nvmlVgpuCapability_enum = 0;
pub const NVML_VGPU_CAP_GPUDIRECT: nvmlVgpuCapability_enum = 1;
pub const NVML_VGPU_CAP_MULTI_VGPU_EXCLUSIVE: nvmlVgpuCapability_enum = 2;
pub const NVML_VGPU_CAP_EXCLUSIVE_TYPE: nvmlVgpuCapability_enum = 3;
pub const NVML_VGPU_CAP_EXCLUSIVE_SIZE: nvmlVgpuCapability_enum = 4;
pub const NVML_VGPU_CAP_COUNT: nvmlVgpuCapability_enum = 5;
pub const NVML_VGPU_DRIVER_CAP_HETEROGENEOUS_MULTI_VGPU: nvmlVgpuDriverCapability_enum = 0;
pub const NVML_VGPU_DRIVER_CAP_WARM_UPDATE: nvmlVgpuDriverCapability_enum = 1;
pub const NVML_VGPU_DRIVER_CAP_COUNT: nvmlVgpuDriverCapability_enum = 2;
pub const NVML_DEVICE_VGPU_CAP_FRACTIONAL_MULTI_VGPU: nvmlDeviceVgpuCapability_enum = 0;
pub const NVML_DEVICE_VGPU_CAP_HETEROGENEOUS_TIMESLICE_PROFILES: nvmlDeviceVgpuCapability_enum = 1;
pub const NVML_DEVICE_VGPU_CAP_HETEROGENEOUS_TIMESLICE_SIZES: nvmlDeviceVgpuCapability_enum = 2;
pub const NVML_DEVICE_VGPU_CAP_READ_DEVICE_BUFFER_BW: nvmlDeviceVgpuCapability_enum = 3;
pub const NVML_DEVICE_VGPU_CAP_WRITE_DEVICE_BUFFER_BW: nvmlDeviceVgpuCapability_enum = 4;
pub const NVML_DEVICE_VGPU_CAP_DEVICE_STREAMING: nvmlDeviceVgpuCapability_enum = 5;
pub const NVML_DEVICE_VGPU_CAP_MINI_QUARTER_GPU: nvmlDeviceVgpuCapability_enum = 6;
pub const NVML_DEVICE_VGPU_CAP_COMPUTE_MEDIA_ENGINE_GPU: nvmlDeviceVgpuCapability_enum = 7;
pub const NVML_DEVICE_VGPU_CAP_WARM_UPDATE: nvmlDeviceVgpuCapability_enum = 8;
pub const NVML_DEVICE_VGPU_CAP_HOMOGENEOUS_PLACEMENTS: nvmlDeviceVgpuCapability_enum = 9;
pub const NVML_DEVICE_VGPU_CAP_MIG_TIMESLICING_SUPPORTED: nvmlDeviceVgpuCapability_enum = 10;
pub const NVML_DEVICE_VGPU_CAP_MIG_TIMESLICING_ENABLED: nvmlDeviceVgpuCapability_enum = 11;
pub const NVML_DEVICE_VGPU_CAP_COUNT: nvmlDeviceVgpuCapability_enum = 12;
pub const NVML_GPU_RECOVERY_ACTION_NONE: nvmlDeviceGpuRecoveryAction_s = 0;
pub const NVML_GPU_RECOVERY_ACTION_GPU_RESET: nvmlDeviceGpuRecoveryAction_s = 1;
pub const NVML_GPU_RECOVERY_ACTION_NODE_REBOOT: nvmlDeviceGpuRecoveryAction_s = 2;
pub const NVML_GPU_RECOVERY_ACTION_DRAIN_P2P: nvmlDeviceGpuRecoveryAction_s = 3;
pub const NVML_GPU_RECOVERY_ACTION_DRAIN_AND_RESET: nvmlDeviceGpuRecoveryAction_s = 4;
pub const NVML_FAN_NORMAL: nvmlFanState_enum = 0;
pub const NVML_FAN_FAILED: nvmlFanState_enum = 1;
pub const NVML_LED_COLOR_GREEN: nvmlLedColor_enum = 0;
pub const NVML_LED_COLOR_AMBER: nvmlLedColor_enum = 1;
pub const NVML_ENCODER_QUERY_H264: nvmlEncoderQueryType_enum = 0;
pub const NVML_ENCODER_QUERY_HEVC: nvmlEncoderQueryType_enum = 1;
pub const NVML_ENCODER_QUERY_AV1: nvmlEncoderQueryType_enum = 2;
pub const NVML_ENCODER_QUERY_UNKNOWN: nvmlEncoderQueryType_enum = 255;
pub const NVML_FBC_SESSION_TYPE_UNKNOWN: nvmlFBCSessionType_enum = 0;
pub const NVML_FBC_SESSION_TYPE_TOSYS: nvmlFBCSessionType_enum = 1;
pub const NVML_FBC_SESSION_TYPE_CUDA: nvmlFBCSessionType_enum = 2;
pub const NVML_FBC_SESSION_TYPE_VID: nvmlFBCSessionType_enum = 3;
pub const NVML_FBC_SESSION_TYPE_HWENC: nvmlFBCSessionType_enum = 4;
pub const NVML_DETACH_GPU_KEEP: nvmlDetachGpuState_enum = 0;
pub const NVML_DETACH_GPU_REMOVE: nvmlDetachGpuState_enum = 1;
pub const NVML_PCIE_LINK_KEEP: nvmlPcieLinkState_enum = 0;
pub const NVML_PCIE_LINK_SHUT_DOWN: nvmlPcieLinkState_enum = 1;
pub const NVML_CLOCK_LIMIT_ID_RANGE_START: nvmlClockLimitId_enum = 4294967040;
pub const NVML_CLOCK_LIMIT_ID_TDP: nvmlClockLimitId_enum = 4294967041;
pub const NVML_CLOCK_LIMIT_ID_UNLIMITED: nvmlClockLimitId_enum = 4294967042;
pub const NVML_VGPU_VM_COMPATIBILITY_NONE: nvmlVgpuVmCompatibility_enum = 0;
pub const NVML_VGPU_VM_COMPATIBILITY_COLD: nvmlVgpuVmCompatibility_enum = 1;
pub const NVML_VGPU_VM_COMPATIBILITY_HIBERNATE: nvmlVgpuVmCompatibility_enum = 2;
pub const NVML_VGPU_VM_COMPATIBILITY_SLEEP: nvmlVgpuVmCompatibility_enum = 4;
pub const NVML_VGPU_VM_COMPATIBILITY_LIVE: nvmlVgpuVmCompatibility_enum = 8;
pub const NVML_VGPU_COMPATIBILITY_LIMIT_NONE: nvmlVgpuPgpuCompatibilityLimitCode_enum = 0;
pub const NVML_VGPU_COMPATIBILITY_LIMIT_HOST_DRIVER: nvmlVgpuPgpuCompatibilityLimitCode_enum = 1;
pub const NVML_VGPU_COMPATIBILITY_LIMIT_GUEST_DRIVER: nvmlVgpuPgpuCompatibilityLimitCode_enum = 2;
pub const NVML_VGPU_COMPATIBILITY_LIMIT_GPU: nvmlVgpuPgpuCompatibilityLimitCode_enum = 4;
pub const NVML_VGPU_COMPATIBILITY_LIMIT_OTHER: nvmlVgpuPgpuCompatibilityLimitCode_enum = 2147483648;
pub const NVML_GPM_METRIC_GRAPHICS_UTIL: nvmlGpmMetricId_t = 1;
pub const NVML_GPM_METRIC_SM_UTIL: nvmlGpmMetricId_t = 2;
pub const NVML_GPM_METRIC_SM_OCCUPANCY: nvmlGpmMetricId_t = 3;
pub const NVML_GPM_METRIC_INTEGER_UTIL: nvmlGpmMetricId_t = 4;
pub const NVML_GPM_METRIC_ANY_TENSOR_UTIL: nvmlGpmMetricId_t = 5;
pub const NVML_GPM_METRIC_DFMA_TENSOR_UTIL: nvmlGpmMetricId_t = 6;
pub const NVML_GPM_METRIC_HMMA_TENSOR_UTIL: nvmlGpmMetricId_t = 7;
pub const NVML_GPM_METRIC_IMMA_TENSOR_UTIL: nvmlGpmMetricId_t = 9;
pub const NVML_GPM_METRIC_DRAM_BW_UTIL: nvmlGpmMetricId_t = 10;
pub const NVML_GPM_METRIC_FP64_UTIL: nvmlGpmMetricId_t = 11;
pub const NVML_GPM_METRIC_FP32_UTIL: nvmlGpmMetricId_t = 12;
pub const NVML_GPM_METRIC_FP16_UTIL: nvmlGpmMetricId_t = 13;
pub const NVML_GPM_METRIC_PCIE_TX_PER_SEC: nvmlGpmMetricId_t = 20;
pub const NVML_GPM_METRIC_PCIE_RX_PER_SEC: nvmlGpmMetricId_t = 21;
pub const NVML_GPM_METRIC_NVDEC_0_UTIL: nvmlGpmMetricId_t = 30;
pub const NVML_GPM_METRIC_NVDEC_1_UTIL: nvmlGpmMetricId_t = 31;
pub const NVML_GPM_METRIC_NVDEC_2_UTIL: nvmlGpmMetricId_t = 32;
pub const NVML_GPM_METRIC_NVDEC_3_UTIL: nvmlGpmMetricId_t = 33;
pub const NVML_GPM_METRIC_NVDEC_4_UTIL: nvmlGpmMetricId_t = 34;
pub const NVML_GPM_METRIC_NVDEC_5_UTIL: nvmlGpmMetricId_t = 35;
pub const NVML_GPM_METRIC_NVDEC_6_UTIL: nvmlGpmMetricId_t = 36;
pub const NVML_GPM_METRIC_NVDEC_7_UTIL: nvmlGpmMetricId_t = 37;
pub const NVML_GPM_METRIC_NVJPG_0_UTIL: nvmlGpmMetricId_t = 40;
pub const NVML_GPM_METRIC_NVJPG_1_UTIL: nvmlGpmMetricId_t = 41;
pub const NVML_GPM_METRIC_NVJPG_2_UTIL: nvmlGpmMetricId_t = 42;
pub const NVML_GPM_METRIC_NVJPG_3_UTIL: nvmlGpmMetricId_t = 43;
pub const NVML_GPM_METRIC_NVJPG_4_UTIL: nvmlGpmMetricId_t = 44;
pub const NVML_GPM_METRIC_NVJPG_5_UTIL: nvmlGpmMetricId_t = 45;
pub const NVML_GPM_METRIC_NVJPG_6_UTIL: nvmlGpmMetricId_t = 46;
pub const NVML_GPM_METRIC_NVJPG_7_UTIL: nvmlGpmMetricId_t = 47;
pub const NVML_GPM_METRIC_NVOFA_0_UTIL: nvmlGpmMetricId_t = 50;
pub const NVML_GPM_METRIC_NVOFA_1_UTIL: nvmlGpmMetricId_t = 51;
pub const NVML_GPM_METRIC_NVLINK_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 60;
pub const NVML_GPM_METRIC_NVLINK_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 61;
pub const NVML_GPM_METRIC_NVLINK_L0_RX_PER_SEC: nvmlGpmMetricId_t = 62;
pub const NVML_GPM_METRIC_NVLINK_L0_TX_PER_SEC: nvmlGpmMetricId_t = 63;
pub const NVML_GPM_METRIC_NVLINK_L1_RX_PER_SEC: nvmlGpmMetricId_t = 64;
pub const NVML_GPM_METRIC_NVLINK_L1_TX_PER_SEC: nvmlGpmMetricId_t = 65;
pub const NVML_GPM_METRIC_NVLINK_L2_RX_PER_SEC: nvmlGpmMetricId_t = 66;
pub const NVML_GPM_METRIC_NVLINK_L2_TX_PER_SEC: nvmlGpmMetricId_t = 67;
pub const NVML_GPM_METRIC_NVLINK_L3_RX_PER_SEC: nvmlGpmMetricId_t = 68;
pub const NVML_GPM_METRIC_NVLINK_L3_TX_PER_SEC: nvmlGpmMetricId_t = 69;
pub const NVML_GPM_METRIC_NVLINK_L4_RX_PER_SEC: nvmlGpmMetricId_t = 70;
pub const NVML_GPM_METRIC_NVLINK_L4_TX_PER_SEC: nvmlGpmMetricId_t = 71;
pub const NVML_GPM_METRIC_NVLINK_L5_RX_PER_SEC: nvmlGpmMetricId_t = 72;
pub const NVML_GPM_METRIC_NVLINK_L5_TX_PER_SEC: nvmlGpmMetricId_t = 73;
pub const NVML_GPM_METRIC_NVLINK_L6_RX_PER_SEC: nvmlGpmMetricId_t = 74;
pub const NVML_GPM_METRIC_NVLINK_L6_TX_PER_SEC: nvmlGpmMetricId_t = 75;
pub const NVML_GPM_METRIC_NVLINK_L7_RX_PER_SEC: nvmlGpmMetricId_t = 76;
pub const NVML_GPM_METRIC_NVLINK_L7_TX_PER_SEC: nvmlGpmMetricId_t = 77;
pub const NVML_GPM_METRIC_NVLINK_L8_RX_PER_SEC: nvmlGpmMetricId_t = 78;
pub const NVML_GPM_METRIC_NVLINK_L8_TX_PER_SEC: nvmlGpmMetricId_t = 79;
pub const NVML_GPM_METRIC_NVLINK_L9_RX_PER_SEC: nvmlGpmMetricId_t = 80;
pub const NVML_GPM_METRIC_NVLINK_L9_TX_PER_SEC: nvmlGpmMetricId_t = 81;
pub const NVML_GPM_METRIC_NVLINK_L10_RX_PER_SEC: nvmlGpmMetricId_t = 82;
pub const NVML_GPM_METRIC_NVLINK_L10_TX_PER_SEC: nvmlGpmMetricId_t = 83;
pub const NVML_GPM_METRIC_NVLINK_L11_RX_PER_SEC: nvmlGpmMetricId_t = 84;
pub const NVML_GPM_METRIC_NVLINK_L11_TX_PER_SEC: nvmlGpmMetricId_t = 85;
pub const NVML_GPM_METRIC_NVLINK_L12_RX_PER_SEC: nvmlGpmMetricId_t = 86;
pub const NVML_GPM_METRIC_NVLINK_L12_TX_PER_SEC: nvmlGpmMetricId_t = 87;
pub const NVML_GPM_METRIC_NVLINK_L13_RX_PER_SEC: nvmlGpmMetricId_t = 88;
pub const NVML_GPM_METRIC_NVLINK_L13_TX_PER_SEC: nvmlGpmMetricId_t = 89;
pub const NVML_GPM_METRIC_NVLINK_L14_RX_PER_SEC: nvmlGpmMetricId_t = 90;
pub const NVML_GPM_METRIC_NVLINK_L14_TX_PER_SEC: nvmlGpmMetricId_t = 91;
pub const NVML_GPM_METRIC_NVLINK_L15_RX_PER_SEC: nvmlGpmMetricId_t = 92;
pub const NVML_GPM_METRIC_NVLINK_L15_TX_PER_SEC: nvmlGpmMetricId_t = 93;
pub const NVML_GPM_METRIC_NVLINK_L16_RX_PER_SEC: nvmlGpmMetricId_t = 94;
pub const NVML_GPM_METRIC_NVLINK_L16_TX_PER_SEC: nvmlGpmMetricId_t = 95;
pub const NVML_GPM_METRIC_NVLINK_L17_RX_PER_SEC: nvmlGpmMetricId_t = 96;
pub const NVML_GPM_METRIC_NVLINK_L17_TX_PER_SEC: nvmlGpmMetricId_t = 97;
pub const NVML_GPM_METRIC_C2C_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 100;
pub const NVML_GPM_METRIC_C2C_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 101;
pub const NVML_GPM_METRIC_C2C_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 102;
pub const NVML_GPM_METRIC_C2C_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 103;
pub const NVML_GPM_METRIC_C2C_LINK0_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 104;
pub const NVML_GPM_METRIC_C2C_LINK0_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 105;
pub const NVML_GPM_METRIC_C2C_LINK0_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 106;
pub const NVML_GPM_METRIC_C2C_LINK0_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 107;
pub const NVML_GPM_METRIC_C2C_LINK1_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 108;
pub const NVML_GPM_METRIC_C2C_LINK1_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 109;
pub const NVML_GPM_METRIC_C2C_LINK1_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 110;
pub const NVML_GPM_METRIC_C2C_LINK1_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 111;
pub const NVML_GPM_METRIC_C2C_LINK2_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 112;
pub const NVML_GPM_METRIC_C2C_LINK2_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 113;
pub const NVML_GPM_METRIC_C2C_LINK2_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 114;
pub const NVML_GPM_METRIC_C2C_LINK2_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 115;
pub const NVML_GPM_METRIC_C2C_LINK3_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 116;
pub const NVML_GPM_METRIC_C2C_LINK3_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 117;
pub const NVML_GPM_METRIC_C2C_LINK3_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 118;
pub const NVML_GPM_METRIC_C2C_LINK3_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 119;
pub const NVML_GPM_METRIC_C2C_LINK4_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 120;
pub const NVML_GPM_METRIC_C2C_LINK4_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 121;
pub const NVML_GPM_METRIC_C2C_LINK4_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 122;
pub const NVML_GPM_METRIC_C2C_LINK4_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 123;
pub const NVML_GPM_METRIC_C2C_LINK5_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 124;
pub const NVML_GPM_METRIC_C2C_LINK5_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 125;
pub const NVML_GPM_METRIC_C2C_LINK5_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 126;
pub const NVML_GPM_METRIC_C2C_LINK5_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 127;
pub const NVML_GPM_METRIC_C2C_LINK6_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 128;
pub const NVML_GPM_METRIC_C2C_LINK6_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 129;
pub const NVML_GPM_METRIC_C2C_LINK6_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 130;
pub const NVML_GPM_METRIC_C2C_LINK6_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 131;
pub const NVML_GPM_METRIC_C2C_LINK7_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 132;
pub const NVML_GPM_METRIC_C2C_LINK7_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 133;
pub const NVML_GPM_METRIC_C2C_LINK7_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 134;
pub const NVML_GPM_METRIC_C2C_LINK7_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 135;
pub const NVML_GPM_METRIC_C2C_LINK8_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 136;
pub const NVML_GPM_METRIC_C2C_LINK8_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 137;
pub const NVML_GPM_METRIC_C2C_LINK8_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 138;
pub const NVML_GPM_METRIC_C2C_LINK8_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 139;
pub const NVML_GPM_METRIC_C2C_LINK9_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 140;
pub const NVML_GPM_METRIC_C2C_LINK9_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 141;
pub const NVML_GPM_METRIC_C2C_LINK9_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 142;
pub const NVML_GPM_METRIC_C2C_LINK9_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 143;
pub const NVML_GPM_METRIC_C2C_LINK10_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 144;
pub const NVML_GPM_METRIC_C2C_LINK10_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 145;
pub const NVML_GPM_METRIC_C2C_LINK10_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 146;
pub const NVML_GPM_METRIC_C2C_LINK10_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 147;
pub const NVML_GPM_METRIC_C2C_LINK11_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 148;
pub const NVML_GPM_METRIC_C2C_LINK11_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 149;
pub const NVML_GPM_METRIC_C2C_LINK11_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 150;
pub const NVML_GPM_METRIC_C2C_LINK11_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 151;
pub const NVML_GPM_METRIC_C2C_LINK12_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 152;
pub const NVML_GPM_METRIC_C2C_LINK12_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 153;
pub const NVML_GPM_METRIC_C2C_LINK12_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 154;
pub const NVML_GPM_METRIC_C2C_LINK12_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 155;
pub const NVML_GPM_METRIC_C2C_LINK13_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = 156;
pub const NVML_GPM_METRIC_C2C_LINK13_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = 157;
pub const NVML_GPM_METRIC_C2C_LINK13_DATA_TX_PER_SEC: nvmlGpmMetricId_t = 158;
pub const NVML_GPM_METRIC_C2C_LINK13_DATA_RX_PER_SEC: nvmlGpmMetricId_t = 159;
pub const NVML_GPM_METRIC_HOSTMEM_CACHE_HIT: nvmlGpmMetricId_t = 160;
pub const NVML_GPM_METRIC_HOSTMEM_CACHE_MISS: nvmlGpmMetricId_t = 161;
pub const NVML_GPM_METRIC_PEERMEM_CACHE_HIT: nvmlGpmMetricId_t = 162;
pub const NVML_GPM_METRIC_PEERMEM_CACHE_MISS: nvmlGpmMetricId_t = 163;
pub const NVML_GPM_METRIC_DRAM_CACHE_HIT: nvmlGpmMetricId_t = 164;
pub const NVML_GPM_METRIC_DRAM_CACHE_MISS: nvmlGpmMetricId_t = 165;
pub const NVML_GPM_METRIC_NVENC_0_UTIL: nvmlGpmMetricId_t = 166;
pub const NVML_GPM_METRIC_NVENC_1_UTIL: nvmlGpmMetricId_t = 167;
pub const NVML_GPM_METRIC_NVENC_2_UTIL: nvmlGpmMetricId_t = 168;
pub const NVML_GPM_METRIC_NVENC_3_UTIL: nvmlGpmMetricId_t = 169;
pub const NVML_GPM_METRIC_GR0_CTXSW_CYCLES_ELAPSED: nvmlGpmMetricId_t = 170;
pub const NVML_GPM_METRIC_GR0_CTXSW_CYCLES_ACTIVE: nvmlGpmMetricId_t = 171;
pub const NVML_GPM_METRIC_GR0_CTXSW_REQUESTS: nvmlGpmMetricId_t = 172;
pub const NVML_GPM_METRIC_GR0_CTXSW_CYCLES_PER_REQ: nvmlGpmMetricId_t = 173;
pub const NVML_GPM_METRIC_GR0_CTXSW_ACTIVE_PCT: nvmlGpmMetricId_t = 174;
pub const NVML_GPM_METRIC_GR1_CTXSW_CYCLES_ELAPSED: nvmlGpmMetricId_t = 175;
pub const NVML_GPM_METRIC_GR1_CTXSW_CYCLES_ACTIVE: nvmlGpmMetricId_t = 176;
pub const NVML_GPM_METRIC_GR1_CTXSW_REQUESTS: nvmlGpmMetricId_t = 177;
pub const NVML_GPM_METRIC_GR1_CTXSW_CYCLES_PER_REQ: nvmlGpmMetricId_t = 178;
pub const NVML_GPM_METRIC_GR1_CTXSW_ACTIVE_PCT: nvmlGpmMetricId_t = 179;
pub const NVML_GPM_METRIC_GR2_CTXSW_CYCLES_ELAPSED: nvmlGpmMetricId_t = 180;
pub const NVML_GPM_METRIC_GR2_CTXSW_CYCLES_ACTIVE: nvmlGpmMetricId_t = 181;
pub const NVML_GPM_METRIC_GR2_CTXSW_REQUESTS: nvmlGpmMetricId_t = 182;
pub const NVML_GPM_METRIC_GR2_CTXSW_CYCLES_PER_REQ: nvmlGpmMetricId_t = 183;
pub const NVML_GPM_METRIC_GR2_CTXSW_ACTIVE_PCT: nvmlGpmMetricId_t = 184;
pub const NVML_GPM_METRIC_GR3_CTXSW_CYCLES_ELAPSED: nvmlGpmMetricId_t = 185;
pub const NVML_GPM_METRIC_GR3_CTXSW_CYCLES_ACTIVE: nvmlGpmMetricId_t = 186;
pub const NVML_GPM_METRIC_GR3_CTXSW_REQUESTS: nvmlGpmMetricId_t = 187;
pub const NVML_GPM_METRIC_GR3_CTXSW_CYCLES_PER_REQ: nvmlGpmMetricId_t = 188;
pub const NVML_GPM_METRIC_GR3_CTXSW_ACTIVE_PCT: nvmlGpmMetricId_t = 189;
pub const NVML_GPM_METRIC_GR4_CTXSW_CYCLES_ELAPSED: nvmlGpmMetricId_t = 190;
pub const NVML_GPM_METRIC_GR4_CTXSW_CYCLES_ACTIVE: nvmlGpmMetricId_t = 191;
pub const NVML_GPM_METRIC_GR4_CTXSW_REQUESTS: nvmlGpmMetricId_t = 192;
pub const NVML_GPM_METRIC_GR4_CTXSW_CYCLES_PER_REQ: nvmlGpmMetricId_t = 193;
pub const NVML_GPM_METRIC_GR4_CTXSW_ACTIVE_PCT: nvmlGpmMetricId_t = 194;
pub const NVML_GPM_METRIC_GR5_CTXSW_CYCLES_ELAPSED: nvmlGpmMetricId_t = 195;
pub const NVML_GPM_METRIC_GR5_CTXSW_CYCLES_ACTIVE: nvmlGpmMetricId_t = 196;
pub const NVML_GPM_METRIC_GR5_CTXSW_REQUESTS: nvmlGpmMetricId_t = 197;
pub const NVML_GPM_METRIC_GR5_CTXSW_CYCLES_PER_REQ: nvmlGpmMetricId_t = 198;
pub const NVML_GPM_METRIC_GR5_CTXSW_ACTIVE_PCT: nvmlGpmMetricId_t = 199;
pub const NVML_GPM_METRIC_GR6_CTXSW_CYCLES_ELAPSED: nvmlGpmMetricId_t = 200;
pub const NVML_GPM_METRIC_GR6_CTXSW_CYCLES_ACTIVE: nvmlGpmMetricId_t = 201;
pub const NVML_GPM_METRIC_GR6_CTXSW_REQUESTS: nvmlGpmMetricId_t = 202;
pub const NVML_GPM_METRIC_GR6_CTXSW_CYCLES_PER_REQ: nvmlGpmMetricId_t = 203;
pub const NVML_GPM_METRIC_GR6_CTXSW_ACTIVE_PCT: nvmlGpmMetricId_t = 204;
pub const NVML_GPM_METRIC_GR7_CTXSW_CYCLES_ELAPSED: nvmlGpmMetricId_t = 205;
pub const NVML_GPM_METRIC_GR7_CTXSW_CYCLES_ACTIVE: nvmlGpmMetricId_t = 206;
pub const NVML_GPM_METRIC_GR7_CTXSW_REQUESTS: nvmlGpmMetricId_t = 207;
pub const NVML_GPM_METRIC_GR7_CTXSW_CYCLES_PER_REQ: nvmlGpmMetricId_t = 208;
pub const NVML_GPM_METRIC_GR7_CTXSW_ACTIVE_PCT: nvmlGpmMetricId_t = 209;
pub const NVML_GPM_METRIC_MAX: nvmlGpmMetricId_t = 210;
pub const NVML_POWER_PROFILE_MAX_P: nvmlPowerProfileType_t = 0;
pub const NVML_POWER_PROFILE_MAX_Q: nvmlPowerProfileType_t = 1;
pub const NVML_POWER_PROFILE_COMPUTE: nvmlPowerProfileType_t = 2;
pub const NVML_POWER_PROFILE_MEMORY_BOUND: nvmlPowerProfileType_t = 3;
pub const NVML_POWER_PROFILE_NETWORK: nvmlPowerProfileType_t = 4;
pub const NVML_POWER_PROFILE_BALANCED: nvmlPowerProfileType_t = 5;
pub const NVML_POWER_PROFILE_LLM_INFERENCE: nvmlPowerProfileType_t = 6;
pub const NVML_POWER_PROFILE_LLM_TRAINING: nvmlPowerProfileType_t = 7;
pub const NVML_POWER_PROFILE_RBM: nvmlPowerProfileType_t = 8;
pub const NVML_POWER_PROFILE_DCPCIE: nvmlPowerProfileType_t = 9;
pub const NVML_POWER_PROFILE_HMMA_SPARSE: nvmlPowerProfileType_t = 10;
pub const NVML_POWER_PROFILE_HMMA_DENSE: nvmlPowerProfileType_t = 11;
pub const NVML_POWER_PROFILE_SYNC_BALANCED: nvmlPowerProfileType_t = 12;
pub const NVML_POWER_PROFILE_HPC: nvmlPowerProfileType_t = 13;
pub const NVML_POWER_PROFILE_MIG: nvmlPowerProfileType_t = 14;
pub const NVML_POWER_PROFILE_MAX: nvmlPowerProfileType_t = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmlValue_st {
    pub dVal: f64,
    pub siVal: ::std::os::raw::c_int,
    pub uiVal: ::std::os::raw::c_uint,
    pub ulVal: ::std::os::raw::c_ulong,
    pub ullVal: ::std::os::raw::c_ulonglong,
    pub sllVal: ::std::os::raw::c_longlong,
    pub usVal: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmlUUIDValue_t {
    pub str_: [::std::os::raw::c_char; 41usize],
    pub bytes: [::std::os::raw::c_uchar; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmlVgpuSchedulerParams_t {
    pub vgpuSchedDataWithARR: nvmlVgpuSchedulerParams_t__bindgen_ty_1,
    pub vgpuSchedData: nvmlVgpuSchedulerParams_t__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmlVgpuSchedulerSetParams_t {
    pub vgpuSchedDataWithARR: nvmlVgpuSchedulerSetParams_t__bindgen_ty_1,
    pub vgpuSchedData: nvmlVgpuSchedulerSetParams_t__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmlPRMTLV_v1_t__bindgen_ty_1 {
    pub inData: [::std::os::raw::c_uchar; 496usize],
    pub outData: [::std::os::raw::c_uchar; 496usize],
}
pub use self::nvmlBrandType_enum as nvmlBrandType_t;
pub use self::nvmlBridgeChipType_enum as nvmlBridgeChipType_t;
pub use self::nvmlClockId_enum as nvmlClockId_t;
pub use self::nvmlClockLimitId_enum as nvmlClockLimitId_t;
pub use self::nvmlClockType_enum as nvmlClockType_t;
pub use self::nvmlComputeMode_enum as nvmlComputeMode_t;
pub use self::nvmlCoolerControl_enum as nvmlCoolerControl_t;
pub use self::nvmlCoolerTarget_enum as nvmlCoolerTarget_t;
pub use self::nvmlDetachGpuState_enum as nvmlDetachGpuState_t;
pub use self::nvmlDeviceGpuRecoveryAction_s as nvmlDeviceGpuRecoveryAction_t;
pub use self::nvmlDeviceVgpuCapability_enum as nvmlDeviceVgpuCapability_t;
pub use self::nvmlDriverModel_enum as nvmlDriverModel_t;
pub use self::nvmlEccCounterType_enum as nvmlEccCounterType_t;
pub use self::nvmlEnableState_enum as nvmlEnableState_t;
pub use self::nvmlEncoderQueryType_enum as nvmlEncoderType_t;
pub use self::nvmlFBCSessionType_enum as nvmlFBCSessionType_t;
pub use self::nvmlFanState_enum as nvmlFanState_t;
pub use self::nvmlGom_enum as nvmlGpuOperationMode_t;
pub use self::nvmlGpuLevel_enum as nvmlGpuTopologyLevel_t;
pub use self::nvmlGpuP2PCapsIndex_enum as nvmlGpuP2PCapsIndex_t;
pub use self::nvmlGpuP2PStatus_enum as nvmlGpuP2PStatus_t;
pub use self::nvmlGpuVirtualizationMode as nvmlGpuVirtualizationMode_t;
pub use self::nvmlHostVgpuMode_enum as nvmlHostVgpuMode_t;
pub use self::nvmlInforomObject_enum as nvmlInforomObject_t;
pub use self::nvmlIntNvLinkDeviceType_enum as nvmlIntNvLinkDeviceType_t;
pub use self::nvmlLedColor_enum as nvmlLedColor_t;
pub use self::nvmlMemoryErrorType_enum as nvmlMemoryErrorType_t;
pub use self::nvmlMemoryLocation_enum as nvmlMemoryLocation_t;
pub use self::nvmlNvLinkCapability_enum as nvmlNvLinkCapability_t;
pub use self::nvmlNvLinkErrorCounter_enum as nvmlNvLinkErrorCounter_t;
pub use self::nvmlNvLinkUtilizationCountPktTypes_enum as nvmlNvLinkUtilizationCountPktTypes_t;
pub use self::nvmlNvLinkUtilizationCountUnits_enum as nvmlNvLinkUtilizationCountUnits_t;
pub use self::nvmlNvlinkVersion_enum as nvmlNvlinkVersion_t;
pub use self::nvmlPStates_enum as nvmlPstates_t;
pub use self::nvmlPageRetirementCause_enum as nvmlPageRetirementCause_t;
pub use self::nvmlPcieLinkState_enum as nvmlPcieLinkState_t;
pub use self::nvmlPcieUtilCounter_enum as nvmlPcieUtilCounter_t;
pub use self::nvmlPerfPolicyType_enum as nvmlPerfPolicyType_t;
pub use self::nvmlRestrictedAPI_enum as nvmlRestrictedAPI_t;
pub use self::nvmlReturn_enum as nvmlReturn_t;
pub use self::nvmlSamplingType_enum as nvmlSamplingType_t;
pub use self::nvmlTemperatureSensors_enum as nvmlTemperatureSensors_t;
pub use self::nvmlTemperatureThresholds_enum as nvmlTemperatureThresholds_t;
pub use self::nvmlValueType_enum as nvmlValueType_t;
pub use self::nvmlVgpuCapability_enum as nvmlVgpuCapability_t;
pub use self::nvmlVgpuDriverCapability_enum as nvmlVgpuDriverCapability_t;
pub use self::nvmlVgpuGuestInfoState_enum as nvmlVgpuGuestInfoState_t;
pub use self::nvmlVgpuPgpuCompatibilityLimitCode_enum as nvmlVgpuPgpuCompatibilityLimitCode_t;
pub use self::nvmlVgpuVmCompatibility_enum as nvmlVgpuVmCompatibility_t;
pub use self::nvmlVgpuVmIdType as nvmlVgpuVmIdType_t;
