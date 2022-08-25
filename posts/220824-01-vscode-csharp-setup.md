---
title: VSCode C# setup
author: rainbyte
published: 2022-08-23 22:43:30
updated: 2022-08-24 23:28:00
tags: c#, dotnet, vscode
language: en
---

Follow these steps to configure a C# development environment using
VSCode (Code-OSS variant) with the Free OmniSharp extension.

This setup has been tested on **Arch Linux** with the following package versions:

- **code** 1.70.2-1
- **dotnet-sdk** 6.0.8.sdk108-1

## Step-by-step procedure

1. Install `dotnet` sdk and runtime
   - Arch Linux setup:

      ```sh
      sudo pacman -S dotnet-sdk
      ```

2. Make a directory anywhere you like. Go inside and use `dotnet` tool
   to create a sample project with `console` template:

   ```sh
   mkdir <ProjectName>
   cd <ProjectName>
   dotnet new console
   ```

   Note: this folder can be deleted later, it is just an example
   to test if the extension is working

3. Open the editor inside the project folder

   ```sh
   code .
   ```

4. Install [free-omnisharp-vscode extension](https://open-vsx.org/extension/muhammad-sammy/csharp) for C# development
5. Open `Program.cs` file
   - Note: this step is just to force extension to download OmniSharp implementation
6. Wait until **OmniSharp** dependencies get installed (it takes time)
7. If editor asks about adding `build & run` targets, say `yes`
   - It can also be enabled later inside **Run & Debug** tab
8. Press `F5` to build and execute the example code
   - **Breakpoints** are supported, add them to the left of the code
   - Open **Run & Debug** tab to see variables and watch expressions
9. Now the editor is configured and new projects can be created using `dotnet` tool (like in step 2)

`Happy hacking!` 🐱
