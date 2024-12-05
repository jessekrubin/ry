"""No-op mdbook preprocessor

BASED ON: https://github.com/PyO3/pyo3/blob/main/guide/pyo3_version.py
"""

import json
import sys


def modify_section(section):
    if not isinstance(section, dict) or "Chapter" not in section:
        return


def main():
    # lines = []
    for line in sys.stdin:
        if line:
            [context, book] = json.loads(line)
            for section in book["sections"]:
                modify_section(section)
            json.dump(book, fp=sys.stdout)
        # lines.append(line)
    # with open("docs-data.json", "w") as f:
    #     f.writelines(lines)

if __name__ == '__main__':
    main()
