---
title: Install NVIDIA driver on Debian Trixie
author: rainbyte
published: 2026-03-05 01:56:00
tags: cuda, debian, hardware, kernel, linux, nvidia
language: en
---

I needed to install NVIDIA driver 590 on Debian 13 (Trixie) because vLLM and
Llama.cpp will require CUDA 13.1, but the Debian repo's driver 550 only provides
CUDA 12.4.

So this guide covers two installation methods:

- Debian repo
  - simpler management
  - officially supported
  - limited to CUDA 12.4
- NVIDIA repo
  - provides CUDA 13.1 with driver 590
  - recommended for LLM engines

## TLDR

Choose the appropriate method:

| Requirement            | Method to be used  |
| :--------------------- | :----------------- |
| CUDA 12.4 (driver 550) | Debian repo method |
| CUDA 13.1 (driver 590) | NVIDIA repo method |

Both methods will require:

- Removing old packages
- Installing headers
- Rebooting when ready

<!-- more -->

## Debian repo method

This method is simpler and works well for most cases.

For Debian 13 (Trixie) the steps are these:

1. Remove old NVIDIA packages

   ```sh
   sudo apt purge "*nvidia*"
   sudo apt autoremove
   ```

2. To enable packages edit `/etc/apt/sources.list` and ensure each repo
   includes these attributes:

   ```txt
   contrib non-free non-free-firmware
   ```

3. Update package definitions

   ```sh
   sudo apt update
   ```

4. Install essential dependencies

   ```sh
   sudo apt install linux-headers-amd64 firmware-misc-nonfree
   ```

   Note: firmware is explicit because `nvidia-driver` package doesn't
   indicate a dependency on it
5. Install NVIDIA driver

   ```sh
   sudo apt install nvidia-driver nvidia-kernel-dkms
   ```

6. Reboot

   ```sh
   sudo reboot
   ```

7. Install CUDA dependencies (optional)

   ```sh
   sudo apt install nvidia-cuda-toolkit nvidia-cuda-dev nvidia-cuda-toolkit-gcc
   ```

   Note: CUDA is required by LLM engines like vLLM and Llama.cpp

## NVIDIA repo method

The NVIDIA repository provides a [compute-only installation][1] that avoids X11
dependencies, making it ideal for headless servers like mine.

**Warning**: all steps must be fully completed before rebooting, otherwise
the system will be left unbootable without the driver!

1. Remove old NVIDIA packages

   ```sh
   sudo apt purge "*nvidia*"
   sudo apt autoremove
   ```

2. Add NVIDIA repository

   ```sh
   wget https://developer.download.nvidia.com/compute/cuda/repos/debian13/sbsa/cuda-keyring_1.1-1_all.deb
   dpkg -i cuda-keyring_1.1-1_all.deb
   sudo apt update
   ```

3. Install compute-only system dependencies

   ```sh
   sudo apt install nvidia-driver-cuda nvidia-kernel-open-dkms
   ```

   Note: this is a headless installation that avoids X11 dependencies!
4. Reboot system

   ```sh
   sudo reboot
   ```

5. Verify driver is there by running vendor provided cli tool:

   ```sh
   nvidia-smi
   ```

   Note: this shows power usage, temperature, etc
6. Install CUDA dependencies (optional)

   ```sh
   sudo apt install cuda-compiler-13-1 cuda-libraries-13-1
   sudo apt install cuda-libraries-dev-13-1
   ```

   Note: CUDA is required by LLM engines like vLLM and Llama.cpp
7. Add CUDA bin directory to `PATH` env var with one of these commands:

   - fish shell:

     ```sh
     fish_add_path /usr/local/cuda/bin
     ```

   - bash shell:

     ```sh
     export PATH=${PATH}:/usr/local/cuda/bin
     ```

8. Provide `cicc` and `nvcc` at `/usr/bin`

   ```sh
   sudo update-alternatives \
       --install /usr/bin/nvcc nvcc /usr/local/cuda/bin/nvcc 100
   sudo update-alternatives \
       --install /usr/bin/cicc cicc /usr/local/cuda/nvvm/bin/nvcc 100
   ```

9. Check CUDA `nvcc` availability

   ```sh
   nvcc --version
   ```

10. Export environment variables

    ```sh
    # Using the symlink as base ensures minor updates are still compatible
    export CUDA_HOME="/usr/local/cuda"

    # Add NVCC compiler and CICC internal tools to PATH
    export PATH="$CUDA_HOME/bin:$CUDA_HOME/nvvm/bin:$PATH"

    # Some headers are nested, like cub/cub.cuh
    export CPATH="$CUDA_HOME/include/cccl:$CUDA_HOME/include:$CPATH"

    # Linking paths: stubs provides -lcuda when building without a live GPU
    export LIBRARY_PATH="$CUDA_HOME/lib64:$CUDA_HOME/lib64/stubs:$LIBRARY_PATH"

    # Runtime paths: omit stubs here to avoid runtime conflicts
    export LD_LIBRARY_PATH="$CUDA_HOME/lib64:$LD_LIBRARY_PATH"
    ```

    Note: this will be required by LLM engines to recognize CUDA libraries
11. To make these permanent, add them to shell's config file:

    - fish: `~/.config/fish/config.fish`
    - bash: `~/.bashrc`

## Conclusion

Follow NVIDIA repo instructions if you need CUDA 13.1 and 590 driver, otherwise
just use the drivers provided in Debian official repo.

`Happy hacking!` 🐱

[1]: https://docs.nvidia.com/datacenter/tesla/driver-installation-guide/debian.html
