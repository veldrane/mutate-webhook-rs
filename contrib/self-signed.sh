#!/bin/bash

openssl req -new -newkey rsa:2048 -days 3650 -nodes -x509 -subj "/CN=build.vxland.syscallx86.com" -addext "subjectAltName=DNS:*.vxland.syscallx86.com" -keyout cert.key -out cert.pem