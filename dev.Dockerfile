FROM ubuntu:24.04
LABEL version="1.0.0"
VOLUME /app/

WORKDIR /app/
#RUN apt-get update && apt-get install -y sudo curl wget git-core unzip zsh build-essential llvm-18 libpolly-18-dev libzstd-dev libz-dev
RUN apt-get update && apt-get install -y sudo curl wget git-core unzip zsh build-essential
RUN mkdir -p /opt/rust/cargo

RUN apt install make cmake -y
RUN apt install npm -y
RUN npm install --global yo generator-code

# Set environment variables
ENV RUSTUP_HOME=/opt/rust/rustup \
   CARGO_HOME=/opt/rust/cargo \
   PATH=$PATH:/opt/rust/cargo/bin:/usr/local/go/bin

# Install Rust
RUN mkdir -p /opt/rust/rustup /opt/rust/cargo && \
   curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain 1.87.0 --profile default --no-modify-path -y

# Install rust tools
RUN rustup component add llvm-tools-preview && cargo install grcov
RUN rustup target add wasm32-unknown-unknown

# Install zsh and oh-my-zsh
RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true

# Plugins
RUN git clone https://github.com/zsh-users/zsh-autosuggestions /root/.oh-my-zsh/custom/plugins/zsh-autosuggestions
RUN git clone https://github.com/zsh-users/zsh-syntax-highlighting.git /root/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting
RUN git clone https://github.com/MichaelAquilina/zsh-you-should-use.git /root/.oh-my-zsh/custom/plugins/you-should-use


# zsh configuration
RUN printf "export RUSTUP_HOME=/opt/rust/rustup\n\
export PATH=\${PATH}:/opt/rust/cargo/bin:/usr/local/go/bin\n\
CARGO_HOME=/opt/rust/cargo\n\
export ZSH=\"/root/.oh-my-zsh\"\n\
ZSH_THEME=\"robbyrussell\"\n\
plugins=(git zsh-autosuggestions zsh-syntax-highlighting you-should-use z)\n\
source /root/.oh-my-zsh/oh-my-zsh.sh\n\
git config --global --add safe.directory /app\n\
unalias ls ll l\n\
" > /root/.zshrc

RUN /bin/zsh /root/.zshrc
CMD ["/usr/bin/zsh"]
