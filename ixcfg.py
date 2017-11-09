import os
import os.path
from ix import has_to_recompile as ix_has_to_recompile, compile_file
from ix.utils import index_of, replace_ext
import subprocess

DEBUG_EXTERNS = {
    "porus": os.path.join(ROOTDIR, "target/debug/libporus.rlib"),
    "porus_macros": os.path.join(ROOTDIR, "target/debug/libporus_macros.so"),
}
SOLUTION_PATTERN = r'^(?P<oj>\w+)(?:/.*)?/(?P<problem>[A-Za-z0-9_\-]+)\.rs$'

def extern(externs):
    return sum([['--extern', '{}={}'.format(k,v)] for k,v in externs.items()], [])


def has_to_recompile(source, target, rlibs=DEBUG_EXTERNS):
    if ix_has_to_recompile(source, target):
        return True

    for rlib in rlibs.values():
        if os.stat(rlib).st_mtime >= os.stat(target).st_mtime:
            return True

    return False


def get_compile_argv(filename):
    VERBOSE_FLAG = '-v' if VERBOSE else '-q'
    if compile_file(ROOTDIR, ['cargo', 'build', VERBOSE_FLAG, '--lib'], 'DEBUG LIB', DEBUG_EXTERNS["porus"]) is None:
        raise Exception("failed to build library")

    target = replace_ext(filename,"elf")
    return ['rustc'] + extern(DEBUG_EXTERNS) + ['-o', target, filename], target


def list_generated_files(filename):
    return [replace_ext(filename, ext) for ext in ["elf","bc","s"]]


def pick_env(envs):
    envs = [c for c in envs if c.lang == "C" and c.name in ("GCC", "MinGW")]
    envs.sort(key=lambda c: (index_of(['Linux','Windows'], c.os), index_of(['x86_64','x86'], c.arch)))
    if envs:
        return envs[0]


def get_llvm_target(env):
    return ( ({"x86": "i686", "x86_64": "x86_64"}[env.arch])
             + "-" +
             ({"Windows": "pc-windows", "Linux": "unknown-linux"}[env.os]) + "-gnu")



class SubmissionContext:


    def __init__(self, llvm_target):
        self.llvm_target = llvm_target
        self.externs = {
            "porus": os.path.join(ROOTDIR, "target", llvm_target, "release/libporus.rlib"),
            "porus_macros": os.path.join(ROOTDIR, "target/release/libporus_macros.so"),
        }
        self.verbose = '-v' if VERBOSE else '-q'


    def check(self):
        argv = ['cargo', 'rustc', self.verbose, '-p', 'porus', '--lib', '--release', '--target', self.llvm_target, '--', '--emit', 'llvm-bc']
        if compile_file(ROOTDIR, argv, self.externs["porus"]) is None:
            return False

        argv = ['ar', 't', self.externs["porus"]]
        members = subprocess.run(argv, stdout=subprocess.PIPE, check=True).stdout.splitlines()
        if not members:
            return False

        member = [m.split(b'.',1)[0] for m in members if m.endswith(b'.bytecode.encoded')][0]
        self.bc_path = os.path.join(ROOTDIR, "target", self.llvm_target, "release/deps", member.decode()+'.bc')

        src = 'porus/src/bin/linkbc.rs'
        argv = ['cargo', 'build', self.verbose, '-p', 'porus', '--bin=linkbc']
        if compile_file(ROOTDIR, argv, src) is None:
            return False

        return True


    def get_submit_argv(self, source, target):
        return ['rustc',
                "--emit", "llvm-bc",
                "-C", "opt-level=s",
                "-C", "panic=abort",
                "--target", self.llvm_target] + extern(self.externs) + ["-o", target, source]


    def get_linkbc_argv(self, source, target):
        argv = self.get_submit_argv(source, target)
        return ['cargo', 'run', self.verbose, '-p', 'porus', '--bin', 'linkbc', '--'] + argv[1:] + ["--", target, self.bc_path, source]


    def compile(self, source):
        bc = replace_ext(source, "bc")

        if has_to_recompile(source, bc, self.externs):
            argv = self.get_submit_argv(source, bc)
            if compile_file(ROOTDIR, argv, source, bc) is None:
                return None

        target = replace_ext(source, "s")

        if has_to_recompile(source, target, self.externs):
            argv = self.get_linkbc_argv(bc, target)
            if compile_file(ROOTDIR, argv, bc, target) is None:
                return None

        return target


def prepare_submission(envs, filename):
    env = pick_env(envs)
    if not env:
        return None

    llvm_target = get_llvm_target(env)

    context = SubmissionContext(llvm_target)

    if not context.check():
        return None

    asm = context.compile(filename)
    if asm is None:
        return None

    with open(asm,'rb') as f:
        code = f.read()

    return env, code
