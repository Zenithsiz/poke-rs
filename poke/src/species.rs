//! Species


/// Pokemon Species
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Species {
	Bulbasaur  = 1,
	Ivysaur    = 2,
	Venusaur   = 3,
	Charmander = 4,
	Charmeleon = 5,
	Charizard  = 6,
	Squirtle   = 7,
	Wartortle  = 8,
	Blastoise  = 9,
	Caterpie   = 10,
	Metapod    = 11,
	Butterfree = 12,
	Weedle     = 13,
	Kakuna     = 14,
	Beedrill   = 15,
	Pidgey     = 16,
	Pidgeotto  = 17,
	Pidgeot    = 18,
	Rattata    = 19,
	Raticate   = 20,
	Spearow    = 21,
	Fearow     = 22,
	Ekans      = 23,
	Arbok      = 24,
	Pikachu    = 25,
	Raichu     = 26,
	Sandshrew  = 27,
	Sandslash  = 28,
	NidoranF   = 29,
	Nidorina   = 30,
	Nidoqueen  = 31,
	NidoranM   = 32,
	Nidorino   = 33,
	Nidoking   = 34,
	Clefairy   = 35,
	Clefable   = 36,
	Vulpix     = 37,
	Ninetales  = 38,
	Jigglypuff = 39,
	Wigglytuff = 40,
	Zubat      = 41,
	Golbat     = 42,
	Oddish     = 43,
	Gloom      = 44,
	Vileplume  = 45,
	Paras      = 46,
	Parasect   = 47,
	Venonat    = 48,
	Venomoth   = 49,
	Diglett    = 50,
	Dugtrio    = 51,
	Meowth     = 52,
	Persian    = 53,
	Psyduck    = 54,
	Golduck    = 55,
	Mankey     = 56,
	Primeape   = 57,
	Growlithe  = 58,
	Arcanine   = 59,
	Poliwag    = 60,
	Poliwhirl  = 61,
	Poliwrath  = 62,
	Abra       = 63,
	Kadabra    = 64,
	Alakazam   = 65,
	Machop     = 66,
	Machoke    = 67,
	Machamp    = 68,
	Bellsprout = 69,
	Weepinbell = 70,
	Victreebel = 71,
	Tentacool  = 72,
	Tentacruel = 73,
	Geodude    = 74,
	Graveler   = 75,
	Golem      = 76,
	Ponyta     = 77,
	Rapidash   = 78,
	Slowpoke   = 79,
	Slowbro    = 80,
	Magnemite  = 81,
	Magneton   = 82,
	Farfetchd  = 83,
	Doduo      = 84,
	Dodrio     = 85,
	Seel       = 86,
	Dewgong    = 87,
	Grimer     = 88,
	Muk        = 89,
	Shellder   = 90,
	Cloyster   = 91,
	Gastly     = 92,
	Haunter    = 93,
	Gengar     = 94,
	Onix       = 95,
	Drowzee    = 96,
	Hypno      = 97,
	Krabby     = 98,
	Kingler    = 99,
	Voltorb    = 100,
	Electrode  = 101,
	Exeggcute  = 102,
	Exeggutor  = 103,
	Cubone     = 104,
	Marowak    = 105,
	Hitmonlee  = 106,
	Hitmonchan = 107,
	Lickitung  = 108,
	Koffing    = 109,
	Weezing    = 110,
	Rhyhorn    = 111,
	Rhydon     = 112,
	Chansey    = 113,
	Tangela    = 114,
	Kangaskhan = 115,
	Horsea     = 116,
	Seadra     = 117,
	Goldeen    = 118,
	Seaking    = 119,
	Staryu     = 120,
	Starmie    = 121,
	MrMime     = 122,
	Scyther    = 123,
	Jynx       = 124,
	Electabuzz = 125,
	Magmar     = 126,
	Pinsir     = 127,
	Tauros     = 128,
	Magikarp   = 129,
	Gyarados   = 130,
	Lapras     = 131,
	Ditto      = 132,
	Eevee      = 133,
	Vaporeon   = 134,
	Jolteon    = 135,
	Flareon    = 136,
	Porygon    = 137,
	Omanyte    = 138,
	Omastar    = 139,
	Kabuto     = 140,
	Kabutops   = 141,
	Aerodactyl = 142,
	Snorlax    = 143,
	Articuno   = 144,
	Zapdos     = 145,
	Moltres    = 146,
	Dratini    = 147,
	Dragonair  = 148,
	Dragonite  = 149,
	Mewtwo     = 150,
	Mew        = 151,
	Chikorita  = 152,
	Bayleef    = 153,
	Meganium   = 154,
	Cyndaquil  = 155,
	Quilava    = 156,
	Typhlosion = 157,
	Totodile   = 158,
	Croconaw   = 159,
	Feraligatr = 160,
	Sentret    = 161,
	Furret     = 162,
	Hoothoot   = 163,
	Noctowl    = 164,
	Ledyba     = 165,
	Ledian     = 166,
	Spinarak   = 167,
	Ariados    = 168,
	Crobat     = 169,
	Chinchou   = 170,
	Lanturn    = 171,
	Pichu      = 172,
	Cleffa     = 173,
	Igglybuff  = 174,
	Togepi     = 175,
	Togetic    = 176,
	Natu       = 177,
	Xatu       = 178,
	Mareep     = 179,
	Flaaffy    = 180,
	Ampharos   = 181,
	Bellossom  = 182,
	Marill     = 183,
	Azumarill  = 184,
	Sudowoodo  = 185,
	Politoed   = 186,
	Hoppip     = 187,
	Skiploom   = 188,
	Jumpluff   = 189,
	Aipom      = 190,
	Sunkern    = 191,
	Sunflora   = 192,
	Yanma      = 193,
	Wooper     = 194,
	Quagsire   = 195,
	Espeon     = 196,
	Umbreon    = 197,
	Murkrow    = 198,
	Slowking   = 199,
	Misdreavus = 200,
	Unown      = 201,
	Wobbuffet  = 202,
	Girafarig  = 203,
	Pineco     = 204,
	Forretress = 205,
	Dunsparce  = 206,
	Gligar     = 207,
	Steelix    = 208,
	Snubbull   = 209,
	Granbull   = 210,
	Qwilfish   = 211,
	Scizor     = 212,
	Shuckle    = 213,
	Heracross  = 214,
	Sneasel    = 215,
	Teddiursa  = 216,
	Ursaring   = 217,
	Slugma     = 218,
	Magcargo   = 219,
	Swinub     = 220,
	Piloswine  = 221,
	Corsola    = 222,
	Remoraid   = 223,
	Octillery  = 224,
	Delibird   = 225,
	Mantine    = 226,
	Skarmory   = 227,
	Houndour   = 228,
	Houndoom   = 229,
	Kingdra    = 230,
	Phanpy     = 231,
	Donphan    = 232,
	Porygon2   = 233,
	Stantler   = 234,
	Smeargle   = 235,
	Tyrogue    = 236,
	Hitmontop  = 237,
	Smoochum   = 238,
	Elekid     = 239,
	Magby      = 240,
	Miltank    = 241,
	Blissey    = 242,
	Raikou     = 243,
	Entei      = 244,
	Suicune    = 245,
	Larvitar   = 246,
	Pupitar    = 247,
	Tyranitar  = 248,
	Lugia      = 249,
	HoOh       = 250,
	Celebi     = 251,
	OldUnownB  = 252,
	OldUnownC  = 253,
	OldUnownD  = 254,
	OldUnownE  = 255,
	OldUnownF  = 256,
	OldUnownG  = 257,
	OldUnownH  = 258,
	OldUnownI  = 259,
	OldUnownJ  = 260,
	OldUnownK  = 261,
	OldUnownL  = 262,
	OldUnownM  = 263,
	OldUnownN  = 264,
	OldUnownO  = 265,
	OldUnownP  = 266,
	OldUnownQ  = 267,
	OldUnownR  = 268,
	OldUnownS  = 269,
	OldUnownT  = 270,
	OldUnownU  = 271,
	OldUnownV  = 272,
	OldUnownW  = 273,
	OldUnownX  = 274,
	OldUnownY  = 275,
	OldUnownZ  = 276,
	Treecko    = 277,
	Grovyle    = 278,
	Sceptile   = 279,
	Torchic    = 280,
	Combusken  = 281,
	Blaziken   = 282,
	Mudkip     = 283,
	Marshtomp  = 284,
	Swampert   = 285,
	Poochyena  = 286,
	Mightyena  = 287,
	Zigzagoon  = 288,
	Linoone    = 289,
	Wurmple    = 290,
	Silcoon    = 291,
	Beautifly  = 292,
	Cascoon    = 293,
	Dustox     = 294,
	Lotad      = 295,
	Lombre     = 296,
	Ludicolo   = 297,
	Seedot     = 298,
	Nuzleaf    = 299,
	Shiftry    = 300,
	Nincada    = 301,
	Ninjask    = 302,
	Shedinja   = 303,
	Taillow    = 304,
	Swellow    = 305,
	Shroomish  = 306,
	Breloom    = 307,
	Spinda     = 308,
	Wingull    = 309,
	Pelipper   = 310,
	Surskit    = 311,
	Masquerain = 312,
	Wailmer    = 313,
	Wailord    = 314,
	Skitty     = 315,
	Delcatty   = 316,
	Kecleon    = 317,
	Baltoy     = 318,
	Claydol    = 319,
	Nosepass   = 320,
	Torkoal    = 321,
	Sableye    = 322,
	Barboach   = 323,
	Whiscash   = 324,
	Luvdisc    = 325,
	Corphish   = 326,
	Crawdaunt  = 327,
	Feebas     = 328,
	Milotic    = 329,
	Carvanha   = 330,
	Sharpedo   = 331,
	Trapinch   = 332,
	Vibrava    = 333,
	Flygon     = 334,
	Makuhita   = 335,
	Hariyama   = 336,
	Electrike  = 337,
	Manectric  = 338,
	Numel      = 339,
	Camerupt   = 340,
	Spheal     = 341,
	Sealeo     = 342,
	Walrein    = 343,
	Cacnea     = 344,
	Cacturne   = 345,
	Snorunt    = 346,
	Glalie     = 347,
	Lunatone   = 348,
	Solrock    = 349,
	Azurill    = 350,
	Spoink     = 351,
	Grumpig    = 352,
	Plusle     = 353,
	Minun      = 354,
	Mawile     = 355,
	Meditite   = 356,
	Medicham   = 357,
	Swablu     = 358,
	Altaria    = 359,
	Wynaut     = 360,
	Duskull    = 361,
	Dusclops   = 362,
	Roselia    = 363,
	Slakoth    = 364,
	Vigoroth   = 365,
	Slaking    = 366,
	Gulpin     = 367,
	Swalot     = 368,
	Tropius    = 369,
	Whismur    = 370,
	Loudred    = 371,
	Exploud    = 372,
	Clamperl   = 373,
	Huntail    = 374,
	Gorebyss   = 375,
	Absol      = 376,
	Shuppet    = 377,
	Banette    = 378,
	Seviper    = 379,
	Zangoose   = 380,
	Relicanth  = 381,
	Aron       = 382,
	Lairon     = 383,
	Aggron     = 384,
	Castform   = 385,
	Volbeat    = 386,
	Illumise   = 387,
	Lileep     = 388,
	Cradily    = 389,
	Anorith    = 390,
	Armaldo    = 391,
	Ralts      = 392,
	Kirlia     = 393,
	Gardevoir  = 394,
	Bagon      = 395,
	Shelgon    = 396,
	Salamence  = 397,
	Beldum     = 398,
	Metang     = 399,
	Metagross  = 400,
	Regirock   = 401,
	Regice     = 402,
	Registeel  = 403,
	Kyogre     = 404,
	Groudon    = 405,
	Rayquaza   = 406,
	Latias     = 407,
	Latios     = 408,
	Jirachi    = 409,
	Deoxys     = 410,
	Chimecho   = 411,
	Egg        = 412,
	UnownB     = 413,
	UnownC     = 414,
	UnownD     = 415,
	UnownE     = 416,
	UnownF     = 417,
	UnownG     = 418,
	UnownH     = 419,
	UnownI     = 420,
	UnownJ     = 421,
	UnownK     = 422,
	UnownL     = 423,
	UnownM     = 424,
	UnownN     = 425,
	UnownO     = 426,
	UnownP     = 427,
	UnownQ     = 428,
	UnownR     = 429,
	UnownS     = 430,
	UnownT     = 431,
	UnownU     = 432,
	UnownV     = 433,
	UnownW     = 434,
	UnownX     = 435,
	UnownY     = 436,
	UnownZ     = 437,
	UnownEmark = 438,
	UnownQmark = 439,
}

impl Species {
	/// Returns the `C` name of this species
	pub fn c_name(&self) -> &'static str {
		match self {
			Species::Bulbasaur => "BULBASAUR",
			Species::Ivysaur => "IVYSAUR",
			Species::Venusaur => "VENUSAUR",
			Species::Charmander => "CHARMANDER",
			Species::Charmeleon => "CHARMELEON",
			Species::Charizard => "CHARIZARD",
			Species::Squirtle => "SQUIRTLE",
			Species::Wartortle => "WARTORTLE",
			Species::Blastoise => "BLASTOISE",
			Species::Caterpie => "CATERPIE",
			Species::Metapod => "METAPOD",
			Species::Butterfree => "BUTTERFREE",
			Species::Weedle => "WEEDLE",
			Species::Kakuna => "KAKUNA",
			Species::Beedrill => "BEEDRILL",
			Species::Pidgey => "PIDGEY",
			Species::Pidgeotto => "PIDGEOTTO",
			Species::Pidgeot => "PIDGEOT",
			Species::Rattata => "RATTATA",
			Species::Raticate => "RATICATE",
			Species::Spearow => "SPEAROW",
			Species::Fearow => "FEAROW",
			Species::Ekans => "EKANS",
			Species::Arbok => "ARBOK",
			Species::Pikachu => "PIKACHU",
			Species::Raichu => "RAICHU",
			Species::Sandshrew => "SANDSHREW",
			Species::Sandslash => "SANDSLASH",
			Species::NidoranF => "NIDORAN_F",
			Species::Nidorina => "NIDORINA",
			Species::Nidoqueen => "NIDOQUEEN",
			Species::NidoranM => "NIDORAN_M",
			Species::Nidorino => "NIDORINO",
			Species::Nidoking => "NIDOKING",
			Species::Clefairy => "CLEFAIRY",
			Species::Clefable => "CLEFABLE",
			Species::Vulpix => "VULPIX",
			Species::Ninetales => "NINETALES",
			Species::Jigglypuff => "JIGGLYPUFF",
			Species::Wigglytuff => "WIGGLYTUFF",
			Species::Zubat => "ZUBAT",
			Species::Golbat => "GOLBAT",
			Species::Oddish => "ODDISH",
			Species::Gloom => "GLOOM",
			Species::Vileplume => "VILEPLUME",
			Species::Paras => "PARAS",
			Species::Parasect => "PARASECT",
			Species::Venonat => "VENONAT",
			Species::Venomoth => "VENOMOTH",
			Species::Diglett => "DIGLETT",
			Species::Dugtrio => "DUGTRIO",
			Species::Meowth => "MEOWTH",
			Species::Persian => "PERSIAN",
			Species::Psyduck => "PSYDUCK",
			Species::Golduck => "GOLDUCK",
			Species::Mankey => "MANKEY",
			Species::Primeape => "PRIMEAPE",
			Species::Growlithe => "GROWLITHE",
			Species::Arcanine => "ARCANINE",
			Species::Poliwag => "POLIWAG",
			Species::Poliwhirl => "POLIWHIRL",
			Species::Poliwrath => "POLIWRATH",
			Species::Abra => "ABRA",
			Species::Kadabra => "KADABRA",
			Species::Alakazam => "ALAKAZAM",
			Species::Machop => "MACHOP",
			Species::Machoke => "MACHOKE",
			Species::Machamp => "MACHAMP",
			Species::Bellsprout => "BELLSPROUT",
			Species::Weepinbell => "WEEPINBELL",
			Species::Victreebel => "VICTREEBEL",
			Species::Tentacool => "TENTACOOL",
			Species::Tentacruel => "TENTACRUEL",
			Species::Geodude => "GEODUDE",
			Species::Graveler => "GRAVELER",
			Species::Golem => "GOLEM",
			Species::Ponyta => "PONYTA",
			Species::Rapidash => "RAPIDASH",
			Species::Slowpoke => "SLOWPOKE",
			Species::Slowbro => "SLOWBRO",
			Species::Magnemite => "MAGNEMITE",
			Species::Magneton => "MAGNETON",
			Species::Farfetchd => "FARFETCHD",
			Species::Doduo => "DODUO",
			Species::Dodrio => "DODRIO",
			Species::Seel => "SEEL",
			Species::Dewgong => "DEWGONG",
			Species::Grimer => "GRIMER",
			Species::Muk => "MUK",
			Species::Shellder => "SHELLDER",
			Species::Cloyster => "CLOYSTER",
			Species::Gastly => "GASTLY",
			Species::Haunter => "HAUNTER",
			Species::Gengar => "GENGAR",
			Species::Onix => "ONIX",
			Species::Drowzee => "DROWZEE",
			Species::Hypno => "HYPNO",
			Species::Krabby => "KRABBY",
			Species::Kingler => "KINGLER",
			Species::Voltorb => "VOLTORB",
			Species::Electrode => "ELECTRODE",
			Species::Exeggcute => "EXEGGCUTE",
			Species::Exeggutor => "EXEGGUTOR",
			Species::Cubone => "CUBONE",
			Species::Marowak => "MAROWAK",
			Species::Hitmonlee => "HITMONLEE",
			Species::Hitmonchan => "HITMONCHAN",
			Species::Lickitung => "LICKITUNG",
			Species::Koffing => "KOFFING",
			Species::Weezing => "WEEZING",
			Species::Rhyhorn => "RHYHORN",
			Species::Rhydon => "RHYDON",
			Species::Chansey => "CHANSEY",
			Species::Tangela => "TANGELA",
			Species::Kangaskhan => "KANGASKHAN",
			Species::Horsea => "HORSEA",
			Species::Seadra => "SEADRA",
			Species::Goldeen => "GOLDEEN",
			Species::Seaking => "SEAKING",
			Species::Staryu => "STARYU",
			Species::Starmie => "STARMIE",
			Species::MrMime => "MR_MIME",
			Species::Scyther => "SCYTHER",
			Species::Jynx => "JYNX",
			Species::Electabuzz => "ELECTABUZZ",
			Species::Magmar => "MAGMAR",
			Species::Pinsir => "PINSIR",
			Species::Tauros => "TAUROS",
			Species::Magikarp => "MAGIKARP",
			Species::Gyarados => "GYARADOS",
			Species::Lapras => "LAPRAS",
			Species::Ditto => "DITTO",
			Species::Eevee => "EEVEE",
			Species::Vaporeon => "VAPOREON",
			Species::Jolteon => "JOLTEON",
			Species::Flareon => "FLAREON",
			Species::Porygon => "PORYGON",
			Species::Omanyte => "OMANYTE",
			Species::Omastar => "OMASTAR",
			Species::Kabuto => "KABUTO",
			Species::Kabutops => "KABUTOPS",
			Species::Aerodactyl => "AERODACTYL",
			Species::Snorlax => "SNORLAX",
			Species::Articuno => "ARTICUNO",
			Species::Zapdos => "ZAPDOS",
			Species::Moltres => "MOLTRES",
			Species::Dratini => "DRATINI",
			Species::Dragonair => "DRAGONAIR",
			Species::Dragonite => "DRAGONITE",
			Species::Mewtwo => "MEWTWO",
			Species::Mew => "MEW",
			Species::Chikorita => "CHIKORITA",
			Species::Bayleef => "BAYLEEF",
			Species::Meganium => "MEGANIUM",
			Species::Cyndaquil => "CYNDAQUIL",
			Species::Quilava => "QUILAVA",
			Species::Typhlosion => "TYPHLOSION",
			Species::Totodile => "TOTODILE",
			Species::Croconaw => "CROCONAW",
			Species::Feraligatr => "FERALIGATR",
			Species::Sentret => "SENTRET",
			Species::Furret => "FURRET",
			Species::Hoothoot => "HOOTHOOT",
			Species::Noctowl => "NOCTOWL",
			Species::Ledyba => "LEDYBA",
			Species::Ledian => "LEDIAN",
			Species::Spinarak => "SPINARAK",
			Species::Ariados => "ARIADOS",
			Species::Crobat => "CROBAT",
			Species::Chinchou => "CHINCHOU",
			Species::Lanturn => "LANTURN",
			Species::Pichu => "PICHU",
			Species::Cleffa => "CLEFFA",
			Species::Igglybuff => "IGGLYBUFF",
			Species::Togepi => "TOGEPI",
			Species::Togetic => "TOGETIC",
			Species::Natu => "NATU",
			Species::Xatu => "XATU",
			Species::Mareep => "MAREEP",
			Species::Flaaffy => "FLAAFFY",
			Species::Ampharos => "AMPHAROS",
			Species::Bellossom => "BELLOSSOM",
			Species::Marill => "MARILL",
			Species::Azumarill => "AZUMARILL",
			Species::Sudowoodo => "SUDOWOODO",
			Species::Politoed => "POLITOED",
			Species::Hoppip => "HOPPIP",
			Species::Skiploom => "SKIPLOOM",
			Species::Jumpluff => "JUMPLUFF",
			Species::Aipom => "AIPOM",
			Species::Sunkern => "SUNKERN",
			Species::Sunflora => "SUNFLORA",
			Species::Yanma => "YANMA",
			Species::Wooper => "WOOPER",
			Species::Quagsire => "QUAGSIRE",
			Species::Espeon => "ESPEON",
			Species::Umbreon => "UMBREON",
			Species::Murkrow => "MURKROW",
			Species::Slowking => "SLOWKING",
			Species::Misdreavus => "MISDREAVUS",
			Species::Unown => "UNOWN",
			Species::Wobbuffet => "WOBBUFFET",
			Species::Girafarig => "GIRAFARIG",
			Species::Pineco => "PINECO",
			Species::Forretress => "FORRETRESS",
			Species::Dunsparce => "DUNSPARCE",
			Species::Gligar => "GLIGAR",
			Species::Steelix => "STEELIX",
			Species::Snubbull => "SNUBBULL",
			Species::Granbull => "GRANBULL",
			Species::Qwilfish => "QWILFISH",
			Species::Scizor => "SCIZOR",
			Species::Shuckle => "SHUCKLE",
			Species::Heracross => "HERACROSS",
			Species::Sneasel => "SNEASEL",
			Species::Teddiursa => "TEDDIURSA",
			Species::Ursaring => "URSARING",
			Species::Slugma => "SLUGMA",
			Species::Magcargo => "MAGCARGO",
			Species::Swinub => "SWINUB",
			Species::Piloswine => "PILOSWINE",
			Species::Corsola => "CORSOLA",
			Species::Remoraid => "REMORAID",
			Species::Octillery => "OCTILLERY",
			Species::Delibird => "DELIBIRD",
			Species::Mantine => "MANTINE",
			Species::Skarmory => "SKARMORY",
			Species::Houndour => "HOUNDOUR",
			Species::Houndoom => "HOUNDOOM",
			Species::Kingdra => "KINGDRA",
			Species::Phanpy => "PHANPY",
			Species::Donphan => "DONPHAN",
			Species::Porygon2 => "PORYGON2",
			Species::Stantler => "STANTLER",
			Species::Smeargle => "SMEARGLE",
			Species::Tyrogue => "TYROGUE",
			Species::Hitmontop => "HITMONTOP",
			Species::Smoochum => "SMOOCHUM",
			Species::Elekid => "ELEKID",
			Species::Magby => "MAGBY",
			Species::Miltank => "MILTANK",
			Species::Blissey => "BLISSEY",
			Species::Raikou => "RAIKOU",
			Species::Entei => "ENTEI",
			Species::Suicune => "SUICUNE",
			Species::Larvitar => "LARVITAR",
			Species::Pupitar => "PUPITAR",
			Species::Tyranitar => "TYRANITAR",
			Species::Lugia => "LUGIA",
			Species::HoOh => "HO_OH",
			Species::Celebi => "CELEBI",
			Species::OldUnownB => "OLD_UNOWN_B",
			Species::OldUnownC => "OLD_UNOWN_C",
			Species::OldUnownD => "OLD_UNOWN_D",
			Species::OldUnownE => "OLD_UNOWN_E",
			Species::OldUnownF => "OLD_UNOWN_F",
			Species::OldUnownG => "OLD_UNOWN_G",
			Species::OldUnownH => "OLD_UNOWN_H",
			Species::OldUnownI => "OLD_UNOWN_I",
			Species::OldUnownJ => "OLD_UNOWN_J",
			Species::OldUnownK => "OLD_UNOWN_K",
			Species::OldUnownL => "OLD_UNOWN_L",
			Species::OldUnownM => "OLD_UNOWN_M",
			Species::OldUnownN => "OLD_UNOWN_N",
			Species::OldUnownO => "OLD_UNOWN_O",
			Species::OldUnownP => "OLD_UNOWN_P",
			Species::OldUnownQ => "OLD_UNOWN_Q",
			Species::OldUnownR => "OLD_UNOWN_R",
			Species::OldUnownS => "OLD_UNOWN_S",
			Species::OldUnownT => "OLD_UNOWN_T",
			Species::OldUnownU => "OLD_UNOWN_U",
			Species::OldUnownV => "OLD_UNOWN_V",
			Species::OldUnownW => "OLD_UNOWN_W",
			Species::OldUnownX => "OLD_UNOWN_X",
			Species::OldUnownY => "OLD_UNOWN_Y",
			Species::OldUnownZ => "OLD_UNOWN_Z",
			Species::Treecko => "TREECKO",
			Species::Grovyle => "GROVYLE",
			Species::Sceptile => "SCEPTILE",
			Species::Torchic => "TORCHIC",
			Species::Combusken => "COMBUSKEN",
			Species::Blaziken => "BLAZIKEN",
			Species::Mudkip => "MUDKIP",
			Species::Marshtomp => "MARSHTOMP",
			Species::Swampert => "SWAMPERT",
			Species::Poochyena => "POOCHYENA",
			Species::Mightyena => "MIGHTYENA",
			Species::Zigzagoon => "ZIGZAGOON",
			Species::Linoone => "LINOONE",
			Species::Wurmple => "WURMPLE",
			Species::Silcoon => "SILCOON",
			Species::Beautifly => "BEAUTIFLY",
			Species::Cascoon => "CASCOON",
			Species::Dustox => "DUSTOX",
			Species::Lotad => "LOTAD",
			Species::Lombre => "LOMBRE",
			Species::Ludicolo => "LUDICOLO",
			Species::Seedot => "SEEDOT",
			Species::Nuzleaf => "NUZLEAF",
			Species::Shiftry => "SHIFTRY",
			Species::Nincada => "NINCADA",
			Species::Ninjask => "NINJASK",
			Species::Shedinja => "SHEDINJA",
			Species::Taillow => "TAILLOW",
			Species::Swellow => "SWELLOW",
			Species::Shroomish => "SHROOMISH",
			Species::Breloom => "BRELOOM",
			Species::Spinda => "SPINDA",
			Species::Wingull => "WINGULL",
			Species::Pelipper => "PELIPPER",
			Species::Surskit => "SURSKIT",
			Species::Masquerain => "MASQUERAIN",
			Species::Wailmer => "WAILMER",
			Species::Wailord => "WAILORD",
			Species::Skitty => "SKITTY",
			Species::Delcatty => "DELCATTY",
			Species::Kecleon => "KECLEON",
			Species::Baltoy => "BALTOY",
			Species::Claydol => "CLAYDOL",
			Species::Nosepass => "NOSEPASS",
			Species::Torkoal => "TORKOAL",
			Species::Sableye => "SABLEYE",
			Species::Barboach => "BARBOACH",
			Species::Whiscash => "WHISCASH",
			Species::Luvdisc => "LUVDISC",
			Species::Corphish => "CORPHISH",
			Species::Crawdaunt => "CRAWDAUNT",
			Species::Feebas => "FEEBAS",
			Species::Milotic => "MILOTIC",
			Species::Carvanha => "CARVANHA",
			Species::Sharpedo => "SHARPEDO",
			Species::Trapinch => "TRAPINCH",
			Species::Vibrava => "VIBRAVA",
			Species::Flygon => "FLYGON",
			Species::Makuhita => "MAKUHITA",
			Species::Hariyama => "HARIYAMA",
			Species::Electrike => "ELECTRIKE",
			Species::Manectric => "MANECTRIC",
			Species::Numel => "NUMEL",
			Species::Camerupt => "CAMERUPT",
			Species::Spheal => "SPHEAL",
			Species::Sealeo => "SEALEO",
			Species::Walrein => "WALREIN",
			Species::Cacnea => "CACNEA",
			Species::Cacturne => "CACTURNE",
			Species::Snorunt => "SNORUNT",
			Species::Glalie => "GLALIE",
			Species::Lunatone => "LUNATONE",
			Species::Solrock => "SOLROCK",
			Species::Azurill => "AZURILL",
			Species::Spoink => "SPOINK",
			Species::Grumpig => "GRUMPIG",
			Species::Plusle => "PLUSLE",
			Species::Minun => "MINUN",
			Species::Mawile => "MAWILE",
			Species::Meditite => "MEDITITE",
			Species::Medicham => "MEDICHAM",
			Species::Swablu => "SWABLU",
			Species::Altaria => "ALTARIA",
			Species::Wynaut => "WYNAUT",
			Species::Duskull => "DUSKULL",
			Species::Dusclops => "DUSCLOPS",
			Species::Roselia => "ROSELIA",
			Species::Slakoth => "SLAKOTH",
			Species::Vigoroth => "VIGOROTH",
			Species::Slaking => "SLAKING",
			Species::Gulpin => "GULPIN",
			Species::Swalot => "SWALOT",
			Species::Tropius => "TROPIUS",
			Species::Whismur => "WHISMUR",
			Species::Loudred => "LOUDRED",
			Species::Exploud => "EXPLOUD",
			Species::Clamperl => "CLAMPERL",
			Species::Huntail => "HUNTAIL",
			Species::Gorebyss => "GOREBYSS",
			Species::Absol => "ABSOL",
			Species::Shuppet => "SHUPPET",
			Species::Banette => "BANETTE",
			Species::Seviper => "SEVIPER",
			Species::Zangoose => "ZANGOOSE",
			Species::Relicanth => "RELICANTH",
			Species::Aron => "ARON",
			Species::Lairon => "LAIRON",
			Species::Aggron => "AGGRON",
			Species::Castform => "CASTFORM",
			Species::Volbeat => "VOLBEAT",
			Species::Illumise => "ILLUMISE",
			Species::Lileep => "LILEEP",
			Species::Cradily => "CRADILY",
			Species::Anorith => "ANORITH",
			Species::Armaldo => "ARMALDO",
			Species::Ralts => "RALTS",
			Species::Kirlia => "KIRLIA",
			Species::Gardevoir => "GARDEVOIR",
			Species::Bagon => "BAGON",
			Species::Shelgon => "SHELGON",
			Species::Salamence => "SALAMENCE",
			Species::Beldum => "BELDUM",
			Species::Metang => "METANG",
			Species::Metagross => "METAGROSS",
			Species::Regirock => "REGIROCK",
			Species::Regice => "REGICE",
			Species::Registeel => "REGISTEEL",
			Species::Kyogre => "KYOGRE",
			Species::Groudon => "GROUDON",
			Species::Rayquaza => "RAYQUAZA",
			Species::Latias => "LATIAS",
			Species::Latios => "LATIOS",
			Species::Jirachi => "JIRACHI",
			Species::Deoxys => "DEOXYS",
			Species::Chimecho => "CHIMECHO",
			Species::Egg => "EGG",
			Species::UnownB => "UNOWN_B",
			Species::UnownC => "UNOWN_C",
			Species::UnownD => "UNOWN_D",
			Species::UnownE => "UNOWN_E",
			Species::UnownF => "UNOWN_F",
			Species::UnownG => "UNOWN_G",
			Species::UnownH => "UNOWN_H",
			Species::UnownI => "UNOWN_I",
			Species::UnownJ => "UNOWN_J",
			Species::UnownK => "UNOWN_K",
			Species::UnownL => "UNOWN_L",
			Species::UnownM => "UNOWN_M",
			Species::UnownN => "UNOWN_N",
			Species::UnownO => "UNOWN_O",
			Species::UnownP => "UNOWN_P",
			Species::UnownQ => "UNOWN_Q",
			Species::UnownR => "UNOWN_R",
			Species::UnownS => "UNOWN_S",
			Species::UnownT => "UNOWN_T",
			Species::UnownU => "UNOWN_U",
			Species::UnownV => "UNOWN_V",
			Species::UnownW => "UNOWN_W",
			Species::UnownX => "UNOWN_X",
			Species::UnownY => "UNOWN_Y",
			Species::UnownZ => "UNOWN_Z",
			Species::UnownEmark => "UNOWN_EMARK",
			Species::UnownQmark => "UNOWN_QMARK",
		}
	}
}