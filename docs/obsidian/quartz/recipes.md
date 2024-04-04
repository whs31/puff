---
title: Рецепты
---

Для того, чтобы пакет мог собираться в дереве зависимостей автоматически, ему нужен рецепт сборки (`.puff/recipe.yml`)

Что должен делать рецепт:
- Собирать пакет, если требуется (в любой папке);
- Устанавливать или копировать все необходимое в папку `target/export` в корне пакета.
Пример структуры папок для *CMake*-проекта:
```
. 
├── [ *sources* ] 
├── .puff 
│ └── recipe.yml
├── CMakeLists.txt 
├── Puff.toml 
└── target 
	├── [ *build artifacts* ] 
	└── export 
		└── [ *exported files* ]
```

Пример структуры папок для проекта без *CMake*:
```
. 
├── [ *sources* ] 
├── .puff 
│ └── recipe.yml
├── Puff.toml 
└── target 
	└── export 
		└── [ *exported files* ]
```

