//! Data types for the MeetState column.

use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;
use strum::ParseError;

use std::fmt;

use crate::Country;

/// The State column.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    InArgentina(ArgentinaState),
    InAustralia(AustraliaState),
    InBrazil(BrazilState),
    InCanada(CanadaState),
    InChina(ChinaState),
    InEngland(EnglandState),
    InGermany(GermanyState),
    InIndia(IndiaState),
    InMexico(MexicoState),
    InNetherlands(NetherlandsState),
    InNewZealand(NewZealandState),
    InRomania(RomaniaState),
    InRussia(RussiaState),
    InSouthAfrica(SouthAfricaState),
    InUSA(USAState),
}

impl State {
    /// Constructs a State for a specific Country.
    ///
    /// This is how the checker interprets the State column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::{State, USAState};
    /// let state = State::from_str_and_country("NY", Country::USA).unwrap();
    /// assert_eq!(state, State::InUSA(USAState::NY));
    /// ```
    pub fn from_str_and_country(s: &str, country: Country) -> Result<State, ParseError> {
        match country {
            Country::Argentina => Ok(State::InArgentina(s.parse::<ArgentinaState>()?)),
            Country::Australia => Ok(State::InAustralia(s.parse::<AustraliaState>()?)),
            Country::Brazil => Ok(State::InBrazil(s.parse::<BrazilState>()?)),
            Country::Canada => Ok(State::InCanada(s.parse::<CanadaState>()?)),
            Country::China => Ok(State::InChina(s.parse::<ChinaState>()?)),
            Country::England => Ok(State::InEngland(s.parse::<EnglandState>()?)),
            Country::Germany => Ok(State::InGermany(s.parse::<GermanyState>()?)),
            Country::India => Ok(State::InIndia(s.parse::<IndiaState>()?)),
            Country::Mexico => Ok(State::InMexico(s.parse::<MexicoState>()?)),
            Country::Netherlands => Ok(State::InNetherlands(s.parse::<NetherlandsState>()?)),
            Country::NewZealand => Ok(State::InNewZealand(s.parse::<NewZealandState>()?)),
            Country::Romania => Ok(State::InRomania(s.parse::<RomaniaState>()?)),
            Country::Russia => Ok(State::InRussia(s.parse::<RussiaState>()?)),
            Country::SouthAfrica => Ok(State::InSouthAfrica(s.parse::<SouthAfricaState>()?)),
            Country::USA => Ok(State::InUSA(s.parse::<USAState>()?)),
            _ => Err(ParseError::VariantNotFound),
        }
    }

    /// Constructs a State given a full, unambiguous code like "USA-NY".
    ///
    /// This is how the server interprets the State column.
    /// Codes of this format are the result of serializing a State value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::{State, USAState};
    /// let state = State::from_full_code("USA-NY").unwrap();
    /// assert_eq!(state, State::InUSA(USAState::NY));
    /// ```
    pub fn from_full_code(s: &str) -> Result<State, ParseError> {
        // The codes are of the form "{Country}-{State}".
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(ParseError::VariantNotFound);
        }

        let country: Country = parts[0].parse::<Country>()?;
        Self::from_str_and_country(parts[1], country)
    }

    /// Returns the Country for the given State.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::{State, USAState};
    /// let state = State::from_full_code("USA-NY").unwrap();
    /// assert_eq!(state.to_country(), Country::USA);
    /// ```
    pub fn to_country(self) -> Country {
        match self {
            State::InArgentina(_) => Country::Argentina,
            State::InAustralia(_) => Country::Australia,
            State::InBrazil(_) => Country::Brazil,
            State::InCanada(_) => Country::Canada,
            State::InChina(_) => Country::China,
            State::InEngland(_) => Country::England,
            State::InGermany(_) => Country::Germany,
            State::InIndia(_) => Country::India,
            State::InMexico(_) => Country::Mexico,
            State::InNetherlands(_) => Country::Netherlands,
            State::InNewZealand(_) => Country::NewZealand,
            State::InRomania(_) => Country::Romania,
            State::InRussia(_) => Country::Russia,
            State::InSouthAfrica(_) => Country::SouthAfrica,
            State::InUSA(_) => Country::USA,
        }
    }

    /// Returns a String describing just the given State (no Country).
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::{State, USAState};
    /// let state = State::from_full_code("USA-NY").unwrap();
    /// assert_eq!(state.to_state_string(), "NY");
    /// ```
    pub fn to_state_string(self) -> String {
        match self {
            State::InArgentina(s) => s.to_string(),
            State::InAustralia(s) => s.to_string(),
            State::InBrazil(s) => s.to_string(),
            State::InCanada(s) => s.to_string(),
            State::InChina(s) => s.to_string(),
            State::InEngland(s) => s.to_string(),
            State::InGermany(s) => s.to_string(),
            State::InIndia(s) => s.to_string(),
            State::InMexico(s) => s.to_string(),
            State::InNetherlands(s) => s.to_string(),
            State::InNewZealand(s) => s.to_string(),
            State::InRomania(s) => s.to_string(),
            State::InRussia(s) => s.to_string(),
            State::InSouthAfrica(s) => s.to_string(),
            State::InUSA(s) => s.to_string(),
        }
    }
}

impl Serialize for State {
    /// Serialization for the server. The checker uses from_str_and_country().
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let country = self.to_country().to_string();
        let state = self.to_state_string();
        format!("{}-{}", country, state).serialize(serializer)
    }
}

/// Helper struct for State deserialization.
///
/// This is only used by the server, not by the checker.
/// The checker uses from_str_and_country().
struct StateVisitor;

impl<'de> Visitor<'de> for StateVisitor {
    type Value = State;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A Country-State code like USA-NY")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<State, E> {
        State::from_full_code(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<State, D::Error> {
        deserializer.deserialize_str(StateVisitor)
    }
}

/// A state in Argentina.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum ArgentinaState {
    /// Ciudad Aut??noma de Buenos Aires.
    CA,
    /// Buenos Aires.
    BA,
    /// Catamarca.
    CT,
    /// Chaco.
    CC,
    /// Chubut.
    CH,
    /// C??rdoba.
    CB,
    /// Corrientes.
    CN,
    /// Entre R??os.
    ER,
    /// Formosa.
    FM,
    /// Jujuy.
    JY,
    /// La Pampa.
    LP,
    /// La Rioja.
    LR,
    /// Mendoza.
    MZ,
    /// Misiones.
    MN,
    /// Neuqu??n.
    NQ,
    /// R??o Negro.
    RN,
    /// Salta.
    SA,
    /// San Juan.
    SJ,
    /// San Luis.
    SL,
    /// Santa Cruz.
    SC,
    /// Santa Fe.
    SF,
    /// Santiago del Estero.
    SE,
    /// Tierra del Fuego.
    TF,
    /// Tucum??n.
    TM,
}

/// A state in Australia.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum AustraliaState {
    /// Australian Capital Territory.
    ACT,
    /// Jervis Bay Territory.
    JBT,
    /// New South Wales.
    NSW,
    /// Northern Territory.
    NT,
    /// Queensland.
    QLD,
    /// South Australia.
    SA,
    /// Tasmania.
    TAS,
    /// Victoria.
    VIC,
    /// Western Australia.
    WA,
}

/// A state in Brazil.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum BrazilState {
    /// Acre.
    AC,
    /// Alagoas.
    AL,
    /// Amap??.
    AP,
    /// Amazonas.
    AM,
    /// Bahia.
    BA,
    /// Cear??.
    CE,
    /// Distrito Federal.
    DF,
    /// Esp??rito Santo.
    ES,
    /// Goi??s.
    GO,
    /// Maranh??o.
    MA,
    /// Mato Grosso.
    MT,
    /// Mato Grosso do Sul.
    MS,
    /// Minas Gerais.
    MG,
    /// Par??.
    PA,
    /// Para??ba.
    PB,
    /// Paran??.
    PR,
    /// Pernambuco.
    PE,
    /// Piau??.
    PI,
    /// Rio de Janeiro.
    RJ,
    /// Rio Grande do Norte.
    RN,
    /// Rio Grande do Sul.
    RS,
    /// Rond??nia.
    RO,
    /// Roraima.
    RR,
    /// Santa Catarina.
    SC,
    /// S??o Paulo.
    SP,
    /// Sergipe.
    SE,
    /// Tocantins.
    TO,
}

/// A state in Canada.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum CanadaState {
    AB, BC, MB, NB, NL, NT, NS, NU, ON, PE, QC, SK, YT
}

/// A province in China.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum ChinaState {
    /// Anhui Province (?????????, ??nhu?? Sh??ng).
    AH,
    /// Beijing Municipality (?????????, B??ij??ng Sh??).
    BJ,
    /// Chongqing Municipality (?????????, Ch??ngq??ng Sh??).
    CQ,
    /// Fujian Province (?????????, F??ji??n Sh??ng).
    FJ,
    /// Guangdong Province (?????????, Gu??ngd??ng Sh??ng).
    GD,
    /// Gansu Province (?????????, G??ns?? Sh??ng).
    GS,
    /// Guangxi Zhuang Autonomous Region (?????????????????????, Gu??ngx?? Zhu??ngz?? Z??zh??q??).
    GX,
    /// Guizhou Province (?????????, Gu??zh??u Sh??ng).
    GZ,
    /// Henan Province (?????????, H??n??n Sh??ng).
    HEN,
    /// Hubei Province (?????????, H??b??i Sh??ng).
    HUB,
    /// Hebei Province (?????????, H??b??i Sh??ng).
    HEB,
    /// Hainan Province (?????????, H??in??n Sh??ng).
    HI,
    /// Hong Kong Special Administrative Region (?????????????????????, Xi??ngg??ng T??bi?? X??ngzh??ngq??).
    ///
    /// We usually treat Hong Kong as a separate country. This is here for completeness.
    HK,
    /// Heilongjiang Province (????????????, H??il??ngji??ng Sh??ng).
    HL,
    /// Hunan Province (?????????, H??n??n Sh??ng).
    HUN,
    /// Jilin Province (?????????, J??l??n Sh??ng).
    JL,
    /// Jiangsu Province (?????????, Ji??ngs?? Sh??ng).
    JS,
    /// Jiangxi Province (?????????, Ji??ngx?? Sh??ng).
    JX,
    /// Liaoning Province (?????????, Li??on??ng Sh??ng).
    LN,
    /// Macau Special Administrative Region (?????????????????????, ??om??n T??bi?? X??ngzh??ngq??).
    MO,
    /// Inner Mongolia Autonomous Region (??????????????????, N??i M??ngg?? Z??zh??q??).
    NM,
    /// Ningxia Hui Autonomous Region (?????????????????????, N??ngxi?? Hu??z?? Z??zh??q??).
    NX,
    /// Qinghai Province (?????????, Q??ngh??i Sh??ng).
    QH,
    /// Sichuan Province (?????????, S??chu??n Sh??ng).
    SC,
    /// Shandong Province (?????????, Sh??nd??ng Sh??ng).
    SD,
    /// Shanghai Municipality (?????????, Sh??ngh??i Sh??).
    SH,
    /// Shaanxi Province (?????????, Sh??nx?? Sh??ng).
    SAA,
    /// Shanxi Province (?????????, Sh??nx?? Sh??ng).
    SAX,
    /// Tianjin Municipality (?????????, Ti??nj??n Sh??).
    TJ,
    /// Xinjiang Uyghur Autonomous Region (????????????????????????, X??nji??ng W??iw??'??r Z??zh??q??).
    XJ,
    /// Tibet Autonomous Region (???????????????, X??z??ng Z??zh??q??).
    XZ,
    /// Yunnan Province (?????????, Y??nn??n Sh??ng).
    YN,
    /// Zhejiang Province (?????????, Zh??ji??ng Sh??ng).
    ZJ,
}

/// A region in England, ill-defined and used only by BP.
///
/// This omits other divisions not in England: Scotland, N.Ireland, and Wales.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum EnglandState {
    /// East Midlands.
    EM,
    /// Greater London.
    GL,
    /// North Midlands.
    NM,
    /// North West.
    NW,
    /// South East.
    SE,
    /// South Midlands.
    SM,
    /// South West.
    SW,
    /// West Midlands.
    WM,
    /// Yorkshire North East.
    YNE,
}

/// A state in Germany.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum GermanyState {
    /// Baden-W??rttemberg.
    BW,
    /// Bavaria.
    BY,
    /// Berlin.
    BE,
    /// Brandenburg.
    BB,
    /// Bremen.
    HB,
    /// Hesse.
    HE,
    /// Hamburg.
    HH,
    /// Mecklenburg-Vorpommern.
    MV,
    /// Lower Saxony.
    NI,
    /// North Rhine-Westphalia.
    NRW,
    /// Rhineland-Palatinate.
    RP,
    /// Schleswig-Holstein.
    SH,
    /// Saarland.
    SL,
    /// Saxony.
    SN,
    /// Saxony-Anhalt.
    ST,
    /// Thuringia.
    TH,
}

/// A state in India.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum IndiaState {
    /// Andaman and Nicobar Islands.
    AN,
    /// Andhra Pradesh.
    AP,
    /// Arunachal Pradesh.
    AR,
    /// Assam.
    AS,
    /// Bihar.
    BR,
    /// Chhattisgarh.
    CG,
    /// Chandigarh.
    CH,
    /// Daman and Diu.
    DD,
    /// Dadra and Nagar Haveli.
    DH,
    /// Delhi.
    DL,
    /// Goa.
    GA,
    /// Gujarat.
    GJ,
    /// Haryana.
    HR,
    /// Himachal Pradesh.
    HP,
    /// Jammu and Kashmir.
    JK,
    /// Jharkhand.
    JH,
    /// Karnataka.
    KA,
    /// Kerala.
    KL,
    /// Lakshadweep.
    LD,
    /// Madhya Pradesh.
    MP,
    /// Maharashtra.
    MH,
    /// Manipur.
    MN,
    /// Meghalaya.
    ML,
    /// Mizoram.
    MZ,
    /// Nagaland.
    NL,
    /// Orissa.
    OR,
    /// Punjab.
    PB,
    /// Pondicherry / Puducherry.
    PY,
    /// Rajasthan.
    RJ,
    /// Sikkim.
    SK,
    /// Tamil Nadu.
    TN,
    /// Tripura.
    TR,
    /// Uttarakhand.
    UK,
    /// Uttar Pradesh.
    UP,
    /// West Bengal.
    WB,
}

/// A state in Mexico.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum MexicoState {
    /// Aguascalientes.
    AG,
    /// Baja California.
    BC,
    /// Baja California Sur.
    BS,
    /// Campeche.
    CM,
    /// Chiapas.
    CS,
    /// Chihuahua.
    CH,
    /// Coahuila.
    CO,
    /// Colima.
    CL,
    /// Mexico City.
    DF,
    /// Durango.
    DG,
    /// Guanajuato.
    GT,
    /// Guerrero.
    GR,
    /// Hidalgo.
    HG,
    /// Jalisco.
    JA,
    /// M??xico.
    EM,
    /// Michoac??n.
    MI,
    /// Morelos.
    MO,
    /// Nayarit.
    NA,
    /// Nuevo Le??n.
    NL,
    /// Oaxaca.
    OA,
    /// Puebla.
    PU,
    /// Quer??taro.
    QT,
    /// Quintana Roo.
    QR,
    /// San Luis Potos??.
    SL,
    /// Sinaloa.
    SI,
    /// Sonora.
    SO,
    /// Tabasco.
    TB,
    /// Tamaulipas.
    TM,
    /// Tlaxcala.
    TL,
    /// Veracruz.
    VE,
    /// Yucat??n.
    YU,
    /// Zacatecas.
    ZA,
}

/// A state in the Netherlands.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum NetherlandsState {
    /// Drenthe.
    DR,
    /// Flevoland.
    FL,
    /// Friesland / Frysl??n.
    FR,
    /// Gelderland.
    GE,
    /// Groningen.
    GR,
    /// Limburg.
    LI,
    /// North Brabant / Noord-Brabant.
    NB,
    /// North Holland / Noord-Holland.
    NH,
    /// Overijssel / Overissel.
    OV,
    /// Utrecht.
    UT,
    /// Zeeland.
    ZE,
    /// South Holland / Zuid-Holland.
    ZH,
}

/// A region in New Zealand.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum NewZealandState {
    /// Northland.
    NTL,
    /// Auckland.
    AKL,
    /// Waikato.
    WKO,
    /// Bay of Plenty.
    BOP,
    /// Gisborne (East Coast).
    GIS,
    /// Hawke's Bay.
    HKB,
    /// Taranaki.
    TKI,
    /// Manawatu-Whanganui.
    MWT,
    /// Wellington.
    WGN,
    /// Tasman.
    TAS,
    /// Nelson.
    NSN,
    /// Marlborough.
    MBH,
    /// West Coast.
    WTC,
    /// Canterbury.
    CAN,
    /// Otago.
    OTA,
    /// Southland.
    STL,
}

/// A county in Romania.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum RomaniaState {
    /// Alba.
    AB,
    /// Arge??.
    AG,
    /// Arad.
    AR,
    /// Bucharest.
    B,
    /// Bac??u.
    BC,
    /// Bihor.
    BH,
    /// Bistri??a-N??s??ud.
    BN,
    /// Br??ila.
    BR,
    /// Boto??ani.
    BT,
    /// Bra??ov.
    BV,
    /// Buz??u.
    BZ,
    /// Cluj.
    CJ,
    /// C??l??ra??i.
    CL,
    /// Cara??-Severin.
    CS,
    /// Constan??a.
    CT,
    /// Covasna.
    CV,
    /// D??mbovi??a.
    DB,
    /// Dolj.
    DJ,
    /// Gorj.
    GJ,
    /// Gala??i.
    GL,
    /// Giurgiu.
    GR,
    /// Hunedoara.
    HD,
    /// Harghita.
    HR,
    /// Ilfov.
    IF,
    /// Ialomi??a.
    IL,
    /// Ia??i.
    IS,
    /// Mehedin??i.
    MH,
    /// Maramure??.
    MM,
    /// Mure??.
    MS,
    /// Neam??.
    NT,
    /// Olt.
    OT,
    /// Prahova.
    PH,
    /// Sibiu.
    SB,
    /// S??laj.
    SJ,
    /// Satu Mare.
    SM,
    /// Suceava.
    SV,
    /// Tulcea.
    TL,
    /// Timi??.
    TM,
    /// Teleorman.
    TR,
    /// V??lcea.
    VL,
    /// Vrancea.
    VN,
    /// Vaslui.
    VS,
}

/// An oblast in Russia.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum RussiaState {
    AD, AL, BA, BU, CE, CU, DA, IN, KB, KL, KC, KR, KK, KO, ME, MO, SA,
    SE, TA, TY, UD, ALT, KAM, KHA, KDA, KYA, PER, PRI, STA, ZAB, AMU, ARK,
    AST, BEL, BRY, CHE, IRK, IVA, KGD, KLU, KEM, KIR, KOS, KGN, KRS, LEN,
    LIP, MAG, MOS, MUR, NIZ, NGR, NVS, OMS, ORE, ORL, PNZ, PSK, ROS, RYA, 
    SAK, SAM, SAR, SMO, SVE, TAM, TOM, TUL, TVE, TYE, TYU, ULY, VLA, VGG,
    VLG, VOR, YAR, MOW, SPE, YEV, CHU, KHM, NEN, YAN
}

/// A province in South Africa, using conventional acronyms (non-ISO).
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum SouthAfricaState {
    /// Eastern Cape.
    EC,
    /// Free State.
    FS,
    /// Gauteng.
    GT,
    /// KwaZulu-Natal (ISO: NL).
    KZN,
    /// Limpopo.
    LP,
    /// Mpumalanga.
    MP,
    /// Northern Cape.
    NC,
    /// North-West.
    NW,
    /// Western Cape.
    WC,
}

/// A state in the USA.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize, ToString)]
pub enum USAState {
    AL, AK, AZ, AR, CA, CO, CT, DE, DC, FL, GA, HI, ID, IL, IN, IA, KS,
    KY, LA, ME, MD, MA, MI, MN, MS, MO, MT, NE, NV, NH, NJ, NM, NY, NC,
    ND, OH, OK, OR, PA, RI, SC, SD, TN, TX, UT, VT, VA, WA, WV, WI, WY,

    /// Guam is an unincorporated territory of the USA.
    Guam,
}
