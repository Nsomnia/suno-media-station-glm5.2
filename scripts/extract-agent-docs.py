#!/usr/bin/env python3
"""Extract per-file markdown docs from docs/.agent/PROMPTS/INITIAL_PROJECT_LAYOUT_CHAT_SESSION_EXPORT_WHOLE.md.

Each doc in the chat export is fenced as:

    ```
    <relative/path>
    ```
    ```<lang>
    <content>
    ```

Blocks whose path line contains "APPEND", "INSERT", or "UPDATE" are written
to .patch/ instead of overwriting the target file (apply manually).

Usage:
    scripts/extract-agent-docs.py list   # show what would be extracted
    scripts/extract-agent-docs.py write  # write files
"""
import os
import re
import sys

SRC = 'docs/.agent/PROMPTS/INITIAL_PROJECT_LAYOUT_CHAT_SESSION_EXPORT_WHOLE.md'
PATH_RE = re.compile(r'^([A-Za-z0-9_][\w./-]*|\.\S+)\s*(—.*)?$')


def find_blocks(src):
    n = len(src)
    blocks = []
    i = 0
    while i < n - 3:
        if src[i] == '```' and src[i + 2] == '```':
            m = PATH_RE.match(src[i + 1].strip())
            lang = re.match(r'^```\w*\s*$', src[i + 3])
            if m and m.group(1) != '```':
                j = i + 4
                while j < n:
                    if src[j] == '```':
                        nxt = next((src[k] for k in range(j + 1, min(j + 4, n))
                                    if src[k].strip()), '')
                        if (nxt == '' or nxt == '---' or nxt.startswith('### ')
                                or nxt.startswith('**continue')
                                or nxt.startswith('— **END')
                                or nxt.startswith('*(')
                                or nxt == '```'):
                            break
                    j += 1
                blocks.append((i, m.group(1), m.group(2) or '', i + 4, j))
                i = j + 1
                continue
        i += 1
    return blocks


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else 'list'
    with open(SRC) as f:
        src = f.read().splitlines()
    for _, path, note, s, e in find_blocks(src):
        content = '\n'.join(src[s:e]).rstrip() + '\n'
        if content.startswith('```'):
            lines = content.split('\n')
            if re.match(r'^```\w*\s*$', lines[0]):
                content = '\n'.join(lines[1:])
                content = re.sub(r'\n```\s*$', '\n', content)
        if mode == 'list':
            print(path, note[:60], f'(lines {s+1}-{e+1}, {len(content)} bytes)')
            continue
        os.makedirs(os.path.dirname(path) or '.', exist_ok=True)
        if any(k in note for k in ('APPEND', 'INSERT', 'UPDATE')):
            out = '.patch/' + path.replace('/', '__') + '.patch'
            os.makedirs('.patch', exist_ok=True)
            with open(out, 'w') as f:
                f.write(content)
            print('PATCH', path, '->', out)
        else:
            with open(path, 'w') as f:
                f.write(content)
            print('WROTE', path, len(content), 'bytes')


if __name__ == '__main__':
    main()
