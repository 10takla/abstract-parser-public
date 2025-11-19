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

#!/usr/bin/env python3
import sys
import os
import base64
from bs4 import BeautifulSoup

BENCHMARK_DIR = os.path.join("target", "criterion", "features grammar")

html_path = os.path.join(BENCHMARK_DIR, "report", "index.html")
with open(html_path, 'r', encoding='utf-8') as f:
    soup = BeautifulSoup(f, 'html.parser')

body = soup.body

for tag in body.select('.additional_plots, .plots, .explanation, div#footer'):
    tag.decompose()

os.makedirs("output", exist_ok=True)

clean_html = str(body).replace('<body>', '').replace('</body>', '')

print(clean_html)