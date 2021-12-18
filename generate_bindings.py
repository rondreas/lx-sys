"""

    Get all the included headers and generate the bindings.h

"""

import os

if __name__ == "__main__":
    headers = os.listdir("lxsdk/include")
    with open("bindings.h", "w") as f:
        f.writelines(['#include "{}"\n'.format(header) for header in headers if header.lower().endswith(".h")])
