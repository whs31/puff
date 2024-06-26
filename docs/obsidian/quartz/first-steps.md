---
title: Начало работы
---

В этой части будет краткая инструкция по тому, как создать свой пакет с использованием пакетного менеджера `puff`, добавить ему зависимость в виде библиотеки и скомпилировать код.

Для того, чтобы создать пакет, необходимо в директории с проектом создать файл манифеста и файл рецепта.
*Манифест* - это описание нашего пакета и его зависимостей.
*Рецепт* - это описание шагов, необходимых для сборки этого пакета. Для сборки этого проекта нам не потребуется рецепт, так как мы не планируем его публиковать.

#### Манифест
Создадим в корне *CMake*-проекта файл **Puff.toml** рядом с файлом *CMakeLists.txt* так, чтобы получилась следующая структура папок:
```
src 
└── c++ 
CMakeLists.txt 
Puff.toml
```

Манифест обычно содержит 3 секции: 
- Секция `[this]`, в которой описан сам пакет, а именно:
	- `name` - имя пакета (*обязательно*)
	- `version` - версия пакета (*обязательно*)
	- `description` - краткое описание пакета
	- `authors` - список авторов пакета в виде массива строк, разделенных запятой
	- `license` - лицензия, по которой поставляется исходный код
- Секция `[needs]`, в которой описаны зависимости пакета от других пакетов *Puff*. Она может быть пустой, или и вовсе отсутствовать.
- Секция `[build]`, в которой описаны зависимости сборки (также опциональная).

Пример манифеста с использованием всех возможностей контроля версий:
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

Заполним свой манифест-файл необходимыми полями:
```toml
[this]
name = "hello_world"
version = "1.0.0"
authors = [ "me <me@local>" ]
description = "Example showcase project"
```

Теперь добавим в манифест зависимость от библиотеки `fmt` для примера:
```toml
[needs]
fmt = "^10.0.0@shared"
```

Готово! Манифест полностью настроен.

#### Установка зависимостей
Для того, чтобы из манифеста установить необходимые библиотеки, нужно запустить команду `puff install`, в которую передается путь к папке, содержащей манифест (`Puff.toml`):

```shell
puff install . --fresh
```

> Аргумент `--fresh` сообщает, что любые старые зависимости будут удалены перед установкой новых. Рекомендуется указывать его при установке зависимостей.

Готово! Зависимости установились в папку `dependencies` в корне проекта.

#### Сборка проекта
Сборка проекта с пакетным менеджером отличается только одним аргументом: необходимо передать задать путь для поиска библиотек равным `dependencies`:
```shell
cmake -S . -B target -DCMAKE_PREFIX_PATH="dependencies"
```