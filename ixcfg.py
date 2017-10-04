import os
import os.path
from ix import has_to_recompile as ix_has_to_recompile, compile_file
from ix.utils import index_of, replace_ext
import subprocess


DEBUG_RLIB = os.path.join(ROOTDIR, "target/debug/libporus.rlib")
SOLUTION_PATTERN = r'^(?P<oj>\w+)(?:/.*)?/(?P<problem>[A-Za-z0-9_\-]+)\.rs$'


def has_to_recompile(source, target, rlib=DEBUG_RLIB):
    if ix_has_to_recompile(source, target):
        return True

    if os.stat(rlib).st_mtime >= os.stat(target).st_mtime:
        return True

    return False


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


def get_llvm_target(compiler):
    return ( ({"x86": "i686", "x86_64": "x86_64"}[compiler.arch])
             + "-" +
             ({"Windows": "pc-windows", "Linux": "unknown-linux"}[compiler.os]) + "-gnu")



class SubmissionCompilerContext:
    LINKBC = os.path.join(ROOTDIR, "target/debug/linkbc")


    def __init__(self, llvm_target):
        self.llvm_target = llvm_target
        self.rlib_path = os.path.join(ROOTDIR, "target", llvm_target, "release/libporus.rlib")


    def check(self):
        argv = ['cargo', 'rustc', '--lib', '--release', '--target', self.llvm_target, '--', '--emit', 'llvm-bc']
        if compile_file(ROOTDIR, argv, self.rlib_path) is None:
            return False

        argv = ['ar', 't', self.rlib_path]
        members = subprocess.run(argv, stdout=subprocess.PIPE, check=True).stdout.splitlines()
        if not members:
            return False

        member = [m for m in members if m.endswith(b'.0.bytecode.encoded')][0]
        self.bc_path = os.path.join(ROOTDIR, "target", self.llvm_target, "release/deps", member[:-19].decode()+'.bc')

        src = 'src/bin/linkbc.rs'
        if ix_has_to_recompile(src, self.LINKBC):
            argv = ['cargo', 'build', '--bin=linkbc']
            if compile_file(ROOTDIR, argv, src, self.LINKBC) is None:
                return False

        return True


    def get_submit_argv(self, source, target):
        return ['rustc',
                "--emit", "llvm-bc",
                "-C", "opt-level=s",
                "-C", "panic=abort",
                "--target", self.llvm_target,
                '--extern', 'porus='+self.rlib_path,
                "-o", target, source]


    def get_linkbc_argv(self, source, target):
        argv = self.get_submit_argv(source, target)
        argv[0] = self.LINKBC
        return argv + ["--", target, self.bc_path, source]


    def compile(self, source):
        bc = replace_ext(source, "bc")

        if has_to_recompile(source, bc, self.rlib_path):
            argv = self.get_submit_argv(source, bc)
            if compile_file(ROOTDIR, argv, source, bc) is None:
                return None

        target = replace_ext(source, "s")

        if has_to_recompile(source, target, self.rlib_path):
            argv = self.get_linkbc_argv(bc, target)
            if compile_file(ROOTDIR, argv, bc, target) is None:
                return None

        return target



def prepare_submission(compilers, filename):
    compiler = pick_compiler(compilers)
    if not compiler:
        return None

    llvm_target = get_llvm_target(compiler)

    context = SubmissionCompilerContext(llvm_target)

    if not context.check():
        return None

    asm = context.compile(filename)
    if asm is None:
        return None

    with open(asm,'rb') as f:
        code = f.read()

    return compiler, code
