"""Turn a saved browser_batch tool result (JSON array of {type,text}) into JSONL."""
import json, sys, pathlib
src, dst = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
items = json.loads(src.read_text())
dec = json.JSONDecoder()
lines = []
for it in items:
    t = it.get('text', '')
    prefix = '[javascript_tool:javascript_exec] '
    if t.startswith(prefix):
        t = t[len(prefix):]
    s, _ = dec.raw_decode(t)
    for l in s.split('\n'):
        l = l.strip()
        if l.startswith('{'):
            json.loads(l); lines.append(l)
dst.write_text('\n'.join(lines) + '\n')
types = [json.loads(l)['t'] for l in lines]
print(len(lines), 'blocks,', len(set(types)), 'unique types ->', dst)
