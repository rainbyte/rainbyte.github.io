---
title: VSCode title-bar colors
author: rainbyte
published: 2022-09-03 15:02:00
tags: colors, customization, organization, vscode
language: en
---

Today's objective is to highlight different projects folders in VSCode by
changing the window title-bars with specific colors. It will be similar to
the [Peacock extension](https://open-vsx.org/extension/johnpapa/vscode-peacock) but without any dependencies.

VSCode allows some UI customization via `.json` settings, there are 2 commands to access config files:

- **Open User Settings (JSON)**
- **Open Workspace Settings (JSON)**

In this case the **Workspace** command is preferred, given that it affects only
the current folder opened in VSCode.

## Configuration steps

1. Open project folder with VSCode
2. Launch command palette (press `ctrl+shift+p`)
3. Enter **Open Workspace Settings (JSON)** command
4. To set the custom title-bar color add a code similar to this:

   ```json
   {
       "workbench.colorCustomizations": {
           "titleBar.activeBackground": "#0f0",
           "titleBar.activeForeground": "#000",
           "titleBar.inactiveBackground": "#0f09",
           "titleBar.inactiveForeground": "#0009"
       }
   }
   ```

   Note: the numbers can be changed to select other colors

`Have fun! Happy hacking` 🐱
