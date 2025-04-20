#!/bin/bash
args=${1:-提交文件}
git add .
git commit -m $args
#clear
git log --oneline -n 2