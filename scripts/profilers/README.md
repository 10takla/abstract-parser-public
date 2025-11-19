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

cachegrind:

```sh
scripts/profilers/cachegrind.sh <path_to_target> <cargo build args>
```

flamegraph:
```sh
scripts/profilers/flamegraph.sh <cargo build args>
```


пример inline-оптимизаций:
```sh
mkdir -p output && \
RUST_LOG=INFO cargo run --example parser --features logs --profile perf > output/parser-trace.log && \
scripts/profilers/cachegrind.sh target/perf/examples/parser --example parser --profile perf
```

[В логах](/output/parser-trace.log) смотрим на растущий расход стека и соотносим его с [аннотациями cachegrind](/output/cachegrind.annotated.txt). Находим горячие функции и навешиваем на них #[inline(always)].