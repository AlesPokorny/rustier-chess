use super::magic_generation::Magic;
use crate::types::bitboard::BitBoard;

pub const ROOK_MAGICS: [Magic; 64] = [
    Magic {
        mask: BitBoard(0x101010101017e),
        magic: 0x108000c00080aa10,
        shift: 52,
    },
    Magic {
        mask: BitBoard(0x202020202027c),
        magic: 0x4000300041200a,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x404040404047a),
        magic: 0x200220028804090,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x8080808080876),
        magic: 0x280100080040800,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x1010101010106e),
        magic: 0x4180028048001400,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x2020202020205e),
        magic: 0x80050200140080,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x4040404040403e),
        magic: 0x2c00020401219008,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x8080808080807e),
        magic: 0x91001c4082002100,
        shift: 52,
    },
    Magic {
        mask: BitBoard(0x1010101017e00),
        magic: 0x890800080204004,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x2020202027c00),
        magic: 0x4000c00040201002,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x4040404047a00),
        magic: 0x801002001014230,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x8080808087600),
        magic: 0xa010808010000800,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x10101010106e00),
        magic: 0x102800800440080,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x20202020205e00),
        magic: 0x820048100d0200,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x40404040403e00),
        magic: 0x3011004100041200,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x80808080807e00),
        magic: 0x20000825c0201,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x10101017e0100),
        magic: 0x418024800840008c,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x20202027c0200),
        magic: 0x20008040008021,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x40404047a0400),
        magic: 0x220c420022028030,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x8080808760800),
        magic: 0x9010004008040040,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x101010106e1000),
        magic: 0x1010010080024,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x202020205e2000),
        magic: 0x8402808004004200,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x404040403e4000),
        magic: 0x1000040005881042,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x808080807e8000),
        magic: 0x20004028043,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x101017e010100),
        magic: 0x400080002882,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x202027c020200),
        magic: 0x2643c00440201001,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x404047a040400),
        magic: 0x8500080802000,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x8080876080800),
        magic: 0x4090100100020,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x1010106e101000),
        magic: 0x5008008080240009,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x2020205e202000),
        magic: 0x4000480020080,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x4040403e404000),
        magic: 0x85000300020014,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x8080807e808000),
        magic: 0x868011200028044,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x1017e01010100),
        magic: 0xb040401082800020,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x2027c02020200),
        magic: 0x120100842400062,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x4047a04040400),
        magic: 0x480a200041001500,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x8087608080800),
        magic: 0x100809000804800,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x10106e10101000),
        magic: 0x1001005000800,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x20205e20202000),
        magic: 0x200142a001018,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x40403e40404000),
        magic: 0x20800100800200,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x80807e80808000),
        magic: 0x2000a88042003104,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x17e0101010100),
        magic: 0x1a00800040008020,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x27c0202020200),
        magic: 0xc410052000484000,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x47a0404040400),
        magic: 0x1011e20080320042,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x8760808080800),
        magic: 0x82200a00120040,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x106e1010101000),
        magic: 0x2018010008450010,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x205e2020202000),
        magic: 0x8416008004008012,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x403e4040404000),
        magic: 0x1000100108040042,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x807e8080808000),
        magic: 0x2b000840830012,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x7e010101010100),
        magic: 0x48FFFE99FECFAA00,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x7c020202020200),
        magic: 0x48FFFE99FECFAA00,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x7a040404040400),
        magic: 0x497FFFADFF9C2E00,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x76080808080800),
        magic: 0x613FFFDDFFCE9200,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x6e101010101000),
        magic: 0xffffffe9ffe7ce00,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x5e202020202000),
        magic: 0xfffffff5fff3e600,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x3e404040404000),
        magic: 0x0003ff95e5e6a4c0,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x7e808080808000),
        magic: 0x510FFFF5F63C96A0,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x7e01010101010100),
        magic: 0xEBFFFFB9FF9FC526,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x7c02020202020200),
        magic: 0x61FFFEDDFEEDAEAE,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x7a04040404040400),
        magic: 0x53BFFFEDFFDEB1A2,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x7608080808080800),
        magic: 0x127FFFB9FFDFB5F6,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x6e10101010101000),
        magic: 0x411FFFDDFFDBF4D6,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x5e20202020202000),
        magic: 0x1015000812640081,
        shift: 53,
    },
    Magic {
        mask: BitBoard(0x3e40404040404000),
        magic: 0x0003ffef27eebe74,
        shift: 54,
    },
    Magic {
        mask: BitBoard(0x7e80808080808000),
        magic: 0x7645FFFECBFEA79E,
        shift: 53,
    },
];

pub const BISHOP_MAGICS: [Magic; 64] = [
    Magic {
        mask: BitBoard(0x40201008040200),
        magic: 0xffedf9fd7cfcffff,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x402010080400),
        magic: 0xfc0962854a77f576,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x4020100a00),
        magic: 0x800c410401004000,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x40221400),
        magic: 0x904404008331105e,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x2442800),
        magic: 0x100a021110080c04,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x204085000),
        magic: 0x1011032010000c00,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x20408102000),
        magic: 0xfc0a66c64a7ef576,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x2040810204000),
        magic: 0x7ffdfdfcbd79ffff,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x20100804020000),
        magic: 0xfc0846a64a34fff6,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x40201008040000),
        magic: 0xfc087a874a3cf7f6,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x4020100a0000),
        magic: 0x800041802004000,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x4022140000),
        magic: 0x90141400803100,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x244280000),
        magic: 0x60211004000,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x20408500000),
        magic: 0xc8010148400022,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x2040810200000),
        magic: 0xfc0864ae59b4ff76,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x4081020400000),
        magic: 0x3c0860af4b35ff76,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x10080402000200),
        magic: 0x73C01AF56CF4CFFB,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x20100804000400),
        magic: 0x41A01CFAD64AAFFC,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x4020100a000a00),
        magic: 0x2408103000222020,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x402214001400),
        magic: 0x18403a801421208,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x24428002800),
        magic: 0x820400a01000,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x2040850005000),
        magic: 0x2000c0a880504000,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x4081020002000),
        magic: 0x7c0c028f5b34ff76,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x8102040004000),
        magic: 0xfc0a028e5ab4df76,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x8040200020400),
        magic: 0x18440008109040,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x10080400040800),
        magic: 0x802184011012800,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x20100a000a1000),
        magic: 0x28040002440180,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x40221400142200),
        magic: 0x407080003004100,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x2442800284400),
        magic: 0x208840080802020,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x4085000500800),
        magic: 0x1240820109010080,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x8102000201000),
        magic: 0xa000822020881402,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x10204000402000),
        magic: 0x6012020120208a26,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x4020002040800),
        magic: 0x801c0440ce204240,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x8040004081000),
        magic: 0x8504101200094600,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x100a000a102000),
        magic: 0x2080220101981801,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x22140014224000),
        magic: 0x2020200800250104,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x44280028440200),
        magic: 0x6068020400201010,
        shift: 55,
    },
    Magic {
        mask: BitBoard(0x8500050080400),
        magic: 0x612008600010800,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x10200020100800),
        magic: 0x8828081310818280,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x20400040201000),
        magic: 0x904040140018044,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x2000204081000),
        magic: 0xDCEFD9B54BFCC09F,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x4000408102000),
        magic: 0xF95FFA765AFD602B,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0xa000a10204000),
        magic: 0x141188001000,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x14001422400000),
        magic: 0x2010028420210400,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x28002844020000),
        magic: 0x824700200608200,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x50005008040200),
        magic: 0x91100104400200,
        shift: 57,
    },
    Magic {
        mask: BitBoard(0x20002010080400),
        magic: 0x43ff9a5cf4ca0c01,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x40004020100800),
        magic: 0x4BFFCD8E7C587601,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x20408102000),
        magic: 0xfc0ff2865334f576,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x40810204000),
        magic: 0xfc0bf6ce5924f576,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0xa1020400000),
        magic: 0x120a01040400,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x142240000000),
        magic: 0x3200145220882008,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x284402000000),
        magic: 0x2080024008222242,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x500804020000),
        magic: 0x506083021021044,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x201008040200),
        magic: 0xc3ffb7dc36ca8c89,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x402010080400),
        magic: 0xc3ff8a54f4ca2c89,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x2040810204000),
        magic: 0xfffffcfcfd79edff,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x4081020400000),
        magic: 0xfc0863fccb147576,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0xa102040000000),
        magic: 0x400002080482208,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x14224000000000),
        magic: 0x8100000420208802,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x28440200000000),
        magic: 0x24000200a0425400,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x50080402000000),
        magic: 0x100001080a280200,
        shift: 59,
    },
    Magic {
        mask: BitBoard(0x20100804020000),
        magic: 0xfc087e8e4bb2f736,
        shift: 60,
    },
    Magic {
        mask: BitBoard(0x40201008040200),
        magic: 0x43ff9e4ef4ca2c89,
        shift: 59,
    },
];
