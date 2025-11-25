"""A Duckity SDK for Python 3."""

import httpx
import asyncio
import ipaddress

from duckity import duckity_rs


type IP = ipaddress.IPv4Address | ipaddress.IPv6Address


class DuckityClient:
    """A client for solving Duckity challenges."""

    domain: str
    """The duckling server domain.
    
    By default, this is "quack.duckity.dev".
    """

    def __init__(self, domain: str = "quack.duckity.dev") -> None:
        """Initialize the Duckity client.

        Args:
            domain (str): The Duckity server domain.
        """

        self.domain = domain

    def fetch(self, app_id: str, profile_code: str) -> "Challenge":
        """Fetch a Duckity challenge from the server.

        See `DuckityClient.afetch()` for the asynchronous version.

        Args:
            app_id (str): The Duckity application ID.
            profile_code (str): The Duckity profile code.

        Returns:
            Challenge: The fetched Duckity challenge.

        Raises:
            httpx.HTTPError: If the request to fetch the challenge fails.
        """

        response = httpx.post(
            f"https://{self.domain}/v1/challenges/{app_id}",
            json={"profile": profile_code},
        )

        response.raise_for_status()

        return Challenge(response.content)

    async def afetch(self, app_id: str, profile_code: str) -> "Challenge":
        """Asynchronously fetch a Duckity challenge from the server.

        See `DuckityClient.fetch()` for the synchronous version.

        Args:
            app_id (str): The Duckity application ID.
            profile_code (str): The Duckity profile code.

        Returns:
            Challenge: The fetched Duckity challenge.

        Raises:
            httpx.HTTPError: If the request to fetch the challenge fails.
        """

        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"https://{self.domain}/v1/challenges/{app_id}",
                json={"profile": profile_code},
            )

            response.raise_for_status()

            return Challenge(response.content)

    def validate(self, app_id: str, app_secret: str, profile_code: str, ip: IP, solution: str) -> bool:
        """Validate a Duckity challenge solution with the server.

        See `DuckityClient.avalidate()` for the asynchronous version.

        Args:
            app_id (str): The Duckity application ID.
            app_secret (str): The Duckity application secret.
            profile_code (str): The Duckity profile code.
            ip (IP): The IP address of the client.
            solution (str): The encoded solution to the challenge.

        Returns:
            bool: Whether the solution is valid.

        Raises:
            httpx.HTTPError: If the request to validate the solution fails.
        """

        response = httpx.post(
            f"https://{self.domain}/v1/challenges/{app_id}/validate",
            json={"profile": profile_code, "token": solution, "ip": ip},
            headers={
                "Authorization": f"Bearer {app_secret}",
            },
        )

        if response.status_code in range(200, 300):
            return True

        else:
            return False

    async def avalidate(
        self, app_id: str, app_secret: str, profile_code: str, ip: IP, solution: str
    ) -> bool:
        """Asynchronously validate a Duckity challenge solution with the server.

        See `DuckityClient.validate()` for the synchronous version.

        Args:
            app_id (str): The Duckity application ID.
            app_secret (str): The Duckity application secret.
            profile_code (str): The Duckity profile code.
            ip (IP): The IP address of the client.
            solution (str): The encoded solution to the challenge.

        Returns:
            bool: Whether the solution is valid.

        Raises:
            httpx.HTTPError: If the request to validate the solution fails.
        """

        async with httpx.AsyncClient() as client:
            response = await client.post(
                f"https://{self.domain}/v1/challenges/{app_id}/validate",
                json={"profile": profile_code, "token": solution, "ip": ip},
                headers={
                    "Authorization": f"Bearer {app_secret}",
                },
            )

            if response.status_code in range(200, 300):
                return True

            else:
                return False


class Challenge:
    """A Duckity challenge."""

    raw: bytes

    def __init__(self, raw: bytes) -> None:
        """Initialize the Duckity challenge.

        Prefer using `DuckityClient.fetch` to fetch challenges from the server.

        Args:
            raw (bytes): The raw challenge data.
        """

        self.raw = raw

    def solve(self) -> str:
        """Solve the Duckity challenge.

        See `Challenge.asolve()` for the asynchronous version.

        Returns:
            str: The encoded solution to the challenge.

        Raises:
            DecodingError: If decoding the challenge fails. This is never raised when the challenge
                is fetched using `DuckityClient.fetch`.
        """

        try:
            return duckity_rs.solve(self.raw)

        except Exception as e:
            raise DecodingError("Failed to decode the Duckity challenge.") from e

    async def asolve(self) -> str:
        """Asynchronously solve the Duckity challenge.

        See `Challenge.solve()` for the synchronous version.

        Internally, it uses a blocking Rust implementation to solve the challenge. It's wrapped in
        `asyncio.to_thread` to avoid blocking the event loop.

        Returns:
            str: The encoded solution to the challenge.

        Raises:
            DecodingError: If decoding the challenge fails. This is never raised when the challenge
                is fetched using `DuckityClient.fetch`.
        """

        try:
            return await asyncio.to_thread(duckity_rs.solve, self.raw)

        except Exception as e:
            raise DecodingError("Failed to decode the Duckity challenge.") from e


class DecodingError(Exception):
    """An error occurred in the Duckity SDK."""

    pass
