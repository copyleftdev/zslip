

## **🔥 ZipSlip Exploit: Deep Dive & Real-World Use Cases 🔥**

### **What is Zip Slip?**
**Zip Slip** is a critical security vulnerability that affects archive extraction libraries. It occurs when an attacker creates a malicious archive (e.g., a zip file) that contains file paths designed to "slip" out of the target directory during extraction. The malicious archive typically contains specially crafted paths like `../../../../../etc/passwd`, which use **directory traversal** characters (`../`) to escape the intended extraction directory.

This flaw is dangerous because it allows an attacker to **write files outside the intended extraction folder**, potentially **overwriting system files** or **executing arbitrary code** if critical files are modified.

### **How Does Zip Slip Work?**
The attack is enabled due to poor handling of file paths during the archive extraction process. In a vulnerable application, if the file paths are not properly validated, the extraction can create or overwrite files **anywhere** on the filesystem.

Let’s break it down with a quick example:

1. **An attacker creates a zip file** with a file path like `../../../../../../../../tmp/malicious_file.sh`.
2. When extracted, the zip file is intended to create a harmless file in the extraction folder.
3. However, the `../../` pattern **traverses up the directory tree**, and the resulting path ends up in `/tmp/malicious_file.sh`.
4. **If a script or system utility executes this file**, the attacker can gain control over the system.

### **Real-World Use Cases & Exploits**
#### **1. Malicious File Overwrite:**
An attacker targets a server that extracts zip files uploaded by users. They craft a zip file containing a path like `../../../etc/cron.d/malicious_cron`. Upon extraction, the malicious file is written to the `/etc/cron.d/` directory, setting up a scheduled task that grants the attacker a reverse shell at midnight.

#### **2. Code Execution Through Sensitive File Injection:**
A vulnerable system extracts zip files as part of a CI/CD process. An attacker uploads a zip containing `../../../opt/service/start.sh`, which overwrites the existing startup script. The next time the service restarts, the attacker's payload is executed, leading to remote code execution (RCE).

#### **3. Stealthy Configuration Manipulation:**
An archive contains `../../../../../../home/user/.ssh/authorized_keys`. When extracted, the file is placed in the target’s SSH configuration directory, giving the attacker SSH access to the victim's machine.

### **Common Attack Scenarios**
- **Web Applications:** Allowing users to upload zip files without proper validation.
- **CI/CD Pipelines:** Automated systems that unpack files for build processes.
- **Plugin & Theme Installers:** Unpacking third-party extensions and assets without sanitizing paths.
- **Backup Systems:** Systems that restore from compressed archives without verifying paths.

### **Educational Use Only ⚠️**
The tool provided here is for **educational purposes only** and should be used **solely** in controlled environments where you have explicit permission to test. Testing on systems without authorization can have serious legal implications.

If you’re developing or maintaining software that handles zip file extractions, make sure to **validate file paths** during extraction and **sanitize any path inputs** to prevent attackers from exploiting this vulnerability.

### **How to Safeguard Against Zip Slip**
- **Sanitize Paths:** Always resolve paths to their absolute values before extraction and ensure that they remain within the target directory.
- **Use Security Libraries:** Use libraries or functions that explicitly handle file paths and directory traversal issues.
- **Reject Suspicious Paths:** Reject any file paths containing `..` or absolute path patterns.

---

## **🚀 Build and Usage**

### **Prerequisites:**
1. **Install Rust**: Make sure you have Rust installed on your system. If not, install it via:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

### **Build Steps:**
1. **Clone the Repository**:

    ```bash
    git clone https://github.com/your-zip-slip-demo.git
    cd zip_slip_demo
    ```

    Alternatively, if you prefer starting from scratch:

    ```bash
    cargo new zip_slip_demo
    cd zip_slip_demo
    ```

2. **Add Dependencies:**
   Add the following to your `Cargo.toml` file:

    ```toml
    [dependencies]
    zip = "0.6"
    ```

3. **Compile the Project**:

    ```bash
    cargo build
    ```

### **Usage:**
To generate a malicious zip file, run:

```bash
cargo run -- --path='../../../../../tmp/malicious_file.txt' --data='This is a malicious file!' malicious.zip
```

- **--path**: The relative path you want to include in the zip file.
- **--data**: The content of the file.
- **malicious.zip**: The output zip file.

### **Testing the Malicious Zip:**
1. Extract the zip file using a tool like `unzip`:

    ```bash
    unzip malicious.zip
    ```

2. Check the `/tmp` directory to see if the file was created outside the intended extraction folder:

    ```bash
    cat /tmp/malicious_file.txt
    ```

If the extraction tool is vulnerable, the file should contain:

```
This is a malicious file!
```

---

### **Additional Reference: System Path Structures**
For more details on common directory paths across different operating systems (Linux, Windows, and macOS), refer to the [`PATHS`](PATHS.md) file. This document contains a comprehensive list of critical system paths, user-specific directories, logs, and configuration file locations for each OS. Understanding these paths is crucial for both offense and defense, ensuring you know exactly where files may be written or modified.

### **Key Takeaways**
- **Zip Slip** is a **serious vulnerability** that can lead to **arbitrary file writes** or **code execution**.
- Proper **path validation and sanitization** is key to defending against this exploit.
- Use the tool in a **controlled environment** to understand and mitigate the risks.

If you have any questions or want to learn more, feel free to reach out! And remember: **stay safe, stay legal**! 🛡️

---