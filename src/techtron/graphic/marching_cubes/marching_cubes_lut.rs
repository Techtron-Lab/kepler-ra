// MIT License

// Copyright (c) 2023 Techtron-Lab

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


/*The second hexadecimal digit is the MC33 case number, the last two digits contain information
about the position in the respective case array and the order of triangle vertices*/
pub const MARCHING_CUBES_LUT: [u16; 2310] = [
    0x0000, 0x0885, 0x0886, 0x0895, 0x0883, 0x1816, 0x089D, 0x0943,//007
    0x0884, 0x0897, 0x1814, 0x0916, 0x0891, 0x094C, 0x091F, 0x048F,//015
    0x0882, 0x089B, 0x1808, 0x0934, 0x2803, 0x3817, 0x3814, 0x0525,//023
    0x180E, 0x0928, 0x4802, 0x049D, 0x3815, 0x0541, 0x6004, 0x0110,//031
    0x0881, 0x180A, 0x0899, 0x0922, 0x1806, 0x4800, 0x0913, 0x0499,//039
    0x2802, 0x380E, 0x3811, 0x053D, 0x380D, 0x6002, 0x0521, 0x010D,//047
    0x088B, 0x0946, 0x0937, 0x0493, 0x3813, 0x6003, 0x0545, 0x012E,//055
    0x380F, 0x0531, 0x600A, 0x011C, 0x5001, 0x3001, 0x3009, 0x0087,//063
    0x0880, 0x2801, 0x1804, 0x3807, 0x088F, 0x380B, 0x092B, 0x0539,//071
    0x1802, 0x3806, 0x4806, 0x6001, 0x0925, 0x051D, 0x0495, 0x010A,//079
    0x1812, 0x380A, 0x4805, 0x6009, 0x3816, 0x5002, 0x6008, 0x3010,//087
    0x4801, 0x6000, 0x7000, 0x4003, 0x6006, 0x3004, 0x4007, 0x1010,//095
    0x0889, 0x3808, 0x093A, 0x052D, 0x0949, 0x6005, 0x0491, 0x0131,//103
    0x380C, 0x5000, 0x600B, 0x3012, 0x0549, 0x3002, 0x0119, 0x008D,//111
    0x0907, 0x0535, 0x04A1, 0x013D, 0x0529, 0x3005, 0x0140, 0x0093,//119
    0x6007, 0x3000, 0x4004, 0x1000, 0x3003, 0x2000, 0x100C, 0x007f,//127
   
   /*
     Vertices:            Edges:               Faces:
       3 ___________2        _____2______         ____________
      /|           /|      /|           /|      /|           /|
     / |          / |     B |          A |     / |    2     / |
   7/___________6/  |    /_____6_____ /  |    /___________ /  |
   |   |        |   |   |   3        |   1   |   |     4  |   |
   |   |        |   |   |   |        |   |   | 3 |        | 1 |     z
   |   0________|___1   |   |_____0__|___|   |   |_5______|___|     |
   |  /         |  /    7  /         5  /    |  /         |  /      |____y
   | /          | /     | 8          | 9     | /      0   | /      /
   4/___________5/      |/_____4_____|/      |/___________|/      x
   
   */
   /*
   Vertices order in triangles:
          0     1-----2
         / \     \ x /   o: Front surface
        / o \     \ /    x: Back surface
       1-----2     0
   */
   
   /*position*index#vertices*/
   // Case 1 (128)
   /* 0*127#0*/0x0380,
   /* 1*064#1*/0x0109,
   /* 2*032#2*/0x021A,
   /* 3*016#3*/0x0B32,
   /* 4*004#5*/0x0945,
   /* 5*008#4*/0x0748,
   /* 6*001#7*/0x07B6,
   /* 7*002#6*/0x06A5,
   
   // Case 2 (136)
   /* 0*063#01*/0x1189,0x0138,
   /* 1*096#12*/0x129A,0x0092,
   /* 2*048#23*/0x1B3A,0x031A,
   /* 3*111#03*/0x12B0,0x0B80,
   /* 4*068#15*/0x1045,0x0105,
   /* 5*012#45*/0x1975,0x0987,
   /* 6*119#04*/0x1340,0x0374,
   /* 7*003#67*/0x1BA5,0x07B5,
   /* 8*009#47*/0x1486,0x0B68,
   /* 9*034#26*/0x1615,0x0216,
   /*10*017#37*/0x1726,0x0732,
   /*11*006#56*/0x146A,0x094A,
   
   // Case 3.1 (160)
   /* 0*123#05*/0x1945,0x0038,
   /* 1*072#14*/0x1109,0x0748,
   /* 2*066#16*/0x16A5,0x0109,
   /* 3*036#25*/0x1945,0x021A,
   /* 4*018#36*/0x16A5,0x032B,
   /* 5*033#27*/0x17B6,0x021A,
   /* 6*126#07*/0x17B6,0x0038,
   /* 7*024#34*/0x1B32,0x0748,
   /* 8*095#02*/0x121A,0x0038,
   /* 9*080#13*/0x1B32,0x0109,
   /*10*010#46*/0x16A5,0x0874,
   /*11*005#57*/0x1945,0x07B6,
   
   // Case 3.2 (184)
   /* 0*123#05*/0x1905,0x1035,0x1453,0x0843,
   /* 1*072#14*/0x1974,0x1917,0x1871,0x0081,
   /* 2*066#16*/0x1605,0x1950,0x116A,0x0106,
   /* 3*036#25*/0x1A45,0x1942,0x124A,0x0192,
   /* 4*018#36*/0x13A5,0x1B56,0x132A,0x0B35,
   /* 5*033#27*/0x11A6,0x17B2,0x1217,0x0671,
   /* 6*126#07*/0x1786,0x1806,0x1B60,0x03B0,
   /* 7*024#34*/0x1834,0x1324,0x1742,0x0B72,
   /* 8*095#02*/0x123A,0x138A,0x11A8,0x0018,
   /* 9*080#13*/0x1129,0x12B9,0x109B,0x030B,
   /*10*010#46*/0x14A5,0x1A86,0x148A,0x0768,
   /*11*005#57*/0x1B65,0x1794,0x17B9,0x059B,
   
   // Case 4.1.1 (232)
   /* 0*125#06*/0x16A5,0x0380,
   /* 1*065#17*/0x17B6,0x0109,
   /* 2*040#24*/0x121A,0x0748,
   /* 3*020#35*/0x1945,0x0B32,
   
   //The numbers in parentheses are the diagonal (interior test)
   // Case 4.1.2 (240)
   /* 0*(06)*125#06*/0x10A5,0x1805,0x1863,0x1685,0x16A3,0x03A0,
   /* 1*(17)*065#17*/0x1796,0x11B6,0x1169,0x1970,0x17B0,0x00B1,
   /* 2*(24)*040#24*/0x174A,0x17A2,0x1872,0x1A41,0x1481,0x0182,
   /* 3*(35)*020#35*/0x12B5,0x1B34,0x1943,0x15B4,0x1592,0x0293,
   
   // Case 5 (264)
   /* 0*112#123*/0x1B9A,0x1930,0x09B3,
   /* 1*079#023*/0x180A,0x101A,0x0B8A,
   /* 2*047#013*/0x189B,0x1B12,0x0B91,
   /* 3*031#012*/0x128A,0x1382,0x09A8,
   /* 4*038#256*/0x1246,0x1419,0x0421,
   /* 5*011#467*/0x18A5,0x1485,0x0BA8,
   /* 6*110#037*/0x1026,0x1067,0x0078,
   /* 7*059#015*/0x1845,0x1538,0x0135,
   /* 8*014#456*/0x176A,0x1A87,0x098A,
   /* 9*035#267*/0x1715,0x11B2,0x017B,
   /*10*076#145*/0x1175,0x1708,0x0710,
   /*11*025#347*/0x1426,0x1832,0x0248,
   /*12*070#156*/0x106A,0x1460,0x010A,
   /*13*055#014*/0x1149,0x1174,0x0371,
   /*14*103#034*/0x142B,0x174B,0x0024,
   /*15*019#367*/0x12A5,0x1532,0x0735,
   /*16*050#326*/0x1635,0x136B,0x0153,
   /*17*098#126*/0x1695,0x1609,0x0206,
   /*18*115#045*/0x1935,0x1390,0x0753,
   /*19*118#047*/0x13B6,0x1360,0x0406,
   /*20*007#576*/0x19BA,0x17B4,0x0B94,
   /*21*049#237*/0x17A6,0x171A,0x0317,
   /*22*100#125*/0x1A25,0x1245,0x0042,
   /*23*013#457*/0x1965,0x19B6,0x08B9,
   
   // Case 6.1.1 (336)
   /* 0*121#056*/0x146A,0x1A94,0x0380,
   /* 1*061#016*/0x16A5,0x1189,0x0138,
   /* 2*109#036*/0x16A5,0x180B,0x002B,
   /* 3*124#067*/0x1BA5,0x157B,0x0038,
   /* 4*093#026*/0x1615,0x1621,0x0803,
   /* 5*117#046*/0x16A5,0x1034,0x0374,
   /* 6*073#147*/0x18B6,0x1648,0x0109,
   /* 7*067#167*/0x1BA5,0x17B5,0x0109,
   /* 8*097#127*/0x129A,0x17B6,0x0092,
   /* 9*062#017*/0x17B6,0x1189,0x0138,
   /*10*081#137*/0x1726,0x1732,0x0109,
   /*11*069#157*/0x1045,0x17B6,0x0105,
   /*12*104#124*/0x129A,0x1209,0x0748,
   /*13*044#245*/0x1975,0x121A,0x0879,
   /*14*041#247*/0x1486,0x121A,0x08B6,
   /*15*056#234*/0x131A,0x1B3A,0x0874,
   /*16*087#024*/0x121A,0x1374,0x0403,
   /*17*042#246*/0x1615,0x1216,0x0874,
   /*18*107#035*/0x1945,0x12B0,0x0B80,
   /*19*052#235*/0x1945,0x1B3A,0x031A,
   /*20*022#356*/0x146A,0x194A,0x032B,
   /*21*028#345*/0x1975,0x1987,0x02B3,
   /*22*084#135*/0x1045,0x1510,0x0B32,
   /*23*021#357*/0x1945,0x1726,0x0732,
   
   // Case 6.1.2 (408)
   /* 0*(06)*121#056*/0x136A,0x190A,0x1094,0x1804,0x1684,0x1386,0x03A0,
   /* 1*(06)*061#016*/0x1895,0x11A5,0x136A,0x1591,0x13A1,0x1863,0x0856,
   /* 2*(06)*109#036*/0x10A5,0x1AB6,0x102A,0x1A2B,0x186B,0x1568,0x0058,
   /* 3*(06)*124#067*/0x10A5,0x1785,0x187B,0x138B,0x1A3B,0x103A,0x0058,
   /* 4*(06)*093#026*/0x1685,0x1236,0x1321,0x1031,0x1501,0x1805,0x0863,
   /* 5*(06)*117#046*/0x10A5,0x1456,0x1376,0x1674,0x1054,0x13A0,0x0A36,
   /* 6*(17)*073#147*/0x1496,0x1948,0x1098,0x1B08,0x110B,0x161B,0x0169,
   /* 7*(17)*067#167*/0x1795,0x11A5,0x11BA,0x1915,0x1097,0x1B07,0x00B1,
   /* 8*(17)*097#127*/0x19A6,0x16A2,0x1B62,0x10B2,0x17B0,0x1970,0x0796,
   /* 9*(17)*062#017*/0x1796,0x113B,0x1B38,0x17B8,0x1978,0x1169,0x01B6,
   /*10*(17)*081#137*/0x1796,0x1730,0x1032,0x1102,0x1612,0x1916,0x0970,
   /*11*(17)*069#157*/0x1165,0x1745,0x1756,0x1704,0x1B61,0x10B1,0x0B07,
   /*12*(24)*104#124*/0x149A,0x1208,0x1809,0x1489,0x174A,0x127A,0x0728,
   /*13*(24)*044#245*/0x19A5,0x175A,0x11A9,0x1819,0x1218,0x1728,0x027A,
   /*14*(24)*041#247*/0x14A6,0x18B2,0x12B6,0x1A26,0x11A4,0x1814,0x0182,
   /*15*(24)*056#234*/0x141A,0x1B7A,0x17B3,0x1873,0x1183,0x1481,0x04A7,
   /*16*(24)*087#024*/0x141A,0x1014,0x1103,0x1213,0x1723,0x1A27,0x04A7,
   /*17*(24)*042#246*/0x1645,0x1415,0x1746,0x1276,0x1872,0x1182,0x0814,
   /*18*(35)*107#035*/0x1925,0x1B84,0x1480,0x1940,0x1290,0x1B52,0x05B4,
   /*19*(35)*052#235*/0x19A5,0x1319,0x191A,0x1B5A,0x145B,0x134B,0x0439,
   /*20*(35)*022#356*/0x1B6A,0x1B46,0x12BA,0x192A,0x1329,0x1439,0x034B,
   /*21*(35)*028#345*/0x12B5,0x1398,0x1387,0x1B37,0x15B7,0x1925,0x0293,
   /*22*(35)*084#135*/0x1B45,0x1125,0x1210,0x1320,0x1430,0x1B34,0x0B52,
   /*23*(35)*021#357*/0x1265,0x1574,0x1567,0x1347,0x1943,0x1293,0x0925,
   
   // Case 6.2 (576)
   /* 0*121#056*/0x136A,0x190A,0x13A0,0x1684,0x0386,
   /* 1*061#016*/0x1685,0x136A,0x1895,0x113A,0x0863,
   /* 2*109#036*/0x10A5,0x1856,0x102A,0x186B,0x0058,
   /* 3*124#067*/0x10A5,0x1785,0x1058,0x1A3B,0x003A,
   /* 4*093#026*/0x1685,0x1236,0x1863,0x1501,0x0805,
   /* 5*117#046*/0x10A5,0x1A36,0x1054,0x1376,0x03A0,
   /* 6*073#147*/0x1496,0x1169,0x1B08,0x110B,0x061B,
   /* 7*067#167*/0x1795,0x11BA,0x10B1,0x1097,0x0B07,
   /* 8*097#127*/0x19A6,0x1796,0x10B2,0x17B0,0x0970,
   /* 9*062#017*/0x1796,0x113B,0x161B,0x1978,0x0169,
   /*10*081#137*/0x1796,0x1126,0x1730,0x1970,0x0916,
   /*11*069#157*/0x1165,0x1704,0x1B07,0x1B61,0x00B1,
   /*12*104#124*/0x149A,0x1208,0x1728,0x174A,0x027A,
   /*13*044#245*/0x1A75,0x127A,0x1819,0x1218,0x0728,
   /*14*041#247*/0x14A6,0x18B2,0x1182,0x11A4,0x0814,
   /*15*056#234*/0x174A,0x1B7A,0x1183,0x1481,0x0A41,
   /*16*087#024*/0x141A,0x1014,0x1723,0x1A27,0x04A7,
   /*17*042#246*/0x1415,0x1276,0x1814,0x1872,0x0182,
   /*18*107#035*/0x1925,0x1B84,0x15B4,0x1290,0x0B52,
   /*19*052#235*/0x1AB5,0x1319,0x1439,0x145B,0x034B,
   /*20*022#356*/0x1B46,0x192A,0x134B,0x1329,0x0439,
   /*21*028#345*/0x12B5,0x1398,0x1293,0x15B7,0x0925,
   /*22*084#135*/0x12B5,0x1125,0x1430,0x1B34,0x05B4,
   /*23*021#357*/0x1265,0x1925,0x1734,0x1943,0x0293,
   
   // Case 7.1 (696)
   /* 0*037#257*/0x1945,0x121A,0x07B6,
   /* 1*088#134*/0x12B3,0x1874,0x0109,
   /* 2*026#346*/0x16A5,0x1B32,0x0874,
   /* 3*091#025*/0x1945,0x121A,0x0038,
   /* 4*122#057*/0x1945,0x17B6,0x0038,
   /* 5*082#136*/0x16A5,0x1B32,0x0109,
   /* 6*074#146*/0x16A5,0x1109,0x0748,
   /* 7*094#027*/0x121A,0x17B6,0x0380,
   
   //The characters inside of the square bracket are face test results
   // Case 7.2 (720)
   /* 0*037#257*[.--..+]*/0x1B65,0x19B5,0x121A,0x1794,0x0B97,
   /* 1*088#134*[-..-+.]*/0x1B92,0x1874,0x1B30,0x19B0,0x0912,
   /* 2*026#346*[..--.+]*/0x14A5,0x1A86,0x1B32,0x1876,0x08A4,
   /* 3*091#025*[--..+.]*/0x1945,0x138A,0x1801,0x1A81,0x0A23,
   /* 4*122#057*[-..-.+]*/0x1B65,0x1380,0x1794,0x1B97,0x09B5,
   /* 5*082#136*[.--.+.]*/0x16A5,0x1129,0x1B92,0x1B30,0x09B0,
   /* 6*074#146*[--...+]*/0x14A5,0x1876,0x1109,0x18A4,0x0A86,
   /* 7*094#027*[..--+.]*/0x17B6,0x123A,0x18A3,0x1801,0x0A81,
   //(760)
   /* 0*037#257*[.+-..-]*/0x1A45,0x17B6,0x124A,0x1219,0x0429,
   /* 1*088#134*[-..+-.]*/0x1109,0x1483,0x1243,0x12B7,0x0427,
   /* 2*026#346*[..-+.-]*/0x16A5,0x1274,0x12B7,0x1483,0x0243,
   /* 3*091#025*[-+..-.]*/0x1A45,0x1038,0x1219,0x1429,0x024A,
   /* 4*122#057*[-..+.-]*/0x1945,0x1806,0x1678,0x103B,0x060B,
   /* 5*082#136*[.+-.-.]*/0x1605,0x1095,0x116A,0x132B,0x0061,
   /* 6*074#146*[-+...-]*/0x1605,0x1095,0x116A,0x1748,0x0061,
   /* 7*094#027*[..-+-.]*/0x1786,0x121A,0x103B,0x160B,0x0068,
   //(800)
   /* 0*037#257*[.-+..-]*/0x1945,0x11A6,0x1716,0x17B2,0x0172,
   /* 1*088#134*[+..--.]*/0x132B,0x1749,0x1108,0x1718,0x0179,
   /* 2*026#346*[..+-.-]*/0x13A5,0x1B56,0x1748,0x135B,0x032A,
   /* 3*091#025*[+-..-.]*/0x1905,0x121A,0x1350,0x1384,0x0534,
   /* 4*122#057*[+..-.-]*/0x1905,0x1534,0x17B6,0x1384,0x0350,
   /* 5*082#136*[.-+.-.]*/0x13A5,0x1B56,0x1109,0x132A,0x035B,
   /* 6*074#146*[+-...-]*/0x16A5,0x1749,0x1179,0x1108,0x0718,
   /* 7*094#027*[..+--.]*/0x11A6,0x1380,0x17B2,0x1172,0x0716,
   
   // Case 7.3 (840)
   /* 0*037#257*[.+-..+]*/0x1C65,0x1C5A,0x17C4,0x1C7B,0x1C94,0x1C19,0x1C21,0x1CA2,0x0CB6,
   /* 1*088#134*[-..++.]*/0x1C74,0x1CB7,0x1C2B,0x11C9,0x1C12,0x1C09,0x1C30,0x1C83,0x0C48,
   /* 2*026#346*[..-+.+]*/0x1CA5,0x1C6A,0x1C32,0x1C83,0x1C48,0x1C54,0x1C76,0x1CB7,0x0C2B,
   /* 3*091#025*[-+..+.]*/0x1AC5,0x1C38,0x1C23,0x1CA2,0x1C45,0x1C94,0x1C19,0x1C01,0x0C80,
   /* 4*122#057*[-..+.+]*/0x1C65,0x19C5,0x1CB6,0x1C3B,0x1C03,0x1C80,0x17C4,0x1C78,0x0C94,
   /* 5*082#136*[.+-.+.]*/0x16C5,0x1C6A,0x1C95,0x1C09,0x1C30,0x1CB3,0x1C2B,0x1C12,0x0CA1,
   /* 6*074#146*[-+...+]*/0x1C95,0x1C6A,0x1C10,0x1CA1,0x1C48,0x1C76,0x1C87,0x1C54,0x0C09,
   /* 7*094#027*[..-++.]*/0x1C1A,0x17C6,0x1C01,0x1C80,0x1C78,0x1CB6,0x1C3B,0x1C23,0x0CA2,
   //(912)
   /* 0*037#257*[.-+..+]*/0x1C65,0x1CA6,0x1C59,0x11C2,0x1CB2,0x17C4,0x1C7B,0x1C94,0x0C1A,
   /* 1*088#134*[+..-+.]*/0x1C2B,0x11C9,0x1C12,0x1C49,0x1C74,0x1C87,0x1C08,0x1C30,0x0CB3,
   /* 2*026#346*[..+-.+]*/0x1CA5,0x1C54,0x1C76,0x1C48,0x1C2A,0x1C32,0x1CB3,0x1C6B,0x0C87,
   /* 3*091#025*[+-..+.]*/0x1C45,0x12CA,0x1C84,0x1C38,0x1C23,0x1C59,0x1C1A,0x1C01,0x0C90,
   /* 4*122#057*[+..-.+]*/0x1C65,0x1C03,0x1C90,0x1C59,0x1CB6,0x17C4,0x1C7B,0x1C84,0x0C38,
   /* 5*082#136*[.-+.+.]*/0x1CA5,0x1C56,0x1C09,0x1C30,0x1CB3,0x1C6B,0x1C2A,0x1C12,0x0C91,
   /* 6*074#146*[+-...+]*/0x1CA5,0x1C6A,0x1C54,0x1C76,0x1C87,0x1C08,0x11C9,0x1C10,0x0C49,
   /* 7*094#027*[..+-+.]*/0x1CA6,0x17C6,0x1C1A,0x1C01,0x1C80,0x1C38,0x1C23,0x1CB2,0x0C7B,
   //(984)
   /* 0*037#257*[.++..-]*/0x1AC5,0x1CA6,0x1C94,0x1C19,0x1C21,0x1CB2,0x1C7B,0x1C67,0x0C45,
   /* 1*088#134*[+..+-.]*/0x1C2B,0x11C9,0x1C49,0x1C74,0x1CB7,0x1C32,0x1C83,0x1C08,0x0C10,
   /* 2*026#346*[..++.-]*/0x1CA5,0x1C56,0x1C2A,0x1C32,0x1C83,0x1C48,0x1C74,0x1CB7,0x0C6B,
   /* 3*091#025*[++..-.]*/0x1AC5,0x12CA,0x1C45,0x1C84,0x1C38,0x1C03,0x1C90,0x1C19,0x0C21,
   /* 4*122#057*[+..+.-]*/0x1C45,0x1CB6,0x1C3B,0x1C03,0x1C90,0x1C59,0x1C84,0x1C78,0x0C67,
   /* 5*082#136*[.++.-.]*/0x16C5,0x1C2A,0x13CB,0x1C6B,0x1C95,0x1C09,0x1C10,0x1CA1,0x0C32,
   /* 6*074#146*[++...-]*/0x16C5,0x1C6A,0x1C95,0x1C74,0x17C8,0x1C08,0x1C10,0x1CA1,0x0C49,
   /* 7*094#027*[..++-.]*/0x1CA6,0x1C80,0x1C78,0x1C67,0x1C1A,0x1C21,0x1CB2,0x1C3B,0x0C03,
   
   // Case 7.4.1 (1056)
   /* 0*037#257*/0x1A65,0x1419,0x11B2,0x17B4,0x04B1,
   /* 1*088#134*/0x174B,0x1149,0x12B1,0x11B4,0x0830,
   /* 2*026#346*/0x12A5,0x1485,0x1B76,0x1832,0x0582,
   /* 3*091#025*/0x1A25,0x1238,0x1458,0x1528,0x0019,
   /* 4*122#057*/0x1965,0x1390,0x1B63,0x1693,0x0784,
   /* 5*082#136*/0x1695,0x112A,0x1093,0x1B36,0x0396,
   /* 6*074#146*/0x1495,0x176A,0x110A,0x1870,0x07A0,
   /* 7*094#027*/0x17A6,0x1780,0x11A0,0x1A70,0x03B2,
   
   // Case 7.4.2 (1096)
   /* 0*(06)*037#257*/0x1465,0x1459,0x121A,0x1AB2,0x1BA6,0x17B6,0x1476,0x15A1,0x0951,
   /* 1*(06)*088#134*/0x1084,0x1748,0x18B7,0x1B83,0x12B3,0x1109,0x1130,0x1123,0x0904,
   /* 2*(17)*026#346*/0x16A5,0x132B,0x1B83,0x1874,0x18B7,0x1576,0x1547,0x16B2,0x0A62,
   /* 3*(17)*091#025*/0x1945,0x115A,0x1038,0x1023,0x1201,0x1A21,0x1519,0x1908,0x0498,
   /* 4*(24)*122#057*/0x1465,0x1380,0x1890,0x1984,0x1594,0x1647,0x1B67,0x1783,0x0B73,
   /* 5*(24)*082#136*/0x16A5,0x1A95,0x19A1,0x1091,0x1312,0x1301,0x1B32,0x12A6,0x0B26,
   /* 6*(35)*074#146*/0x16A5,0x1109,0x19A1,0x1A95,0x1754,0x1765,0x1874,0x1490,0x0840,
   /* 7*(35)*094#027*/0x1BA6,0x1380,0x1378,0x173B,0x167B,0x1AB2,0x11A2,0x1230,0x0120,
   
   // Case 8 (1168)
   /* 0*015#0123*/0x1B9A,0x09B8,
   /* 1*102#0347*/0x1426,0x0024,
   /* 2*051#0145*/0x1375,0x0135,
   
   // Case 9 (1174)
   /* 0*078#0237*/0x17A6,0x1180,0x1781,0x0A71,
   /* 1*039#0134*/0x1B42,0x1129,0x1492,0x04B7,
   /* 2*027#0125*/0x1A35,0x13A2,0x1853,0x0584,
   /* 3*114#0457*/0x1965,0x13B0,0x10B6,0x0069,
   
   // Case 10.1.1 (1190)
   /* 0*105#0356*[-.-...]*/0x146A,0x194A,0x1028,0x082B,
   /* 1*060#0167*[.-.-..]*/0x17A5,0x1189,0x1381,0x07BA,
   /* 2*085#0246*[....--]*/0x1625,0x1340,0x1743,0x0521,
   //(1202)
   /* 0*105#0356*[+.+...]*/0x1846,0x190A,0x186B,0x002A,
   /* 1*060#0167*[.+.+..]*/0x1795,0x11BA,0x1789,0x03B1,
   /* 2*085#0246*[....++]*/0x1015,0x1236,0x1540,0x0763,
   
   // Case 10.1.2 (1214)
   /* 0*(06)(35)*105#0356*[-.-...]*/0x126A,0x1029,0x12A9,0x1B62,0x16B8,0x1468,0x1489,0x0809,
   /* 1*(06)(17)*060#0167*[.-.-..]*/0x19A5,0x1789,0x1957,0x11A9,0x1A13,0x1BA3,0x1B37,0x0387,
   /* 2*(06)(24)*085#0246*[....--]*/0x1405,0x1756,0x1015,0x1021,0x1320,0x1237,0x1627,0x0745,
   //(1238)
   /* 0*(17)(24)*105#0356*[+.+...]*/0x146A,0x1A94,0x1904,0x1408,0x1B80,0x1B02,0x1AB2,0x0A6B,
   /* 1*(35)(24)*060#0167*[.+.+..]*/0x1BA5,0x1189,0x1138,0x13B8,0x18B7,0x157B,0x115A,0x0195,
   /* 2*(17)(35)*085#0246*[....++]*/0x1465,0x1156,0x1621,0x1231,0x1130,0x1403,0x1437,0x0647,
   
   // Case 10.2 (1262)
   /* 0*105#0356*[+.-...]*/0x19CA,0x1AC6,0x190C,0x102C,0x12BC,0x18CB,0x184C,0x046C,
   /* 1*060#0167*[.+.-..]*/0x1C95,0x1CBA,0x1C7B,0x1C57,0x19C8,0x1C38,0x1C13,0x01CA,
   /* 2*085#0246*[....-+]*/0x1C15,0x12C6,0x1C21,0x14C5,0x1C76,0x17C3,0x1C03,0x0C40,
   //(1286)
   /* 0*105#0356*[-.+...]*/0x1C46,0x1C2A,0x1C94,0x1CA9,0x12C0,0x1C80,0x1CB8,0x0BC6,
   /* 1*060#0167*[.-.+..]*/0x1CA5,0x17C5,0x1CBA,0x1C3B,0x11C9,0x13C1,0x1C89,0x08C7,
   /* 2*085#0246*[....+-]*/0x16C5,0x12C6,0x123C,0x174C,0x137C,0x10C4,0x101C,0x015C,
   
   // Case 11 (1310)
   /* 0*077#0236*/0x16B5,0x1B80,0x15B0,0x0150,
   /* 1*046#0137*/0x1786,0x1189,0x1126,0x0681,
   /* 2*023#0124*/0x129A,0x1974,0x1792,0x0372,
   /* 3*116#0467*/0x14A5,0x13BA,0x13A4,0x0340,
   /* 4*099#0345*/0x1975,0x1902,0x1927,0x072B,
   /* 5*057#0156*/0x116A,0x1846,0x1861,0x0813,
   
   // Case 14 (1334)
   /* 0*113#0456*/0x176A,0x1A90,0x17A0,0x0370,
   /* 1*071#0234*/0x1B1A,0x1140,0x11B4,0x074B,
   /* 2*043#0135*/0x1125,0x1285,0x12B8,0x0458,
   /* 3*029#0126*/0x1695,0x1236,0x1396,0x0389,
   /* 4*054#0147*/0x1496,0x1139,0x1369,0x063B,
   /* 5*108#0367*/0x12A5,0x1785,0x1825,0x0802,
   
   // Case 12.1.1 (1358)
   /* 0*089#0256*[-...-.]*/0x1246,0x1192,0x1429,0x0038,
   /* 1*075#0235*[--....]*/0x1945,0x181A,0x1180,0x0B8A,
   /* 2*045#0136*[.--...]*/0x16A5,0x1129,0x1B92,0x089B,
   /* 3*053#0146*[.-...-]*/0x16A5,0x1749,0x1179,0x0371,
   /* 4*030#0127*[..--..]*/0x123A,0x17B6,0x18A3,0x09A8,
   /* 5*101#0346*[..-..-]*/0x16A5,0x1274,0x172B,0x0024,
   /* 6*092#0267*[...--.]*/0x1715,0x17B2,0x1172,0x0380,
   /* 7*120#0567*[-..-..]*/0x19BA,0x1794,0x1B97,0x0803,
   /* 8*086#0247*[..-.-.]*/0x1406,0x121A,0x103B,0x060B,
   /* 9*083#0245*[.-..-.]*/0x1905,0x121A,0x1350,0x0753,
   /*10*058#0157*[...-.-]*/0x1345,0x17B6,0x1384,0x0135,
   /*11*106#0357*[-....-]*/0x1945,0x1786,0x1068,0x0260,
   //(1406)
   /* 0*089#0256*[+...+.]*/0x1246,0x1384,0x1342,0x0190,
   /* 1*075#0235*[++....]*/0x1A45,0x14A8,0x18AB,0x0019,
   /* 2*045#0136*[.++...]*/0x16B5,0x15B9,0x112A,0x09B8,
   /* 3*053#0146*[.+...+]*/0x1495,0x116A,0x1617,0x0713,
   /* 4*030#0127*[..++..]*/0x1786,0x168A,0x1A89,0x023B,
   /* 5*101#0346*[..+..+]*/0x14A5,0x1B76,0x1A42,0x0240,
   /* 6*092#0267*[...++.]*/0x1715,0x1180,0x1817,0x0B23,
   /* 7*120#0567*[+..+..]*/0x19BA,0x13B0,0x10B9,0x0784,
   /* 8*086#0247*[..+.+.]*/0x11A6,0x1160,0x1064,0x03B2,
   /* 9*083#0245*[.+..+.]*/0x1A35,0x123A,0x1537,0x0019,
   /*10*058#0157*[...+.+]*/0x1B65,0x1B53,0x1351,0x0784,
   /*11*106#0357*[+....+]*/0x1065,0x1905,0x1602,0x0784,
   
   // Case 12.1.2 (1454)
   /* 0*(06)*089#0256*[-...-.]*/0x1846,0x1948,0x1980,0x1901,0x1863,0x1362,0x1132,0x0103,
   /* 1*(35)*075#0235*[--....]*/0x1AB5,0x1159,0x11A5,0x1190,0x15B4,0x14B8,0x1048,0x0094,
   /* 2*(06)*045#0136*[.--...]*/0x1685,0x126A,0x1589,0x11A5,0x12B6,0x16B8,0x12A1,0x0159,
   /* 3*(06)*053#0146*[.-...-]*/0x19A5,0x1A36,0x191A,0x1A13,0x1954,0x1637,0x1467,0x0456,
   /* 4*(17)*030#0127*[..--..]*/0x126A,0x1738,0x137B,0x1789,0x13B2,0x1796,0x169A,0x02B6,
   /* 5*(06)*101#0346*[..-..-]*/0x10A5,0x1B6A,0x1745,0x1756,0x1540,0x176B,0x1A02,0x0BA2,
   /* 6*(06)*092#0267*[...--.]*/0x1015,0x1102,0x1203,0x123B,0x1058,0x1857,0x1B87,0x0B38,
   /* 7*(06)*120#0567*[-..-..]*/0x13BA,0x1784,0x137B,0x1738,0x13A0,0x10A9,0x1409,0x0480,
   /* 8*(24)*086#0247*[..-.-.]*/0x1B6A,0x1BA2,0x1A64,0x1B23,0x1A41,0x1140,0x1310,0x0321,
   /* 9*(24)*083#0245*[.-..-.]*/0x19A5,0x1320,0x1021,0x1237,0x1019,0x127A,0x1A75,0x091A,
   /*10*(17)*058#0157*[...-.-]*/0x1165,0x1456,0x1467,0x1478,0x161B,0x1B13,0x18B3,0x087B,
   /*11*(35)*106#0357*[-....-]*/0x1265,0x1809,0x1894,0x1902,0x1847,0x1925,0x1756,0x0745,
   //(1550)
   /* 0*(17)*089#0256*[+...+.]*/0x1946,0x1312,0x1013,0x1621,0x1803,0x1961,0x1498,0x0908,
   /* 1*(17)*075#0235*[++....]*/0x1945,0x115A,0x1084,0x1904,0x1B80,0x11B0,0x1AB1,0x0195,
   /* 2*(24)*045#0136*[.++...]*/0x16A5,0x1195,0x1A15,0x1891,0x1281,0x1B82,0x1B26,0x02A6,
   /* 3*(35)*053#0146*[.+...+]*/0x16A5,0x1764,0x1546,0x1374,0x1934,0x1139,0x119A,0x095A,
   /* 4*(35)*030#0127*[..++..]*/0x12A6,0x1B26,0x19A2,0x17B6,0x1392,0x1893,0x1837,0x03B7,
   /* 5*(17)*101#0346*[..+..+]*/0x16A5,0x1B2A,0x16BA,0x102B,0x1754,0x170B,0x1407,0x0765,
   /* 6*(35)*092#0267*[...++.]*/0x1B25,0x178B,0x13B8,0x157B,0x1038,0x1152,0x1120,0x0230,
   /* 7*(24)*120#0567*[+..+..]*/0x194A,0x1490,0x1840,0x1380,0x17A4,0x1BA7,0x1B73,0x0783,
   /* 8*(35)*086#0247*[..+.+.]*/0x1BA6,0x1130,0x1231,0x1403,0x1A21,0x1B43,0x164B,0x0B2A,
   /* 9*(17)*083#0245*[.+..+.]*/0x1A95,0x119A,0x1759,0x121A,0x1079,0x1370,0x1302,0x0012,
   /*10*(24)*058#0157*[...+.+]*/0x1465,0x13B8,0x178B,0x1138,0x167B,0x1418,0x1514,0x0476,
   /*11*(24)*106#0357*[+....+]*/0x1945,0x1765,0x1754,0x1267,0x1827,0x1028,0x1089,0x0849,
   
   // Case 12.2 (1646)
   /* 0*089#0256*[-...+.]*/0x12C6,0x1C19,0x11C0,0x13C2,0x180C,0x18C3,0x194C,0x046C,
   /* 1*075#0235*[-+....]*/0x1AC5,0x119C,0x14C9,0x145C,0x1ABC,0x10C8,0x1B8C,0x01C0,
   /* 2*045#0136*[.-+...]*/0x1CA5,0x156C,0x12AC,0x16BC,0x1B8C,0x11C9,0x189C,0x02C1,
   /* 3*053#0146*[.-...+]*/0x1CA5,0x1AC6,0x14C5,0x16C7,0x137C,0x11C9,0x113C,0x09C4,
   /* 4*030#0127*[..-+..]*/0x12CA,0x1CB6,0x13BC,0x178C,0x167C,0x189C,0x19AC,0x03C2,
   /* 5*101#0346*[..-..+]*/0x1CA5,0x154C,0x176C,0x1AC6,0x140C,0x1BC2,0x102C,0x07CB,
   /* 6*092#0267*[...-+.]*/0x1C15,0x123C,0x101C,0x18C3,0x180C,0x1BC7,0x157C,0x02CB,
   /* 7*120#0567*[+..-..]*/0x1CBA,0x1C38,0x17C4,0x1BC7,0x1C84,0x1C90,0x1CA9,0x03C0,
   /* 8*086#0247*[..+.-.]*/0x16CA,0x11C2,0x1C03,0x12CB,0x1C3B,0x1C40,0x1C64,0x0AC1,
   /* 9*083#0245*[.+..-.]*/0x1AC5,0x1C19,0x11C2,0x13C0,0x10C9,0x1C37,0x1C75,0x02CA,
   /*10*058#0157*[...+.-]*/0x1C45,0x17C6,0x178C,0x1BC3,0x16CB,0x113C,0x151C,0x04C8,
   /*11*106#0357*[+....-]*/0x1C45,0x1C26,0x184C,0x190C,0x159C,0x102C,0x17C6,0x08C7,
   //(1742)
   /* 0*089#0256*[+...-.]*/0x146C,0x190C,0x184C,0x13C0,0x138C,0x11C2,0x162C,0x09C1,
   /* 1*075#0235*[+-....]*/0x1C45,0x10C9,0x14C8,0x159C,0x1B8C,0x11AC,0x1ABC,0x01C0,
   /* 2*045#0136*[.+-...]*/0x16C5,0x16AC,0x15C9,0x11CA,0x189C,0x12BC,0x1B8C,0x02C1,
   /* 3*053#0146*[.+...-]*/0x16C5,0x16AC,0x195C,0x1A1C,0x113C,0x1C74,0x137C,0x09C4,
   /* 4*030#0127*[..+-..]*/0x16CA,0x12CB,0x17BC,0x17C6,0x19AC,0x138C,0x189C,0x03C2,
   /* 5*101#0346*[..+..-]*/0x16C5,0x15CA,0x1BC6,0x1AC2,0x102C,0x174C,0x140C,0x07CB,
   /* 6*092#0267*[...+-.]*/0x17C5,0x1C3B,0x18C7,0x103C,0x10C8,0x121C,0x115C,0x02CB,
   /* 7*120#0567*[-..+..]*/0x19CA,0x1C80,0x1C94,0x18C7,0x1C47,0x1BC3,0x1CBA,0x03C0,
   /* 8*086#0247*[..-.+.]*/0x12CA,0x1CB6,0x1C23,0x1BC3,0x1C64,0x1C01,0x1C40,0x0AC1,
   /* 9*083#0245*[.-..+.]*/0x19C5,0x1C1A,0x11C0,0x1C90,0x1C75,0x13C2,0x1C37,0x02CA,
   /*10*058#0157*[...-.+]*/0x1C65,0x17C4,0x1BC7,0x1B6C,0x151C,0x18C3,0x113C,0x04C8,
   /*11*106#0357*[-....+]*/0x1C65,0x17C4,0x194C,0x19C5,0x126C,0x180C,0x102C,0x08C7,
   
   // Case 13.1 (1838)
   /* 0*090#0257*[------]*/0x1945,0x121A,0x17B6,0x0380,
   /* 1*090#0257*[++++++]*/0x1A65,0x1190,0x1B23,0x0784,
   
   // Case 13.2 (1846)
   /* 0*090#0257*[-----+]*/0x1B65,0x1B59,0x121A,0x1380,0x1794,0x0B97,
   /* 1*090#0257*[----+-]*/0x1945,0x17B6,0x181A,0x1801,0x1A38,0x0A23,
   /* 2*090#0257*[---+--]*/0x1945,0x121A,0x10B6,0x103B,0x1680,0x0678,
   /* 3*090#0257*[--+---]*/0x1945,0x11A6,0x1038,0x1172,0x17B2,0x0167,
   /* 4*090#0257*[-+----]*/0x1A45,0x17B6,0x1380,0x1429,0x1219,0x04A2,
   /* 5*090#0257*[-+++++]*/0x1A65,0x1794,0x1197,0x13B2,0x1817,0x0801,
   /* 6*090#0257*[+-----]*/0x1905,0x121A,0x1345,0x17B6,0x1350,0x0384,
   /* 7*090#0257*[+-++++]*/0x1065,0x1590,0x11A6,0x1784,0x1B23,0x0016,
   /* 8*090#0257*[++-+++]*/0x1B65,0x135A,0x1784,0x1019,0x1B53,0x0A23,
   /* 9*090#0257*[+++-++]*/0x1A65,0x1190,0x1342,0x1384,0x1472,0x07B2,
   /*10*090#0257*[++++-+]*/0x1A65,0x1784,0x10B9,0x103B,0x1B29,0x0219,
   /*11*090#0257*[+++++-]*/0x1A45,0x18A6,0x1190,0x13B2,0x14A8,0x0678,
   
   // Case 13.3 (1918)
   /* 0*090#0257*[---+-+]*/0x1C65,0x1C59,0x121A,0x14C9,0x10C8,0x1C78,0x1C47,0x1C3B,0x16CB,0x0C03,
   /* 1*090#0257*[--++--]*/0x1945,0x1CA6,0x13C0,0x11C2,0x1CB2,0x1C3B,0x1C80,0x1C78,0x17C6,0x0C1A,
   /* 2*090#0257*[--+--+]*/0x1C65,0x1CA6,0x19C5,0x11AC,0x1C21,0x1CB2,0x17C4,0x1BC7,0x1C94,0x0803,
   /* 3*090#0257*[---++-]*/0x1945,0x12CA,0x1CB6,0x1C3B,0x1C23,0x1C1A,0x1C01,0x1C78,0x10C8,0x0C67,
   /* 4*090#0257*[--++++]*/0x1C65,0x1A6C,0x17C4,0x159C,0x194C,0x178C,0x180C,0x11AC,0x11C0,0x03B2,
   /* 5*090#0257*[--+-+-]*/0x1945,0x1CA6,0x17BC,0x18C3,0x1C23,0x1CB2,0x1C67,0x1C01,0x1AC1,0x0C80,
   /* 6*090#0257*[-+---+]*/0x1C65,0x1C5A,0x1CB6,0x12CA,0x17C4,0x1C7B,0x1C19,0x14C9,0x1C21,0x0038,
   /* 7*090#0257*[-++---]*/0x1AC5,0x1CA6,0x1C45,0x17C6,0x1C94,0x1C19,0x1CB2,0x11C2,0x1C7B,0x0380,
   /* 8*090#0257*[-+++-+]*/0x1A65,0x1C3B,0x17C4,0x18C7,0x180C,0x103C,0x1B2C,0x1C19,0x121C,0x094C,
   /* 9*090#0257*[-+--+-]*/0x1AC5,0x1C45,0x17B6,0x10C8,0x14C9,0x1C19,0x1C01,0x1C38,0x1C23,0x02CA,
   /*10*090#0257*[-++-++]*/0x1A65,0x119C,0x11C0,0x13C2,0x138C,0x180C,0x194C,0x17BC,0x17C4,0x0B2C,
   /*11*090#0257*[-++++-]*/0x1AC5,0x1A6C,0x1C19,0x194C,0x145C,0x167C,0x180C,0x18C7,0x101C,0x0B23,
   /*12*090#0257*[+-++-+]*/0x1C65,0x16CA,0x12CB,0x159C,0x121C,0x11AC,0x103C,0x10C9,0x13BC,0x0784,
   /*13*090#0257*[+--+--]*/0x1C45,0x17C6,0x1C59,0x121A,0x1C84,0x1C78,0x1CB6,0x1C3B,0x1C90,0x03C0,
   /*14*090#0257*[+----+]*/0x1C65,0x19C5,0x121A,0x1C38,0x17C4,0x1BC7,0x1C84,0x1C03,0x1C90,0x0CB6,
   /*15*090#0257*[+-+++-]*/0x1C45,0x16CA,0x10C9,0x14C8,0x159C,0x101C,0x11AC,0x167C,0x178C,0x023B,
   /*16*090#0257*[+--+++]*/0x1C65,0x12CA,0x13C2,0x159C,0x11C0,0x11AC,0x13BC,0x1B6C,0x190C,0x0784,
   /*17*090#0257*[+---+-]*/0x19C5,0x12CA,0x1C45,0x17B6,0x1AC1,0x1C01,0x1C90,0x1C84,0x1C23,0x08C3,
   /*28*090#0257*[+++--+]*/0x1A65,0x14C8,0x10C9,0x103C,0x138C,0x147C,0x17BC,0x121C,0x12CB,0x019C,
   /*19*090#0257*[++----]*/0x1AC5,0x15C4,0x17B6,0x1C19,0x11C2,0x13C0,0x1C90,0x1CA2,0x1C84,0x0C38,
   /*20*090#0257*[++-+-+]*/0x1AC5,0x1B6C,0x1C19,0x1A2C,0x121C,0x190C,0x103C,0x1BC3,0x165C,0x0784,
   /*21*090#0257*[+++-+-]*/0x1AC5,0x16CA,0x12CB,0x145C,0x167C,0x17BC,0x123C,0x138C,0x14C8,0x0019,
   /*22*090#0257*[++--++]*/0x1C65,0x15AC,0x17C4,0x17BC,0x1B6C,0x1A2C,0x138C,0x13C2,0x184C,0x0190,
   /*23*090#0257*[++-++-]*/0x1AC5,0x1B6C,0x145C,0x1C78,0x1BC3,0x167C,0x184C,0x1A2C,0x123C,0x0019,
   
   // Case 13.4 (2158)
   /* 0*090#0257*[++---+]*/0x1C65,0x1C5A,0x1C90,0x1C03,0x1C38,0x1C84,0x1C47,0x1C7B,0x1CB6,0x1CA2,0x1C21,0x0C19,
   /* 1*090#0257*[-++-+-]*/0x1AC5,0x1A6C,0x138C,0x123C,0x1B2C,0x145C,0x17BC,0x167C,0x194C,0x119C,0x101C,0x080C,
   /* 2*090#0257*[--++-+]*/0x1C65,0x1CA6,0x1CB2,0x1C59,0x1C21,0x1C1A,0x1C94,0x1C47,0x1C78,0x1C80,0x1C03,0x0C3B,
   /* 3*090#0257*[+--++-]*/0x1C45,0x1B6C,0x159C,0x11AC,0x101C,0x190C,0x184C,0x178C,0x167C,0x13BC,0x123C,0x0A2C,
   
   // Case 13.5.2 (2206)
   /* 0*(06)*090#0257*[-++--+]*/0x1A65,0x1784,0x1B87,0x18B3,0x1804,0x1094,0x1190,0x1310,0x1213,0x0B23,
   /* 1*(17)*090#0257*[++--+-]*/0x1945,0x115A,0x17B6,0x1380,0x1302,0x1012,0x1908,0x1498,0x1951,0x0A21,
   /* 2*(24)*090#0257*[+--+-+]*/0x1A65,0x19A5,0x1A91,0x1A26,0x12B6,0x13B2,0x1132,0x1031,0x1901,0x0784,
   /* 3*(35)*090#0257*[--+++-]*/0x1945,0x1BA6,0x1380,0x1837,0x13B7,0x1230,0x1120,0x121A,0x12AB,0x067B,
   //(2246)
   /* 0*(06)*090#0257*[-++--+]*/0x1465,0x1BA6,0x1519,0x121A,0x12AB,0x15A1,0x1594,0x1476,0x17B6,0x0803,
   /* 1*(17)*090#0257*[++--+-]*/0x1A65,0x13B2,0x18B3,0x1574,0x1B87,0x1B62,0x16A2,0x1756,0x1847,0x0190,
   /* 2*(24)*090#0257*[+--+-+]*/0x1465,0x1459,0x121A,0x1380,0x1089,0x1849,0x1783,0x1B73,0x17B6,0x0764,
   /* 3*(35)*090#0257*[--+++-]*/0x1A65,0x1190,0x1A91,0x19A5,0x1940,0x1480,0x1784,0x1574,0x1675,0x03B2,
   
   // Case 13.5.1 (2286)
   /* 0*090#0257*[-++--+]*/0x1A65,0x1380,0x1219,0x1942,0x14B2,0x07B4,
   /* 1*090#0257*[++--+-]*/0x1A35,0x1584,0x17B6,0x1190,0x123A,0x0385,
   /* 2*090#0257*[+--+-+]*/0x1965,0x121A,0x1B03,0x1B60,0x1690,0x0784,
   /* 3*090#0257*[--+++-]*/0x1945,0x17A6,0x13B2,0x1018,0x1178,0x01A7
   ];
   