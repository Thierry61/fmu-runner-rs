/* automatically generated by rust-bindgen 0.65.1 */

pub const fmi2TypesPlatform: &[u8; 8usize] = b"default\0";
pub const fmi2True: u32 = 1;
pub const fmi2False: u32 = 0;
pub const fmi2Version: &[u8; 4usize] = b"2.0\0";
pub type fmi2Component = *mut ::std::os::raw::c_void;
pub type fmi2ComponentEnvironment = *mut ::std::os::raw::c_void;
pub type fmi2FMUstate = *mut ::std::os::raw::c_void;
pub type fmi2ValueReference = ::std::os::raw::c_uint;
pub type fmi2Real = f64;
pub type fmi2Integer = ::std::os::raw::c_int;
pub type fmi2Boolean = ::std::os::raw::c_int;
pub type fmi2Char = ::std::os::raw::c_char;
pub type fmi2String = *const fmi2Char;
pub type fmi2Byte = ::std::os::raw::c_char;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum fmi2Status {
    fmi2OK = 0,
    fmi2Warning = 1,
    fmi2Discard = 2,
    fmi2Error = 3,
    fmi2Fatal = 4,
    fmi2Pending = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum fmi2Type {
    fmi2ModelExchange = 0,
    fmi2CoSimulation = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum fmi2StatusKind {
    fmi2DoStepStatus = 0,
    fmi2PendingStatus = 1,
    fmi2LastSuccessfulTime = 2,
    fmi2Terminated = 3,
}
pub type fmi2CallbackLogger = ::std::option::Option<
    unsafe extern "C" fn(
        componentEnvironment: fmi2ComponentEnvironment,
        instanceName: fmi2String,
        status: fmi2Status,
        category: fmi2String,
        message: fmi2String,
        ...
    ),
>;
pub type fmi2CallbackAllocateMemory = ::std::option::Option<
    unsafe extern "C" fn(nobj: usize, size: usize) -> *mut ::std::os::raw::c_void,
>;
pub type fmi2CallbackFreeMemory =
    ::std::option::Option<unsafe extern "C" fn(obj: *mut ::std::os::raw::c_void)>;
pub type fmi2StepFinished = ::std::option::Option<
    unsafe extern "C" fn(componentEnvironment: fmi2ComponentEnvironment, status: fmi2Status),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmi2CallbackFunctions {
    pub logger: fmi2CallbackLogger,
    pub allocateMemory: fmi2CallbackAllocateMemory,
    pub freeMemory: fmi2CallbackFreeMemory,
    pub stepFinished: fmi2StepFinished,
    pub componentEnvironment: fmi2ComponentEnvironment,
}
#[test]
fn bindgen_test_layout_fmi2CallbackFunctions() {
    const UNINIT: ::std::mem::MaybeUninit<fmi2CallbackFunctions> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<fmi2CallbackFunctions>(),
        40usize,
        concat!("Size of: ", stringify!(fmi2CallbackFunctions))
    );
    assert_eq!(
        ::std::mem::align_of::<fmi2CallbackFunctions>(),
        8usize,
        concat!("Alignment of ", stringify!(fmi2CallbackFunctions))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).logger) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2CallbackFunctions),
            "::",
            stringify!(logger)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).allocateMemory) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2CallbackFunctions),
            "::",
            stringify!(allocateMemory)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).freeMemory) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2CallbackFunctions),
            "::",
            stringify!(freeMemory)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stepFinished) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2CallbackFunctions),
            "::",
            stringify!(stepFinished)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).componentEnvironment) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2CallbackFunctions),
            "::",
            stringify!(componentEnvironment)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmi2EventInfo {
    pub newDiscreteStatesNeeded: fmi2Boolean,
    pub terminateSimulation: fmi2Boolean,
    pub nominalsOfContinuousStatesChanged: fmi2Boolean,
    pub valuesOfContinuousStatesChanged: fmi2Boolean,
    pub nextEventTimeDefined: fmi2Boolean,
    pub nextEventTime: fmi2Real,
}
#[test]
fn bindgen_test_layout_fmi2EventInfo() {
    const UNINIT: ::std::mem::MaybeUninit<fmi2EventInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<fmi2EventInfo>(),
        32usize,
        concat!("Size of: ", stringify!(fmi2EventInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<fmi2EventInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(fmi2EventInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).newDiscreteStatesNeeded) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2EventInfo),
            "::",
            stringify!(newDiscreteStatesNeeded)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).terminateSimulation) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2EventInfo),
            "::",
            stringify!(terminateSimulation)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).nominalsOfContinuousStatesChanged) as usize - ptr as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2EventInfo),
            "::",
            stringify!(nominalsOfContinuousStatesChanged)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).valuesOfContinuousStatesChanged) as usize - ptr as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2EventInfo),
            "::",
            stringify!(valuesOfContinuousStatesChanged)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nextEventTimeDefined) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2EventInfo),
            "::",
            stringify!(nextEventTimeDefined)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nextEventTime) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(fmi2EventInfo),
            "::",
            stringify!(nextEventTime)
        )
    );
}
#[doc = "Types for Common Functions"]
pub type fmi2GetTypesPlatformTYPE =
    ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>;
pub type fmi2GetVersionTYPE =
    ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>;
pub type fmi2SetDebugLoggingTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        loggingOn: fmi2Boolean,
        nCategories: usize,
        categories: *const fmi2String,
    ) -> fmi2Status,
>;
pub type fmi2InstantiateTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        instanceName: fmi2String,
        fmuType: fmi2Type,
        fmuGUID: fmi2String,
        fmuResourceLocation: fmi2String,
        functions: *const fmi2CallbackFunctions,
        visible: fmi2Boolean,
        loggingOn: fmi2Boolean,
    ) -> fmi2Component,
>;
pub type fmi2FreeInstanceTYPE = ::std::option::Option<unsafe extern "C" fn(c: fmi2Component)>;
pub type fmi2SetupExperimentTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        toleranceDefined: fmi2Boolean,
        tolerance: fmi2Real,
        startTime: fmi2Real,
        stopTimeDefined: fmi2Boolean,
        stopTime: fmi2Real,
    ) -> fmi2Status,
>;
pub type fmi2EnterInitializationModeTYPE =
    ::std::option::Option<unsafe extern "C" fn(c: fmi2Component) -> fmi2Status>;
pub type fmi2ExitInitializationModeTYPE =
    ::std::option::Option<unsafe extern "C" fn(c: fmi2Component) -> fmi2Status>;
pub type fmi2TerminateTYPE =
    ::std::option::Option<unsafe extern "C" fn(c: fmi2Component) -> fmi2Status>;
pub type fmi2ResetTYPE =
    ::std::option::Option<unsafe extern "C" fn(c: fmi2Component) -> fmi2Status>;
pub type fmi2GetRealTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Real,
    ) -> fmi2Status,
>;
pub type fmi2GetIntegerTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Integer,
    ) -> fmi2Status,
>;
pub type fmi2GetBooleanTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Boolean,
    ) -> fmi2Status,
>;
pub type fmi2GetStringTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2String,
    ) -> fmi2Status,
>;
pub type fmi2SetRealTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Real,
    ) -> fmi2Status,
>;
pub type fmi2SetIntegerTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Integer,
    ) -> fmi2Status,
>;
pub type fmi2SetBooleanTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Boolean,
    ) -> fmi2Status,
>;
pub type fmi2SetStringTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2String,
    ) -> fmi2Status,
>;
pub type fmi2GetFMUstateTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, FMUstate: *mut fmi2FMUstate) -> fmi2Status,
>;
pub type fmi2SetFMUstateTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, FMUstate: fmi2FMUstate) -> fmi2Status,
>;
pub type fmi2FreeFMUstateTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, FMUstate: *mut fmi2FMUstate) -> fmi2Status,
>;
pub type fmi2SerializedFMUstateSizeTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, FMUstate: fmi2FMUstate, size: *mut usize) -> fmi2Status,
>;
pub type fmi2SerializeFMUstateTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        FMUstate: fmi2FMUstate,
        serializedState: *mut fmi2Byte,
        size: usize,
    ) -> fmi2Status,
>;
pub type fmi2DeSerializeFMUstateTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        serializedState: *const fmi2Byte,
        size: usize,
        FMUstate: *mut fmi2FMUstate,
    ) -> fmi2Status,
>;
pub type fmi2GetDirectionalDerivativeTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vUnknown_ref: *const fmi2ValueReference,
        nUnknown: usize,
        vKnown_ref: *const fmi2ValueReference,
        nKnown: usize,
        dvKnown: *const fmi2Real,
        dvUnknown: *mut fmi2Real,
    ) -> fmi2Status,
>;
#[doc = "Types for Functions for FMI2 for Model Exchange"]
pub type fmi2EnterEventModeTYPE =
    ::std::option::Option<unsafe extern "C" fn(c: fmi2Component) -> fmi2Status>;
pub type fmi2NewDiscreteStatesTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, fmi2eventInfo: *mut fmi2EventInfo) -> fmi2Status,
>;
pub type fmi2EnterContinuousTimeModeTYPE =
    ::std::option::Option<unsafe extern "C" fn(c: fmi2Component) -> fmi2Status>;
pub type fmi2CompletedIntegratorStepTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        noSetFMUStatePriorToCurrentPoint: fmi2Boolean,
        enterEventMode: *mut fmi2Boolean,
        terminateSimulation: *mut fmi2Boolean,
    ) -> fmi2Status,
>;
pub type fmi2SetTimeTYPE =
    ::std::option::Option<unsafe extern "C" fn(c: fmi2Component, time: fmi2Real) -> fmi2Status>;
pub type fmi2SetContinuousStatesTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, x: *const fmi2Real, nx: usize) -> fmi2Status,
>;
pub type fmi2GetDerivativesTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, derivatives: *mut fmi2Real, nx: usize) -> fmi2Status,
>;
pub type fmi2GetEventIndicatorsTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, eventIndicators: *mut fmi2Real, ni: usize) -> fmi2Status,
>;
pub type fmi2GetContinuousStatesTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, x: *mut fmi2Real, nx: usize) -> fmi2Status,
>;
pub type fmi2GetNominalsOfContinuousStatesTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, x_nominal: *mut fmi2Real, nx: usize) -> fmi2Status,
>;
#[doc = "Types for Functions for FMI2 for Co-Simulation"]
pub type fmi2SetRealInputDerivativesTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        order: *const fmi2Integer,
        value: *const fmi2Real,
    ) -> fmi2Status,
>;
pub type fmi2GetRealOutputDerivativesTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        order: *const fmi2Integer,
        value: *mut fmi2Real,
    ) -> fmi2Status,
>;
pub type fmi2DoStepTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        currentCommunicationPoint: fmi2Real,
        communicationStepSize: fmi2Real,
        noSetFMUStatePriorToCurrentPoint: fmi2Boolean,
    ) -> fmi2Status,
>;
pub type fmi2CancelStepTYPE =
    ::std::option::Option<unsafe extern "C" fn(c: fmi2Component) -> fmi2Status>;
pub type fmi2GetStatusTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, s: fmi2StatusKind, value: *mut fmi2Status) -> fmi2Status,
>;
pub type fmi2GetRealStatusTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, s: fmi2StatusKind, value: *mut fmi2Real) -> fmi2Status,
>;
pub type fmi2GetIntegerStatusTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        s: fmi2StatusKind,
        value: *mut fmi2Integer,
    ) -> fmi2Status,
>;
pub type fmi2GetBooleanStatusTYPE = ::std::option::Option<
    unsafe extern "C" fn(
        c: fmi2Component,
        s: fmi2StatusKind,
        value: *mut fmi2Boolean,
    ) -> fmi2Status,
>;
pub type fmi2GetStringStatusTYPE = ::std::option::Option<
    unsafe extern "C" fn(c: fmi2Component, s: fmi2StatusKind, value: *mut fmi2String) -> fmi2Status,
>;
extern "C" {
    pub fn fmi2GetTypesPlatform() -> *const ::std::os::raw::c_char;
    pub fn fmi2GetVersion() -> *const ::std::os::raw::c_char;
    pub fn fmi2SetDebugLogging(
        c: fmi2Component,
        loggingOn: fmi2Boolean,
        nCategories: usize,
        categories: *const fmi2String,
    ) -> fmi2Status;
    pub fn fmi2Instantiate(
        instanceName: fmi2String,
        fmuType: fmi2Type,
        fmuGUID: fmi2String,
        fmuResourceLocation: fmi2String,
        functions: *const fmi2CallbackFunctions,
        visible: fmi2Boolean,
        loggingOn: fmi2Boolean,
    ) -> fmi2Component;
    pub fn fmi2FreeInstance(c: fmi2Component);
    pub fn fmi2SetupExperiment(
        c: fmi2Component,
        toleranceDefined: fmi2Boolean,
        tolerance: fmi2Real,
        startTime: fmi2Real,
        stopTimeDefined: fmi2Boolean,
        stopTime: fmi2Real,
    ) -> fmi2Status;
    pub fn fmi2EnterInitializationMode(c: fmi2Component) -> fmi2Status;
    pub fn fmi2ExitInitializationMode(c: fmi2Component) -> fmi2Status;
    pub fn fmi2Terminate(c: fmi2Component) -> fmi2Status;
    pub fn fmi2Reset(c: fmi2Component) -> fmi2Status;
    pub fn fmi2GetReal(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Real,
    ) -> fmi2Status;
    pub fn fmi2GetInteger(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Integer,
    ) -> fmi2Status;
    pub fn fmi2GetBoolean(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2Boolean,
    ) -> fmi2Status;
    pub fn fmi2GetString(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *mut fmi2String,
    ) -> fmi2Status;
    pub fn fmi2SetReal(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Real,
    ) -> fmi2Status;
    pub fn fmi2SetInteger(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Integer,
    ) -> fmi2Status;
    pub fn fmi2SetBoolean(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2Boolean,
    ) -> fmi2Status;
    pub fn fmi2SetString(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        value: *const fmi2String,
    ) -> fmi2Status;
    pub fn fmi2GetFMUstate(c: fmi2Component, FMUstate: *mut fmi2FMUstate) -> fmi2Status;
    pub fn fmi2SetFMUstate(c: fmi2Component, FMUstate: fmi2FMUstate) -> fmi2Status;
    pub fn fmi2FreeFMUstate(c: fmi2Component, FMUstate: *mut fmi2FMUstate) -> fmi2Status;
    pub fn fmi2SerializedFMUstateSize(
        c: fmi2Component,
        FMUstate: fmi2FMUstate,
        size: *mut usize,
    ) -> fmi2Status;
    pub fn fmi2SerializeFMUstate(
        c: fmi2Component,
        FMUstate: fmi2FMUstate,
        serializedState: *mut fmi2Byte,
        size: usize,
    ) -> fmi2Status;
    pub fn fmi2DeSerializeFMUstate(
        c: fmi2Component,
        serializedState: *const fmi2Byte,
        size: usize,
        FMUstate: *mut fmi2FMUstate,
    ) -> fmi2Status;
    pub fn fmi2GetDirectionalDerivative(
        c: fmi2Component,
        vUnknown_ref: *const fmi2ValueReference,
        nUnknown: usize,
        vKnown_ref: *const fmi2ValueReference,
        nKnown: usize,
        dvKnown: *const fmi2Real,
        dvUnknown: *mut fmi2Real,
    ) -> fmi2Status;
    pub fn fmi2EnterEventMode(c: fmi2Component) -> fmi2Status;
    pub fn fmi2NewDiscreteStates(c: fmi2Component, fmi2eventInfo: *mut fmi2EventInfo)
        -> fmi2Status;
    pub fn fmi2EnterContinuousTimeMode(c: fmi2Component) -> fmi2Status;
    pub fn fmi2CompletedIntegratorStep(
        c: fmi2Component,
        noSetFMUStatePriorToCurrentPoint: fmi2Boolean,
        enterEventMode: *mut fmi2Boolean,
        terminateSimulation: *mut fmi2Boolean,
    ) -> fmi2Status;
    pub fn fmi2SetTime(c: fmi2Component, time: fmi2Real) -> fmi2Status;
    pub fn fmi2SetContinuousStates(c: fmi2Component, x: *const fmi2Real, nx: usize) -> fmi2Status;
    pub fn fmi2GetDerivatives(
        c: fmi2Component,
        derivatives: *mut fmi2Real,
        nx: usize,
    ) -> fmi2Status;
    pub fn fmi2GetEventIndicators(
        c: fmi2Component,
        eventIndicators: *mut fmi2Real,
        ni: usize,
    ) -> fmi2Status;
    pub fn fmi2GetContinuousStates(c: fmi2Component, x: *mut fmi2Real, nx: usize) -> fmi2Status;
    pub fn fmi2GetNominalsOfContinuousStates(
        c: fmi2Component,
        x_nominal: *mut fmi2Real,
        nx: usize,
    ) -> fmi2Status;
    pub fn fmi2SetRealInputDerivatives(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        order: *const fmi2Integer,
        value: *const fmi2Real,
    ) -> fmi2Status;
    pub fn fmi2GetRealOutputDerivatives(
        c: fmi2Component,
        vr: *const fmi2ValueReference,
        nvr: usize,
        order: *const fmi2Integer,
        value: *mut fmi2Real,
    ) -> fmi2Status;
    pub fn fmi2DoStep(
        c: fmi2Component,
        currentCommunicationPoint: fmi2Real,
        communicationStepSize: fmi2Real,
        noSetFMUStatePriorToCurrentPoint: fmi2Boolean,
    ) -> fmi2Status;
    pub fn fmi2CancelStep(c: fmi2Component) -> fmi2Status;
    pub fn fmi2GetStatus(c: fmi2Component, s: fmi2StatusKind, value: *mut fmi2Status)
        -> fmi2Status;
    pub fn fmi2GetRealStatus(
        c: fmi2Component,
        s: fmi2StatusKind,
        value: *mut fmi2Real,
    ) -> fmi2Status;
    pub fn fmi2GetIntegerStatus(
        c: fmi2Component,
        s: fmi2StatusKind,
        value: *mut fmi2Integer,
    ) -> fmi2Status;
    pub fn fmi2GetBooleanStatus(
        c: fmi2Component,
        s: fmi2StatusKind,
        value: *mut fmi2Boolean,
    ) -> fmi2Status;
    pub fn fmi2GetStringStatus(
        c: fmi2Component,
        s: fmi2StatusKind,
        value: *mut fmi2String,
    ) -> fmi2Status;
}
