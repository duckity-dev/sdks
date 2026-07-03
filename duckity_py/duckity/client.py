from httpx import AsyncClient


_client = AsyncClient()


class Client:
    base_url: str = "https://quack.duckity.dev"

    def __init__(self, base_url: str | None = None):
        if base_url is not None:
            self.base_url = base_url

    async def get_challenge(
        self,
        protection_profile_id: str,
        *,
        keys: dict[str, str] = {},
    ) -> str:
        """Fetches a challenge from the Duckling API.

        Args:
            protection_profile_id (str): The protection profile ID for which to fetch the challenge.
            keys (dict[str, str], optional): A map of CCTC keys to values. Defaults to no KV pairs.

        Returns:
            str: The challenge string to be solved.
        """

        response = await _client.post(
            f"{self.base_url}/v1/challenge",
            json={
                "id": protection_profile_id,
                "keys": keys,
            },
        )
        response.raise_for_status()

        data = response.json()

        return data["challenge"]

    async def validate_challenge(
        self,
        application_id: str,
        application_secret: str,
        protection_profile_id: str,
    ) -> bool:
        """Validates a solved challenge with the Duckling API.

        Args:
            application_id (str): The application ID for which to validate the challenge.
            application_secret (str): The secret of the application for which to validate the
                challenge.
            protection_profile_id (str): The protection profile ID for which to validate the
                challenge.

        Returns:
            bool: Whether the solution is valid or not.
        """

        response = await _client.post(
            f"{self.base_url}/v1/validate",
            headers={"Authorization": f"Bearer {application_secret}"},
            json={
                "application_id": application_id,
                "protection_profile_id": protection_profile_id,
            },
        )
        response.raise_for_status()

        data = response.json()

        return data["is_valid"]
