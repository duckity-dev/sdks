"""An example of solving a Duckity challenge using the Duckity SDK.

To run this example using `uv`, you can run

```sh
uv run examples/challenge.py
```
"""

import time

from duckity import DuckityClient


DUCKITY_APP_ID = ""
DUCKITY_PROFILE_CODE = ""
DUCKITY_DOMAIN = "quack.duckity.dev"


print("Welcome to the Duckity Python SDK example!")
print()

if not DUCKITY_APP_ID or not DUCKITY_PROFILE_CODE or not DUCKITY_DOMAIN:
    print(
        "Seems like it's your first time here! Open python/examples/challenge.py and update the "
        "DUCKITY_APP_ID and DUCKITY_PROFILE_CODE constants at the top of the file. This'll work "
        "once that's done!"
    )
    exit(1)

print(f"App ID: {DUCKITY_APP_ID}")
print(f"Profile Code: {DUCKITY_PROFILE_CODE}")
print(f"Domain: {DUCKITY_DOMAIN}")
print()

client = DuckityClient(DUCKITY_DOMAIN)

print("Fetching challenge...")

challenge = client.fetch(DUCKITY_APP_ID, DUCKITY_PROFILE_CODE)

print("Solving challenge...")

start = time.perf_counter()

challenge.solve()

end = time.perf_counter()
duration = end - start

print(f"Challenge solved in {duration:.2f} seconds!")
