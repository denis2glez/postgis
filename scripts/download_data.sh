#!/usr/bin/env bash

set -x
set -eo pipefail

DATA_DIR="data"

if [ ! -d $DATA_DIR ]; then
    mkdir $DATA_DIR

    # Download the `data.zip` file from Google Drive which contains the following:
    # - tf_mnist/input/*.png
    # - nyc_data.backup
    # - crnn.onnx
    # - alphabet_36.txt
    # - frozen_east_text_detection.pb
    # - macaque.jpg
    # - roads.geojson
    # Since it is around 120 MiB, we need the following workaround
    FILE_ID="1Vimn78opM6jMIdWoR3Hznqu2RbzrmOY5"
    FILE_NAME="data.zip"
    curl -c ./cookie -s -L "https://drive.google.com/uc?export=download&id=${FILE_ID}" >/dev/null
    curl -Lb ./cookie "https://drive.google.com/uc?export=download&confirm=$(awk '/download/ {print $NF}' ./cookie)&id=${FILE_ID}" -o ${FILE_NAME}

    unzip ${FILE_NAME}
    rm ${FILE_NAME} ./cookie

    echo "Data downloaded and unzipped!"
fi
