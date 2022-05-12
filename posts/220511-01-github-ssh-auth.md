---
title: Autenticación en GitHub usando SSH
author: rainbyte
published: 2022-05-11 23:41:00
tags: git, github, ssh
language: es
---

Este post se muestra cómo autenticarse en GitHub mediante llaves SSH para poder interactuar con repositorios Git.

SSH (Secure SHell) es un protocolo que permite acceso remoto trabajando con pares de llaves (o claves) para generar canales seguros de comunicación.

El procedimiento es similar para otros proveedores de Git, como por ejemplo Bitbucket o GitLab.

## Registrar una llave pública

A continuación se explica como generar un par de llaves, una privada que es secreta, y otra pública que se registrará en GitHub u otro proveedor Git:

<!-- more -->

1. Generar el par de llaves SSH reemplazando los campos `<email>` y `<llave>` con valores apropiados

   ```sh
   ssh-keygen -t ed25519 -C <email> -f ~/.ssh/<llave>
   ```

   Nota: el nombre `id_ed25519_github` podría ser adecuado para el campo
   `<llave>`, ya que el par de llaves se creó para GitHub usando el esquema
   `ed25519`.

2. Verificar que los archivos `~/.ssh/<llave>` y `~/.ssh/<llave>.pub` existen, siendo el de extensión `.pub` la llave pública (eso significa que puede mostrarse a 3ros sin preocupaciones)

3. Abrir el archivo `~/.ssh/config` con un editor de texto, crearlo si no existe, y dependiendo del sistema operativo agregarle lo siguiente para cada caso:

   Sistema Linux:

   ```txt
   Host github.com
     IdentityFile ~/.ssh/<llave>
   ```

   Sistema macOS:

   ```txt
   Host github.com
     AddKeysToAgent yes
     UseKeychain    yes
     IdentityFile   ~/.ssh/<llave>
   ```

4. Agregar la llave privada para que el programa `ssh-agent` las recuerde

   Sistema Linux:

   ```sh
   ssh-add ~/.ssh/<llave>
   ```

   Sistema macOS:

   ```sh
   ssh-add -K ~/.ssh/<llave>
   ```

5. Agregar la llave pública a GitHub (esto debería ser similar para otros proveedores de repositorios Git)

   1. Ir a `Settings` en el menú superior derecho
   2. Abrir `SSH and GPG keys` en la columna izquierda
   3. Clickear el botón `New SSH Key`
   4. Llenar el campo `Title` a gusto con un nombre representativo
   5. Ejecutar el comando `cat ~/.ssh/<llave>.pub` (¡atención! extensión `.pub`), copiar el resultado y pegarlo en el campo `Key`
   6. Clickear el botón `Save SSH Key`

## Repositorios locales

Al abrir un repositorio en GitHub se puede obtener la url para clonarlo, para ello debe hacerse click en el botón verde que dice `Code ▼`, y luego abrir la pestaña SSH.

Una dirección url SSH tiene la forma `git@github.com:<usuario>/<repositorio>.git`,
y ese repositorio remoto puede clonarse de la siguiente manera:

```sh
git clone <url>
```

En caso de tener un repositorio previamente clonado desde un remoto llamado `origin` (es lo habitual), es posible cambiar su url así:

```sh
git remote set-url origin <url>
```

En ambos casos, y asumiendo que el remoto se llama `origin`, se puede verificar
la url del mismo con este comando:

```sh
git remote get-url origin <url>
```
