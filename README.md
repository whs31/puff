![Header](docs/images/header.png)



### Описание
**Poppy** - это пакетный менеджер для `C`/`C++`.
- *Язык:* ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
- *Платформы:* ![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
- *Система сборки:* ![Static Badge](https://img.shields.io/badge/Cargo--%23dce0e8?style=for-the-badge&logo=rust&label=CARGO&labelColor=%23dc8a78&color=%23dc8a78)![CMake](https://img.shields.io/badge/CMake-%23008FBA.svg?style=for-the-badge&logo=cmake&logoColor=white)![CI](https://img.shields.io/badge/gitlab%20ci-%23181717.svg?style=for-the-badge&logo=gitlab&logoColor=white)

**Poppy** состоит из 3-х частей:
- `poppy-cli` - консольная утилита-фронтенд для работы с удаленным хранилищем артефактов и пакетов. Представляет из себя собранный без зависимостей на `glibc` нативный исполняемый файл.
- `poppup` - утилита для установки фронтенда в систему. На данный момент поддерживается только операционная система `Linux`, для установки на нативную `Windows` необходимо собрать и установить в PATH пакет вручную.

### Предварительная конфигурация
Для работы с `poppy` пользователю потребуется:
- ![Gitlab logo](docs/images/gitlab-logo.png)  Логин/пароль **GitLab**, в котором лежит реестр (если вы читаете это на GitLab, у вас они уже есть)
- ![Artifactory logo](docs/images/artifactory-logo.png) Логин/токен **Artifactory**:
  - Логин можно получить у системного администратора.
  - С логином и паролем нужно зайти в UI-часть [(сюда)](http://uav.radar-mms.com/ui) и сгенерировать токен:
    - Нужно нажать на *Set me up*: 
    - ![img.png](docs/images/img.png)
    - Сгенерировать токен в появившемся окне:
    - ![img_1.png](docs/images/img_1.png)
    - Запомнить/сохранить токен в надежном месте.
- Установить `python3` и модуль `requests`:
  - Для пользователей **Arch Linux/Manjaro**:
    - `sudo pacman -Sy python python-requests --noconfirm`
  - Для пользователей **Microsoft Windows**:
    - `python3` - откроет страницу в MSS если Python не установлен
    - `pip install requests`

### Установка Poppy
- Скачиваем `poppup.py` отсюда: [ссылка](http://uav.radar-mms.com/gitlab/test/essentials/poppy/poppy-cli/-/raw/main/poppup.py?ref_type=heads)
- Запускаем `poppup.py`: 
  - `sudo python3 poppup.py --install-latest --where=/usr/bin --arch=linux-x64 --user=ЛОГИН_АРТИФАКТОРИ --token=ТОКЕН_АРТИФАКТОРИ`
- Установка прошла успешно, если последняя строка в выводе скрипта имеет вид:
  - `installed poppy to ....`
- Проверяем установку командой `poppy --version`

### Использование
`Poppy` использует manifest-файлы для разрешения зависимостей. 

В существующем или новом проекте рядом с `CMakeLists.txt` нужно создать файл с именем:
- `poppy-manifest.toml`

В этом файле будет находится описание пакета: имя, версии и зависимости. В каждом пакете обязательно должен быть manifest-файл.

Пример манифеста:
```toml
[package]
name = "test"
authors = ["whs31 <whs31@github.io>"]
description = "This is an example manifest"

[package.version]
major = 1
minor = 0
patch = 17

[dependencies]
spdlog = { version = { major = 12, minor = 4, patch = 4 }, distribution = "static" }
cmake = { version = { major = 0, minor = 1, patch = 0 }, distribution = "sources" }
fmt = { version = { major = 1, minor = 2, patch = 3 }, distribution = "shared" }
```
Этот манифест описывает пакет с именем `test` версии `1.0.17`, который зависит от библиотек:
- `spdlog@12.4.4/static`
- `cmake@0.1.0/sources`
- `fmt@1.2.3/shared`

#### Как узнать какие библиотеки доступны?
Для этого можно написать команду `poppy --sync`, которая выдаст список библиотек в реестре `poppy`:
![img.png](img.png)

#### Poppy требует авторизацию в артифактори! Что делать?
Необходимо написать следующие команды:
- `poppy --purge`
- `poppy --sync`
  - Ввести логин, затем токен

### Установка библиотек
В директории с манифестом запускаем:
- `poppy -si`
Если в манифесте нет ошибок, то библиотеки будут установленны в локальную папку `dependencies`.
Для сборки CMake-проекта с использованием `poppy` нужно указать префикс этой директории:
- `cmake -GNinja -DCMAKE_PREFIX_PATH=./dependencies ..`
- `cmake --build .`

### Помощь и поддержка
При возникших проблемах:
- `poppy --help`
- Если не помогло, открываем тикет здесь: [**YouTrack**](https://whs31.youtrack.cloud/projects/0-4?isNew=default)

### CI/CD
Вызов из раннера осуществляется следующими командами:
```shell
poppy --purge
poppy --sync --install --username gitlab_ci --token ${ARTIFACTORY_REFERENCE_KEY} --ci-git-username gitlab-ci-token --ci-git-token ${CI_JOB_TOKEN}
```