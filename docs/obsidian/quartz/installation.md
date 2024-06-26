---
title: Установка
---

## Универсальный способ 
1. Необходимо установить **Rust** и **Cargo**:
	- Для ОС Windows: [ссылка](https://www.rust-lang.org/tools/install)
	- Для OC Linux (*любой дистрибутив*): ввести следующую команду 
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Необходимо добавить кастомный репозиторий *Cargo*. Для этого необходимо создать/отредактировать файл конфигурации *Cargo*, который находится по следующему пути:
	- Для ОС Windows: `C:\Users\<user>\.cargo\config.toml`
	- Для ОС Linux: `~\.cargo\config.toml`
	В файл необходимо записать следующее содержимое:
```toml
[registry]
global-credential-providers = ["cargo:token"]

[registries.artifactory]
index = "sparse+http://uav.radar-mms.com/artifactory/api/cargo/cargo-main/index/"
```
3. Для репозитория необходимо добавить аутентификацию. Для этого создаем/редактируем файл `credentials.toml`, который лежит рядом с вышеуказанным `config.toml`. В файле должно быть следующее содержимое:
```toml
[registries.artifactory]
token = "Bearer ВАШ_ТОКЕН"
```
`ВАШ_ТОКЕН` нужно заменить на действительный токен *Artifactory* (без имени пользователя).
4. Выполняем команду:
```shell
cargo install puff --registry "artifactory"   
```

Готово! Пакетный менеджер установлен в систему. Проверить его версию можно командой:
```shell
puff --version
```

## Debian Linux (Ubuntu, Mint, Debian, Astra)
Для ОС на основе *Debian* доступна установка через пакетный менеджер **apt**.
Для этого выполняем следующую команду:
```shell
curl https://gist.githubusercontent.com/whs31/635f4331a5d668f83e8de9e830fbb54d/raw/d28ae2b382886508f8ed0584b89d2a5791f9a44c/debian-local.pub -o debian-local.pub 
sudo apt-key add debian-local.pub 
sudo echo 'deb http://anonymous:cmVmdGtuOjAxOjAwMDAwMDAwMDA6bFpGQ2syeHozZGZZUVQ2cEhzdGJvZEpJcnlx@213.170.107.251/artifactory/radar-local-repo astra main'>>/etc/apt/sources.list 
sudo apt update
sudo apt install -y puff
puff --version
```
## Arch Linux (Arch, Manjaro, EndeavourOS)
- Добавление репозитория
```shell
sudo echo -e "\n[radar]\nSigLevel = Never\nServer = http://uav.radar-mms.com/pacman-packages" | sudo tee -a /etc/pacman.conf
```
- Установка пакета
```shell
sudo pacman -Sy puff
```

## Сборка вручную
- Необходимо установить **Rust**: [ссылка](https://www.rust-lang.org/tools/install)
- Склонировать репозиторий с пакетным менеджером через *Git*, либо скачать архив с исходным кодом
- Запустить в корневой папке репозитория команду:
  ```shell
  cargo build --release
	```
- В папке `target/release` появится исполняемый файл с названием `puff.exe`. Его необходимо поместить в директорию, находящуюся в системном `PATH`. Подробнее для Windows: [ссылка](https://learn.microsoft.com/ru-ru/previous-versions/office/developer/sharepoint-2010/ee537574(v=office.14))
- Теперь в терминале можно запустить команду `puff --version`. Ожидаемый вывод:
![image1](img1.png)
