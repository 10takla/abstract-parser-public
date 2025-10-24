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
set -e

header="scripts/license/FILE_HEADER"
files=$(git ls-files --cached --others --exclude-standard \
    | grep -v '\.abs$' \
    | grep -Fxv -e "LICENSE.md" -e "$header")
license_lines=$(wc -l < $header)

declare -A head=(
    [sh]="$(
        while IFS= read -r line; do
            echo "# ${line}"
        done < "$header"
    )"
)

for path in $files; do
    tmpfile=$(mktemp)
    case "$path" in
        (*.sh|**/pre-commit) {
            first_line=$(head -n1 "$path")

            if [[ $first_line == '#!'* ]]; then
                if diff -q \
                    <(tail -n +2 "$path" | head -n "$license_lines") \
                    <(printf '%s\n' "${head[sh]}") >/dev/null; then
                    echo "Already have: $path"
                    rm -f "$tmpfile"
                    continue
                fi

                {  
                    echo "$first_line"
                    printf '%s\n' "${head[sh]}"
                    tail -n +2 "$path"
                } > "$tmpfile"
            else
                if diff -q \
                    <(head -n "$license_lines" "$path") \
                    <(printf '%s\n' "${head[sh]}") >/dev/null; then
                    echo "Already have: $path"
                    rm -f "$tmpfile"
                    continue
                fi

                {
                    printf '%s\n' "${head[sh]}"
                    cat "$path"
                } > "$tmpfile"
            fi
        };;
        (*) {
            case "$path" in
                (*.toml) prefix="# ";;
                (*.rs) prefix="// ";;
                (*) prefix="# ";;
            esac
            while IFS= read -r line; do
                echo "${prefix}${line}"
            done < "$header" > "$tmpfile"

            if head -n $(wc -l < "$tmpfile") "$path" | diff -q "$tmpfile" - >/dev/null; then
                echo "Already have: $path"
                rm -f "$tmpfile"
                continue
            fi

            echo >> "$tmpfile"
            cat "$path" >> "$tmpfile"
        }
    esac
    
    mv "$tmpfile" "$path"
    rm -f "$tmpfile"
    echo "Inserted $path"
done