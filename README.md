# vpm-repos-gen

## Installation

```sh
cargo install --locked --git https://github.com/koyashiro/vpm-repos-gen
```

## Example

```sh
vpm-repos-gen --name "koyashiro" --author 'koyashiro' --url 'https://vpm.koyashiro.net/repos.json' --id 'net.koyashiro.vpm' --repos 'koyashiro/udon-list,koyashiro/udon-dictionary'
```

<details>
<summary>Output</summary>

```json
{
  "name": "koyashiro",
  "author": "koyashiro",
  "url": "https://vpm.koyashiro.net/repos.json",
  "packages": {
    "net.koyashiro.udonexception": {
      "versions": {
        "0.1.0": {
          "version": "0.1.0",
          "name": "net.koyashiro.udonexception",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-exception/issues"
          },
          "description": "List implementation for UdonSharp.",
          "displayName": "UdonException",
          "homepage": "https://github.com/koyashiro/udon-exception",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-exception"
          },
          "samples": [
            {
              "description": "Sample of UdonException.",
              "displayName": "UdonExceptionSample",
              "path": "Samples~/UdonExceptionSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-exception/releases/download/v0.1.0/net.koyashiro.udonexception-0.1.0.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.5"
          }
        },
        "0.2.0": {
          "version": "0.2.0",
          "name": "net.koyashiro.udonexception",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-exception/issues"
          },
          "description": "List implementation for UdonSharp.",
          "displayName": "UdonException",
          "homepage": "https://github.com/koyashiro/udon-exception",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-exception"
          },
          "samples": [
            {
              "description": "Sample of UdonException.",
              "displayName": "UdonExceptionSample",
              "path": "Samples~/UdonExceptionSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-exception/releases/download/v0.2.0/net.koyashiro.udonexception-0.2.0.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.5"
          }
        }
      }
    },
    "net.koyashiro.udontest": {
      "versions": {
        "0.1.0": {
          "version": "0.1.0",
          "name": "net.koyashiro.udontest",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-test/issues"
          },
          "description": "Simple test library for UdonSharp.",
          "displayName": "UdonTest",
          "homepage": "https://github.com/koyashiro/udon-test",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-test"
          },
          "samples": [
            {
              "description": "Sample of UdonTest.",
              "displayName": "UdonTestSample",
              "path": "Samples~/UdonTestSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-test/releases/download/v0.1.0/net.koyashiro.udontest-0.1.0.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.5"
          }
        },
        "0.1.1": {
          "version": "0.1.1",
          "name": "net.koyashiro.udontest",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-test/issues"
          },
          "description": "Simple test library for UdonSharp.",
          "displayName": "UdonTest",
          "homepage": "https://github.com/koyashiro/udon-test",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-test"
          },
          "samples": [
            {
              "description": "Sample of UdonTest.",
              "displayName": "UdonTestSample",
              "path": "Samples~/UdonTestSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-test/releases/download/v0.1.1/net.koyashiro.udontest-0.1.1.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.5"
          }
        },
        "0.1.2": {
          "version": "0.1.2",
          "name": "net.koyashiro.udontest",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-test/issues"
          },
          "description": "Simple test library for UdonSharp.",
          "displayName": "UdonTest",
          "homepage": "https://github.com/koyashiro/udon-test",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-test"
          },
          "samples": [
            {
              "description": "Sample of UdonTest.",
              "displayName": "UdonTestSample",
              "path": "Samples~/UdonTestSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-test/releases/download/v0.1.2/net.koyashiro.udontest-0.1.2.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.5"
          }
        },
        "0.2.0": {
          "version": "0.2.0",
          "name": "net.koyashiro.udontest",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-test/issues"
          },
          "description": "Simple test library for UdonSharp.",
          "displayName": "UdonTest",
          "homepage": "https://github.com/koyashiro/udon-test",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-test"
          },
          "samples": [
            {
              "description": "Sample of UdonTest.",
              "displayName": "UdonTestSample",
              "path": "Samples~/UdonTestSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-test/releases/download/v0.2.0/net.koyashiro.udontest-0.2.0.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.5"
          }
        },
        "0.3.0": {
          "version": "0.3.0",
          "name": "net.koyashiro.udontest",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-test/issues"
          },
          "description": "Simple test library for UdonSharp.",
          "displayName": "UdonTest",
          "homepage": "https://github.com/koyashiro/udon-test",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-test"
          },
          "samples": [
            {
              "description": "Sample of UdonTest.",
              "displayName": "UdonTestSample",
              "path": "Samples~/UdonTestSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-test/releases/download/v0.3.0/net.koyashiro.udontest-0.3.0.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.5"
          }
        },
        "0.4.0": {
          "version": "0.4.0",
          "name": "net.koyashiro.udontest",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-test/issues"
          },
          "description": "Simple test library for UdonSharp.",
          "displayName": "UdonTest",
          "homepage": "https://github.com/koyashiro/udon-test",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-test"
          },
          "samples": [
            {
              "description": "Sample of UdonTest.",
              "displayName": "UdonTestSample",
              "path": "Samples~/UdonTestSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-test/releases/download/v0.4.0/net.koyashiro.udontest-0.4.0.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.5"
          }
        },
        "0.4.1": {
          "version": "0.4.1",
          "name": "net.koyashiro.udontest",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-test/issues"
          },
          "description": "Simple test library for UdonSharp.",
          "displayName": "UdonTest",
          "homepage": "https://github.com/koyashiro/udon-test",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-test"
          },
          "samples": [
            {
              "description": "Sample of UdonTest.",
              "displayName": "UdonTestSample",
              "path": "Samples~/UdonTestSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-test/releases/download/v0.4.1/net.koyashiro.udontest-0.4.1.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.6"
          }
        },
        "0.4.2": {
          "version": "0.4.2",
          "name": "net.koyashiro.udontest",
          "author": {
            "email": "develop@koyashi.ro",
            "name": "koyashiro",
            "url": "https://github.com/koyashiro"
          },
          "bugs": {
            "url": "https://github.com/koyashiro/udon-test/issues"
          },
          "description": "Simple test library for UdonSharp.",
          "displayName": "UdonTest",
          "homepage": "https://github.com/koyashiro/udon-test",
          "license": "MIT",
          "repository": {
            "type": "git",
            "url": "https://github.com/koyashiro/udon-test"
          },
          "samples": [
            {
              "description": "Sample of UdonTest.",
              "displayName": "UdonTestSample",
              "path": "Samples~/UdonTestSample"
            }
          ],
          "url": "https://github.com/koyashiro/udon-test/releases/download/v0.4.2/net.koyashiro.udontest-0.4.2.zip",
          "vpmDependencies": {
            "com.vrchat.udonsharp": "1.1.7"
          }
        }
      }
    }
  }
}
```

</details>
