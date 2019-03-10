/* Unfortunately, <xref linkend="BS1387" /> is in some aspects underspecified
* or inconstent. Additionally, <xref linkend="Kabal03" /> proposes a few
* reasonable deviations from the exact wording of <xref linkend="BS1387" />.
* To facillitate easy exploration of the alternatives, they can be chosen
* using the preprocessor macros defined herein.
*/

pub struct Config {

  /**
   * SWAP_MOD_PATTS_FOR_NOISE_LOUDNESS_MOVS:
   *
   * Controls whether the modulation patterns are exchanged along with excitation
   * patters for RmsMssingComponentsA in peaq_mov_noise_loud_asym() and
   * AvgLinDistA n peaq_mov_lin_dist(). While <xref linkend="BS1387" /> only
   * describes changing the excitation patterns, in <xref linkend="Kabal03" />
   * the modulation patterns are also exchanged accordingly. Set to zero or
   * undefine to take <xref linkend="BS1387" /> literally, set to some non-zero
   * value to use the interpretation from <xref linkend="Kabal03" />.
   */
  //#define SWAP_MOD_PATTS_FOR_NOISE_LOUDNESS_MOVS 1
  pub swap_mod_patts_for_noise_loudness_movs: bool,

  /**
   * CENTER_EHS_CORRELATION_WINDOW:
   *
   * Controls whether the Hann window applied to the correlation when computing
   * the error harmonic structure in peaq_mov_ehs() is centered at bin zero as
   * proposed in <xref linkend="Kabal03" />.
   */
  //#define CENTER_EHS_CORRELATION_WINDOW 0
  pub center_ehs_correlation_window: bool,

  /**
   * EHS_SUBTRACT_DC_BEFORE_WINDOW:
   *
   * Controls whether the DC component is removed from the correlation before (as
   * proposed by <xref linkend="Kabal03" />) or after (as described in <xref
   * linkend="Kabal03" />) applying the Hann window to the correlation when
   * computing the error harmonic structure in peaq_mov_ehs().
   */
  //#define EHS_SUBTRACT_DC_BEFORE_WINDOW 1
  pub ehs_subtract_dc_before_window: bool,

  /**
   * USE_FLOOR_FOR_STEPS_ABOVE_THRESHOLD:
   *
   * Controls whether the INT operation used in the calculation of the steps
   * above threshold in peaq_mov_prob_detect() is implemented as trunc() (which
   * is the usual meaning) or as floor() (which according to <xref
   * linkend="Kabal03" /> makes more sense).
   */
  //#define USE_FLOOR_FOR_STEPS_ABOVE_THRESHOLD 0
  pub use_floor_for_steps_above_threshold: bool,

  /**
   * CLAMP_MOVS:
   *
   * Controls whether the model output variables are clamped to the range [amin,
   * amax] before calulating the distortion index in peaq_calculate_di_basic() or
   * peaq_calculate_di_advanced(). This is proposed in <xref linkend="Kabal03" />
   * but not mentioned at all in <xref linkend="BS1387" />.
   */
  //#define CLAMP_MOVS 0
  pub clamp_movs: bool,

  /**
   * SWAP_SLOPE_FILTER_COEFFICIENTS:
   *
   * Controls whether the coefficients of time smoothing filter for the frequency
   * domain spreading slopes in the filterbank-based ear model are swapped
   * compared to the pseudo code of <xref linkend="BS1387" />. As remarked
   * in <xref linkend="Kabal03" />, the pseudo code is incosistent with the
   * textual description and results in an extremely short time constant.
   */
  //#define SWAP_SLOPE_FILTER_COEFFICIENTS 0
  pub swap_slope_filter_coefficients: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            swap_mod_patts_for_noise_loudness_movs: true,
            center_ehs_correlation_window: false,
            ehs_subtract_dc_before_window: true,
            use_floor_for_steps_above_threshold: false,
            clamp_movs: false,
            swap_slope_filter_coefficients: false,
        }
    }
}
