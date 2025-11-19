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


## Добавить профайлинг стека по рекурсивным вызовам (для инлайн оптимизации)

### Алогоритм

#### 1. Инициализация

* Запуск:

  ```bash
  RUST_LOG=INFO cargo run --example parser --features logs > output/parser-trace.log
  ```
* Определить точку с самым худшим состоянием стека — **LogPoint**
* Зафиксировать **Метрики стека**

#### 2. Определение точек внедрения атрибута

* Определить точки, где возможно внедрение атрибута `#[inline(...)]`
* Зафиксировать текущее состояние атрибута **перед функцией**

#### 3. Для каждой точки

* Определить текущее состояние атрибута.
  Возможные состояния:

  * `#[inline(always)]`
  * `#[inline(never)]`
  * `#[inline]`
  * *без атрибута*
* Последовательно заменить текущее состояние на неиспользованные варианты
* После каждой замены зафиксировать **Метрики стека**

#### 4. Метрики стека

Для каждого варианта N раз выполнить:

```bash
RUST_LOG=INFO cargo run --example parser --features logs > output/parser-trace.log
```

Затем:

* Получить из `parser-trace.log` состояние стека в точке **LogPoint**
* Зафиксировать результаты для анализа
