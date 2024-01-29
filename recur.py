#recursively find every file inside contracts directory and save it to a json
# file

import os
import json
import sys
import re

path = "contracts/"
all_file_names = []

def get_all_file_names(path):
    for root, dirs, files in os.walk(path):
        for file in files:
            if file.endswith(".sol"):
                all_file_names.append(os.path.join(root, file))
    return all_file_names

all_file_names = get_all_file_names(path)

print(all_file_names)
#save to a json
with open('all_file_names.json', 'w') as outfile:
    json.dump(all_file_names, outfile,indent=4)


all_abis = []
abi_path = "abis/"
def get_all_abi_names(abi_path):
    for root, dirs, files in os.walk(abi_path):
        for file in files:
            if file.endswith(".json"):
                all_abis.append(os.path.join(root, file))
    return all_abis

all_abis = get_all_abi_names(abi_path)
print(all_abis)
#save to a json
with open('all_abis.json', 'w') as outfile:
    json.dump(all_abis, outfile,indent=4)

#save to a json
