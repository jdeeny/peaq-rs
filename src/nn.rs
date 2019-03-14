/* These functions encapsulate the neural-network-based computations described
 * in chapter 6 of <xref linkend="BS1387" />. They calculate the distortion
 * index from the model output variables or the objective difference grade from
 * the distortion index. While the calculation of the distortion index differs
 * between basic and advanced version, calculation of the objective difference
 * grade is the same.
 */


pub fn calc_di_basic(movs: &[f64]) -> f64 {
    0.
}
pub fn calc_di_advanced(movs: &[f64]) -> f64 {
    0.
}
pub fn calc_odg(di: f64) -> f64 {
    BMIN + (BMAX - BMIN) / (1. + f64::exp(-di))
}

static BMIN: f64 = -3.98;
static BMAX: f64 = 0.22;
static WYB_ADVANCED: f64 = -1.360308;
static WYB_BASIC: f64 = -0.307594;

lazy_static! {
    static ref ANIM_BASIC: Vec<f64> = vec! (
        393.916656, 361.965332, -24.045116, 1.110661, -0.206623, 0.074318, 1.113683,
        0.950345, 0.029985, 0.000101, 0.
    );
    static ref AMAX_BASIC: Vec<f64> = vec! (
        921., 881.131226, 16.212030, 107.137772, 2.886017, 13.933351, 63.257874,
        1145.018555, 14.819740, 1., 1.
    );
    static ref WX_BASIC: [[f64; 3]; 11] = [
        [-0.502657, 0.436333, 1.219602],
        [4.307481, 3.246017, 1.123743],
        [4.984241, -2.211189, -0.192096],
        [0.051056, -1.762424, 4.331315],
        [2.321580, 1.789971, -0.754560],
        [-5.303901, -3.452257, -10.814982],
        [2.730991, -6.111805, 1.519223],
        [0.624950, -1.331523, -5.955151],
        [3.102889, 0.871260, -5.922878],
        [-1.051468, -0.939882, -0.142913],
        [-1.804679, -0.503610, -0.620456],
    ];
    static ref WXB_BASIC: Vec<f64> = vec!( -2.518254, 0.654841, -2.207228 );
    static ref WY_BASIC: Vec<f64> = vec!( -3.817048, 4.107138, 4.629582, -0.307594 );

    static ref AMIN_ADVANCED: Vec<f64> = vec!(13.298751, 0.041073, -25.018791, 0.061560, 0.02452);
    static ref AMAX_ADVANCED: Vec<f64> = vec!(2166.5, 13.24326, 13.46708, 10.226771, 14.224874);
    static ref WX_ADVANCED: [[f64; 5]; 5] = [
        [21.211773, -39.013052, -1.382553, -14.545348, -0.320899],
        [-8.981803, 19.956049, 0.935389, -1.686586, -3.238586],
        [1.633830, -2.877505, -7.442935, 5.606502, -1.783120],
        [6.103821, 19.587435, -0.240284, 1.088213, -0.511314],
        [11.556344, 3.892028, 9.720441, -3.287205, -11.031250],
    ];

    static ref WXB_ADVANCED: Vec<f64> = vec!(1.330890, 2.686103, 2.096598, -1.327851, 3.087055);

    static ref WY_ADVANCED: Vec<f64> = vec!(-4.696996, -3.289959, 7.004782, 6.651897, 4.009144);


}
