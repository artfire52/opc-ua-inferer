#!/bin/bash
for i in {0..100};do
  docker rm -f "server_$i" >/dev/null 2>&1
done

