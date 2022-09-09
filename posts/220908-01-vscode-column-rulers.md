---
title: VSCode column rulers
author: rainbyte
published: 2022-09-08 21:25:00
tags: colors, customization, vscode
language: en
---

Today we will configure VSCode to add custom **column rulers** to measure text
width. This feature doesn't require any 3rd party extension. Rulers are useful
to indicate us when a line of code is getting **too large**.

I recommend a width of maximum 74 or 80 columns for programming code. That
setup is convenient to allow showing files side-by-side.

VSCode provides `color` and `column` attributes to customize each ruler. The
colors are expressed by name or as hexadecimal values, and the columns are
just numbers.

## Configuration steps

Follow these steps to create as many column rulers as you want:

1. Open VSCode
2. Launch command palette (press `ctrl+shift+p`)
3. Enter **one** of the following commands to edit config:

    - `Open User Settings (JSON)`
      - This one affects all VSCode instances (global config)
    - `Open Workspace Settings (JSON)`
      - This one affects only the current project (folder config)
      - Config will be saved in **.vscode/settings.json**

4. To set 2 custom column rulers add a section similar to this:

    ```json
    {
        "editor.rulers": [
            60,
            {
                "column": 80,
                "color": "#f008"
            },
        ],
    }
    ```

    Notes:

    - This config creates a 1st ruler at column `60` with default color,
    and a 2nd ruler with explicit `column` and `color` fields.
    - All the values can be changed as wanted, and even more rulers can
    be added.
5. Save the config to apply the changes each time is modified.

`Happy hacking` 🐱
