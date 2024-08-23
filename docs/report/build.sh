#!/bin/sh
USAGE="USAGE: The program can be executed without arguments or with the argument '--release' (to overwrite the main report)."

# Arguments
if [[ $# -gt 1 || ( $# -eq 1 && $1 != "--release" ) ]]; then
    echo $USAGE
    exit 1
fi

# System requirements
if [ ! -x "$(command -v docker)" ]; then
    echo "Docker engine is required to continue, visit https://docs.docker.com/engine/install/"
    exit 1
fi

# Build Docker image if not exists
if docker image inspect latex-builder >/dev/null 2>&1; then
    echo "Docker 'latex-builder' image exists locally"
else
    echo "Building the Docker 'latex-builder' image to compile the document... (it will take several minutes)"
    docker build . -t latex-builder
fi

if [ $? -eq 0 ]; then
  echo "Compiling the document in the docs/build/ directory..."
  docker run --rm -i --user="$(id -u):$(id -g)" --net=none -v "$PWD/src":/data -v "$PWD/build":/build latex-builder latexmk -gg -pdf -bibtex-cond1 -quiet -f -outdir=/build uclmTFGesi.tex || true # true to pick pdf despite errors
  if [ $? -eq 0 ]; then
    echo "Document generated in ./build/uclmTFGesi.pdf"
    # If release flag included
    if [[ $1 = "--release" ]]; then
        echo "Copying report to ./TFG_Adrian_Ramos_Report.pdf"
        cp ./build/uclmTFGesi.pdf ./TFG_Adrian_Ramos_Report.pdf
    fi
  else
    echo "FAIL: compilation not completed"
    exit 1
  fi
else
  echo "FAIL: image not created"
  exit 1
fi

