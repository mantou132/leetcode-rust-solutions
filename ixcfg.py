import os
import os.path
from ix import has_to_recompile as ix_has_to_recompile, compile_file
from ix.utils import index_of, replace_ext

SOLUTION_PATTERN = r'^(?P<oj>\w+)(?:/.*)?/(?P<problem>[A-Za-z0-9_\-]+)\.rs$'

def rlib_path(build):
    return os.path.join(ROOTDIR, "target", build, "libporus.rlib")

def bc_path(build):
    return os.path.join(ROOTDIR, "target", build, "deps/porus.bc")


DEBUG_RLIB = rlib_path("debug")
LINKBC = os.path.join(ROOTDIR, "target/debug/linkbc")


def has_to_recompile(source, target, build="debug"):
    if ix_has_to_recompile(source, target):
        return True

    if os.stat(rlib_path(build)).st_mtime >= os.stat(target).st_mtime:
        return True

    return False


def get_llvm_target(compiler):
    return ( ({"x86": "i686", "x86_64": "x86_64"}[compiler.arch])
             + "-" +
             ({"Windows": "pc-windows", "Linux": "unknown-linux"}[compiler.os]) + "-gnu")


def get_compile_argv(filename):
    if compile_file(ROOTDIR, ['cargo', 'build', '--lib'], 'src/lib.rs', DEBUG_RLIB) is None:
        raise Exception("failed to build library")

    target = replace_ext(filename,"elf")
    return ['rustc', '--extern', 'porus='+DEBUG_RLIB, "-o", target, filename], target


def pick_compiler(compilers):
    compilers = [c for c in compilers if c.lang == "C" and c.name in ("GCC", "MinGW")]
    compilers.sort(key=lambda c: (index_of(['Linux','Windows'], c.os), index_of(['x86_64','x86'], c.arch)))
    if compilers:
        return compilers[0]


def check_lib(llvm_target):
    build = llvm_target+ "/release"
    rlib = rlib_path(build)

    argv = ['cargo', 'build', '--lib', '--release', '--target', llvm_target]
    if compile_file(ROOTDIR, argv, "src/lib.rs", rlib) is None:
        return False

    bc = bc_path(build)
    argv = ['cargo', 'rustc', '--lib', '--release', '--target', llvm_target, '--', '--emit', 'llvm-bc']

    if compile_file(ROOTDIR, argv, rlib, bc) is None:
        return False

    return True


def check_linkbc():
    src = 'src/bin/linkbc.rs'
    if ix_has_to_recompile(src, LINKBC):
        argv = ['cargo', 'build', '--bin=linkbc']
        if compile_file(ROOTDIR, argv, src, LINKBC) is None:
            return False
    return True


def get_submit_argv(filename, target, llvm_target):
    return ['rustc',
            "--emit", "llvm-bc",
            "-C", "opt-level=s",
            "-C", "panic=abort",
            "--target", llvm_target,
            '--extern', 'porus='+rlib_path(llvm_target+"/release"),
            "-o", target, filename]


def get_linkbc_argv(filename, target, llvm_target):
    argv = get_submit_argv(filename, target, llvm_target)
    argv[0] = LINKBC
    return argv + ["--", target, filename, bc_path(llvm_target+"/release")]


ESCAPE_CHARS = {
    '\\':'\\\\','\"':r'\"','\n':r'\n','\t':r'\t'
}

def escape_asm(asm):
    return '__asm__("' + ''.join(ESCAPE_CHARS.get(c,c) for c in asm) + '");'


def prepare_submission(compilers, filename):
    compiler = pick_compiler(compilers)
    if not compiler:
        return None

    llvm_target = get_llvm_target(compiler)
    build = llvm_target + '/release'

    if not check_lib(llvm_target):
        return None

    if not check_linkbc():
        return None

    bc = replace_ext(filename, "bc")
    if has_to_recompile(filename, bc, build):
        argv = get_submit_argv(filename, bc, llvm_target)
        if compile_file(ROOTDIR, argv, filename, bc) is None:
            return None

    asm = replace_ext(filename, "s")
    if has_to_recompile(filename, asm, build):
        argv = get_linkbc_argv(bc, asm, llvm_target)
        if compile_file(ROOTDIR, argv, bc, asm) is None:
            return None

    with open(asm,'rb') as f:
        code = f.read()

    return compiler, escape_asm(code.decode("utf-8"))
