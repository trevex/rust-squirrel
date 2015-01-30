pub type SQInteger = ::libc::c_longlong;
pub type SQUnsignedInteger = ::libc::c_ulonglong;
pub type SQHash = ::libc::c_ulonglong;
pub type SQInt32 = ::libc::c_int;
pub type SQUnsignedInteger32 = ::libc::c_uint;
pub type SQFloat = ::libc::c_float;
pub type SQRawObjectVal = ::libc::c_longlong;
pub type SQUserPointer = *mut ::libc::c_void;
pub type SQBool = SQUnsignedInteger;
pub type SQRESULT = SQInteger;
pub enum Struct_SQVM { }
pub enum Struct_SQTable { }
pub enum Struct_SQArray { }
pub enum Struct_SQString { }
pub enum Struct_SQClosure { }
pub enum Struct_SQGenerator { }
pub enum Struct_SQNativeClosure { }
pub enum Struct_SQUserData { }
pub enum Struct_SQFunctionProto { }
pub enum Struct_SQRefCounted { }
pub enum Struct_SQClass { }
pub enum Struct_SQInstance { }
pub enum Struct_SQDelegable { }
pub enum Struct_SQOuter { }
pub type SQChar = ::libc::c_char;
pub type Enum_tagSQObjectType = ::libc::c_uint;
pub const OT_NULL: ::libc::c_uint = 16777217;
pub const OT_INTEGER: ::libc::c_uint = 83886082;
pub const OT_FLOAT: ::libc::c_uint = 83886084;
pub const OT_BOOL: ::libc::c_uint = 16777224;
pub const OT_STRING: ::libc::c_uint = 134217744;
pub const OT_TABLE: ::libc::c_uint = 167772192;
pub const OT_ARRAY: ::libc::c_uint = 134217792;
pub const OT_USERDATA: ::libc::c_uint = 167772288;
pub const OT_CLOSURE: ::libc::c_uint = 134217984;
pub const OT_NATIVECLOSURE: ::libc::c_uint = 134218240;
pub const OT_GENERATOR: ::libc::c_uint = 134218752;
pub const OT_USERPOINTER: ::libc::c_uint = 2048;
pub const OT_THREAD: ::libc::c_uint = 134221824;
pub const OT_FUNCPROTO: ::libc::c_uint = 134225920;
pub const OT_CLASS: ::libc::c_uint = 134234112;
pub const OT_INSTANCE: ::libc::c_uint = 167804928;
pub const OT_WEAKREF: ::libc::c_uint = 134283264;
pub const OT_OUTER: ::libc::c_uint = 134348800;
pub type SQObjectType = Enum_tagSQObjectType;
pub enum Struct_SQWeakRef { }
#[repr(C)]
#[derive(Copy)]
pub struct Union_tagSQObjectValue {
    pub _bindgen_data_: [u64; 1us],
}
impl Union_tagSQObjectValue {
    pub unsafe fn pTable(&mut self) -> *mut *mut Struct_SQTable {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pArray(&mut self) -> *mut *mut Struct_SQArray {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pClosure(&mut self) -> *mut *mut Struct_SQClosure {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pOuter(&mut self) -> *mut *mut Struct_SQOuter {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pGenerator(&mut self) -> *mut *mut Struct_SQGenerator {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pNativeClosure(&mut self)
     -> *mut *mut Struct_SQNativeClosure {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pString(&mut self) -> *mut *mut Struct_SQString {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pUserData(&mut self) -> *mut *mut Struct_SQUserData {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn nInteger(&mut self) -> *mut SQInteger {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn fFloat(&mut self) -> *mut SQFloat {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pUserPointer(&mut self) -> *mut SQUserPointer {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pFunctionProto(&mut self)
     -> *mut *mut Struct_SQFunctionProto {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pRefCounted(&mut self) -> *mut *mut Struct_SQRefCounted {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pDelegable(&mut self) -> *mut *mut Struct_SQDelegable {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pThread(&mut self) -> *mut *mut Struct_SQVM {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pClass(&mut self) -> *mut *mut Struct_SQClass {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pInstance(&mut self) -> *mut *mut Struct_SQInstance {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn pWeakRef(&mut self) -> *mut *mut Struct_SQWeakRef {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn raw(&mut self) -> *mut SQRawObjectVal {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_tagSQObjectValue {
    fn default() -> Union_tagSQObjectValue { unsafe { ::std::mem::zeroed() } }
}
pub type SQObjectValue = Union_tagSQObjectValue;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_tagSQObject {
    pub _type: SQObjectType,
    pub _unVal: SQObjectValue,
}
impl ::std::default::Default for Struct_tagSQObject {
    fn default() -> Struct_tagSQObject { unsafe { ::std::mem::zeroed() } }
}
pub type SQObject = Struct_tagSQObject;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_tagSQMemberHandle {
    pub _static: SQBool,
    pub _index: SQInteger,
}
impl ::std::default::Default for Struct_tagSQMemberHandle {
    fn default() -> Struct_tagSQMemberHandle {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type SQMemberHandle = Struct_tagSQMemberHandle;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_tagSQStackInfos {
    pub funcname: *const SQChar,
    pub source: *const SQChar,
    pub line: SQInteger,
}
impl ::std::default::Default for Struct_tagSQStackInfos {
    fn default() -> Struct_tagSQStackInfos { unsafe { ::std::mem::zeroed() } }
}
pub type SQStackInfos = Struct_tagSQStackInfos;
pub type HSQUIRRELVM = *mut Struct_SQVM;
pub type HSQOBJECT = SQObject;
pub type HSQMEMBERHANDLE = SQMemberHandle;
pub type SQFUNCTION =
    ::std::option::Option<extern "C" fn(arg1: HSQUIRRELVM) -> SQInteger>;
pub type SQRELEASEHOOK =
    ::std::option::Option<extern "C" fn(arg1: SQUserPointer, size: SQInteger)
                              -> SQInteger>;
pub type SQCOMPILERERROR =
    ::std::option::Option<extern "C" fn
                              (arg1: HSQUIRRELVM, arg2: *const SQChar,
                               arg3: *const SQChar, arg4: SQInteger,
                               arg5: SQInteger) -> ()>;
pub type SQPRINTFUNCTION =
    ::std::option::Option<extern "C" fn
                              (arg1: HSQUIRRELVM, arg2: *const SQChar, ...)
                              -> ()>;
pub type SQDEBUGHOOK =
    ::std::option::Option<extern "C" fn
                              (arg1: HSQUIRRELVM, arg2: SQInteger,
                               arg3: *const SQChar, arg4: SQInteger,
                               arg5: *const SQChar) -> ()>;
pub type SQWRITEFUNC =
    ::std::option::Option<extern "C" fn
                              (arg1: SQUserPointer, arg2: SQUserPointer,
                               arg3: SQInteger) -> SQInteger>;
pub type SQREADFUNC =
    ::std::option::Option<extern "C" fn
                              (arg1: SQUserPointer, arg2: SQUserPointer,
                               arg3: SQInteger) -> SQInteger>;
pub type SQLEXREADFUNC =
    ::std::option::Option<extern "C" fn(arg1: SQUserPointer) -> SQInteger>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_tagSQRegFunction {
    pub name: *const SQChar,
    pub f: SQFUNCTION,
    pub nparamscheck: SQInteger,
    pub typemask: *const SQChar,
}
impl ::std::default::Default for Struct_tagSQRegFunction {
    fn default() -> Struct_tagSQRegFunction {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type SQRegFunction = Struct_tagSQRegFunction;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_tagSQFunctionInfo {
    pub funcid: SQUserPointer,
    pub name: *const SQChar,
    pub source: *const SQChar,
}
impl ::std::default::Default for Struct_tagSQFunctionInfo {
    fn default() -> Struct_tagSQFunctionInfo {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type SQFunctionInfo = Struct_tagSQFunctionInfo;

extern "C" {
    pub fn sq_open(initialstacksize: SQInteger) -> HSQUIRRELVM;
    pub fn sq_newthread(friendvm: HSQUIRRELVM, initialstacksize: SQInteger) -> HSQUIRRELVM;
    pub fn sq_seterrorhandler(v: HSQUIRRELVM) -> ();
    pub fn sq_close(v: HSQUIRRELVM) -> ();
    pub fn sq_setforeignptr(v: HSQUIRRELVM, p: SQUserPointer) -> ();
    pub fn sq_getforeignptr(v: HSQUIRRELVM) -> SQUserPointer;
    pub fn sq_setprintfunc(v: HSQUIRRELVM, printfunc: SQPRINTFUNCTION, errfunc: SQPRINTFUNCTION) -> ();
    pub fn sq_getprintfunc(v: HSQUIRRELVM) -> SQPRINTFUNCTION;
    pub fn sq_geterrorfunc(v: HSQUIRRELVM) -> SQPRINTFUNCTION;
    pub fn sq_suspendvm(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_wakeupvm(v: HSQUIRRELVM, resumedret: SQBool, retval: SQBool, raiseerror: SQBool, throwerror: SQBool) -> SQRESULT;
    pub fn sq_getvmstate(v: HSQUIRRELVM) -> SQInteger;
    pub fn sq_getversion() -> SQInteger;
    pub fn sq_compile(v: HSQUIRRELVM, read: SQLEXREADFUNC, p: SQUserPointer, sourcename: *const SQChar, raiseerror: SQBool) -> SQRESULT;
    pub fn sq_compilebuffer(v: HSQUIRRELVM, s: *const SQChar, size: SQInteger, sourcename: *const SQChar, raiseerror: SQBool) -> SQRESULT;
    pub fn sq_enabledebuginfo(v: HSQUIRRELVM, enable: SQBool) -> ();
    pub fn sq_notifyallexceptions(v: HSQUIRRELVM, enable: SQBool) -> ();
    pub fn sq_setcompilererrorhandler(v: HSQUIRRELVM, f: SQCOMPILERERROR) -> ();
    pub fn sq_push(v: HSQUIRRELVM, idx: SQInteger) -> ();
    pub fn sq_pop(v: HSQUIRRELVM, nelemstopop: SQInteger) -> ();
    pub fn sq_poptop(v: HSQUIRRELVM) -> ();
    pub fn sq_remove(v: HSQUIRRELVM, idx: SQInteger) -> ();
    pub fn sq_gettop(v: HSQUIRRELVM) -> SQInteger;
    pub fn sq_settop(v: HSQUIRRELVM, newtop: SQInteger) -> ();
    pub fn sq_reservestack(v: HSQUIRRELVM, nsize: SQInteger) -> SQRESULT;
    pub fn sq_cmp(v: HSQUIRRELVM) -> SQInteger;
    pub fn sq_move(dest: HSQUIRRELVM, src: HSQUIRRELVM, idx: SQInteger) -> ();
    pub fn sq_newuserdata(v: HSQUIRRELVM, size: SQUnsignedInteger) -> SQUserPointer;
    pub fn sq_newtable(v: HSQUIRRELVM) -> ();
    pub fn sq_newtableex(v: HSQUIRRELVM, initialcapacity: SQInteger) -> ();
    pub fn sq_newarray(v: HSQUIRRELVM, size: SQInteger) -> ();
    pub fn sq_newclosure(v: HSQUIRRELVM, func: SQFUNCTION, nfreevars: SQUnsignedInteger) -> ();
    pub fn sq_setparamscheck(v: HSQUIRRELVM, nparamscheck: SQInteger, typemask: *const SQChar) -> SQRESULT;
    pub fn sq_bindenv(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_pushstring(v: HSQUIRRELVM, s: *const SQChar, len: SQInteger) -> ();
    pub fn sq_pushfloat(v: HSQUIRRELVM, f: SQFloat) -> ();
    pub fn sq_pushinteger(v: HSQUIRRELVM, n: SQInteger) -> ();
    pub fn sq_pushbool(v: HSQUIRRELVM, b: SQBool) -> ();
    pub fn sq_pushuserpointer(v: HSQUIRRELVM, p: SQUserPointer) -> ();
    pub fn sq_pushnull(v: HSQUIRRELVM) -> ();
    pub fn sq_gettype(v: HSQUIRRELVM, idx: SQInteger) -> SQObjectType;
    pub fn sq_typeof(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getsize(v: HSQUIRRELVM, idx: SQInteger) -> SQInteger;
    pub fn sq_gethash(v: HSQUIRRELVM, idx: SQInteger) -> SQHash;
    pub fn sq_getbase(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_instanceof(v: HSQUIRRELVM) -> SQBool;
    pub fn sq_tostring(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_tobool(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool) -> ();
    pub fn sq_getstring(v: HSQUIRRELVM, idx: SQInteger, c: *mut *const SQChar) -> SQRESULT;
    pub fn sq_getinteger(v: HSQUIRRELVM, idx: SQInteger, i: *mut SQInteger) -> SQRESULT;
    pub fn sq_getfloat(v: HSQUIRRELVM, idx: SQInteger, f: *mut SQFloat) -> SQRESULT;
    pub fn sq_getbool(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool) -> SQRESULT;
    pub fn sq_getthread(v: HSQUIRRELVM, idx: SQInteger, thread: *mut HSQUIRRELVM) -> SQRESULT;
    pub fn sq_getuserpointer(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer) -> SQRESULT;
    pub fn sq_getuserdata(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer, typetag: *mut SQUserPointer) -> SQRESULT;
    pub fn sq_settypetag(v: HSQUIRRELVM, idx: SQInteger, typetag: SQUserPointer) -> SQRESULT;
    pub fn sq_gettypetag(v: HSQUIRRELVM, idx: SQInteger, typetag: *mut SQUserPointer) -> SQRESULT;
    pub fn sq_setreleasehook(v: HSQUIRRELVM, idx: SQInteger, hook: SQRELEASEHOOK) -> ();
    pub fn sq_getscratchpad(v: HSQUIRRELVM, minsize: SQInteger) -> *mut SQChar;
    pub fn sq_getfunctioninfo(v: HSQUIRRELVM, level: SQInteger, fi: *mut SQFunctionInfo) -> SQRESULT;
    pub fn sq_getclosureinfo(v: HSQUIRRELVM, idx: SQInteger, nparams: *mut SQUnsignedInteger, nfreevars: *mut SQUnsignedInteger) -> SQRESULT;
    pub fn sq_getclosurename(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_setnativeclosurename(v: HSQUIRRELVM, idx: SQInteger, name: *const SQChar) -> SQRESULT;
    pub fn sq_setinstanceup(v: HSQUIRRELVM, idx: SQInteger, p: SQUserPointer) -> SQRESULT;
    pub fn sq_getinstanceup(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer, typetag: SQUserPointer) -> SQRESULT;
    pub fn sq_setclassudsize(v: HSQUIRRELVM, idx: SQInteger, udsize: SQInteger) -> SQRESULT;
    pub fn sq_newclass(v: HSQUIRRELVM, hasbase: SQBool) -> SQRESULT;
    pub fn sq_createinstance(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_setattributes(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getattributes(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getclass(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_weakref(v: HSQUIRRELVM, idx: SQInteger) -> ();
    pub fn sq_getdefaultdelegate(v: HSQUIRRELVM, t: SQObjectType) -> SQRESULT;
    pub fn sq_getmemberhandle(v: HSQUIRRELVM, idx: SQInteger, handle: *mut HSQMEMBERHANDLE) -> SQRESULT;
    pub fn sq_getbyhandle(v: HSQUIRRELVM, idx: SQInteger, handle: *const HSQMEMBERHANDLE) -> SQRESULT;
    pub fn sq_setbyhandle(v: HSQUIRRELVM, idx: SQInteger, handle: *const HSQMEMBERHANDLE) -> SQRESULT;
    pub fn sq_pushroottable(v: HSQUIRRELVM) -> ();
    pub fn sq_pushregistrytable(v: HSQUIRRELVM) -> ();
    pub fn sq_pushconsttable(v: HSQUIRRELVM) -> ();
    pub fn sq_setroottable(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_setconsttable(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_newslot(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
    pub fn sq_deleteslot(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
    pub fn sq_set(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_get(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_rawget(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_rawset(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_rawdeleteslot(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
    pub fn sq_newmember(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
    pub fn sq_rawnewmember(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
    pub fn sq_arrayappend(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_arraypop(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
    pub fn sq_arrayresize(v: HSQUIRRELVM, idx: SQInteger, newsize: SQInteger) -> SQRESULT;
    pub fn sq_arrayreverse(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_arrayremove(v: HSQUIRRELVM, idx: SQInteger, itemidx: SQInteger) -> SQRESULT;
    pub fn sq_arrayinsert(v: HSQUIRRELVM, idx: SQInteger, destpos: SQInteger) -> SQRESULT;
    pub fn sq_setdelegate(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getdelegate(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_clone(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_setfreevariable(v: HSQUIRRELVM, idx: SQInteger, nval: SQUnsignedInteger) -> SQRESULT;
    pub fn sq_next(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getweakrefval(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_clear(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_call(v: HSQUIRRELVM, params: SQInteger, retval: SQBool, raiseerror: SQBool) -> SQRESULT;
    pub fn sq_resume(v: HSQUIRRELVM, retval: SQBool, raiseerror: SQBool) -> SQRESULT;
    pub fn sq_getlocal(v: HSQUIRRELVM, level: SQUnsignedInteger, idx: SQUnsignedInteger) -> *const SQChar;
    pub fn sq_getcallee(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_getfreevariable(v: HSQUIRRELVM, idx: SQInteger, nval: SQUnsignedInteger) -> *const SQChar;
    pub fn sq_throwerror(v: HSQUIRRELVM, err: *const SQChar) -> SQRESULT;
    pub fn sq_throwobject(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_reseterror(v: HSQUIRRELVM) -> ();
    pub fn sq_getlasterror(v: HSQUIRRELVM) -> ();
    pub fn sq_getstackobj(v: HSQUIRRELVM, idx: SQInteger, po: *mut HSQOBJECT) -> SQRESULT;
    pub fn sq_pushobject(v: HSQUIRRELVM, obj: HSQOBJECT) -> ();
    pub fn sq_addref(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> ();
    pub fn sq_release(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQBool;
    pub fn sq_getrefcount(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQUnsignedInteger;
    pub fn sq_resetobject(po: *mut HSQOBJECT) -> ();
    pub fn sq_objtostring(o: *const HSQOBJECT) -> *const SQChar;
    pub fn sq_objtobool(o: *const HSQOBJECT) -> SQBool;
    pub fn sq_objtointeger(o: *const HSQOBJECT) -> SQInteger;
    pub fn sq_objtofloat(o: *const HSQOBJECT) -> SQFloat;
    pub fn sq_objtouserpointer(o: *const HSQOBJECT) -> SQUserPointer;
    pub fn sq_getobjtypetag(o: *const HSQOBJECT, typetag: *mut SQUserPointer) -> SQRESULT;
    pub fn sq_collectgarbage(v: HSQUIRRELVM) -> SQInteger;
    pub fn sq_resurrectunreachable(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_writeclosure(vm: HSQUIRRELVM, writef: SQWRITEFUNC, up: SQUserPointer) -> SQRESULT;
    pub fn sq_readclosure(vm: HSQUIRRELVM, readf: SQREADFUNC, up: SQUserPointer) -> SQRESULT;
    pub fn sq_malloc(size: SQUnsignedInteger) -> *mut ::libc::c_void;
    pub fn sq_realloc(p: *mut ::libc::c_void, oldsize: SQUnsignedInteger, newsize: SQUnsignedInteger) -> *mut ::libc::c_void;
    pub fn sq_free(p: *mut ::libc::c_void, size: SQUnsignedInteger) -> ();
    pub fn sq_stackinfos(v: HSQUIRRELVM, level: SQInteger, si: *mut SQStackInfos) -> SQRESULT;
    pub fn sq_setdebughook(v: HSQUIRRELVM) -> ();
    pub fn sq_setnativedebughook(v: HSQUIRRELVM, hook: SQDEBUGHOOK) -> ();

    // HELPER
    pub fn sq_helper_setup_default_callbacks(v: HSQUIRRELVM) -> ();
}
