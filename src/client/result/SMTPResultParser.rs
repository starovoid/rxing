/*
 * Copyright 2010 ZXing authors
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

// package com.google.zxing.client.result;

// import com.google.zxing.RXingResult;

use crate::RXingResult;

use super::{ResultParser, ParsedClientResult, EmailAddressParsedRXingResult};

/**
 * <p>Parses an "smtp:" URI result, whose format is not standardized but appears to be like:
 * {@code smtp[:subject[:body]]}.</p>
 *
 * @author Sean Owen
 */
pub fn parse(result: &RXingResult) -> Option<ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(result);
    if !(rawText.starts_with("smtp:") || rawText.starts_with("SMTP:")) {
      return None;
    }
    let mut emailAddress = &rawText[5..];
    let mut subject = "";
    let mut body = "";
    if let Some(colon) = emailAddress.find(':') {
      subject = &emailAddress[colon+1..];
      emailAddress = &emailAddress[..colon];
      if let Some(new_colon) = subject.find(':') {
        body = &subject[new_colon+1..];
        subject = &subject[..new_colon];
      }
    }
    // let colon = emailAddress.indexOf(':');
    // if (colon >= 0) {
    //   subject = emailAddress.substring(colon + 1);
    //   emailAddress = emailAddress.substring(0, colon);
    //   colon = subject.indexOf(':');
    //   if (colon >= 0) {
    //     body = subject.substring(colon + 1);
    //     subject = subject.substring(0, colon);
    //   }
    // }
    Some(ParsedClientResult::EmailResult(EmailAddressParsedRXingResult::with_details(vec![emailAddress.to_owned()],Vec::new(), Vec::new(), subject.to_owned(), body.to_owned())))
    // return new EmailAddressParsedRXingResult(new String[] {emailAddress},
    //                                     null,
    //                                     null,
    //                                     subject,
    //                                     body);
  }
