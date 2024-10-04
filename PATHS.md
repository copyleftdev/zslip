# 📂 **Common System Paths Cheat Sheet**

This document serves as a reference for common system paths across different operating systems: **Linux**, **Windows**, and **macOS**. Each section uses tables to organize paths for user directories, system files, logs, and more. Additionally, it highlights specific paths relevant to **Web Applications**, **CI/CD Pipelines**, **Plugin & Theme Installers**, and **Backup Systems**, where improper handling of path validation may lead to vulnerabilities like **Zip Slip**.

---

## **🌐 Linux Directory Structure**

| **Path**                     | **Description**                                         |
|------------------------------|---------------------------------------------------------|
| `/bin`                       | Essential binary executables (e.g., `ls`, `cp`, `mv`).   |
| `/sbin`                      | System binaries (e.g., `ifconfig`, `fdisk`).             |
| `/boot`                      | Boot loader files, such as the kernel and `initrd`.      |
| `/dev`                       | Device files (e.g., `/dev/sda1`, `/dev/null`).           |
| `/etc`                       | Configuration files (e.g., `/etc/hosts`, `/etc/passwd`). |
| `/lib`                       | Shared libraries used by system binaries.               |
| `/opt`                       | Optional add-on software and third-party applications.  |
| `/usr`                       | User binaries, libraries, documentation, and source code.|
| `/var`                       | Variable files (e.g., logs, spool files, databases).     |
| `/tmp`                       | Temporary files (often cleared on reboot).              |
| `/home`                      | Home directories for regular users.                     |
| `/root`                      | Home directory for the `root` user.                     |

### **User-Specific Paths**

| **Path**                             | **Description**                                     |
|--------------------------------------|-----------------------------------------------------|
| `/home/[username]`                   | Home directory for a specific user.                 |
| `/home/[username]/Documents`         | Default document storage.                           |
| `/home/[username]/Downloads`         | Downloaded files.                                   |
| `/home/[username]/.ssh`              | SSH keys and configurations.                        |
| `/home/[username]/.bashrc`           | User-specific shell initialization script.          |

### **System Configuration Files**

| **Path**                             | **Description**                                     |
|--------------------------------------|-----------------------------------------------------|
| `/etc/passwd`                        | User account information.                           |
| `/etc/shadow`                        | Secure account information (hashed passwords).      |
| `/etc/hostname`                      | System's hostname.                                  |
| `/etc/fstab`                         | Filesystem mount points.                            |
| `/etc/hosts`                         | Static table lookup for hostnames.                  |
| `/etc/resolv.conf`                   | DNS resolver configuration.                         |

### **Network Configuration Files**

| **Path**                             | **Description**                                     |
|--------------------------------------|-----------------------------------------------------|
| `/etc/network/interfaces`            | Network interface configuration (Debian-based).     |
| `/etc/sysconfig/network-scripts/`    | Network scripts (RedHat-based systems).             |

### **Log Files**

| **Path**                             | **Description**                                     |
|--------------------------------------|-----------------------------------------------------|
| `/var/log/syslog`                    | General system log.                                 |
| `/var/log/auth.log`                  | Authentication and authorization logs.              |
| `/var/log/kern.log`                  | Kernel log messages.                                |
| `/var/log/secure`                    | Security logs (RedHat-based systems).               |

---

## **🪟 Windows Directory Structure**

| **Path**                                     | **Description**                                      |
|----------------------------------------------|------------------------------------------------------|
| `C:\Program Files`                           | Installed 64-bit applications.                       |
| `C:\Program Files (x86)`                     | Installed 32-bit applications.                       |
| `C:\Users`                                   | User directories.                                    |
| `C:\Windows`                                 | Windows operating system files.                      |
| `C:\Windows\System32`                        | System binaries and executables.                     |
| `C:\Windows\SysWOW64`                        | 32-bit binaries on 64-bit Windows.                   |
| `C:\Windows\Temp`                            | Temporary files.                                     |
| `C:\Windows\System32\drivers\etc`            | Configuration files (`hosts`, `lmhosts`).             |
| `C:\Windows\System32\config`                 | Registry hives.                                      |
| `C:\Windows\Logs`                            | System log files.                                    |

### **User-Specific Paths**

| **Path**                                       | **Description**                                    |
|------------------------------------------------|----------------------------------------------------|
| `C:\Users\[username]\AppData`                  | Application data.                                   |
| `C:\Users\[username]\AppData\Local`            | Local user data.                                    |
| `C:\Users\[username]\AppData\Roaming`          | Roaming user data.                                  |
| `C:\Users\[username]\Documents`                | User documents.                                     |
| `C:\Users\[username]\Downloads`                | User downloads.                                     |
| `C:\Users\[username]\Desktop`                  | User desktop files.                                 |
| `C:\Users\[username]\.ssh`                     | SSH keys and configurations.                        |

### **System Configuration Files**

| **Path**                                     | **Description**                                      |
|----------------------------------------------|------------------------------------------------------|
| `C:\Windows\System32\drivers\etc\hosts`      | Static hostname resolution.                           |
| `C:\Windows\System32\config\SAM`             | Security Account Manager (user account database).     |
| `C:\Windows\System32\config\SOFTWARE`        | Registry hive for installed software.                 |
| `C:\Windows\System32\config\SYSTEM`          | System configuration.                                 |

### **Registry Paths**

| **Registry Key**                                  | **Description**                                     |
|---------------------------------------------------|-----------------------------------------------------|
| `HKEY_LOCAL_MACHINE (HKLM)`                       | Machine-specific settings.                           |
| `HKLM\SOFTWARE`                                   | Installed software settings.                         |
| `HKLM\SYSTEM`                                     | System configuration settings.                       |
| `HKEY_CURRENT_USER (HKCU)`                        | Current user settings.                               |
| `HKCU\Software`                                   | Per-user software settings.                          |

### **Temporary Files and Startup Directories**

| **Path**                                        | **Description**                                     |
|-------------------------------------------------|-----------------------------------------------------|
| `%TEMP%`                                        | Environment variable for temporary files.            |
| `C:\Windows\Temp`                                | System temporary files.                              |
| `C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup` | Global startup folder.                               |
| `C:\Users\[username]\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup` | User-specific startup folder.                        |

---

## **🍏 macOS Directory Structure**

| **Path**                               | **Description**                                     |
|----------------------------------------|-----------------------------------------------------|
| `/`                                    | Root directory.                                      |
| `/Applications`                        | Installed applications.                               |
| `/System`                              | macOS system files.                                   |
| `/Library`                             | System-wide libraries and application support files.  |
| `/Library/Preferences`                 | Global application preferences.                       |
| `/Library/Logs`                        | System and application log files.                     |
| `/Library/StartupItems`                | Legacy startup items.                                 |

### **User-Specific Paths**

| **Path**                                       | **Description**                                    |
|------------------------------------------------|----------------------------------------------------|
| `/Users/[username]`                            | Home directory for a specific user.                 |
| `/Users/[username]/Desktop`                    | User desktop files.                                 |
| `/Users/[username]/Documents`                  | User documents.                                     |
