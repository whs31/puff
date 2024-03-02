import requests
import argparse

default_credentials = {
    'user': 'gitlab_ci',
    'token': 'cmVmdGtuOjAxOjAwMDAwMDAwMDA6dHJycVdLTWI1RjJFcnVwQlJnSlVBWEVDcEdC',
    'token-long': 'eyJ2ZXIiOiIyIiwidHlwIjoiSldUIiwiYWxnIjoiUlMyNTYiLCJraWQiOiJ\
    zVGg1LVB3SWtGSlJiNk9CV1Zoakt0cG92TmVJMjFGaUtiZ2t0Si1XYUxFIn0.eyJzdWIiOiJqZ\
    mFjQDAxaHFxZHMzZGtzY3czMW1tcWVtd2gwOHhnL3VzZXJzL2dpdGxhYl9jaSIsInNjcCI6ImF\
    wcGxpZWQtcGVybWlzc2lvbnMvdXNlciIsImF1ZCI6IipAKiIsImlzcyI6ImpmZmVAMDFocXFkc\
    zNka3NjdzMxbW1xZW13aDA4eGciLCJpYXQiOjE3MDkzOTk3NTYsImp0aSI6IjAyZGIzMGQ0LWY\
    1NzUtNDA4Yi05OWZjLTUzYzY4OWIzOTg1MSJ9.SH-4wNd8Z6Egf3a_viyA9HrDicmDlMpvor0r\
    D5tkwI2N4b9U7aie4_QhdujcQkDevNvrfo3Ar3J_RMvmcHF3WVxOIwckDvbde2viipFA5RGRHg\
    E1u9hbD_Y_lHu-uexwkTSGw_yXL55rMTMJLlF4pQTVJYddxFFU1ezUGtGoJTvMJair4Vt4x6T4\
    hv8C7Qyu84URFBbORWw6Bwbz7eT0m7GGuoHPN9CbKkvFP1XnCX8GM3tmb0LQjkvLBYcX9pOacx\
    IgwjhQlcw5P9T_DgxnoRE0uOwAyWKa9Jf0ILU66oJeKb2AF7e-lh8wCyUcwA-UkH6LXRmvSFvb\
    _PoxS3jtKg'
}

class Credentials(object):
    def __init__(self, credentials=default_credentials):
        self.user = credentials['user']
        self.token = credentials['token']
        self.token_long = credentials['token-long']

class Artifactory(object):
    def __init__(self, credentials):
        self.credentials = credentials
        self.url = 'http://uav.radar-mms.com/artifactory/poppy-cxx-repo/radar/{name}/{name}-{version}-{arch}-{dist}.tar.gz'

    def push(self, file, _name, _version, _arch, _dist):
        url = self.url.format(name=_name, version=_version, arch=_arch, dist=_dist)
        r = requests.put(url, data=file, auth=(self.credentials.user, self.credentials.token))
        return r

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--push', action='store_true')
    parser.add_argument('--file', type=str)
    parser.add_argument('--name', type=str)
    parser.add_argument('--version', type=str)
    parser.add_argument('--arch', type=str)
    parser.add_argument('--dist', type=str)
    args = parser.parse_args()

    print(args.file, args.name, args.version, args.arch, args.dist)

    artifactory = Artifactory(Credentials())
    if args.push:
        file = open(args.file, 'rb')
        artifactory.push(file, args.name, args.version, args.arch, args.dist)


if __name__ == '__main__':
    main()