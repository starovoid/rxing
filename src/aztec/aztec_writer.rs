/*
 * Copyright 2013 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;

use crate::{
    common::{BitMatrix, CharacterSet, Result},
    exceptions::Exceptions,
    BarcodeFormat, EncodeHintType, EncodeHintValue, Writer,
};

use super::encoder::{aztec_encoder, AztecCode};

/**
 * Renders an Aztec code as a {@link BitMatrix}.
 */
#[derive(Default)]
pub struct AztecWriter;

impl Writer for AztecWriter {
    fn encode(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<crate::common::BitMatrix> {
        self.encode_with_hints(contents, format, width, height, &HashMap::new())
    }

    fn encode_with_hints(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
        hints: &std::collections::HashMap<crate::EncodeHintType, crate::EncodeHintValue>,
    ) -> Result<crate::common::BitMatrix> {
        let mut charset = None; // Do not add any ECI code by default
        let mut ecc_percent = aztec_encoder::DEFAULT_EC_PERCENT;
        let mut layers = aztec_encoder::DEFAULT_AZTEC_LAYERS;
        let mut dotsize = None;
        if let Some(EncodeHintValue::CharacterSet(cset_name)) =
            hints.get(&EncodeHintType::CHARACTER_SET)
        {
            if cset_name.to_lowercase() != "iso-8859-1" {
                charset = CharacterSet::get_character_set_by_name(cset_name);
            }
        }
        if let Some(EncodeHintValue::ErrorCorrection(ecc_level)) =
            hints.get(&EncodeHintType::ERROR_CORRECTION)
        {
            ecc_percent = ecc_level.parse().unwrap_or(23);
        }
        if let Some(EncodeHintValue::AztecLayers(az_layers)) =
            hints.get(&EncodeHintType::AZTEC_LAYERS)
        {
            layers = *az_layers;
        }
        if let Some(EncodeHintValue::DotSizePixels(size)) = hints.get(&EncodeHintType::DOT_SIZE) {
            dotsize = Some(*size);
        }
        encode(
            contents,
            *format,
            width as u32,
            height as u32,
            charset,
            ecc_percent,
            layers,
            dotsize,
        )
    }
}

fn encode(
    contents: &str,
    format: BarcodeFormat,
    width: u32,
    height: u32,
    charset: Option<CharacterSet>,
    ecc_percent: u32,
    layers: i32,
    dotsize: Option<u32>,
) -> Result<BitMatrix> {
    if format != BarcodeFormat::AZTEC {
        return Err(Exceptions::illegal_argument_with(format!(
            "can only encode AZTEC, but got {format:?}"
        )));
    }
    let aztec = if let Some(cset) = charset {
        // dbg!(cset.name(), cset.whatwg_name());
        aztec_encoder::encode_with_charset(contents, ecc_percent, layers, cset)?
    } else {
        aztec_encoder::encode(contents, ecc_percent, layers)?
    };
    renderRXingResult(&aztec, width, height, dotsize)
}

fn renderRXingResult(
    code: &AztecCode,
    width: u32,
    height: u32,
    dotsize: Option<u32>,
) -> Result<BitMatrix> {
    let input = code.getMatrix();

    let input_width = input.getWidth();
    let input_height = input.getHeight();

    // The "dotsize" parameter sets the size of the barcode element, instead of the preferred size
    // of the barcode itself.
    // The paddings will be zero when the "dotsize" parameter is set.
    let (width, height) = match dotsize {
        Some(m) => (input_width * m, input_height * m),
        None => (width, height),
    };

    let output_width = width.max(input_width);
    let output_height = height.max(input_height);

    let multiple = (output_width / input_width).min(output_height / input_height);
    let left_padding = (output_width - (input_width * multiple)) / 2;
    let top_padding = (output_height - (input_height * multiple)) / 2;

    let mut output = BitMatrix::new(output_width, output_height)?;

    let mut input_y = 0;
    let mut output_y = top_padding;
    while input_y < input_height {
        let mut input_x = 0;
        let mut output_x = left_padding;
        while input_x < input_width {
            if input.get(input_x, input_y) {
                output.setRegion(output_x, output_y, multiple, multiple)?;
            }

            input_x += 1;
            output_x += multiple;
        }

        input_y += 1;
        output_y += multiple
    }
    Ok(output)
}
