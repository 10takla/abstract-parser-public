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
tag_name=abstract-parser/flamegraph

{
  docker build -t $tag_name - <<'EOF'
  FROM rustlang/rust:nightly
  RUN apt-get update && apt-get install -y linux-perf graphviz \
  && rm -rf /var/lib/apt/lists/*
  RUN cargo install flamegraph
  WORKDIR /app
  ENTRYPOINT ["cargo", "flamegraph"]
EOF
} &&

mkdir -p output &&
docker run --rm -it \
  --cap-add=SYS_ADMIN \
  --security-opt seccomp=unconfined \
  -v "$(pwd -W):/app" \
  -v abstract-parser-profiler-target:/app/target \
  -t $tag_name \
  "$@" --output output/flamegraph.svg &&

docker rmi $tag_name
