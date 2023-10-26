FROM ubuntu:22.04
RUN apt update && apt install -y \
    git \
    autoconf automake autotools-dev curl \
    python3 python3-pip libmpc-dev \
    libmpfr-dev libgmp-dev gawk build-essential \
    bison flex texinfo gperf libtool patchutils \
    bc zlib1g-dev libexpat-dev ninja-build cmake libglib2.0-dev \
    libpixman-1-dev 

RUN git clone --depth=1 https://github.com/riscv-collab/riscv-gnu-toolchain.git \
    && cd riscv-gnu-toolchain \
    && git submodule update --init --jobs 4 --recursive \
    && ./configure --prefix=/opt/riscv-32 --with-arch=rv32gc --with-abi=ilp32d \
    && make -j 4 && make -j 4 linux \
    && make clean \
    && ./configure --prefix=/opt/riscv-64 \
    && make -j 4 && make -j 4 linux \
    && cd qemu \
    && ./configure --target-list=riscv32-softmmu,riscv64-softmmu \
    && make -j 4 && make install \
    && cd ../.. \
    && rm -rf riscv-gnu-toolchain
ENV PATH="/opt/riscv-32/bin:/opt/riscv-64/bin:${PATH}"

ARG USER=riscv
ENV USER ${USER}
ENV HOME /home/${USER}

RUN apt update && apt install -y sudo curl

RUN useradd -m -s /bin/bash -N $USER && \
    echo "${USER} ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers && \
    chmod 0440 /etc/sudoers && \
    chmod g+w /etc/passwd 
USER $USER
WORKDIR $HOME
SHELL ["/bin/bash" , "-c"]
RUN git clone https://github.com/jkoeppeler/riscv-bare-metal.git
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y \
    && source "$HOME/.cargo/env" \
    && rustup target add riscv32imac-unknown-none-elf riscv64imac-unknown-none-elf

RUN echo "target remote :1234" > .gdbinit
