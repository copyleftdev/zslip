#!/bin/bash

OUTPUT_DIR="example_zips"
DUMMY_CONTENT="This is a placeholder file for demonstrating Zip Slip."
EXAMPLES=(
    "/bin"
    "/sbin"
    "/boot/initrd"
    "/dev/sda1"
    "/etc/passwd"
    "/lib/systemd"
    "/opt/application"
    "/usr/local/bin"
    "/var/log/syslog"
    "/tmp/testfile"
    "/home/sampleuser/Documents"
    "/root/.bashrc"
    "/etc/hostname"
    "/etc/hosts"
    "/etc/network/interfaces"
    "/var/log/secure"
    "C:/Program Files/AppFolder"
    "C:/Users/sampleuser/AppData/Local"
    "C:/Windows/System32/config/SYSTEM"
    "C:/Windows/System32/drivers/etc/hosts"
    "C:/Users/sampleuser/Documents"
    "C:/Users/sampleuser/Desktop"
    "/Users/sampleuser/Documents"
    "/Users/sampleuser/Library/Preferences"
)

6
mkdir -p $OUTPUT_DIR

generate_zip() {
  local target_path="$1"
  local zip_name="$OUTPUT_DIR/${target_path//\//_}.zip"

  temp_dir=$(mktemp -d)

  mkdir -p "$temp_dir/$(dirname "$target_path")"
  

  echo "$DUMMY_CONTENT" > "$temp_dir/$target_path"

  (cd "$temp_dir" && zip -r "$OLDPWD/$zip_name" .)


  rm -rf "$temp_dir"
  
  echo "Generated: $zip_name"
}


for path in "${EXAMPLES[@]}"; do
  generate_zip "$path"
done

echo "All example zips have been generated in the $OUTPUT_DIR directory."
