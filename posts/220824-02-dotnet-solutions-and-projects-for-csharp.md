---
title: .NET Solutions and Projects for C#
author: rainbyte
published: 2022-08-24 23:28:00
tags: c#, dotnet, vscode
language: en
---

### This publication is a follow up of the [VSCode C# setup](220824-01-vscode-csharp-setup.html) post

## .NET Solutions

The `dotnet` tool allows us to create projects with the `dotnet new` subcommand,
and because of that it is easy to end up with many projects mixed everywhere.

It is a common practice in C# to group related projects inside a **Solution**
folder to have them well organized.

This workflow can also be handled by the `dotnet` tool in this way:

1. Create a new Solution inside `<SolutionName>` folder to contain a group
   of related .NET projects

   ```sh
   dotnet new sln --output <SolutionName>
   ```

   Note: [PascalCase](https://en.wikipedia.org/wiki/Camel_case) (initial uppercase letters) is preferred for Solution names

2. Go inside the Solution folder

   ```sh
   cd <SolutionName>
   ```

3. Verify that there is a `.sln` file (eg. using the `ls` command)
4. Create a new project inside `<ProjectName>` subfolder:

   ```sh
   dotnet new console --output <ProjectName>
   ```

   Also add the project to the `.sln` config:

   ```sh
   dotnet sln add <ProjectName>
   ```

   Note: this should be done for each project we want to create

5. Modify the project code as needed
6. Build and execute the project using the following command

   ```sh
   dotnet run --project <ProjectName>
   ```

7. A project can also be removed from the Solution:

   ```sh
   dotnet sln remove <ProjectName>
   ```

   The remaining files should be removed by hand:

   ```sh
   rm -rf <ProjectName>
   ```

   Note: **be careful** with `rm`, deleted files will disappear forever! 😱

## VSCode and .NET Solutions

This editor also has support for .NET Solutions if the correct extension is installed

1. Go inside the Solution folder and start a VSCode instance

   ```sh
   cd <SolutionName>
   code .
   ```

2. If VSCode asks about **adding assets**, just **ignore** the message!
   - Note: the next section will show how to generate the assets for each specific project
3. Click the **Extensions** tab on the left toolbar
4. Look for the [vscode-solution-explorer](https://open-vsx.org/extension/fernandoescolar/vscode-solution-explorer) extension and install it
5. A new **Solution** tab should appear on the left toolbar
6. Click the **Solution** tab, a list of projects will be shown
7. Right-click any project in the list to see the context menu
8. Select `SolutionExplorer: Run` to build and execute that project
9. The context menues also allow other options:
   - Add a new project
   - Create a file (ie. class, interface, etc)
   - Remove a project from the Solution
   - etc ...

## Run & Debug a single Project

To use `Run & Debug` features the easiest option is to open a single Project
subfolder instead of a Solution folder.

1. Go inside a Project folder and start a VSCode instance

   ```sh
   cd <SolutionName>/<ProjectName>
   code .
   ```

2. If VSCode asks about **adding assets**, this time say **yes** and jump to step 6
   - Note: this creates a `.vscode` subfolder with the required configurations
3. Otherwise open a `.cs` file to launch C# extension
4. Go to **Run & Debug** tab on the left toolbar
5. Click on `Generate C# Assets for Build and Debug`
   - Note: this option only appears if `.vscode` assets were not created yet
6. Press `F5` or the `Start Debugging` button in the **Run & Debug** tab
   - Note: this step requires the assets to be already inside the `.vscode`
   subdirectory

`Happy hacking!` 🐱
