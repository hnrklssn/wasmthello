import sys
import json

with open(sys.argv[1], 'rb') as file_t:
    blob_data = bytearray(file_t.read())

print(json.dumps(list(blob_data)))
