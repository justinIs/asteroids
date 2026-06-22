# Custom cargo-quad-apk build image for macroquad Android builds.
#
# This Dockerfile is the upstream cargo-quad-apk Dockerfile with two changes:
#   1. RUST_VERSION defaults to `stable` (>= 1.85) instead of the pinned 1.79.0.
#   2. cargo-quad-apk is `git clone`d during the build instead of COPYed from the
#      build context, so this file is self-contained (no repo checkout needed).
#
# Build it via ./build-android.sh (which builds this image once, then reuses it).

FROM archlinux:latest

# Use bash with pipefail so failures in piped commands aren't masked.
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Override with: docker build --build-arg RUST_VERSION=1.85.0 ...
ARG RUST_VERSION=stable
ARG CARGO_APK_REPO=https://github.com/not-fl3/cargo-quad-apk

RUN pacman -Syu --noconfirm --needed \
        jdk17-openjdk unzip wget cmake rustup openssl pkgconf gcc zip git

# github overrides HOME, so pin rustup/cargo to a stable location
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN rustup toolchain install ${RUST_VERSION} && \
    rustup default ${RUST_VERSION} && \
    rustc --version && \
    rustup target add armv7-linux-androideabi \
                      aarch64-linux-android \
                      i686-linux-android \
                      x86_64-linux-android

# Install Android SDK
ENV ANDROID_HOME=/opt/android-sdk-linux
ENV JAVA_HOME=/usr/lib/jvm/default
WORKDIR ${ANDROID_HOME}
RUN wget -q https://dl.google.com/android/repository/commandlinetools-linux-13114758_latest.zip && \
    unzip -q commandlinetools-linux-13114758_latest.zip && \
    rm commandlinetools-linux-13114758_latest.zip && \
    mv cmdline-tools latest && \
    mkdir cmdline-tools/ && \
    mv latest cmdline-tools/ && \
    chown -R root:root /opt && \
    mkdir -p ~/.android && touch ~/.android/repositories.cfg && \
    { yes | ${ANDROID_HOME}/cmdline-tools/latest/bin/sdkmanager "platform-tools" | grep -v = || true; } && \
    { yes | ${ANDROID_HOME}/cmdline-tools/latest/bin/sdkmanager "platforms;android-36" | grep -v = || true; } && \
    { yes | ${ANDROID_HOME}/cmdline-tools/latest/bin/sdkmanager "build-tools;36.0.0-rc5" | grep -v = || true; } && \
    { ${ANDROID_HOME}/tools/bin/sdkmanager --update | grep -v = || true; }

# Install Android NDK
WORKDIR /usr/local
RUN wget -q http://dl.google.com/android/repository/android-ndk-r25-linux.zip && \
    unzip -q android-ndk-r25-linux.zip && \
    rm android-ndk-r25-linux.zip
ENV NDK_HOME=/usr/local/android-ndk-r25

# Build and install the cargo-quad-apk binary from source
RUN git clone --depth 1 ${CARGO_APK_REPO} /root/cargo-apk && \
    cargo install --path /root/cargo-apk && \
    rm -rf /root/cargo-apk

# Add build-tools to PATH, for apksigner
ENV PATH="/opt/android-sdk-linux/build-tools/36.0.0-rc5/:${PATH}"

# Make directory for user code (WORKDIR creates it)
WORKDIR /root/src
