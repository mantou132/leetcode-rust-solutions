import os
import os.path
import re
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

    deps = ['-L', 'dependency='+os.path.join(ROOTDIR, "target/debug/deps")]
    target = replace_ext(filename,"elf")
    return ['rustc', VERBOSE_FLAG] + deps + extern(DEBUG_EXTERNS) + ['-o', target, filename], target


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
        argv = ['cargo', 'build', self.verbose, '--lib', '--release', '--target', self.llvm_target]
        if compile_file(ROOTDIR, argv, self.externs["porus"]) is None:
            return False
        return True

    def get_submit_argv(self, source, target):
        deps = ['-L', 'dependency='+os.path.join(ROOTDIR, "target/release/deps")]
        return ['rustc', self.verbose,
                "--crate-type", "cdylib",
                "--emit", "asm",
                "-C", "llvm-args=-disable-debug-info-print",
                "-C", "lto=fat",
                "-C", "opt-level=s",
                "-C", "panic=abort",
                "--target", self.llvm_target] + deps + extern(self.externs) + ["-o", target, source]

    def compile(self, source):
        target = replace_ext(source, "s")

        if has_to_recompile(source, target, self.externs):
            argv = self.get_submit_argv(source, target)
            if compile_file(ROOTDIR, argv, source, target) is None:
                return None

        return target

LABEL = re.compile(rb'^([^:\s]+):$', re.M)
CHARS = b'_.0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz'

def encode_int(n):
    while n > 0:
        yield CHARS[n % 64]
        n //= 64

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

    labels = set(LABEL.findall(code))
    labels.discard(b"main")
    labels.discard(b"_main")
    pattern = b"|".join(map(re.escape, labels))
    labels = {l: b"L"+bytes(encode_int(n))
              for n,l in enumerate(labels)}
    def repl(m):
        return labels[m.group(0)]
    code = re.sub(pattern, repl, code)

    return env, code
