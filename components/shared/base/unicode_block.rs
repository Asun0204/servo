/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// Do not edit:
// Generated via: https://www.unicode.org/Public/UNIDATA/Blocks.txt.
// $ ./generate-unicode-block.py Blocks.txt > unicode_block.rs

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnicodeBlock {
    BasicLatin,
    Latin1Supplement,
    LatinExtendedA,
    LatinExtendedB,
    IPAExtensions,
    SpacingModifierLetters,
    CombiningDiacriticalMarks,
    GreekandCoptic,
    Cyrillic,
    CyrillicSupplement,
    Armenian,
    Hebrew,
    Arabic,
    Syriac,
    ArabicSupplement,
    Thaana,
    NKo,
    Samaritan,
    Mandaic,
    SyriacSupplement,
    ArabicExtendedB,
    ArabicExtendedA,
    Devanagari,
    Bengali,
    Gurmukhi,
    Gujarati,
    Oriya,
    Tamil,
    Telugu,
    Kannada,
    Malayalam,
    Sinhala,
    Thai,
    Lao,
    Tibetan,
    Myanmar,
    Georgian,
    HangulJamo,
    Ethiopic,
    EthiopicSupplement,
    Cherokee,
    UnifiedCanadianAboriginalSyllabics,
    Ogham,
    Runic,
    Tagalog,
    Hanunoo,
    Buhid,
    Tagbanwa,
    Khmer,
    Mongolian,
    UnifiedCanadianAboriginalSyllabicsExtended,
    Limbu,
    TaiLe,
    NewTaiLue,
    KhmerSymbols,
    Buginese,
    TaiTham,
    CombiningDiacriticalMarksExtended,
    Balinese,
    Sundanese,
    Batak,
    Lepcha,
    OlChiki,
    CyrillicExtendedC,
    GeorgianExtended,
    SundaneseSupplement,
    VedicExtensions,
    PhoneticExtensions,
    PhoneticExtensionsSupplement,
    CombiningDiacriticalMarksSupplement,
    LatinExtendedAdditional,
    GreekExtended,
    GeneralPunctuation,
    SuperscriptsandSubscripts,
    CurrencySymbols,
    CombiningDiacriticalMarksforSymbols,
    LetterlikeSymbols,
    NumberForms,
    Arrows,
    MathematicalOperators,
    MiscellaneousTechnical,
    ControlPictures,
    OpticalCharacterRecognition,
    EnclosedAlphanumerics,
    BoxDrawing,
    BlockElements,
    GeometricShapes,
    MiscellaneousSymbols,
    Dingbats,
    MiscellaneousMathematicalSymbolsA,
    SupplementalArrowsA,
    BraillePatterns,
    SupplementalArrowsB,
    MiscellaneousMathematicalSymbolsB,
    SupplementalMathematicalOperators,
    MiscellaneousSymbolsandArrows,
    Glagolitic,
    LatinExtendedC,
    Coptic,
    GeorgianSupplement,
    Tifinagh,
    EthiopicExtended,
    CyrillicExtendedA,
    SupplementalPunctuation,
    CJKRadicalsSupplement,
    KangxiRadicals,
    IdeographicDescriptionCharacters,
    CJKSymbolsandPunctuation,
    Hiragana,
    Katakana,
    Bopomofo,
    HangulCompatibilityJamo,
    Kanbun,
    BopomofoExtended,
    CJKStrokes,
    KatakanaPhoneticExtensions,
    EnclosedCJKLettersandMonths,
    CJKCompatibility,
    CJKUnifiedIdeographsExtensionA,
    YijingHexagramSymbols,
    CJKUnifiedIdeographs,
    YiSyllables,
    YiRadicals,
    Lisu,
    Vai,
    CyrillicExtendedB,
    Bamum,
    ModifierToneLetters,
    LatinExtendedD,
    SylotiNagri,
    CommonIndicNumberForms,
    Phagspa,
    Saurashtra,
    DevanagariExtended,
    KayahLi,
    Rejang,
    HangulJamoExtendedA,
    Javanese,
    MyanmarExtendedB,
    Cham,
    MyanmarExtendedA,
    TaiViet,
    MeeteiMayekExtensions,
    EthiopicExtendedA,
    LatinExtendedE,
    CherokeeSupplement,
    MeeteiMayek,
    HangulSyllables,
    HangulJamoExtendedB,
    HighSurrogates,
    HighPrivateUseSurrogates,
    LowSurrogates,
    PrivateUseArea,
    CJKCompatibilityIdeographs,
    AlphabeticPresentationForms,
    ArabicPresentationFormsA,
    VariationSelectors,
    VerticalForms,
    CombiningHalfMarks,
    CJKCompatibilityForms,
    SmallFormVariants,
    ArabicPresentationFormsB,
    HalfwidthandFullwidthForms,
    Specials,
    LinearBSyllabary,
    LinearBIdeograms,
    AegeanNumbers,
    AncientGreekNumbers,
    AncientSymbols,
    PhaistosDisc,
    Lycian,
    Carian,
    CopticEpactNumbers,
    OldItalic,
    Gothic,
    OldPermic,
    Ugaritic,
    OldPersian,
    Deseret,
    Shavian,
    Osmanya,
    Osage,
    Elbasan,
    CaucasianAlbanian,
    Vithkuqi,
    LinearA,
    LatinExtendedF,
    CypriotSyllabary,
    ImperialAramaic,
    Palmyrene,
    Nabataean,
    Hatran,
    Phoenician,
    Lydian,
    MeroiticHieroglyphs,
    MeroiticCursive,
    Kharoshthi,
    OldSouthArabian,
    OldNorthArabian,
    Manichaean,
    Avestan,
    InscriptionalParthian,
    InscriptionalPahlavi,
    PsalterPahlavi,
    OldTurkic,
    OldHungarian,
    HanifiRohingya,
    RumiNumeralSymbols,
    Yezidi,
    ArabicExtendedC,
    OldSogdian,
    Sogdian,
    OldUyghur,
    Chorasmian,
    Elymaic,
    Brahmi,
    Kaithi,
    SoraSompeng,
    Chakma,
    Mahajani,
    Sharada,
    SinhalaArchaicNumbers,
    Khojki,
    Multani,
    Khudawadi,
    Grantha,
    Newa,
    Tirhuta,
    Siddham,
    Modi,
    MongolianSupplement,
    Takri,
    Ahom,
    Dogra,
    WarangCiti,
    DivesAkuru,
    Nandinagari,
    ZanabazarSquare,
    Soyombo,
    UnifiedCanadianAboriginalSyllabicsExtendedA,
    PauCinHau,
    DevanagariExtendedA,
    Bhaiksuki,
    Marchen,
    MasaramGondi,
    GunjalaGondi,
    Makasar,
    Kawi,
    LisuSupplement,
    TamilSupplement,
    Cuneiform,
    CuneiformNumbersandPunctuation,
    EarlyDynasticCuneiform,
    CyproMinoan,
    EgyptianHieroglyphs,
    EgyptianHieroglyphFormatControls,
    AnatolianHieroglyphs,
    BamumSupplement,
    Mro,
    Tangsa,
    BassaVah,
    PahawhHmong,
    Medefaidrin,
    Miao,
    IdeographicSymbolsandPunctuation,
    Tangut,
    TangutComponents,
    KhitanSmallScript,
    TangutSupplement,
    KanaExtendedB,
    KanaSupplement,
    KanaExtendedA,
    SmallKanaExtension,
    Nushu,
    Duployan,
    ShorthandFormatControls,
    ZnamennyMusicalNotation,
    ByzantineMusicalSymbols,
    MusicalSymbols,
    AncientGreekMusicalNotation,
    KaktovikNumerals,
    MayanNumerals,
    TaiXuanJingSymbols,
    CountingRodNumerals,
    MathematicalAlphanumericSymbols,
    SuttonSignWriting,
    LatinExtendedG,
    GlagoliticSupplement,
    CyrillicExtendedD,
    NyiakengPuachueHmong,
    Toto,
    Wancho,
    NagMundari,
    EthiopicExtendedB,
    MendeKikakui,
    Adlam,
    IndicSiyaqNumbers,
    OttomanSiyaqNumbers,
    ArabicMathematicalAlphabeticSymbols,
    MahjongTiles,
    DominoTiles,
    PlayingCards,
    EnclosedAlphanumericSupplement,
    EnclosedIdeographicSupplement,
    MiscellaneousSymbolsandPictographs,
    Emoticons,
    OrnamentalDingbats,
    TransportandMapSymbols,
    AlchemicalSymbols,
    GeometricShapesExtended,
    SupplementalArrowsC,
    SupplementalSymbolsandPictographs,
    ChessSymbols,
    SymbolsandPictographsExtendedA,
    SymbolsforLegacyComputing,
    CJKUnifiedIdeographsExtensionB,
    CJKUnifiedIdeographsExtensionC,
    CJKUnifiedIdeographsExtensionD,
    CJKUnifiedIdeographsExtensionE,
    CJKUnifiedIdeographsExtensionF,
    CJKUnifiedIdeographsExtensionI,
    CJKCompatibilityIdeographsSupplement,
    CJKUnifiedIdeographsExtensionG,
    CJKUnifiedIdeographsExtensionH,
    Tags,
    VariationSelectorsSupplement,
    SupplementaryPrivateUseAreaA,
    SupplementaryPrivateUseAreaB,
}

pub trait UnicodeBlockMethod {
    fn block(&self) -> Option<UnicodeBlock>;
}

impl UnicodeBlockMethod for char {
    fn block(&self) -> Option<UnicodeBlock> {
        match *self as u32 {
            0x000000..=0x00007F => Some(UnicodeBlock::BasicLatin),
            0x000080..=0x0000FF => Some(UnicodeBlock::Latin1Supplement),
            0x000100..=0x00017F => Some(UnicodeBlock::LatinExtendedA),
            0x000180..=0x00024F => Some(UnicodeBlock::LatinExtendedB),
            0x000250..=0x0002AF => Some(UnicodeBlock::IPAExtensions),
            0x0002B0..=0x0002FF => Some(UnicodeBlock::SpacingModifierLetters),
            0x000300..=0x00036F => Some(UnicodeBlock::CombiningDiacriticalMarks),
            0x000370..=0x0003FF => Some(UnicodeBlock::GreekandCoptic),
            0x000400..=0x0004FF => Some(UnicodeBlock::Cyrillic),
            0x000500..=0x00052F => Some(UnicodeBlock::CyrillicSupplement),
            0x000530..=0x00058F => Some(UnicodeBlock::Armenian),
            0x000590..=0x0005FF => Some(UnicodeBlock::Hebrew),
            0x000600..=0x0006FF => Some(UnicodeBlock::Arabic),
            0x000700..=0x00074F => Some(UnicodeBlock::Syriac),
            0x000750..=0x00077F => Some(UnicodeBlock::ArabicSupplement),
            0x000780..=0x0007BF => Some(UnicodeBlock::Thaana),
            0x0007C0..=0x0007FF => Some(UnicodeBlock::NKo),
            0x000800..=0x00083F => Some(UnicodeBlock::Samaritan),
            0x000840..=0x00085F => Some(UnicodeBlock::Mandaic),
            0x000860..=0x00086F => Some(UnicodeBlock::SyriacSupplement),
            0x000870..=0x00089F => Some(UnicodeBlock::ArabicExtendedB),
            0x0008A0..=0x0008FF => Some(UnicodeBlock::ArabicExtendedA),
            0x000900..=0x00097F => Some(UnicodeBlock::Devanagari),
            0x000980..=0x0009FF => Some(UnicodeBlock::Bengali),
            0x000A00..=0x000A7F => Some(UnicodeBlock::Gurmukhi),
            0x000A80..=0x000AFF => Some(UnicodeBlock::Gujarati),
            0x000B00..=0x000B7F => Some(UnicodeBlock::Oriya),
            0x000B80..=0x000BFF => Some(UnicodeBlock::Tamil),
            0x000C00..=0x000C7F => Some(UnicodeBlock::Telugu),
            0x000C80..=0x000CFF => Some(UnicodeBlock::Kannada),
            0x000D00..=0x000D7F => Some(UnicodeBlock::Malayalam),
            0x000D80..=0x000DFF => Some(UnicodeBlock::Sinhala),
            0x000E00..=0x000E7F => Some(UnicodeBlock::Thai),
            0x000E80..=0x000EFF => Some(UnicodeBlock::Lao),
            0x000F00..=0x000FFF => Some(UnicodeBlock::Tibetan),
            0x001000..=0x00109F => Some(UnicodeBlock::Myanmar),
            0x0010A0..=0x0010FF => Some(UnicodeBlock::Georgian),
            0x001100..=0x0011FF => Some(UnicodeBlock::HangulJamo),
            0x001200..=0x00137F => Some(UnicodeBlock::Ethiopic),
            0x001380..=0x00139F => Some(UnicodeBlock::EthiopicSupplement),
            0x0013A0..=0x0013FF => Some(UnicodeBlock::Cherokee),
            0x001400..=0x00167F => Some(UnicodeBlock::UnifiedCanadianAboriginalSyllabics),
            0x001680..=0x00169F => Some(UnicodeBlock::Ogham),
            0x0016A0..=0x0016FF => Some(UnicodeBlock::Runic),
            0x001700..=0x00171F => Some(UnicodeBlock::Tagalog),
            0x001720..=0x00173F => Some(UnicodeBlock::Hanunoo),
            0x001740..=0x00175F => Some(UnicodeBlock::Buhid),
            0x001760..=0x00177F => Some(UnicodeBlock::Tagbanwa),
            0x001780..=0x0017FF => Some(UnicodeBlock::Khmer),
            0x001800..=0x0018AF => Some(UnicodeBlock::Mongolian),
            0x0018B0..=0x0018FF => Some(UnicodeBlock::UnifiedCanadianAboriginalSyllabicsExtended),
            0x001900..=0x00194F => Some(UnicodeBlock::Limbu),
            0x001950..=0x00197F => Some(UnicodeBlock::TaiLe),
            0x001980..=0x0019DF => Some(UnicodeBlock::NewTaiLue),
            0x0019E0..=0x0019FF => Some(UnicodeBlock::KhmerSymbols),
            0x001A00..=0x001A1F => Some(UnicodeBlock::Buginese),
            0x001A20..=0x001AAF => Some(UnicodeBlock::TaiTham),
            0x001AB0..=0x001AFF => Some(UnicodeBlock::CombiningDiacriticalMarksExtended),
            0x001B00..=0x001B7F => Some(UnicodeBlock::Balinese),
            0x001B80..=0x001BBF => Some(UnicodeBlock::Sundanese),
            0x001BC0..=0x001BFF => Some(UnicodeBlock::Batak),
            0x001C00..=0x001C4F => Some(UnicodeBlock::Lepcha),
            0x001C50..=0x001C7F => Some(UnicodeBlock::OlChiki),
            0x001C80..=0x001C8F => Some(UnicodeBlock::CyrillicExtendedC),
            0x001C90..=0x001CBF => Some(UnicodeBlock::GeorgianExtended),
            0x001CC0..=0x001CCF => Some(UnicodeBlock::SundaneseSupplement),
            0x001CD0..=0x001CFF => Some(UnicodeBlock::VedicExtensions),
            0x001D00..=0x001D7F => Some(UnicodeBlock::PhoneticExtensions),
            0x001D80..=0x001DBF => Some(UnicodeBlock::PhoneticExtensionsSupplement),
            0x001DC0..=0x001DFF => Some(UnicodeBlock::CombiningDiacriticalMarksSupplement),
            0x001E00..=0x001EFF => Some(UnicodeBlock::LatinExtendedAdditional),
            0x001F00..=0x001FFF => Some(UnicodeBlock::GreekExtended),
            0x002000..=0x00206F => Some(UnicodeBlock::GeneralPunctuation),
            0x002070..=0x00209F => Some(UnicodeBlock::SuperscriptsandSubscripts),
            0x0020A0..=0x0020CF => Some(UnicodeBlock::CurrencySymbols),
            0x0020D0..=0x0020FF => Some(UnicodeBlock::CombiningDiacriticalMarksforSymbols),
            0x002100..=0x00214F => Some(UnicodeBlock::LetterlikeSymbols),
            0x002150..=0x00218F => Some(UnicodeBlock::NumberForms),
            0x002190..=0x0021FF => Some(UnicodeBlock::Arrows),
            0x002200..=0x0022FF => Some(UnicodeBlock::MathematicalOperators),
            0x002300..=0x0023FF => Some(UnicodeBlock::MiscellaneousTechnical),
            0x002400..=0x00243F => Some(UnicodeBlock::ControlPictures),
            0x002440..=0x00245F => Some(UnicodeBlock::OpticalCharacterRecognition),
            0x002460..=0x0024FF => Some(UnicodeBlock::EnclosedAlphanumerics),
            0x002500..=0x00257F => Some(UnicodeBlock::BoxDrawing),
            0x002580..=0x00259F => Some(UnicodeBlock::BlockElements),
            0x0025A0..=0x0025FF => Some(UnicodeBlock::GeometricShapes),
            0x002600..=0x0026FF => Some(UnicodeBlock::MiscellaneousSymbols),
            0x002700..=0x0027BF => Some(UnicodeBlock::Dingbats),
            0x0027C0..=0x0027EF => Some(UnicodeBlock::MiscellaneousMathematicalSymbolsA),
            0x0027F0..=0x0027FF => Some(UnicodeBlock::SupplementalArrowsA),
            0x002800..=0x0028FF => Some(UnicodeBlock::BraillePatterns),
            0x002900..=0x00297F => Some(UnicodeBlock::SupplementalArrowsB),
            0x002980..=0x0029FF => Some(UnicodeBlock::MiscellaneousMathematicalSymbolsB),
            0x002A00..=0x002AFF => Some(UnicodeBlock::SupplementalMathematicalOperators),
            0x002B00..=0x002BFF => Some(UnicodeBlock::MiscellaneousSymbolsandArrows),
            0x002C00..=0x002C5F => Some(UnicodeBlock::Glagolitic),
            0x002C60..=0x002C7F => Some(UnicodeBlock::LatinExtendedC),
            0x002C80..=0x002CFF => Some(UnicodeBlock::Coptic),
            0x002D00..=0x002D2F => Some(UnicodeBlock::GeorgianSupplement),
            0x002D30..=0x002D7F => Some(UnicodeBlock::Tifinagh),
            0x002D80..=0x002DDF => Some(UnicodeBlock::EthiopicExtended),
            0x002DE0..=0x002DFF => Some(UnicodeBlock::CyrillicExtendedA),
            0x002E00..=0x002E7F => Some(UnicodeBlock::SupplementalPunctuation),
            0x002E80..=0x002EFF => Some(UnicodeBlock::CJKRadicalsSupplement),
            0x002F00..=0x002FDF => Some(UnicodeBlock::KangxiRadicals),
            0x002FF0..=0x002FFF => Some(UnicodeBlock::IdeographicDescriptionCharacters),
            0x003000..=0x00303F => Some(UnicodeBlock::CJKSymbolsandPunctuation),
            0x003040..=0x00309F => Some(UnicodeBlock::Hiragana),
            0x0030A0..=0x0030FF => Some(UnicodeBlock::Katakana),
            0x003100..=0x00312F => Some(UnicodeBlock::Bopomofo),
            0x003130..=0x00318F => Some(UnicodeBlock::HangulCompatibilityJamo),
            0x003190..=0x00319F => Some(UnicodeBlock::Kanbun),
            0x0031A0..=0x0031BF => Some(UnicodeBlock::BopomofoExtended),
            0x0031C0..=0x0031EF => Some(UnicodeBlock::CJKStrokes),
            0x0031F0..=0x0031FF => Some(UnicodeBlock::KatakanaPhoneticExtensions),
            0x003200..=0x0032FF => Some(UnicodeBlock::EnclosedCJKLettersandMonths),
            0x003300..=0x0033FF => Some(UnicodeBlock::CJKCompatibility),
            0x003400..=0x004DBF => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionA),
            0x004DC0..=0x004DFF => Some(UnicodeBlock::YijingHexagramSymbols),
            0x004E00..=0x009FFF => Some(UnicodeBlock::CJKUnifiedIdeographs),
            0x00A000..=0x00A48F => Some(UnicodeBlock::YiSyllables),
            0x00A490..=0x00A4CF => Some(UnicodeBlock::YiRadicals),
            0x00A4D0..=0x00A4FF => Some(UnicodeBlock::Lisu),
            0x00A500..=0x00A63F => Some(UnicodeBlock::Vai),
            0x00A640..=0x00A69F => Some(UnicodeBlock::CyrillicExtendedB),
            0x00A6A0..=0x00A6FF => Some(UnicodeBlock::Bamum),
            0x00A700..=0x00A71F => Some(UnicodeBlock::ModifierToneLetters),
            0x00A720..=0x00A7FF => Some(UnicodeBlock::LatinExtendedD),
            0x00A800..=0x00A82F => Some(UnicodeBlock::SylotiNagri),
            0x00A830..=0x00A83F => Some(UnicodeBlock::CommonIndicNumberForms),
            0x00A840..=0x00A87F => Some(UnicodeBlock::Phagspa),
            0x00A880..=0x00A8DF => Some(UnicodeBlock::Saurashtra),
            0x00A8E0..=0x00A8FF => Some(UnicodeBlock::DevanagariExtended),
            0x00A900..=0x00A92F => Some(UnicodeBlock::KayahLi),
            0x00A930..=0x00A95F => Some(UnicodeBlock::Rejang),
            0x00A960..=0x00A97F => Some(UnicodeBlock::HangulJamoExtendedA),
            0x00A980..=0x00A9DF => Some(UnicodeBlock::Javanese),
            0x00A9E0..=0x00A9FF => Some(UnicodeBlock::MyanmarExtendedB),
            0x00AA00..=0x00AA5F => Some(UnicodeBlock::Cham),
            0x00AA60..=0x00AA7F => Some(UnicodeBlock::MyanmarExtendedA),
            0x00AA80..=0x00AADF => Some(UnicodeBlock::TaiViet),
            0x00AAE0..=0x00AAFF => Some(UnicodeBlock::MeeteiMayekExtensions),
            0x00AB00..=0x00AB2F => Some(UnicodeBlock::EthiopicExtendedA),
            0x00AB30..=0x00AB6F => Some(UnicodeBlock::LatinExtendedE),
            0x00AB70..=0x00ABBF => Some(UnicodeBlock::CherokeeSupplement),
            0x00ABC0..=0x00ABFF => Some(UnicodeBlock::MeeteiMayek),
            0x00AC00..=0x00D7AF => Some(UnicodeBlock::HangulSyllables),
            0x00D7B0..=0x00D7FF => Some(UnicodeBlock::HangulJamoExtendedB),
            0x00D800..=0x00DB7F => Some(UnicodeBlock::HighSurrogates),
            0x00DB80..=0x00DBFF => Some(UnicodeBlock::HighPrivateUseSurrogates),
            0x00DC00..=0x00DFFF => Some(UnicodeBlock::LowSurrogates),
            0x00E000..=0x00F8FF => Some(UnicodeBlock::PrivateUseArea),
            0x00F900..=0x00FAFF => Some(UnicodeBlock::CJKCompatibilityIdeographs),
            0x00FB00..=0x00FB4F => Some(UnicodeBlock::AlphabeticPresentationForms),
            0x00FB50..=0x00FDFF => Some(UnicodeBlock::ArabicPresentationFormsA),
            0x00FE00..=0x00FE0F => Some(UnicodeBlock::VariationSelectors),
            0x00FE10..=0x00FE1F => Some(UnicodeBlock::VerticalForms),
            0x00FE20..=0x00FE2F => Some(UnicodeBlock::CombiningHalfMarks),
            0x00FE30..=0x00FE4F => Some(UnicodeBlock::CJKCompatibilityForms),
            0x00FE50..=0x00FE6F => Some(UnicodeBlock::SmallFormVariants),
            0x00FE70..=0x00FEFF => Some(UnicodeBlock::ArabicPresentationFormsB),
            0x00FF00..=0x00FFEF => Some(UnicodeBlock::HalfwidthandFullwidthForms),
            0x00FFF0..=0x00FFFF => Some(UnicodeBlock::Specials),
            0x010000..=0x01007F => Some(UnicodeBlock::LinearBSyllabary),
            0x010080..=0x0100FF => Some(UnicodeBlock::LinearBIdeograms),
            0x010100..=0x01013F => Some(UnicodeBlock::AegeanNumbers),
            0x010140..=0x01018F => Some(UnicodeBlock::AncientGreekNumbers),
            0x010190..=0x0101CF => Some(UnicodeBlock::AncientSymbols),
            0x0101D0..=0x0101FF => Some(UnicodeBlock::PhaistosDisc),
            0x010280..=0x01029F => Some(UnicodeBlock::Lycian),
            0x0102A0..=0x0102DF => Some(UnicodeBlock::Carian),
            0x0102E0..=0x0102FF => Some(UnicodeBlock::CopticEpactNumbers),
            0x010300..=0x01032F => Some(UnicodeBlock::OldItalic),
            0x010330..=0x01034F => Some(UnicodeBlock::Gothic),
            0x010350..=0x01037F => Some(UnicodeBlock::OldPermic),
            0x010380..=0x01039F => Some(UnicodeBlock::Ugaritic),
            0x0103A0..=0x0103DF => Some(UnicodeBlock::OldPersian),
            0x010400..=0x01044F => Some(UnicodeBlock::Deseret),
            0x010450..=0x01047F => Some(UnicodeBlock::Shavian),
            0x010480..=0x0104AF => Some(UnicodeBlock::Osmanya),
            0x0104B0..=0x0104FF => Some(UnicodeBlock::Osage),
            0x010500..=0x01052F => Some(UnicodeBlock::Elbasan),
            0x010530..=0x01056F => Some(UnicodeBlock::CaucasianAlbanian),
            0x010570..=0x0105BF => Some(UnicodeBlock::Vithkuqi),
            0x010600..=0x01077F => Some(UnicodeBlock::LinearA),
            0x010780..=0x0107BF => Some(UnicodeBlock::LatinExtendedF),
            0x010800..=0x01083F => Some(UnicodeBlock::CypriotSyllabary),
            0x010840..=0x01085F => Some(UnicodeBlock::ImperialAramaic),
            0x010860..=0x01087F => Some(UnicodeBlock::Palmyrene),
            0x010880..=0x0108AF => Some(UnicodeBlock::Nabataean),
            0x0108E0..=0x0108FF => Some(UnicodeBlock::Hatran),
            0x010900..=0x01091F => Some(UnicodeBlock::Phoenician),
            0x010920..=0x01093F => Some(UnicodeBlock::Lydian),
            0x010980..=0x01099F => Some(UnicodeBlock::MeroiticHieroglyphs),
            0x0109A0..=0x0109FF => Some(UnicodeBlock::MeroiticCursive),
            0x010A00..=0x010A5F => Some(UnicodeBlock::Kharoshthi),
            0x010A60..=0x010A7F => Some(UnicodeBlock::OldSouthArabian),
            0x010A80..=0x010A9F => Some(UnicodeBlock::OldNorthArabian),
            0x010AC0..=0x010AFF => Some(UnicodeBlock::Manichaean),
            0x010B00..=0x010B3F => Some(UnicodeBlock::Avestan),
            0x010B40..=0x010B5F => Some(UnicodeBlock::InscriptionalParthian),
            0x010B60..=0x010B7F => Some(UnicodeBlock::InscriptionalPahlavi),
            0x010B80..=0x010BAF => Some(UnicodeBlock::PsalterPahlavi),
            0x010C00..=0x010C4F => Some(UnicodeBlock::OldTurkic),
            0x010C80..=0x010CFF => Some(UnicodeBlock::OldHungarian),
            0x010D00..=0x010D3F => Some(UnicodeBlock::HanifiRohingya),
            0x010E60..=0x010E7F => Some(UnicodeBlock::RumiNumeralSymbols),
            0x010E80..=0x010EBF => Some(UnicodeBlock::Yezidi),
            0x010EC0..=0x010EFF => Some(UnicodeBlock::ArabicExtendedC),
            0x010F00..=0x010F2F => Some(UnicodeBlock::OldSogdian),
            0x010F30..=0x010F6F => Some(UnicodeBlock::Sogdian),
            0x010F70..=0x010FAF => Some(UnicodeBlock::OldUyghur),
            0x010FB0..=0x010FDF => Some(UnicodeBlock::Chorasmian),
            0x010FE0..=0x010FFF => Some(UnicodeBlock::Elymaic),
            0x011000..=0x01107F => Some(UnicodeBlock::Brahmi),
            0x011080..=0x0110CF => Some(UnicodeBlock::Kaithi),
            0x0110D0..=0x0110FF => Some(UnicodeBlock::SoraSompeng),
            0x011100..=0x01114F => Some(UnicodeBlock::Chakma),
            0x011150..=0x01117F => Some(UnicodeBlock::Mahajani),
            0x011180..=0x0111DF => Some(UnicodeBlock::Sharada),
            0x0111E0..=0x0111FF => Some(UnicodeBlock::SinhalaArchaicNumbers),
            0x011200..=0x01124F => Some(UnicodeBlock::Khojki),
            0x011280..=0x0112AF => Some(UnicodeBlock::Multani),
            0x0112B0..=0x0112FF => Some(UnicodeBlock::Khudawadi),
            0x011300..=0x01137F => Some(UnicodeBlock::Grantha),
            0x011400..=0x01147F => Some(UnicodeBlock::Newa),
            0x011480..=0x0114DF => Some(UnicodeBlock::Tirhuta),
            0x011580..=0x0115FF => Some(UnicodeBlock::Siddham),
            0x011600..=0x01165F => Some(UnicodeBlock::Modi),
            0x011660..=0x01167F => Some(UnicodeBlock::MongolianSupplement),
            0x011680..=0x0116CF => Some(UnicodeBlock::Takri),
            0x011700..=0x01174F => Some(UnicodeBlock::Ahom),
            0x011800..=0x01184F => Some(UnicodeBlock::Dogra),
            0x0118A0..=0x0118FF => Some(UnicodeBlock::WarangCiti),
            0x011900..=0x01195F => Some(UnicodeBlock::DivesAkuru),
            0x0119A0..=0x0119FF => Some(UnicodeBlock::Nandinagari),
            0x011A00..=0x011A4F => Some(UnicodeBlock::ZanabazarSquare),
            0x011A50..=0x011AAF => Some(UnicodeBlock::Soyombo),
            0x011AB0..=0x011ABF => Some(UnicodeBlock::UnifiedCanadianAboriginalSyllabicsExtendedA),
            0x011AC0..=0x011AFF => Some(UnicodeBlock::PauCinHau),
            0x011B00..=0x011B5F => Some(UnicodeBlock::DevanagariExtendedA),
            0x011C00..=0x011C6F => Some(UnicodeBlock::Bhaiksuki),
            0x011C70..=0x011CBF => Some(UnicodeBlock::Marchen),
            0x011D00..=0x011D5F => Some(UnicodeBlock::MasaramGondi),
            0x011D60..=0x011DAF => Some(UnicodeBlock::GunjalaGondi),
            0x011EE0..=0x011EFF => Some(UnicodeBlock::Makasar),
            0x011F00..=0x011F5F => Some(UnicodeBlock::Kawi),
            0x011FB0..=0x011FBF => Some(UnicodeBlock::LisuSupplement),
            0x011FC0..=0x011FFF => Some(UnicodeBlock::TamilSupplement),
            0x012000..=0x0123FF => Some(UnicodeBlock::Cuneiform),
            0x012400..=0x01247F => Some(UnicodeBlock::CuneiformNumbersandPunctuation),
            0x012480..=0x01254F => Some(UnicodeBlock::EarlyDynasticCuneiform),
            0x012F90..=0x012FFF => Some(UnicodeBlock::CyproMinoan),
            0x013000..=0x01342F => Some(UnicodeBlock::EgyptianHieroglyphs),
            0x013430..=0x01345F => Some(UnicodeBlock::EgyptianHieroglyphFormatControls),
            0x014400..=0x01467F => Some(UnicodeBlock::AnatolianHieroglyphs),
            0x016800..=0x016A3F => Some(UnicodeBlock::BamumSupplement),
            0x016A40..=0x016A6F => Some(UnicodeBlock::Mro),
            0x016A70..=0x016ACF => Some(UnicodeBlock::Tangsa),
            0x016AD0..=0x016AFF => Some(UnicodeBlock::BassaVah),
            0x016B00..=0x016B8F => Some(UnicodeBlock::PahawhHmong),
            0x016E40..=0x016E9F => Some(UnicodeBlock::Medefaidrin),
            0x016F00..=0x016F9F => Some(UnicodeBlock::Miao),
            0x016FE0..=0x016FFF => Some(UnicodeBlock::IdeographicSymbolsandPunctuation),
            0x017000..=0x0187FF => Some(UnicodeBlock::Tangut),
            0x018800..=0x018AFF => Some(UnicodeBlock::TangutComponents),
            0x018B00..=0x018CFF => Some(UnicodeBlock::KhitanSmallScript),
            0x018D00..=0x018D7F => Some(UnicodeBlock::TangutSupplement),
            0x01AFF0..=0x01AFFF => Some(UnicodeBlock::KanaExtendedB),
            0x01B000..=0x01B0FF => Some(UnicodeBlock::KanaSupplement),
            0x01B100..=0x01B12F => Some(UnicodeBlock::KanaExtendedA),
            0x01B130..=0x01B16F => Some(UnicodeBlock::SmallKanaExtension),
            0x01B170..=0x01B2FF => Some(UnicodeBlock::Nushu),
            0x01BC00..=0x01BC9F => Some(UnicodeBlock::Duployan),
            0x01BCA0..=0x01BCAF => Some(UnicodeBlock::ShorthandFormatControls),
            0x01CF00..=0x01CFCF => Some(UnicodeBlock::ZnamennyMusicalNotation),
            0x01D000..=0x01D0FF => Some(UnicodeBlock::ByzantineMusicalSymbols),
            0x01D100..=0x01D1FF => Some(UnicodeBlock::MusicalSymbols),
            0x01D200..=0x01D24F => Some(UnicodeBlock::AncientGreekMusicalNotation),
            0x01D2C0..=0x01D2DF => Some(UnicodeBlock::KaktovikNumerals),
            0x01D2E0..=0x01D2FF => Some(UnicodeBlock::MayanNumerals),
            0x01D300..=0x01D35F => Some(UnicodeBlock::TaiXuanJingSymbols),
            0x01D360..=0x01D37F => Some(UnicodeBlock::CountingRodNumerals),
            0x01D400..=0x01D7FF => Some(UnicodeBlock::MathematicalAlphanumericSymbols),
            0x01D800..=0x01DAAF => Some(UnicodeBlock::SuttonSignWriting),
            0x01DF00..=0x01DFFF => Some(UnicodeBlock::LatinExtendedG),
            0x01E000..=0x01E02F => Some(UnicodeBlock::GlagoliticSupplement),
            0x01E030..=0x01E08F => Some(UnicodeBlock::CyrillicExtendedD),
            0x01E100..=0x01E14F => Some(UnicodeBlock::NyiakengPuachueHmong),
            0x01E290..=0x01E2BF => Some(UnicodeBlock::Toto),
            0x01E2C0..=0x01E2FF => Some(UnicodeBlock::Wancho),
            0x01E4D0..=0x01E4FF => Some(UnicodeBlock::NagMundari),
            0x01E7E0..=0x01E7FF => Some(UnicodeBlock::EthiopicExtendedB),
            0x01E800..=0x01E8DF => Some(UnicodeBlock::MendeKikakui),
            0x01E900..=0x01E95F => Some(UnicodeBlock::Adlam),
            0x01EC70..=0x01ECBF => Some(UnicodeBlock::IndicSiyaqNumbers),
            0x01ED00..=0x01ED4F => Some(UnicodeBlock::OttomanSiyaqNumbers),
            0x01EE00..=0x01EEFF => Some(UnicodeBlock::ArabicMathematicalAlphabeticSymbols),
            0x01F000..=0x01F02F => Some(UnicodeBlock::MahjongTiles),
            0x01F030..=0x01F09F => Some(UnicodeBlock::DominoTiles),
            0x01F0A0..=0x01F0FF => Some(UnicodeBlock::PlayingCards),
            0x01F100..=0x01F1FF => Some(UnicodeBlock::EnclosedAlphanumericSupplement),
            0x01F200..=0x01F2FF => Some(UnicodeBlock::EnclosedIdeographicSupplement),
            0x01F300..=0x01F5FF => Some(UnicodeBlock::MiscellaneousSymbolsandPictographs),
            0x01F600..=0x01F64F => Some(UnicodeBlock::Emoticons),
            0x01F650..=0x01F67F => Some(UnicodeBlock::OrnamentalDingbats),
            0x01F680..=0x01F6FF => Some(UnicodeBlock::TransportandMapSymbols),
            0x01F700..=0x01F77F => Some(UnicodeBlock::AlchemicalSymbols),
            0x01F780..=0x01F7FF => Some(UnicodeBlock::GeometricShapesExtended),
            0x01F800..=0x01F8FF => Some(UnicodeBlock::SupplementalArrowsC),
            0x01F900..=0x01F9FF => Some(UnicodeBlock::SupplementalSymbolsandPictographs),
            0x01FA00..=0x01FA6F => Some(UnicodeBlock::ChessSymbols),
            0x01FA70..=0x01FAFF => Some(UnicodeBlock::SymbolsandPictographsExtendedA),
            0x01FB00..=0x01FBFF => Some(UnicodeBlock::SymbolsforLegacyComputing),
            0x020000..=0x02A6DF => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionB),
            0x02A700..=0x02B73F => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionC),
            0x02B740..=0x02B81F => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionD),
            0x02B820..=0x02CEAF => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionE),
            0x02CEB0..=0x02EBEF => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionF),
            0x02EBF0..=0x02EE5F => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionI),
            0x02F800..=0x02FA1F => Some(UnicodeBlock::CJKCompatibilityIdeographsSupplement),
            0x030000..=0x03134F => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionG),
            0x031350..=0x0323AF => Some(UnicodeBlock::CJKUnifiedIdeographsExtensionH),
            0x0E0000..=0x0E007F => Some(UnicodeBlock::Tags),
            0x0E0100..=0x0E01EF => Some(UnicodeBlock::VariationSelectorsSupplement),
            0x0F0000..=0x0FFFFF => Some(UnicodeBlock::SupplementaryPrivateUseAreaA),
            0x100000..=0x10FFFF => Some(UnicodeBlock::SupplementaryPrivateUseAreaB),
            _ => None,
        }
    }
}