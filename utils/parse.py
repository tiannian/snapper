import requests
import sys
import json

github_prefix = "https://github.com/ethereum/solc-bin/raw/gh-pages/%s/%s"

def main():
    verison_file = sys.argv[1]
    upstream_file = sys.argv[2]
    platform = sys.argv[3]

    versions = json.load(open(verison_file))

    res = requests.get(upstream_file).text

    builds = json.loads(res)

    for build in builds["builds"]:
        version = build["version"]
        keccak256 = build["keccak256"]
        sha256 = build["sha256"]
        path = build["path"]

        art = {
            "url": [ github_prefix % (platform, path) ],
            "keccak256": keccak256,
            "sha256": sha256,
        }

        versions["builds"]["%s-%s" % (version, platform)] = art
    # print(json.dumps(versions, indent = 4))
    json.dump(versions, open(verison_file, "w"), indent = 4)

if __name__ == '__main__':
    main()
