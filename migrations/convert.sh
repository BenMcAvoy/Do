#!/usr/bin/env bash

find . -type f -name "*.sql" | while read file; do
  sed -i 's/\r//g' "$file"
  sed -i 's/ ,/,/g' "$file"
  sed -i 's/﻿//g' "$file"

  awk '!/^--/' "$file" > temp && mv temp "$file"

  sed -i '1{/^$/d;}' "$file"
  sed -i '1{/^$/d;}' "$file"
  sed -i '1{/^$/d;}' "$file"
done
