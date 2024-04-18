FROM debian:sid
MAINTAINER Adrián Ramos <adrianramosrp@gmail.com>

RUN apt-get update && \
    apt-get install -y latexmk && \
    apt-get install -y texlive-lang-spanish && \
    apt-get install -y texlive-latex-recommended && \
    apt-get install -y texlive-fonts-extra && \
    apt-get install -y texlive-science

WORKDIR /data

VOLUME ["/data"]