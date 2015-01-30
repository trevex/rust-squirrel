#include <stdarg.h>
#include <string>
#include "squirrel.h"

// helper functions based on Henry's clean implementation: http://www.henrywrites.com/blog/post/8/Embedding_Squirrel_Lesson_2

#ifdef __cplusplus
extern "C" {
#endif
SQUIRREL_API void sq_helper_stack_trace(HSQUIRRELVM vm);
SQUIRREL_API SQInteger sq_helper_runtime_error(HSQUIRRELVM vm);
SQUIRREL_API void sq_helper_compile_error(HSQUIRRELVM vm, const SQChar* description, const SQChar* file, SQInteger line, SQInteger column);
SQUIRREL_API void sq_helper_print_function(HSQUIRRELVM vm, const SQChar *format, ...);
SQUIRREL_API void sq_helper_error_function(HSQUIRRELVM vm, const SQChar *format, ...);
SQUIRREL_API void sq_helper_setup_default_callbacks(HSQUIRRELVM vm);
#ifdef __cplusplus
}
#endif

void sq_helper_stack_trace(HSQUIRRELVM vm) {
    SQStackInfos stack_info;
    SQInteger stack_depth = 1;

    fprintf(stderr, "Stack Dump:\n");

    while(SQ_SUCCEEDED(sq_stackinfos(vm, stack_depth, &stack_info))) {
        const SQChar *func_name = (stack_info.funcname) ? stack_info.funcname : "unknown_function";
        const SQChar *source_file = (stack_info.source) ? stack_info.source : "unknown_source_file";
#ifdef _SQ64
        fprintf(stderr, "[%lld]: function [%s()] %s line [%lld]\n", stack_depth, func_name, source_file, stack_info.line);
#else
        fprintf(stderr, "[%d]: function [%s()] %s line [%d]\n", stack_depth, func_name, source_file, stack_info.line);
#endif
        stack_depth++;
    }
}

SQInteger sq_helper_runtime_error(HSQUIRRELVM vm) {
    const SQChar* error_message = NULL;
    if(sq_gettop(vm) >= 1) {
        if(SQ_SUCCEEDED(sq_getstring(vm, 2, &error_message))) {
            fprintf(stderr, "vm: %s.\n", error_message);
        }
        sq_helper_stack_trace(vm);
    }
    return 0;
}

void sq_helper_compile_error(HSQUIRRELVM vm, const SQChar* description, const SQChar* file, SQInteger line, SQInteger column) {
#ifdef _SQ64
    fprintf(stderr, "vm: '%s' (Ln:%lld Col:%lld) : %s.\n", file, line, column, description);
#else
    fprintf(stderr, "vm: '%s' (Ln:%d Col:%d) : %s.\n", file, line, column, description);
#endif
}

void sq_helper_print_function(HSQUIRRELVM vm, const SQChar *format, ...) {
    va_list args;
    va_start(args, format);
    vfprintf(stdout, format, args);
    va_end(args);
}

void sq_helper_error_function(HSQUIRRELVM vm, const SQChar *format, ...) {
    va_list args;
    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);
}

void sq_helper_setup_default_callbacks(HSQUIRRELVM vm) {
    sq_setprintfunc(vm, sq_helper_print_function, sq_helper_error_function);
    sq_setcompilererrorhandler(vm, sq_helper_compile_error);
    sq_newclosure(vm, sq_helper_runtime_error, 0);
    sq_seterrorhandler(vm);

    // TODO: remove test code
    const SQChar *program = "::print(\"Hello World!\\n\");";
    if (SQ_FAILED(sq_compilebuffer(vm, program, sizeof(SQChar) * strlen(program), "program", SQTrue))) {
        return;
    }
    sq_pushroottable(vm);
    if (SQ_FAILED(sq_call(vm, 1, SQFalse, SQTrue))) {
        return;
    }
}
