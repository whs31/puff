### **Manifest** file
Manifest describes the package and tells which dependencies it uses to build itself.
##### Mandatory entries:
- Package name
- Package version
##### Optional entries:
- Package authors
- Package description
- Package license
- **Dependencies section**
- **Build dependencies section**

##### Example manifest:
```toml
[this]
name = "example"
version = "0.1.0"
description = "Example parcel for example project!" # optional
authors = [                                         # optional
	"whs31 <whs31@github.io>",
	"example_author <example@example.com>"
]
license = "MIT"                                     # optional

# this is also optional!
[needs]  
fmt = "10.0.0"               # shared is by default...
spdlog = "1.0.63@static"     # ...but you can override it!
kfr = "0.0.1"                # if newer version exists, it will use it
magicenum = "=1.0.15@static" # if 1.0.15 static is not found in registries 
                             # or cannot be built, error will be thrown!

# what package needs to be built (optional)
[build]
g++ = "11.0.0"
gcc = "11.0.0"
cmake = "3.15.0"             # 3.15.0 or higher
gtest = "any"                # any found version is OK
```