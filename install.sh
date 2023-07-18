#! /bin/bash

if [[ "$(uname -m)" = "arm64" ]]
then
  echo 'Downloading fuzzy-cli(arm64)'
  curl -s -o /tmp/fuzzy-cli https://fuzzy-cli.s3.us-west-2.amazonaws.com/arm64/fuzzy-cli
else
  echo 'Downloading fuzzy-cli(x86_64)'
  curl -s -o /tmp/fuzzy-cli https://fuzzy-cli.s3.us-west-2.amazonaws.com/x86_64/fuzzy-cli
fi

echo 'Download complete. Installing...'
sudo mv /tmp/fuzzy-cli /usr/local/bin/.
sudo chmod 755 /usr/local/bin/fuzzy-cli
echo 'Install complete'
