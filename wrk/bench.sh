#!/usr/bin/bash

wrk -t2 -c200 -d5s -s ./wrk.lua http://127.0.0.1:3000
