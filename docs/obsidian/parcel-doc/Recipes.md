### Recipes
Recipes describe, how you packet (*parcel*) should be built on different platforms and distributions.

What recipe should do:
- *(optional)* Build package in any folder.
- Place/install things that needs to be exported in folder `target` and subfolder `export`.
Example folder structure for cmake-based project:
```
. 
├── [ *sources* ] 
├── .parcel 
│ └── [ *recipes* ]
├── CMakeLists.txt 
├── Parcel.toml 
└── target 
	├── [ *build artifacts* ] 
	└── export 
		└── [ *exported files* ]
```

Example folder structure for non-cmake project:
```
. 
├── [ *sources* ] 
├── .parcel 
│ └── [ *recipes* ]
├── Parcel.toml 
└── target 
	└── export 
		└── [ *exported files* ]
```
##### Mandatory: 
Exactly one recipe file named `recipe.yml`, which will describe the most universal way to build **static** or **shared** package on any system and be used as fallback. 

##### Optional:
You can provide recipe for **static** *and/or* **shared** versions of package. All recipes must be placed in the same `recipe.yml` file. 
> You shouldn't specify `-DCMAKE_PREFIX_PATH=./dependencies`, as `parcel` will do it automatically. 

> You also shoudn't use different `cmake` names (such as `mingw-cmake`, etc.). These must be passed in `poppy configure` command.

Example of recipe file for cmake-based project:
```yml
---
static:
  toolchain:
    cmake:
    generator: Ninja
    definitions:
        shared: false
shared:
  toolchain:
    cmake:
      generator: Ninja
      definitions:
        shared: true
```
This will automatically call the following commands during *build* stage:
```shell
# install all required dependencies
cmake -GNinja -DSTATIC=ON -B target -S . 
cmake --build target --config release
cmake --install target --prefix target/export
# + also copying of necessary manifest/recipe files...
# + build folder cleanup
```

Example of recipe file for just sources project:
```yaml
--- 
static:
  toolchain: 
    shell:
      - mkdir -p target/export
      - cp -r src/*.h target/export
      - cp -r src/*.cpp target/export
# shared is missing, meaning that library can only be built in static mode
```
This will execute commands as provided.

> Note: do not copy `Parcel.toml`/`.parcel` directory to exports! It will be done automatically!