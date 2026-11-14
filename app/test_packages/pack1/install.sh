#!/bin/bash

if [ "$1" == "install" ]; then
  sudo echo "installing Hello World $1"
elif [ "$1" == "uninstall" ]; then
  sudo echo "uninstalling Hello World $1"
fi
