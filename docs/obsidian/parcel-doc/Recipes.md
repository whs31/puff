### Recipes
Recipes describe how to build this package from source. Recipes use **YAML** syntax instead of TOML.

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
	- cmake -GNinja -DCMAKE_BUILD_TYPE=Release -B target -S . -DCMAKE_INSTALL_PREFIX=./target/export
	- cmake --build target
	- cmake --install target

shared: 
	- cmake -GNinja -DCMAKE_BUILD_TYPE=Release -B target -S . -DCMAKE_INSTALL_PREFIX=./target/export -DSHARED=ON
	- cmake --build target
	- cmake --install target
```

Example of recipe file for just sources project:
```yaml
--- 
static:
	- mkdir -p target/export
	- cp -r export-dir/*.h ./target/export
	- cp -r export-dir/*.cpp ./target/export
```

> Note: do not copy `Parcel.toml`/`.parcel` directory to exports! It will be done automatically!