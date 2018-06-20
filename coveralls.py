#!/usr/bin/env python3

from glob import iglob
from os.path import relpath
import hashlib
import os
import json


def parse_count(lines):
    for line in lines:
        [count, lno] = line[:15].lstrip().split(':')
        lno = int(lno.lstrip())
        if lno == 0:
            continue
        if count == '-':
            count = 0
        elif count.startswith('#'):
            count = len(count)
        else:
            count = int(count)
        yield (lno, count)


def parse_file(filename):
    with open(filename, 'r') as f:
        lines = f.read().splitlines()
    assert lines[0].startswith("        -:    0:Source:")
    name = relpath(lines[0][23:])
    return (name, list(parse_count(lines[1:])))


def parse():
    d = {}
    for filename in iglob("*##*porus*.gcov"):
        name, line_counts = parse_file(filename)
        f = d.get(name, {})
        for l, c in line_counts:
            f[l] = f.get(l, 0) + c
        d[name] = f
    return d


def coverage(line_counts):
    for i, (l, c) in enumerate(sorted(line_counts.items())):
        assert i+1 == l
        yield c


def source_digest(name):
    md5 = hashlib.md5()
    with open(name, 'rb') as f:
        md5.update(f.read())
    return md5.hexdigest()


def source_files():
    for name, line_counts in parse().items():
        yield {"name": name,
               "source_digest": source_digest(name),
               "coverage": list(coverage(line_counts))}

def json_file():
    return json.dumps({
      'service_name': 'travis-ci',
      'service_job_id': os.environ['TRAVIS_JOB_ID'],
      'source_files': list(source_files())}).encode('utf-8')

def post(json_file):
    import requests
    r = requests.post('https://coveralls.io/api/v1/jobs', files={'json_file': json_file})
    print(r.status_code)
    print(r.headers)
    print(r.content)
    assert r.status_code == 200

if __name__ == '__main__':
    post(json_file())
