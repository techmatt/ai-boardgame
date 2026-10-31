"""Kept for the doc that cites it (experiments/results/screen-034-clocks/clocks.md): a shim, now
`python scripts/cards/screen.py --preset 034 [--gate] DIR [DIR ...]`."""
import os
import runpy
import sys

here = os.path.dirname(os.path.abspath(__file__))
sys.argv[1:1] = ['--preset', '034']
runpy.run_path(os.path.join(here, 'screen.py'), run_name='__main__')
