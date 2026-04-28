type F = f32;

// Units
pub type Heading = F;
pub type Velocity = F;
pub type Mbar = F;
pub type Kelvin = F;
pub type Celcius = F;

// Metrics (after https://www.weather.gov/source/zhu/ZHU_Training_Page/convective_parameters/skewt/skewtinfo.html)
// TODO: Replace "F" types with correct units.
pub type CAPE = F; // Convective Avalible Potential Energy
pub type CIN = F; // Convective Inhibition
pub type LCL = F; // Lifted Condensation Level
pub type LFC = F; // Level of Free Convection
pub type EL = F; // Equilibrium Level
pub type BRN = F; // Bulk Richardson Number
pub type BSHR = F; // Bulk Shear
pub type CAP = Celcius; // Cap strength
pub type CCL = F; // Convective Condensation Level
pub type EHI = F; // Energy Helicity Index
pub type EI = F; // Energy Index
pub type FRZ = F; // Point at which sounding is 0C
pub type THETAE = Kelvin; // Equivalent Potential Energy
pub type HEL = F; // Helicity
pub type HYDROLAPSE = F; // hydrolapse
pub type K = F; // K index
pub type L57 = F; // 700 to 500 milibar lapse rate
pub type LI = F; // Lifted Index
pub type MAXT = Kelvin; // Estimated maximum afternoon temperature
pub type MPL = F; // Maximum parcel level
pub type PTEMP = F; // Potential Temperature
pub type PW = F; // Precipitable water in inches
pub type RH = F; // Relative Humidity
pub type SHR = F; // Positive Shear
pub type SI = F; // Showalter Index
pub type SRDS = F; // Storm Relative Directional Shear
pub type STM = (Heading, Velocity); // Estimated storm motion
pub type SWEAT = F; // Severe weather threat index
pub type TP = F; // Tropopause Level
pub type TT = F; // Total Totals index
pub type WBO = Kelvin; // Wet Bulb Zero temperature
pub type WBPT = Kelvin; // Wet Bulb Potential Temperature

pub struct SoundingRow {
    isobar: Mbar,
    dew_point: Kelvin,
    wind_direction: Heading,
    wind_speed: Velocity,
}

pub struct SoundingData {
    rows: Vec<SoundingRow>,
    time: std::time::Instant,
}

/// Parameters calculated from sounding data
pub struct SoundingCParams {
    cape: CAPE,
    cin: CIN,
    lfc: LFC,
    lcl: LCL,
    el: EL,
    brn: BRN,
    bshr: BSHR,
    cap: CAP,
    ccl: CCL,
    ehi: EHI,
    ei: EI,
    frz: FRZ,
    theta_e: THETAE,
    hel: HEL,
    hydrolapse: HYDROLAPSE,
    k: K,
    l57: L57,
    li: LI,
    maxt: MAXT,
    mpl: MPL,
    ptemp: PTEMP,
    pw: PW,
    rh: RH,
    shr: SHR,
    si: SI,
    srds: SRDS,
    stm: STM,
    sweat: SWEAT,
    tp: TP,
    tt: TT,
    wbo: WBO,
    wbpt: WBPT,
}
