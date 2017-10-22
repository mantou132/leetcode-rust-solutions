#!/usr/bin/env python3
import sys
import os
import os.path

ROOT=os.path.dirname(os.path.abspath(__file__))
sys.path.append(os.path.dirname(ROOT)+"/ix")

import ix.__main__
