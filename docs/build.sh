#!/bin/sh

if [ ! -x "$(command -v docker)" ]; then
    echo "Docker engine is required to continue, visit https://docs.docker.com/engine/install/"
    exit -1
fi

echo "Building the Docker 'latex-builder' image to compile the document..."
docker build . -t latex-builder

if [ $? -eq 0 ]; then
   echo "Compiling the document in the ./build/ directory..."
   docker run --rm -i --user="$(id -u):$(id -g)" --net=none -v "$PWD":/data latex-builder latexmk -gg -pdf -bibtex-cond1 -quiet -outdir=build uclmTFGesi.tex
   if [ $? -eq 0 ]; then
     echo "Document generated in ./build/uclmTFGesi.pdf"
   else
     echo "FAIL: compilation not completed"
     exit 1
   fi

else
   echo "FAIL: image not created"
   exit 1
fi

