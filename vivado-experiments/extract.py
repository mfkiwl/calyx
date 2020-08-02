#!/usr/bin/env python3

import rpt
import sys
import re
import json
from pathlib import Path

def find_row(table, colname, key):
    for row in table:
        if row[colname] == key:
            return row
    raise Exception(f"{key} was not found in column: {colname}")

def to_int(s):
    if s == '-':
        return 0
    else:
        return int(s)

def futil_extract(directory):
    parser = rpt.RPTParser(directory / "main_utilization_placed.rpt")
    slice_logic = parser.get_table(re.compile(r'1\. Slice Logic'), 2)
    dsp_table = parser.get_table(re.compile(r'4\. DSP'), 2)

    return {
        'LUT': find_row(slice_logic, 'Site Type', 'Slice LUTs')['Used'],
        'DSP': find_row(dsp_table, 'Site Type', 'DSPs')['Used']
    }

def hls_extract(directory):
    parser = rpt.RPTParser(directory / "kernel_csynth.rpt")
    summary_table = parser.get_table(re.compile(r'== Utilization Estimates'), 2)
    instance_table = parser.get_table(re.compile(r'\* Instance:'), 0)

    solution_data = json.load((directory / "solution1_data.json").open())
    latency = solution_data['ModuleInfo']['Metrics']['kernel']['Latency']

    total_row = find_row(summary_table, 'Name', 'Total')
    s_axi_row = find_row(instance_table, 'Instance', 'kernel_control_s_axi_U')

    return {
        'TOTAL_LUT': to_int(total_row['LUT']),
        'INSTANCE_LUT': to_int(s_axi_row['LUT']),
        'LUT': to_int(total_row['LUT']) - to_int(s_axi_row['LUT']),
        'DSP': to_int(total_row['DSP48E']) - to_int(s_axi_row['DSP48E']),
        'AVG_LATENCY': latency['LatencyAvg'],
        'BEST_LATENCY': latency['LatencyBest'],
        'WORST_LATENCY': latency['LatencyWorst'],
    }

if __name__ == "__main__":
    directory = sys.argv[1]
    print(json.dumps({
        "futil": futil_extract(Path(directory)),
        "hls": hls_extract(Path(directory))
    }))