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
tag_name=abstract-parser/cachegrind

{
docker build -t $tag_name - <<'EOF'
    FROM rustlang/rust:nightly
    RUN apt-get update && apt-get install -y valgrind \
        && rm -rf /var/lib/apt/lists/*
    WORKDIR /app
EOF
} &&

{
    : ${2:?expect cargo build args}
    cargo_args="${@:2}"
    mkdir -p output &&
    docker run --rm -it \
        -v "$(pwd -W):/app" \
        -v abstract-parser-profiler-target:/app/target \
        -t $tag_name \
        bash -c "\
            cargo build $cargo_args && \
            valgrind --tool=cachegrind --cachegrind-out-file=output/cachegrind.out ${1:?expect path to target} && \
            cg_annotate --auto=yes --show-percs=no --threshold=0.0 --context=200 output/cachegrind.out \
              > output/cachegrind.annotated.txt
        "
} &&
docker rmi $tag_name
