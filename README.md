# 
# abstract-parser — proprietary, source-available software (not open-source).    
# Copyright (c) 2025 Abakar Letifov
# (Летифов Абакар Замединович). All rights reserved.
# 
# Use of this Work is permitted only for viewing and internal evaluation,        
# under the terms of the LICENSE file in the repository root.
# If you do not or cannot agree to those terms, do not use this Work.
# 
# THE WORK IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
# 

## Разработка

### Основные задачи проекта

- типо-безопасный парсер
- Задачи рекурсивного спуска:
  - минимизировать переполнение стека
  - как следстивие: увеличить число возможных рекурсий
- отзывчивый генератор парсера

### Иницализация репозитория

```
scripts/setup.sh
```

### Бенчмарки

Основные criterion-бенчмарки парсера:
```sh
cargo bench --all \
  --bench zpl \
  --bench cpcl \
  --bench features
```