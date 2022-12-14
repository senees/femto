use super::__bid128;
use crate::BID128;

pub const BID_POWER10_TABLE_128: [BID128; 39] = [
  __bid128!(0x0000000000000001, 0x0000000000000000), //  10^0
  __bid128!(0x000000000000000A, 0x0000000000000000), //  10^1
  __bid128!(0x0000000000000064, 0x0000000000000000), //  10^2
  __bid128!(0x00000000000003E8, 0x0000000000000000), //  10^3
  __bid128!(0x0000000000002710, 0x0000000000000000), //  10^4
  __bid128!(0x00000000000186A0, 0x0000000000000000), //  10^5
  __bid128!(0x00000000000F4240, 0x0000000000000000), //  10^6
  __bid128!(0x0000000000989680, 0x0000000000000000), //  10^7
  __bid128!(0x0000000005F5E100, 0x0000000000000000), //  10^8
  __bid128!(0x000000003B9ACA00, 0x0000000000000000), //  10^9
  __bid128!(0x00000002540BE400, 0x0000000000000000), // 10^10
  __bid128!(0x000000174876E800, 0x0000000000000000), // 10^11
  __bid128!(0x000000E8D4A51000, 0x0000000000000000), // 10^12
  __bid128!(0x000009184E72A000, 0x0000000000000000), // 10^13
  __bid128!(0x00005AF3107A4000, 0x0000000000000000), // 10^14
  __bid128!(0x00038D7EA4C68000, 0x0000000000000000), // 10^15
  __bid128!(0x002386F26FC10000, 0x0000000000000000), // 10^16
  __bid128!(0x016345785D8A0000, 0x0000000000000000), // 10^17
  __bid128!(0x0DE0B6B3A7640000, 0x0000000000000000), // 10^18
  __bid128!(0x8AC7230489E80000, 0x0000000000000000), // 10^19
  __bid128!(0x6BC75E2D63100000, 0x0000000000000005), // 10^20
  __bid128!(0x35C9ADC5DEA00000, 0x0000000000000036), // 10^21
  __bid128!(0x19E0C9BAB2400000, 0x000000000000021E), // 10^22
  __bid128!(0x02C7E14AF6800000, 0x000000000000152D), // 10^23
  __bid128!(0x1BCECCEDA1000000, 0x000000000000D3C2), // 10^24
  __bid128!(0x161401484A000000, 0x0000000000084595), // 10^25
  __bid128!(0xDCC80CD2E4000000, 0x000000000052B7D2), // 10^26
  __bid128!(0x9FD0803CE8000000, 0x00000000033B2E3C), // 10^27
  __bid128!(0x3E25026110000000, 0x00000000204FCE5E), // 10^28
  __bid128!(0x6D7217CAA0000000, 0x00000001431E0FAE), // 10^29
  __bid128!(0x4674EDEA40000000, 0x0000000C9F2C9CD0), // 10^30
  __bid128!(0xC0914B2680000000, 0x0000007E37BE2022), // 10^31
  __bid128!(0x85ACEF8100000000, 0x000004EE2D6D415B), // 10^32
  __bid128!(0x38C15B0A00000000, 0x0000314DC6448D93), // 10^33
  __bid128!(0x378D8E6400000000, 0x0001ED09BEAD87C0), // 10^34
  __bid128!(0x2B878FE800000000, 0x0013426172C74D82), // 10^35
  __bid128!(0xB34B9F1000000000, 0x00C097CE7BC90715), // 10^36
  __bid128!(0x00F436A000000000, 0x0785EE10D5DA46D9), // 10^37
  __bid128!(0x098A224000000000, 0x4B3B4CA85A86C47A), // 10^38
];

pub const BID_ROUND_CONST_TABLE_128: [[BID128; 36]; 5] = [
  [
    //RN
    __bid128!(0u64, 0u64),                                // 0 extra digits
    __bid128!(5u64, 0u64),                                // 1 extra digits
    __bid128!(50u64, 0u64),                               // 2 extra digits
    __bid128!(500u64, 0u64),                              // 3 extra digits
    __bid128!(5000u64, 0u64),                             // 4 extra digits
    __bid128!(50000u64, 0u64),                            // 5 extra digits
    __bid128!(500000u64, 0u64),                           // 6 extra digits
    __bid128!(5000000u64, 0u64),                          // 7 extra digits
    __bid128!(50000000u64, 0u64),                         // 8 extra digits
    __bid128!(500000000u64, 0u64),                        // 9 extra digits
    __bid128!(5000000000u64, 0u64),                       // 10 extra digits
    __bid128!(50000000000u64, 0u64),                      // 11 extra digits
    __bid128!(500000000000u64, 0u64),                     // 12 extra digits
    __bid128!(5000000000000u64, 0u64),                    // 13 extra digits
    __bid128!(50000000000000u64, 0u64),                   // 14 extra digits
    __bid128!(500000000000000u64, 0u64),                  // 15 extra digits
    __bid128!(5000000000000000u64, 0u64),                 // 16 extra digits
    __bid128!(50000000000000000u64, 0u64),                // 17 extra digits
    __bid128!(500000000000000000u64, 0u64),               // 18 extra digits
    __bid128!(5000000000000000000u64, 0u64),              // 19 extra digits
    __bid128!(0xb5e3af16b1880000u64, 2u64),               //20
    __bid128!(0x1ae4d6e2ef500000u64, 27u64),              //21
    __bid128!(0xcf064dd59200000u64, 271u64),              //22
    __bid128!(0x8163f0a57b400000u64, 2710u64),            //23
    __bid128!(0xde76676d0800000u64, 27105u64),            //24
    __bid128!(0x8b0a00a425000000u64, 0x422cau64),         //25
    __bid128!(0x6e64066972000000u64, 0x295be9u64),        //26
    __bid128!(0x4fe8401e74000000u64, 0x19d971eu64),       //27
    __bid128!(0x1f12813088000000u64, 0x1027e72fu64),      //28
    __bid128!(0x36b90be550000000u64, 0xa18f07d7u64),      //29
    __bid128!(0x233a76f520000000u64, 0x64f964e68u64),     //30
    __bid128!(0x6048a59340000000u64, 0x3f1bdf1011u64),    //31
    __bid128!(0xc2d677c080000000u64, 0x27716b6a0adu64),   //32
    __bid128!(0x9c60ad8500000000u64, 0x18a6e32246c9u64),  //33
    __bid128!(0x1bc6c73200000000u64, 0xf684df56c3e0u64),  //34
    __bid128!(0x15c3c7f400000000u64, 0x9a130b963a6c1u64), //35
  ],
  [
    //RD
    __bid128!(0u64, 0u64),                  // 0 extra digits
    __bid128!(0u64, 0u64),                  // 1 extra digits
    __bid128!(0u64, 0u64),                  // 2 extra digits
    __bid128!(00u64, 0u64),                 // 3 extra digits
    __bid128!(000u64, 0u64),                // 4 extra digits
    __bid128!(0000u64, 0u64),               // 5 extra digits
    __bid128!(00000u64, 0u64),              // 6 extra digits
    __bid128!(000000u64, 0u64),             // 7 extra digits
    __bid128!(0000000u64, 0u64),            // 8 extra digits
    __bid128!(00000000u64, 0u64),           // 9 extra digits
    __bid128!(000000000u64, 0u64),          // 10 extra digits
    __bid128!(0000000000u64, 0u64),         // 11 extra digits
    __bid128!(00000000000u64, 0u64),        // 12 extra digits
    __bid128!(000000000000u64, 0u64),       // 13 extra digits
    __bid128!(0000000000000u64, 0u64),      // 14 extra digits
    __bid128!(00000000000000u64, 0u64),     // 15 extra digits
    __bid128!(000000000000000u64, 0u64),    // 16 extra digits
    __bid128!(0000000000000000u64, 0u64),   // 17 extra digits
    __bid128!(00000000000000000u64, 0u64),  // 18 extra digits
    __bid128!(000000000000000000u64, 0u64), // 19 extra digits
    __bid128!(0u64, 0u64),                  //20
    __bid128!(0u64, 0u64),                  //21
    __bid128!(0u64, 0u64),                  //22
    __bid128!(0u64, 0u64),                  //23
    __bid128!(0u64, 0u64),                  //24
    __bid128!(0u64, 0u64),                  //25
    __bid128!(0u64, 0u64),                  //26
    __bid128!(0u64, 0u64),                  //27
    __bid128!(0u64, 0u64),                  //28
    __bid128!(0u64, 0u64),                  //29
    __bid128!(0u64, 0u64),                  //30
    __bid128!(0u64, 0u64),                  //31
    __bid128!(0u64, 0u64),                  //32
    __bid128!(0u64, 0u64),                  //33
    __bid128!(0u64, 0u64),                  //34
    __bid128!(0u64, 0u64),                  //35
  ],
  [
    //RU
    __bid128!(0u64, 0u64),                                 // 0 extra digits
    __bid128!(9u64, 0u64),                                 // 1 extra digits
    __bid128!(99u64, 0u64),                                // 2 extra digits
    __bid128!(999u64, 0u64),                               // 3 extra digits
    __bid128!(9999u64, 0u64),                              // 4 extra digits
    __bid128!(99999u64, 0u64),                             // 5 extra digits
    __bid128!(999999u64, 0u64),                            // 6 extra digits
    __bid128!(9999999u64, 0u64),                           // 7 extra digits
    __bid128!(99999999u64, 0u64),                          // 8 extra digits
    __bid128!(999999999u64, 0u64),                         // 9 extra digits
    __bid128!(9999999999u64, 0u64),                        // 10 extra digits
    __bid128!(99999999999u64, 0u64),                       // 11 extra digits
    __bid128!(999999999999u64, 0u64),                      // 12 extra digits
    __bid128!(9999999999999u64, 0u64),                     // 13 extra digits
    __bid128!(99999999999999u64, 0u64),                    // 14 extra digits
    __bid128!(999999999999999u64, 0u64),                   // 15 extra digits
    __bid128!(9999999999999999u64, 0u64),                  // 16 extra digits
    __bid128!(99999999999999999u64, 0u64),                 // 17 extra digits
    __bid128!(999999999999999999u64, 0u64),                // 18 extra digits
    __bid128!(9999999999999999999u64, 0u64),               // 19 extra digits
    __bid128!(0x6BC75E2D630FFFFFu64, 0x5u64),              //20
    __bid128!(0x35C9ADC5DE9FFFFFu64, 0x36u64),             //21
    __bid128!(0x19E0C9BAB23FFFFFu64, 0x21eu64),            //22
    __bid128!(0x2C7E14AF67FFFFFu64, 0x152du64),            //23
    __bid128!(0x1BCECCEDA0FFFFFFu64, 0xd3c2u64),           //24
    __bid128!(0x1614014849FFFFFFu64, 0x84595u64),          //25
    __bid128!(0xDCC80CD2E3FFFFFFu64, 0x52b7d2u64),         //26
    __bid128!(0x9FD0803CE7FFFFFFu64, 0x33B2E3Cu64),        //27
    __bid128!(0x3E2502610FFFFFFFu64, 0x204FCE5Eu64),       //28
    __bid128!(0x6D7217CA9FFFFFFFu64, 0x1431E0FAEu64),      //29
    __bid128!(0x4674EDEA3FFFFFFFu64, 0xC9F2C9CD0u64),      //30
    __bid128!(0xC0914B267FFFFFFFu64, 0x7E37BE2022u64),     //31
    __bid128!(0x85ACEF80FFFFFFFFu64, 0x4EE2D6D415Bu64),    //32
    __bid128!(0x38c15b09ffffffffu64, 0x314dc6448d93u64),   //33
    __bid128!(0x378d8e63ffffffffu64, 0x1ed09bead87c0u64),  //34
    __bid128!(0x2b878fe7ffffffffu64, 0x13426172c74d82u64), //35
  ],
  [
    //RZ
    __bid128!(0u64, 0u64),                  // 0 extra digits
    __bid128!(0u64, 0u64),                  // 1 extra digits
    __bid128!(0u64, 0u64),                  // 2 extra digits
    __bid128!(00u64, 0u64),                 // 3 extra digits
    __bid128!(000u64, 0u64),                // 4 extra digits
    __bid128!(0000u64, 0u64),               // 5 extra digits
    __bid128!(00000u64, 0u64),              // 6 extra digits
    __bid128!(000000u64, 0u64),             // 7 extra digits
    __bid128!(0000000u64, 0u64),            // 8 extra digits
    __bid128!(00000000u64, 0u64),           // 9 extra digits
    __bid128!(000000000u64, 0u64),          // 10 extra digits
    __bid128!(0000000000u64, 0u64),         // 11 extra digits
    __bid128!(00000000000u64, 0u64),        // 12 extra digits
    __bid128!(000000000000u64, 0u64),       // 13 extra digits
    __bid128!(0000000000000u64, 0u64),      // 14 extra digits
    __bid128!(00000000000000u64, 0u64),     // 15 extra digits
    __bid128!(000000000000000u64, 0u64),    // 16 extra digits
    __bid128!(0000000000000000u64, 0u64),   // 17 extra digits
    __bid128!(00000000000000000u64, 0u64),  // 18 extra digits
    __bid128!(000000000000000000u64, 0u64), // 19 extra digits
    __bid128!(0u64, 0u64),                  //20
    __bid128!(0u64, 0u64),                  //21
    __bid128!(0u64, 0u64),                  //22
    __bid128!(0u64, 0u64),                  //23
    __bid128!(0u64, 0u64),                  //24
    __bid128!(0u64, 0u64),                  //25
    __bid128!(0u64, 0u64),                  //26
    __bid128!(0u64, 0u64),                  //27
    __bid128!(0u64, 0u64),                  //28
    __bid128!(0u64, 0u64),                  //29
    __bid128!(0u64, 0u64),                  //30
    __bid128!(0u64, 0u64),                  //31
    __bid128!(0u64, 0u64),                  //32
    __bid128!(0u64, 0u64),                  //33
    __bid128!(0u64, 0u64),                  //34
    __bid128!(0u64, 0u64),                  //35
  ],
  [
    //RN, ties away
    __bid128!(0u64, 0u64),                                // 0 extra digits
    __bid128!(5u64, 0u64),                                // 1 extra digits
    __bid128!(50u64, 0u64),                               // 2 extra digits
    __bid128!(500u64, 0u64),                              // 3 extra digits
    __bid128!(5000u64, 0u64),                             // 4 extra digits
    __bid128!(50000u64, 0u64),                            // 5 extra digits
    __bid128!(500000u64, 0u64),                           // 6 extra digits
    __bid128!(5000000u64, 0u64),                          // 7 extra digits
    __bid128!(50000000u64, 0u64),                         // 8 extra digits
    __bid128!(500000000u64, 0u64),                        // 9 extra digits
    __bid128!(5000000000u64, 0u64),                       // 10 extra digits
    __bid128!(50000000000u64, 0u64),                      // 11 extra digits
    __bid128!(500000000000u64, 0u64),                     // 12 extra digits
    __bid128!(5000000000000u64, 0u64),                    // 13 extra digits
    __bid128!(50000000000000u64, 0u64),                   // 14 extra digits
    __bid128!(500000000000000u64, 0u64),                  // 15 extra digits
    __bid128!(5000000000000000u64, 0u64),                 // 16 extra digits
    __bid128!(50000000000000000u64, 0u64),                // 17 extra digits
    __bid128!(500000000000000000u64, 0u64),               // 18 extra digits
    __bid128!(5000000000000000000u64, 0u64),              // 19 extra digits
    __bid128!(0xb5e3af16b1880000u64, 2u64),               //20
    __bid128!(0x1ae4d6e2ef500000u64, 27u64),              //21
    __bid128!(0xcf064dd59200000u64, 271u64),              //22
    __bid128!(0x8163f0a57b400000u64, 2710u64),            //23
    __bid128!(0xde76676d0800000u64, 27105u64),            //24
    __bid128!(0x8b0a00a425000000u64, 0x422cau64),         //25
    __bid128!(0x6e64066972000000u64, 0x295be9u64),        //26
    __bid128!(0x4fe8401e74000000u64, 0x19d971eu64),       //27
    __bid128!(0x1f12813088000000u64, 0x1027e72fu64),      //28
    __bid128!(0x36b90be550000000u64, 0xa18f07d7u64),      //29
    __bid128!(0x233a76f520000000u64, 0x64f964e68u64),     //30
    __bid128!(0x6048a59340000000u64, 0x3f1bdf1011u64),    //31
    __bid128!(0xc2d677c080000000u64, 0x27716b6a0adu64),   //32
    __bid128!(0x9c60ad8500000000u64, 0x18a6e32246c9u64),  //33
    __bid128!(0x1bc6c73200000000u64, 0xf684df56c3e0u64),  //34
    __bid128!(0x15c3c7f400000000u64, 0x9a130b963a6c1u64), //35
  ],
];

pub const BID_RECIPROCALS10_128: [BID128; 36] = [
  __bid128!(0u64, 0u64),                                   // 0 extra digits
  __bid128!(0x3333333333333334u64, 0x3333333333333333u64), // 1 extra digit
  __bid128!(0x51eb851eb851eb86u64, 0x051eb851eb851eb8u64), // 2 extra digits
  __bid128!(0x3b645a1cac083127u64, 0x0083126e978d4fdfu64), // 3 extra digits
  __bid128!(0x4af4f0d844d013aau64, 0x00346dc5d6388659u64), //  10^(-4) * 2^131
  __bid128!(0x08c3f3e0370cdc88u64, 0x0029f16b11c6d1e1u64), //  10^(-5) * 2^134
  __bid128!(0x6d698fe69270b06du64, 0x00218def416bdb1au64), //  10^(-6) * 2^137
  __bid128!(0xaf0f4ca41d811a47u64, 0x0035afe535795e90u64), //  10^(-7) * 2^141
  __bid128!(0xbf3f70834acdaea0u64, 0x002af31dc4611873u64), //  10^(-8) * 2^144
  __bid128!(0x65cc5a02a23e254du64, 0x00225c17d04dad29u64), //  10^(-9) * 2^147
  __bid128!(0x6fad5cd10396a214u64, 0x0036f9bfb3af7b75u64), // 10^(-10) * 2^151
  __bid128!(0xbfbde3da69454e76u64, 0x002bfaffc2f2c92au64), // 10^(-11) * 2^154
  __bid128!(0x32fe4fe1edd10b92u64, 0x00232f33025bd422u64), // 10^(-12) * 2^157
  __bid128!(0x84ca19697c81ac1cu64, 0x00384b84d092ed03u64), // 10^(-13) * 2^161
  __bid128!(0x03d4e1213067bce4u64, 0x002d09370d425736u64), // 10^(-14) * 2^164
  __bid128!(0x3643e74dc052fd83u64, 0x0024075f3dceac2bu64), // 10^(-15) * 2^167
  __bid128!(0x56d30baf9a1e626bu64, 0x0039a5652fb11378u64), // 10^(-16) * 2^171
  __bid128!(0x12426fbfae7eb522u64, 0x002e1dea8c8da92du64), // 10^(-17) * 2^174
  __bid128!(0x41cebfcc8b9890e8u64, 0x0024e4bba3a48757u64), // 10^(-18) * 2^177
  __bid128!(0x694acc7a78f41b0du64, 0x003b07929f6da558u64), // 10^(-19) * 2^181
  __bid128!(0xbaa23d2ec729af3eu64, 0x002f394219248446u64), // 10^(-20) * 2^184
  __bid128!(0xfbb4fdbf05baf298u64, 0x0025c768141d369eu64), // 10^(-21) * 2^187
  __bid128!(0x2c54c931a2c4b759u64, 0x003c7240202ebdcbu64), // 10^(-22) * 2^191
  __bid128!(0x89dd6dc14f03c5e1u64, 0x00305b66802564a2u64), // 10^(-23) * 2^194
  __bid128!(0xd4b1249aa59c9e4eu64, 0x0026af8533511d4eu64), // 10^(-24) * 2^197
  __bid128!(0x544ea0f76f60fd49u64, 0x003de5a1ebb4fbb1u64), // 10^(-25) * 2^201
  __bid128!(0x76a54d92bf80caa1u64, 0x00318481895d9627u64), // 10^(-26) * 2^204
  __bid128!(0x921dd7a89933d54eu64, 0x00279d346de4781fu64), // 10^(-27) * 2^207
  __bid128!(0x8362f2a75b862215u64, 0x003f61ed7ca0c032u64), // 10^(-28) * 2^211
  __bid128!(0xcf825bb91604e811u64, 0x0032b4bdfd4d668eu64), // 10^(-29) * 2^214
  __bid128!(0x0c684960de6a5341u64, 0x00289097fdd7853fu64), // 10^(-30) * 2^217
  __bid128!(0x3d203ab3e521dc34u64, 0x002073accb12d0ffu64), // 10^(-31) * 2^220
  __bid128!(0x2e99f7863b696053u64, 0x0033ec47ab514e65u64), // 10^(-32) * 2^224
  __bid128!(0x587b2c6b62bab376u64, 0x002989d2ef743eb7u64), // 10^(-33) * 2^227
  __bid128!(0xad2f56bc4efbc2c5u64, 0x00213b0f25f69892u64), // 10^(-34) * 2^230
  __bid128!(0x0f2abc9d8c9689d1u64, 0x01a95a5b7f87a0efu64), // 35 extra digits
];

pub const BID_RECIP_SCALE: [i32; 36] = [
  129 - 128, // 1
  129 - 128, // 1/10
  129 - 128, // 1/10^2
  129 - 128, // 1/10^3
  3,         // 131 - 128
  6,         // 134 - 128
  9,         // 137 - 128
  13,        // 141 - 128
  16,        // 144 - 128
  19,        // 147 - 128
  23,        // 151 - 128
  26,        // 154 - 128
  29,        // 157 - 128
  33,        // 161 - 128
  36,        // 164 - 128
  39,        // 167 - 128
  43,        // 171 - 128
  46,        // 174 - 128
  49,        // 177 - 128
  53,        // 181 - 128
  56,        // 184 - 128
  59,        // 187 - 128
  63,        // 191 - 128
  66,        // 194 - 128
  69,        // 197 - 128
  73,        // 201 - 128
  76,        // 204 - 128
  79,        // 207 - 128
  83,        // 211 - 128
  86,        // 214 - 128
  89,        // 217 - 128
  92,        // 220 - 128
  96,        // 224 - 128
  99,        // 227 - 128
  102,       // 230 - 128
  109,       // 237 - 128, 1/10^35
];
