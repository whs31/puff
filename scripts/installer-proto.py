#!/usr/bin/env python3

import requests
import argparse
import json
import os
import platform
import shutil
import tarfile
import sys
import subprocess


def add_to_path(p: str):
    print("adding to path " + p)
    commands = ["export PATH=\"{}:$PATH\"".format(p), "echo 'export PATH=\"{}:$PATH\"' >> ~/.bashrc".format(p),
                "echo 'export PATH=\"{}:$PATH\"' >> ~/.profile".format(p)]
    if os.path.exists(os.path.expanduser("~/.zshrc")):
        commands.append("echo 'export PATH=\"{}:$PATH\"' >> ~/.zshrc".format(p))

    for command in commands:
        subprocess.run(command, shell=True)


class Credentials(object):
    def __init__(self, user, token, token_long=None):
        self.user = user
        self.token = token
        self.token_long = token_long

class Artifactory(object):
    def __init__(self, credentials):
        self.credentials = credentials
        self.url = 'http://213.170.107.251/artifactory/poppy-cxx-repo/radar/{name}/{name}-{version}-{arch}-{dist}.tar.gz'
        self.api_url = 'http://213.170.107.251/artifactory/api/storage/poppy-cxx-repo/radar/{name}/{name}-{version}-{arch}-{dist}.tar.gz'
        self.aql_url = 'http://213.170.107.251/artifactory/api/search/aql'

    def push(self, file, _name, _version, _arch, _dist):
        url = self.url.format(name=_name, version=_version, arch=_arch, dist=_dist)
        print(f'pushing to url: {url}')
        r = requests.put(url, data=file, auth=(self.credentials.user, self.credentials.token))
        print(f'response status code: {r.status_code}')
        if r.status_code > 400:
            print(r.text)
            sys.exit(1)
        return r

    def install_latest(self, _arch, _where):
        query = 'items.find({"repo": "poppy-cxx-repo", "name": {"$match": "poppy-*"}}).sort({"$desc": ["created"]}).limit(2)'
        r = requests.post(self.aql_url, data=query, auth=(self.credentials.user, self.credentials.token))
        print(f'response status code: {r.status_code}')

        jprint = lambda x: print(json.dumps(x, sort_keys=True, indent=4))
        # jprint(r.json())

        if not os.path.exists(os.path.expanduser('~/.local/bin')):
            os.makedirs(os.path.expanduser('~/.local/bin'))

        latest = r.json()['results'][0]['name']
        print(f'found latest: {latest}')
        latest_url = self.url.format(name='poppy', version='', arch='', dist='')
        latest_url = latest_url[:latest_url.rfind('/')] + '/' + latest
        print(f'latest url: {latest_url}')

        r = requests.get(latest_url, auth=(self.credentials.user, self.credentials.token))
        print(f'response status code: {r.status_code}')
        if r.status_code > 400:
            print(r.text)
            sys.exit(1)

        # save the tar
        with open('poppy-latest.tar.gz', 'wb') as f:
            f.write(r.content)
            for name in tarfile.open('poppy-latest.tar.gz', mode='r|gz').getnames():
                print(name)
        with tarfile.open('poppy-latest.tar.gz', mode='r|gz') as f:
            f.extractall()
            # list the files
            for name in f.getnames():
                print(name)

        os.chmod('poppy', 0o777)
        shutil.copy('poppy', f'{_where}/poppy')
        print(f'installed poppy to {_where}/poppy')

        add_to_path(_where)

        os.remove('poppy-latest.tar.gz')
        os.remove('poppy')
        return r


    def exists(self, _name, _version, _arch, _dist):
        url = self.url.format(name=_name, version=_version, arch=_arch, dist=_dist)
        print(f'checking url: {url}')
        r = requests.head(url, auth=(self.credentials.user, self.credentials.token))
        print(f'response status code: {r.status_code}')
        return r

class Cargo(object):
    def __init__(self, cargo_toml_path):
        self.cargo_toml_path = cargo_toml_path

    def version(self):
        with open(self.cargo_toml_path, 'r') as f:
            for line in f.readlines():
                if line.startswith('version = '):
                    return line.split('=')[1].strip().strip('"')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--push', action='store_true')
    parser.add_argument('--install-latest', action='store_true')
    parser.add_argument('--file', type=str)
    parser.add_argument('--name', type=str)
    parser.add_argument('--arch', type=str)
    parser.add_argument('--user', type=str, required=True)
    parser.add_argument('--token', type=str, required=True)
    parser.add_argument('--token-long', type=str)
    args = parser.parse_args()

    print('args: ', args.file, args.name, args.arch)
    print(f'ci user: {args.user} \nci token: ***{args.token[3:12]}***')
    print('installing in ~/.local/bin')


    artifactory = Artifactory(Credentials(args.user, args.token, args.token_long))
    if args.install_latest:
        artifactory.install_latest(args.arch, os.path.expanduser('~/.local/bin'))

    if args.push:
        cargo = Cargo('./Cargo.toml')
        file = open(args.file, 'rb')
        version = cargo.version()
        print(f'cargo version: {version}')
        if not artifactory.exists(args.name, version, args.arch, 'executable'):
            artifactory.push(file, args.name, version, args.arch, 'executable')
            print(f'file {args.file} pushed to artifactory')
        else:
            print(f'file {args.file} already exists in artifactory')
            
    os.system('~/.local/bin/poppy -u args.user -t args.token sync -r')


if __name__ == '__main__':
    main()
