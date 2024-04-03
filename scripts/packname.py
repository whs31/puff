# script to extract package name and version from Cargo.toml
import tomllib
import platform

with open('../Cargo.toml', 'rb') as f:
    cargo = tomllib.load(f)

name = cargo['package']['name']
version = cargo['package']['version']
operating_system = ''.join(map(lambda x: x.lower(), platform.system()))
arch = ''.join(map(lambda x: x.lower(), platform.machine()))

print(f'{name}-{version}-{operating_system}-{arch}', end='')