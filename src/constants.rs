// Pieces declarations
pub const PAWN: i8 = 1;
pub const KNIGHT: i8 = 2;
pub const BISHOP: i8 = 3;
pub const ROOK: i8 = 4;
pub const QUEEN: i8 = 5;
pub const KING: i8 = 6;

pub const WHITE: i8 = 1;
pub const BLACK: i8 = 2;
pub const EMPTY: i8 = 0;

pub const BOARD_COORDINATES: [[&str; 8]; 8] = [
    ["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"],
    ["a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7"],
    ["a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6"],
    ["a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5"],
    ["a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4"],
    ["a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3"],
    ["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2"],
    ["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"],
];

// MVV_VLA[victim][attacker]
// Most Valued Victim, Least Valued Attacker
pub const MVV_LVA: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15, 0], // victim P, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25, 0], // victim N, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35, 0], // victim B, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45, 0], // victim R, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],       // victim None, attacker K, Q, R, B, N, P, None
];
pub const MG_PAWN_TABLE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [98, 134, 61, 95, 68, 126, 34, -11],
    [-6, 7, 26, 31, 65, 56, 25, -20],
    [-14, 13, 6, 21, 23, 12, 17, -23],
    [-27, -2, -5, 12, 17, 6, 10, -25],
    [-26, -4, -4, -10, 3, 3, 33, -12],
    [-35, -1, -20, -23, -15, 24, 38, -22],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
pub const MG_KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-167, -89, -34, -49, 61, -97, -15, -107],
    [-73, -41, 72, 36, 23, 62, 7, -17],
    [-47, 60, 37, 65, 84, 129, 73, 44],
    [-9, 17, 19, 53, 37, 69, 18, 22],
    [-13, 4, 16, 13, 28, 19, 21, -8],
    [-23, -9, 12, 10, 19, 17, 25, -16],
    [-29, -53, -12, -3, -1, 18, -14, -19],
    [-105, -21, -58, -33, -17, -28, -19, -23],
];
pub const MG_BISHOP_TABLE: [[i32; 8]; 8] = [
    [-29, 4, -82, -37, -25, -42, 7, -8],
    [-26, 16, -18, -13, 30, 59, 18, -47],
    [-16, 37, 43, 40, 35, 50, 37, -2],
    [-4, 5, 19, 50, 37, 37, 7, -2],
    [-6, 13, 13, 26, 34, 12, 10, 4],
    [0, 15, 15, 15, 14, 27, 18, 10],
    [4, 15, 16, 0, 7, 21, 33, 1],
    [-33, -3, -14, -21, -13, -12, -39, -21],
];

pub const MG_ROOK_TABLE: [[i32; 8]; 8] = [
    [32, 42, 32, 51, 63, 9, 31, 43],
    [27, 32, 58, 62, 80, 67, 26, 44],
    [-5, 19, 26, 36, 17, 45, 61, 16],
    [-24, -11, 7, 26, 24, 35, -8, -20],
    [-36, -26, -12, -1, 9, -7, 6, -23],
    [-45, -25, -16, -17, 3, 0, -5, -33],
    [-44, -16, -20, -9, -1, 11, -6, -71],
    [-19, -13, 1, 17, 16, 7, -37, -26],
];
pub const MG_QUEEN_TABLE: [[i32; 8]; 8] = [
    [-28, 0, 29, 12, 59, 44, 43, 45],
    [-24, -39, -5, 1, -16, 57, 28, 54],
    [-13, -17, 7, 8, 29, 56, 47, 57],
    [-27, -27, -16, -16, -1, 17, -2, 1],
    [-9, -26, -9, -10, -2, -4, 3, -3],
    [-14, 2, -11, -2, -5, 2, 14, 5],
    [-35, -8, 11, 2, 8, 15, -3, 1],
    [-1, -18, -9, 10, -15, -25, -31, -50],
];
pub const MG_KING_TABLE: [[i32; 8]; 8] = [
    [-65, 23, 16, -15, -56, -34, 2, 13],
    [29, -1, -20, -7, -8, -4, -38, -29],
    [-9, 24, 2, -16, -20, 6, 22, -22],
    [-17, -20, -12, -27, -30, -25, -14, -36],
    [-49, -1, -27, -39, -46, -44, -33, -51],
    [-14, -14, -22, -46, -44, -30, -15, -27],
    [1, 7, -8, -64, -43, -16, 9, 8],
    [-15, 36, 12, -54, 8, -28, 24, 14],
];

pub const EVAL_WEIGHTS: [[i8; 12]; 12] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 2, 2, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 2, 2, 2, 2, 1, 1, 0, 0],
    [0, 0, 1, 1, 2, 2, 2, 2, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 2, 2, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

pub const BENCH_FENS: [&str; 50] = [
    "r3k2r/2pb1ppp/2pp1q2/p7/1nP1B3/1P2P3/P2N1PPP/R2QK2R w KQkq a6 0 14",
    "4rrk1/2p1b1p1/p1p3q1/4p3/2P2n1p/1P1NR2P/PB3PP1/3R1QK1 b - - 2 24",
    "r3qbrk/6p1/2b2pPp/p3pP1Q/PpPpP2P/3P1B2/2PB3K/R5R1 w - - 16 42",
    "6k1/1R3p2/6p1/2Bp3p/3P2q1/P7/1P2rQ1K/5R2 b - - 4 44",
    "8/8/1p2k1p1/3p3p/1p1P1P1P/1P2PK2/8/8 w - - 3 54",
    "7r/2p3k1/1p1p1qp1/1P1Bp3/p1P2r1P/P7/4R3/Q4RK1 w - - 0 36",
    "r1bq1rk1/pp2b1pp/n1pp1n2/3P1p2/2P1p3/2N1P2N/PP2BPPP/R1BQ1RK1 b - - 2 10",
    "3r3k/2r4p/1p1b3q/p4P2/P2Pp3/1B2P3/3BQ1RP/6K1 w - - 3 87",
    "2r4r/1p4k1/1Pnp4/3Qb1pq/8/4BpPp/5P2/2RR1BK1 w - - 0 42",
    "4q1bk/6b1/7p/p1p4p/PNPpP2P/KN4P1/3Q4/4R3 b - - 0 37",
    "2q3r1/1r2pk2/pp3pp1/2pP3p/P1Pb1BbP/1P4Q1/R3NPP1/4R1K1 w - - 2 34",
    "1r2r2k/1b4q1/pp5p/2pPp1p1/P3Pn2/1P1B1Q1P/2R3P1/4BR1K b - - 1 37",
    "r3kbbr/pp1n1p1P/3ppnp1/q5N1/1P1pP3/P1N1B3/2P1QP2/R3KB1R b KQkq b3 0 17",
    "8/6pk/2b1Rp2/3r4/1R1B2PP/P5K1/8/2r5 b - - 16 42",
    "1r4k1/4ppb1/2n1b1qp/pB4p1/1n1BP1P1/7P/2PNQPK1/3RN3 w - - 8 29",
    "8/p2B4/PkP5/4p1pK/4Pb1p/5P2/8/8 w - - 29 68",
    "3r4/ppq1ppkp/4bnp1/2pN4/2P1P3/1P4P1/PQ3PBP/R4K2 b - - 2 20",
    "5rr1/4n2k/4q2P/P1P2n2/3B1p2/4pP2/2N1P3/1RR1K2Q w - - 1 49",
    "1r5k/2pq2p1/3p3p/p1pP4/4QP2/PP1R3P/6PK/8 w - - 1 51",
    "q5k1/5ppp/1r3bn1/1B6/P1N2P2/BQ2P1P1/5K1P/8 b - - 2 34",
    "r1b2k1r/5n2/p4q2/1ppn1Pp1/3pp1p1/NP2P3/P1PPBK2/1RQN2R1 w - - 0 22",
    "r1bqk2r/pppp1ppp/5n2/4b3/4P3/P1N5/1PP2PPP/R1BQKB1R w KQkq - 0 5",
    "r1bqr1k1/pp1p1ppp/2p5/8/3N1Q2/P2BB3/1PP2PPP/R3K2n b Q - 1 12",
    "r1bq2k1/p4r1p/1pp2pp1/3p4/1P1B3Q/P2B1N2/2P3PP/4R1K1 b - - 2 19",
    "r4qk1/6r1/1p4p1/2ppBbN1/1p5Q/P7/2P3PP/5RK1 w - - 2 25",
    "r7/6k1/1p6/2pp1p2/7Q/8/p1P2K1P/8 w - - 0 32",
    "r3k2r/ppp1pp1p/2nqb1pn/3p4/4P3/2PP4/PP1NBPPP/R2QK1NR w KQkq - 1 5",
    "3r1rk1/1pp1pn1p/p1n1q1p1/3p4/Q3P3/2P5/PP1NBPPP/4RRK1 w - - 0 12",
    "5rk1/1pp1pn1p/p3Brp1/8/1n6/5N2/PP3PPP/2R2RK1 w - - 2 20",
    "8/1p2pk1p/p1p1r1p1/3n4/8/5R2/PP3PPP/4R1K1 b - - 3 27",
    "8/4pk2/1p1r2p1/p1p4p/Pn5P/3R4/1P3PP1/4RK2 w - - 1 33",
    "8/5k2/1pnrp1p1/p1p4p/P6P/4R1PK/1P3P2/4R3 b - - 1 38",
    "8/8/1p1kp1p1/p1pr1n1p/P6P/1R4P1/1P3PK1/1R6 b - - 15 45",
    "8/8/1p1k2p1/p1prp2p/P2n3P/6P1/1P1R1PK1/4R3 b - - 5 49",
    "8/8/1p4p1/p1p2k1p/P2npP1P/4K1P1/1P6/3R4 w - - 6 54",
    "8/8/1p4p1/p1p2k1p/P2n1P1P/4K1P1/1P6/6R1 b - - 6 59",
    "8/5k2/1p4p1/p1pK3p/P2n1P1P/6P1/1P6/4R3 b - - 14 63",
    "8/1R6/1p1K1kp1/p6p/P1p2P1P/6P1/1Pn5/8 w - - 0 67",
    "1rb1rn1k/p3q1bp/2p3p1/2p1p3/2P1P2N/PP1RQNP1/1B3P2/4R1K1 b - - 4 23",
    "4rrk1/pp1n1pp1/q5p1/P1pP4/2n3P1/7P/1P3PB1/R1BQ1RK1 w - - 3 22",
    "r2qr1k1/pb1nbppp/1pn1p3/2ppP3/3P4/2PB1NN1/PP3PPP/R1BQR1K1 w - - 4 12",
    "2r2k2/8/4P1R1/1p6/8/P4K1N/7b/2B5 b - - 0 55",
    "6k1/5pp1/8/2bKP2P/2P5/p4PNb/B7/8 b - - 1 44",
    "2rqr1k1/1p3p1p/p2p2p1/P1nPb3/2B1P3/5P2/1PQ2NPP/R1R4K w - - 3 25",
    "r1b2rk1/p1q1ppbp/6p1/2Q5/8/4BP2/PPP3PP/2KR1B1R b - - 2 14",
    "6r1/5k2/p1b1r2p/1pB1p1p1/1Pp3PP/2P1R1K1/2P2P2/3R4 w - - 1 36",
    "rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2",
    "2rr2k1/1p4bp/p1q1p1p1/4Pp1n/2PB4/1PN3P1/P3Q2P/2RR2K1 w - f6 0 20",
    "3br1k1/p1pn3p/1p3n2/5pNq/2P1p3/1PN3PP/P2Q1PB1/4R1K1 w - - 0 23",
    "2r2b2/5p2/5k2/p1r1pP2/P2pB3/1P3P2/K1P3R1/7R w - - 23 93",
];
