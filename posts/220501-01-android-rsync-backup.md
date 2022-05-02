---
title: Android folder backup via Rsync
author: rainbyte
published: 2022-05-01 23:41:00
tags: android, backup, rsync
language: en
---

This post describes how to backup a folder from an Android phone to a PC and restore it on a 2nd phone.

## Backup procedure

1. Download simplesshd on the 1st phone ([play store](https://play.google.com/store/apps/details?id=org.galexander.sshd) link)
2. Open simplesshd and click `start`. The log will show some relevant information:

   - host: something like ip **192.168.x.y**
   - port: default is **2222**

3. Run rsync backup command on the PC. Replace `<host>` and `<port>` with the correct values for 1st phone!

   ```sh
   rsync -auv --delete -e 'ssh -p <port>' <host>:'/sdcard/orig-dir/' '/path/to/backup-dir/'
   ```

   - Rsync will copy files from 1st phone `orig-dir` folder to PC `backup-dir` folder
   - `delete` removes from PC folder the files not in the 1st phone
   - <u>Note</u>: final `/` on each folder are required!

## Restore procedure

1. Download simplesshd on the 2nd phone ([play store](https://play.google.com/store/apps/details?id=org.galexander.sshd) link)
2. Open simplesshd and click `start`. The log will show some relevant information:

   - host: something like ip **192.168.x.y**
   - port: default is **2222**

3. Run rsync restore command on the PC. Replace `<host>` and `<port>` with the correct values for 2nd phone!

   ```sh
   rsync -uv --omit-dir-times --no-perms --recursive --inplace --delete -e 'ssh -p <port>' '/path/to/backup-dir/' <host>:'/sdcard/dest-dir/'
   ```

   - Rsync will copy files from PC `backup-dir` folder to 2nd phone `dest-dir` folder
   - `inplace` avoids double sdcard write (caused by copy and rename)
   - `no-perms` is useful when perms are not supported, eg.: mtp mounts
   - `omit-dir-times` ignores timestamps
   - `delete` removes from 2nd phone folder the files not in the PC folder
   - <u>Note</u>: final `/` on each folder are required!
