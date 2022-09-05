# -*- coding: utf-8 -*-

import requests
import json 
from io import StringIO

API_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJlbWFpbCI6Inh5ejYwMDYwMEBnbWFpbC5jb20iLCJleHAiOjE2NjIzNzI0NTYsIm9yaWdfaWF0IjoxNjYyMjg2MDU2fQ.R6zdyFM4VNbWWcCsThsMKZcct0YuU5dhTNlWWG6g9Cw"
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

    def submit(self, problem_id: int):

        with open(f"../solution/{problem_id}.txt", 'r') as fin:
            content = fin.read()
            response = requests.post(f"https://robovinci.xyz/api/submissions/{problem_id}/create",
                                    headers={"Authorization": f"Bearer {self.api_key}"},
                                    files={"file": ("submission.isl", StringIO(content))})

        response.raise_for_status()


if __name__ == "__main__":

    client = ICFPCClient(API_KEY)

    submission_id_list = list(range(1, 41))

    for id in submission_id_list:
        client.submit(id)
        print(f"submit {id}")
