# -*- coding: utf-8 -*-

import requests
import json 

API_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6Inh5ejYwMDYwMEBnbWFpbC5jb20iLCJleHAiOjE2NjIyMDY0MTYsIm9yaWdfaWF0IjoxNjYyMTIwMDE2fQ.qnTW6qeneQTkuHE2x1E5c0vbcJ_5TWACrcIvklBYE6Y"

class ICFPCClient:

    def __init__(self, api_key: str):
        self.api_key = api_key
        self.hostname = "https://robovinci.xyz/api"

    def __default_header(self):
        return {
            "Authorization": f"Bearer {self.api_key}"
        }

    def all_problems(self):
        url = f"{self.hostname}/results/user"

    def users(self):
        url = f"{self.hostname}/users"
        response = requests.get(url, headers=self.__default_header())
        response.raise_for_status()

        return json.loads(response.content.decode("utf-8"))

    def submit(self):
        pass

if __name__ == "__main__":

    client = ICFPCClient(API_KEY)

    msg = client.users()
    print(msg)
