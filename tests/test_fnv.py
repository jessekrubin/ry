import pytest

import ry

FNV_TEST_DATA = [
    (b"", 0xCBF29CE484222325),
    (b"a", 0xAF63DC4C8601EC8C),
    (b"b", 0xAF63DF4C8601F1A5),
    (b"c", 0xAF63DE4C8601EFF2),
    (b"d", 0xAF63D94C8601E773),
    (b"e", 0xAF63D84C8601E5C0),
    (b"f", 0xAF63DB4C8601EAD9),
    (b"fo", 0x08985907B541D342),
    (b"foo", 0xDCB27518FED9D577),
    (b"foob", 0xDD120E790C2512AF),
    (b"fooba", 0xCAC165AFA2FEF40A),
    (b"foobar", 0x85944171F73967E8),
    (b"\0", 0xAF63BD4C8601B7DF),
    (b"a\0", 0x089BE207B544F1E4),
    (b"b\0", 0x08A61407B54D9B5F),
    (b"c\0", 0x08A2AE07B54AB836),
    (b"d\0", 0x0891B007B53C4869),
    (b"e\0", 0x088E4A07B5396540),
    (b"f\0", 0x08987C07B5420EBB),
    (b"fo\0", 0xDCB28A18FED9F926),
    (b"foo\0", 0xDD1270790C25B935),
    (b"foob\0", 0xCAC146AFA2FEBF5D),
    (b"fooba\0", 0x8593D371F738ACFE),
    (b"foobar\0", 0x34531CA7168B8F38),
    (b"ch", 0x08A25607B54A22AE),
    (b"cho", 0xF5FAF0190CF90DF3),
    (b"chon", 0xF27397910B3221C7),
    (b"chong", 0x2C8C2B76062F22E0),
    (b"chongo", 0xE150688C8217B8FD),
    (b"chongo ", 0xF35A83C10E4F1F87),
    (b"chongo w", 0xD1EDD10B507344D0),
    (b"chongo wa", 0x2A5EE739B3DDB8C3),
    (b"chongo was", 0xDCFB970CA1C0D310),
    (b"chongo was ", 0x4054DA76DAA6DA90),
    (b"chongo was h", 0xF70A2FF589861368),
    (b"chongo was he", 0x4C628B38AED25F17),
    (b"chongo was her", 0x9DD1F6510F78189F),
    (b"chongo was here", 0xA3DE85BD491270CE),
    (b"chongo was here!", 0x858E2FA32A55E61D),
    (b"chongo was here!\n", 0x46810940EFF5F915),
    (b"ch\0", 0xF5FADD190CF8EDAA),
    (b"cho\0", 0xF273ED910B32B3E9),
    (b"chon\0", 0x2C8C5276062F6525),
    (b"chong\0", 0xE150B98C821842A0),
    (b"chongo\0", 0xF35AA3C10E4F55E7),
    (b"chongo \0", 0xD1ED680B50729265),
    (b"chongo w\0", 0x2A5F0639B3DDED70),
    (b"chongo wa\0", 0xDCFBAA0CA1C0F359),
    (b"chongo was\0", 0x4054BA76DAA6A430),
    (b"chongo was \0", 0xF709C7F5898562B0),
    (b"chongo was h\0", 0x4C62E638AED2F9B8),
    (b"chongo was he\0", 0x9DD1A8510F779415),
    (b"chongo was her\0", 0xA3DE2ABD4911D62D),
    (b"chongo was here\0", 0x858E0EA32A55AE0A),
    (b"chongo was here!\0", 0x46810F40EFF60347),
    (b"chongo was here!\n\0", 0xC33BCE57BEF63EAF),
    (b"cu", 0x08A24307B54A0265),
    (b"cur", 0xF5B9FD190CC18D15),
    (b"curd", 0x4C968290ACE35703),
    (b"curds", 0x07174BD5C64D9350),
    (b"curds ", 0x5A294C3FF5D18750),
    (b"curds a", 0x05B3C1AEB308B843),
    (b"curds an", 0xB92A48DA37D0F477),
    (b"curds and", 0x73CDDDCCD80EBC49),
    (b"curds and ", 0xD58C4C13210A266B),
    (b"curds and w", 0xE78B6081243EC194),
    (b"curds and wh", 0xB096F77096A39F34),
    (b"curds and whe", 0xB425C54FF807B6A3),
    (b"curds and whey", 0x23E520E2751BB46E),
    (b"curds and whey\n", 0x1A0B44CCFE1385EC),
    (b"cu\0", 0xF5BA4B190CC2119F),
    (b"cur\0", 0x4C962690ACE2BAAF),
    (b"curd\0", 0x0716DED5C64CDA19),
    (b"curds\0", 0x5A292C3FF5D150F0),
    (b"curds \0", 0x05B3E0AEB308ECF0),
    (b"curds a\0", 0xB92A5EDA37D119D9),
    (b"curds an\0", 0x73CE41CCD80F6635),
    (b"curds and\0", 0xD58C2C132109F00B),
    (b"curds and \0", 0xE78BAF81243F47D1),
    (b"curds and w\0", 0xB0968F7096A2EE7C),
    (b"curds and wh\0", 0xB425A84FF807855C),
    (b"curds and whe\0", 0x23E4E9E2751B56F9),
    (b"curds and whey\0", 0x1A0B4ECCFE1396EA),
    (b"curds and whey\n\0", 0x54ABD453BB2C9004),
    (b"hi", 0x08BA5F07B55EC3DA),
    (b"hi\0", 0x337354193006CB6E),
    (b"hello", 0xA430D84680AABD0B),
    (b"hello\0", 0xA9BC8ACCA21F39B1),
    (b"\xff\x00\x00\x01", 0x6961196491CC682D),
    (b"\x01\x00\x00\xff", 0xAD2BB1774799DFE9),
    (b"\xff\x00\x00\x02", 0x6961166491CC6314),
    (b"\x02\x00\x00\xff", 0x8D1BB3904A3B1236),
    (b"\xff\x00\x00\x03", 0x6961176491CC64C7),
    (b"\x03\x00\x00\xff", 0xED205D87F40434C7),
    (b"\xff\x00\x00\x04", 0x6961146491CC5FAE),
    (b"\x04\x00\x00\xff", 0xCD3BAF5E44F8AD9C),
    (b"\x40\x51\x4e\x44", 0xE3B36596127CD6D8),
    (b"\x44\x4e\x51\x40", 0xF77F1072C8E8A646),
    (b"\x40\x51\x4e\x4a", 0xE3B36396127CD372),
    (b"\x4a\x4e\x51\x40", 0x6067DCE9932AD458),
    (b"\x40\x51\x4e\x54", 0xE3B37596127CF208),
    (b"\x54\x4e\x51\x40", 0x4B7B10FA9FE83936),
    (b"127.0.0.1", 0xAABAFE7104D914BE),
    (b"127.0.0.1\0", 0xF4D3180B3CDE3EDA),
    (b"127.0.0.2", 0xAABAFD7104D9130B),
    (b"127.0.0.2\0", 0xF4CFB20B3CDB5BB1),
    (b"127.0.0.3", 0xAABAFC7104D91158),
    (b"127.0.0.3\0", 0xF4CC4C0B3CD87888),
    (b"64.81.78.68", 0xE729BAC5D2A8D3A7),
    (b"64.81.78.68\0", 0x74BC0524F4DFA4C5),
    (b"64.81.78.74", 0xE72630C5D2A5B352),
    (b"64.81.78.74\0", 0x6B983224EF8FB456),
    (b"64.81.78.84", 0xE73042C5D2AE266D),
    (b"64.81.78.84\0", 0x8527E324FDEB4B37),
    (b"feedface", 0x0A83C86FEE952ABC),
    (b"feedface\0", 0x7318523267779D74),
    (b"feedfacedaffdeed", 0x3E66D3D56B8CACA1),
    (b"feedfacedaffdeed\0", 0x956694A5C0095593),
    (b"feedfacedeadbeef", 0xCAC54572BB1A6FC8),
    (b"feedfacedeadbeef\0", 0xA7A4C9F3EDEBF0D8),
    (b"line 1\nline 2\nline 3", 0x7829851FAC17B143),
]


def test_fnv1a_empty() -> None:
    assert ry.fnv1a(b"").digest() == 0xCBF29CE484222325


@pytest.mark.parametrize("input,expected", FNV_TEST_DATA)
def test_fnv1a(input: bytes, expected: int) -> None:
    fnvhash = ry.fnv1a(input)
    int_digest = fnvhash.digest()
    assert int_digest == expected
    hex_str_expected = hex(expected)[2:]

    hex_digest_str_og_hasher = fnvhash.hexdigest()
    assert hex_digest_str_og_hasher == hex_str_expected

    hex_digest_str = ry.fnv1a(input).hexdigest()
    assert hex_digest_str == hex_str_expected
    assert hex_digest_str == hex_digest_str.lower()


@pytest.mark.parametrize("input,expected", FNV_TEST_DATA)
def test_fnv1a_hasher(input: bytes, expected: int) -> None:
    thingy = ry.FnvHasher()
    thingy.update(input)
    assert thingy.digest() == expected
    thingy_with_init = ry.FnvHasher(input)
    assert thingy_with_init.digest() == expected


def test_copy_hasher() -> None:
    thingy = ry.FnvHasher()
    thingy.update(b"abc")
    thingy_copy = thingy.copy()
    thingy_copy.update(b"def")
    assert thingy.digest() != thingy_copy.digest()
    assert thingy_copy.digest() == ry.fnv1a(b"abcdef").digest()
    r = thingy_copy.digest()
    assert r is not None
    fnhashing = ry.fnv1a(b"abc")
    fnhashing.update(b"def")
    assert fnhashing.digest() == ry.fnv1a(b"abcdef").digest()
