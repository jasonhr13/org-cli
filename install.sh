#! /bin/bash

if [[ "$(uname -m)" = "arm64" ]]
then
  echo 'Downloading org-cli(arm64)'
  curl -s -o /tmp/org-cli S3_LOCATION
else
  echo 'Downloading org-cli(x86_64)'
  curl -s -o /tmp/org-cli S3_LOCATION
fi

echo 'Download complete. Installing...'
sudo mv /tmp/org-cli /usr/local/bin/.
sudo chmod 755 /usr/local/bin/org-cli
echo 'Install complete'
