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
`ВАШ_ТОКЕН` нужно заменить на действительный токен *Artifactory*.
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
echo "-----BEGIN PGP PUBLIC KEY BLOCK-----
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
nPHPyV9PFKIoFJesoE7HokEIuzs8EI+Fu1c=
=ah1L
-----END PGP PUBLIC KEY BLOCK-----" > ~/.cache/apt-key/debian-local.pub && sudo apt-key add ~/.cache/apt-key/debian-local.pub && `echo 'deb http://ИМЯ_ПОЛЬЗОВАТЕЛЯ:ТОКЕН@213.170.107.251/artifactory/radar-local-repo astra main'>>/etc/apt/sources.list` && sudo apt update && sudo apt install -y puff
```
Поля `ИМЯ_ПОЛЬЗОВАТЕЛЯ` и `ТОКЕН` необходимо заменить на актуальные данные для аутентификации в *Artifactory*.
## Arch Linux (Arch, Manjaro, EndeavourOS)
todo =)

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