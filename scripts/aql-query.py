#!/usr/bin/env python3

import requests
import json
import argparse
from rich import print


class Credentials(object):
    def __init__(self, user, token, token_long=None):
        self.user = user
        self.token = token
        self.token_long = token_long
        print(f'user: [bold green]{user}[/]')
        print(f'token: [bold yellow]***{token[3:12]}***[/]')


class Artifactory(object):
    def __init__(self, credentials):
        self.credentials = credentials
        self.url = 'http://213.170.107.251/artifactory/poppy-cxx-repo/radar/{name}/{name}-{version}-{arch}-{dist}.tar.gz'
        self.api_url = 'http://213.170.107.251/artifactory/api/storage/poppy-cxx-repo/radar/{name}/{name}-{version}-{arch}-{dist}.tar.gz'
        self.aql_url = 'http://213.170.107.251/artifactory/api/search/aql'

    def query(self, query):
        r = requests.post(self.aql_url, data=query, auth=(self.credentials.user, self.credentials.token))
        print(f'response status code: {r.status_code}')
        if r.status_code > 400:
            print(r.text)
            sys.exit(1)
        return r


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--user', help='artifactory username', required=True)
    parser.add_argument('--token', help='artifactory token', required=True)
    args = parser.parse_args()

    artifactory = Artifactory(Credentials(args.user, args.token))
    r = artifactory.query(r"""
items.find({"repo": "poppy-cxx-repo", "name": {"$match": "*"}}).sort({"$desc": ["created"]})
    """)
    names = []
    for item in r.json()['results']:
        names.append(item['name'])
    for name in names:
        print(name)
    # print(r.text)


if __name__ == '__main__':
    main()