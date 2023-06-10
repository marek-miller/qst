use std::ffi::CString;

pub use ffi::{
    bitEncoding as BitEncoding,
    pauliOpType as PauliOpType,
    phaseFunc as PhaseFunc,
    phaseGateType as PhaseGateType,
};

mod ffi;

// TODO: define number abstractions for numerical types
// (use num_traits)
pub type Qreal = f64;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidQuESTInput { err_msg: String, err_func: String },
}

#[derive(Debug)]
pub struct Complex(ffi::Complex);

impl Complex {
    #[must_use]
    pub fn real(&self) -> Qreal {
        self.0.real
    }

    #[must_use]
    pub fn imag(&self) -> Qreal {
        self.0.imag
    }
}

#[derive(Debug)]
pub struct ComplexMatrix2(ffi::ComplexMatrix2);

#[derive(Debug)]
pub struct ComplexMatrix4(ffi::ComplexMatrix4);

#[derive(Debug)]
pub struct ComplexMatrixN(ffi::ComplexMatrixN);

#[derive(Debug)]
pub struct Vector(ffi::Vector);

#[derive(Debug)]
pub struct PauliHamil(ffi::PauliHamil);

#[derive(Debug)]
pub struct DiagonalOp(ffi::DiagonalOp);

#[derive(Debug)]
pub struct Qureg(ffi::Qureg);

impl Qureg {
    #[must_use]
    pub fn is_density_matrix(&self) -> bool {
        self.0.isDensityMatrix != 0
    }

    #[must_use]
    pub fn num_qubits_represented(&self) -> i32 {
        self.0.numQubitsRepresented
    }
}

#[derive(Debug)]
pub struct QuESTEnv(ffi::QuESTEnv);

#[must_use]
pub fn create_qureg(
    num_qubits: i32,
    env: &QuESTEnv,
) -> Qureg {
    Qureg(unsafe { ffi::createQureg(num_qubits, env.0) })
}

#[must_use]
pub fn create_density_qureg(
    num_qubits: i32,
    env: &QuESTEnv,
) -> Qureg {
    Qureg(unsafe { ffi::createDensityQureg(num_qubits, env.0) })
}

#[must_use]
pub fn create_clone_qureg(
    qureg: &Qureg,
    env: &QuESTEnv,
) -> Qureg {
    Qureg(unsafe { ffi::createCloneQureg(qureg.0, env.0) })
}

#[allow(clippy::needless_pass_by_value)]
pub fn destroy_qureg(
    qureg: Qureg,
    env: &QuESTEnv,
) {
    unsafe { ffi::destroyQureg(qureg.0, env.0) }
}

#[must_use]
pub fn create_complex_matrix_n(num_qubits: i32) -> ComplexMatrixN {
    ComplexMatrixN(unsafe { ffi::createComplexMatrixN(num_qubits) })
}

#[allow(clippy::needless_pass_by_value)]
pub fn destroy_complex_matrix_n(matr: ComplexMatrixN) {
    unsafe { ffi::destroyComplexMatrixN(matr.0) }
}

#[allow(clippy::cast_sign_loss)]
pub fn init_complex_matrix_n(
    m: &mut ComplexMatrixN,
    real: &[&[Qreal]],
    imag: &[&[Qreal]],
) {
    let n = m.0.numQubits as usize;
    assert!(real.len() >= n);
    assert!(imag.len() >= n);

    let mut real_ptrs = Vec::with_capacity(n);
    let mut imag_ptrs = Vec::with_capacity(n);
    unsafe {
        for i in 0..n {
            assert!(real[i].len() >= n);
            real_ptrs.push(real[i].as_ptr());
            assert!(imag[i].len() >= n);
            imag_ptrs.push(imag[i].as_ptr());
        }

        ffi::initComplexMatrixN(m.0, real_ptrs.as_ptr(), imag_ptrs.as_ptr());
    }
}

#[must_use]
pub fn create_pauli_hamil(
    num_qubits: i32,
    num_sum_terms: i32,
) -> PauliHamil {
    PauliHamil(unsafe { ffi::createPauliHamil(num_qubits, num_sum_terms) })
}

#[allow(clippy::needless_pass_by_value)]
pub fn destroy_pauli_hamil(hamil: PauliHamil) {
    unsafe { ffi::destroyPauliHamil(hamil.0) }
}

#[must_use]
pub fn create_pauli_hamil_from_file(fn_: &str) -> PauliHamil {
    let filename = CString::new(fn_).unwrap();
    PauliHamil(unsafe { ffi::createPauliHamilFromFile((*filename).as_ptr()) })
}

#[allow(clippy::cast_sign_loss)]
pub fn init_pauli_hamil(
    hamil: &mut PauliHamil,
    coeffs: &[Qreal],
    codes: &[PauliOpType],
) {
    let hamil_len = hamil.0.numSumTerms as usize;
    assert_eq!(coeffs.len(), hamil_len);
    assert_eq!(codes.len(), hamil_len * hamil.0.numQubits as usize);

    unsafe {
        let coeffs_ptr = coeffs.as_ptr();
        let codes_ptr = codes.as_ptr();
        ffi::initPauliHamil(hamil.0, coeffs_ptr, codes_ptr);
    }
}

#[must_use]
pub fn create_diagonal_op(
    num_qubits: i32,
    env: &QuESTEnv,
) -> DiagonalOp {
    DiagonalOp(unsafe { ffi::createDiagonalOp(num_qubits, env.0) })
}

#[allow(clippy::needless_pass_by_value)]
pub fn destroy_diagonal_op(
    op: DiagonalOp,
    env: &QuESTEnv,
) {
    unsafe {
        ffi::destroyDiagonalOp(op.0, env.0);
    }
}

pub fn sync_diagonal_op(op: &mut DiagonalOp) {
    unsafe {
        ffi::syncDiagonalOp(op.0);
    }
}

#[allow(clippy::cast_sign_loss)]
pub fn init_diagonal_op(
    op: &mut DiagonalOp,
    real: &[Qreal],
    imag: &[Qreal],
) {
    let len_required = 2usize.pow(op.0.numQubits as u32);
    assert!(real.len() >= len_required);
    assert!(imag.len() >= len_required);

    unsafe {
        let real_ptr = real.as_ptr();
        let imag_ptr = imag.as_ptr();
        ffi::initDiagonalOp(op.0, real_ptr, imag_ptr);
    }
}

pub fn init_diagonal_op_from_pauli_hamil(
    op: &mut DiagonalOp,
    hamil: &PauliHamil,
) {
    assert_eq!(op.0.numQubits, hamil.0.numQubits);
    unsafe { ffi::initDiagonalOpFromPauliHamil(op.0, hamil.0) }
}

#[must_use]
pub fn create_diagonal_op_from_pauli_hamil_file(
    fn_: &str,
    env: &QuESTEnv,
) -> DiagonalOp {
    let filename = CString::new(fn_).unwrap();
    DiagonalOp(unsafe {
        ffi::createDiagonalOpFromPauliHamilFile((*filename).as_ptr(), env.0)
    })
}

pub fn set_diagonal_op_elems(
    op: &mut DiagonalOp,
    start_ind: i64,
    real: &[Qreal],
    imag: &[Qreal],
    num_elems: i64,
) {
    assert!(real.len() >= num_elems as usize);
    assert!(imag.len() >= num_elems as usize);

    unsafe {
        let real_ptr = real.as_ptr();
        let imag_ptr = imag.as_ptr();

        ffi::setDiagonalOpElems(op.0, start_ind, real_ptr, imag_ptr, num_elems);
    }
}

pub fn apply_diagonal_op(
    qureg: &mut Qureg,
    op: &DiagonalOp,
) {
    unsafe {
        ffi::applyDiagonalOp(qureg.0, op.0);
    }
}

#[must_use]
pub fn calc_expec_diagonal_op(
    qureg: &Qureg,
    op: &DiagonalOp,
) -> Complex {
    Complex(unsafe { ffi::calcExpecDiagonalOp(qureg.0, op.0) })
}

pub fn report_state(qureg: &Qureg) {
    unsafe { ffi::reportState(qureg.0) }
}

pub fn report_state_to_screen(
    qureg: &Qureg,
    env: &QuESTEnv,
    report_rank: i32,
) {
    unsafe { ffi::reportStateToScreen(qureg.0, env.0, report_rank) }
}

pub fn report_qureg_params(qureg: &Qureg) {
    unsafe {
        ffi::reportQuregParams(qureg.0);
    }
}

pub fn report_pauli_hamil(hamil: &PauliHamil) {
    unsafe {
        ffi::reportPauliHamil(hamil.0);
    }
}

#[must_use]
pub fn get_num_qubits(qureg: &Qureg) -> i32 {
    unsafe { ffi::getNumQubits(qureg.0) }
}

#[must_use]
pub fn get_num_amps(qureg: &Qureg) -> i64 {
    unsafe { ffi::getNumAmps(qureg.0) }
}

pub fn init_blank_state(qureg: &mut Qureg) {
    unsafe {
        ffi::initBlankState(qureg.0);
    }
}

pub fn init_zero_state(qureg: &mut Qureg) {
    unsafe {
        ffi::initZeroState(qureg.0);
    }
}

pub fn init_plus_state(qureg: &mut Qureg) {
    unsafe {
        ffi::initPlusState(qureg.0);
    }
}

pub fn init_classical_state(
    qureg: &mut Qureg,
    state_ind: i64,
) {
    unsafe {
        ffi::initClassicalState(qureg.0, state_ind);
    }
}

pub fn init_pure_state(
    qureg: &mut Qureg,
    pure_: &Qureg,
) {
    unsafe {
        ffi::initPureState(qureg.0, pure_.0);
    }
}

pub fn init_debug_state(qureg: &mut Qureg) {
    unsafe {
        ffi::initDebugState(qureg.0);
    }
}

pub fn init_state_from_amps(
    qureg: &mut Qureg,
    reals: &[Qreal],
    imags: &[Qreal],
) {
    unsafe {
        let reals_ptr = reals.as_ptr();
        let imags_ptr = imags.as_ptr();

        ffi::initStateFromAmps(qureg.0, reals_ptr, imags_ptr);
    }
}

pub fn set_amps(
    qureg: &mut Qureg,
    start_ind: i64,
    reals: &[Qreal],
    imags: &[Qreal],
    num_amps: i64,
) {
    unsafe {
        let reals_ptr = reals.as_ptr();
        let imags_ptr = imags.as_ptr();

        ffi::setAmps(qureg.0, start_ind, reals_ptr, imags_ptr, num_amps);
    }
}

pub fn set_density_amps(
    qureg: &mut Qureg,
    start_row: i64,
    start_col: i64,
    reals: &[Qreal],
    imags: &[Qreal],
    num_amps: i64,
) {
    unsafe {
        let reals_ptr = reals.as_ptr();
        let imags_ptr = imags.as_ptr();

        ffi::setDensityAmps(
            qureg.0, start_row, start_col, reals_ptr, imags_ptr, num_amps,
        );
    }
}

pub fn clone_qureg(
    target_qureg: &mut Qureg,
    copy_qureg: &Qureg,
) {
    unsafe {
        ffi::cloneQureg(target_qureg.0, copy_qureg.0);
    }
}

pub fn phase_shift(
    qureg: &mut Qureg,
    target_quibit: i32,
    angle: Qreal,
) {
    unsafe {
        ffi::phaseShift(qureg.0, target_quibit, angle);
    }
}

pub fn controlled_phase_shift(
    qureg: &mut Qureg,
    id_qubit1: i32,
    id_qubit2: i32,
    angle: Qreal,
) {
    unsafe {
        ffi::controlledPhaseShift(qureg.0, id_qubit1, id_qubit2, angle);
    }
}

pub fn multi_controlled_phase_shift(
    qureg: &mut Qureg,
    control_qubits: &[i32],
    num_control_qubits: i32,
    angle: Qreal,
) {
    unsafe {
        let control_qubits_ptr = control_qubits.as_ptr();
        ffi::multiControlledPhaseShift(
            qureg.0,
            control_qubits_ptr,
            num_control_qubits,
            angle,
        );
    }
}

pub fn controlled_phase_flip(
    qureg: &mut Qureg,
    id_qubit1: i32,
    id_qubit2: i32,
) {
    unsafe {
        ffi::controlledPhaseFlip(qureg.0, id_qubit1, id_qubit2);
    }
}

pub fn multi_controlled_phase_flip(
    qureg: &mut Qureg,
    control_qubits: &[i32],
    num_control_qubits: i32,
) {
    unsafe {
        let control_qubits_ptr = control_qubits.as_ptr();
        ffi::multiControlledPhaseFlip(
            qureg.0,
            control_qubits_ptr,
            num_control_qubits,
        );
    }
}

pub fn s_gate(
    qureg: &mut Qureg,
    target_qubit: i32,
) {
    unsafe {
        ffi::sGate(qureg.0, target_qubit);
    }
}

pub fn t_gate(
    qureg: &mut Qureg,
    target_qubit: i32,
) {
    unsafe {
        ffi::tGate(qureg.0, target_qubit);
    }
}

#[must_use]
pub fn create_quest_env() -> QuESTEnv {
    QuESTEnv(unsafe { ffi::createQuESTEnv() })
}

pub fn destroy_quest_env(env: QuESTEnv) {
    unsafe {
        ffi::destroyQuESTEnv(env.0);
    }
}

pub fn sync_quest_env(env: &QuESTEnv) {
    unsafe {
        ffi::syncQuESTEnv(env.0);
    }
}

#[must_use]
pub fn sync_quuest_success(success_code: i32) -> i32 {
    unsafe { ffi::syncQuESTSuccess(success_code) }
}

pub fn report_quest_env(env: &QuESTEnv) {
    unsafe {
        ffi::reportQuESTEnv(env.0);
    }
}

#[must_use]
pub fn get_environment_string(env: &QuESTEnv) -> String {
    let mut cstr =
        CString::new("CUDA=x OpenMP=x MPI=x threads=xxxxxxx ranks=xxxxxxx")
            .unwrap();

    unsafe {
        let cstr_ptr = cstr.into_raw();
        ffi::getEnvironmentString(env.0, cstr_ptr);
        cstr = CString::from_raw(cstr_ptr);
    }
    cstr.into_string().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn init_complex_matrix_n_02() {
        let mut m = create_complex_matrix_n(2);
        init_complex_matrix_n(
            &mut m,
            &[&[1., 2.], &[3., 4.]],
            &[&[11., 12.], &[13., 14.]],
        );

        unsafe {
            let row = &*(*m.0.real).cast::<[&[f64; 2]; 2]>();
            assert_eq!(row, &[&[1., 2.,], &[3., 4.]]);
        }
        unsafe {
            let row = &*(*m.0.imag).cast::<[&[f64; 2]; 2]>();
            assert_eq!(row, &[&[11., 12.], &[13., 14.],]);
        }
        destroy_complex_matrix_n(m);
    }

    #[test]
    fn init_complex_matrix_n_03() {
        let mut m = create_complex_matrix_n(3);
        init_complex_matrix_n(
            &mut m,
            &[&[1., 2., 3.], &[4., 5., 6.], &[7., 8., 9.]],
            &[&[11., 12., 13.], &[14., 15., 16.], &[17., 18., 19.]],
        );

        unsafe {
            let row = &*(*m.0.real).cast::<[&[f64; 3]; 3]>();
            assert_eq!(row, &[&[1., 2., 3.], &[4., 5., 6.], &[7., 8., 9.]]);
        }
        unsafe {
            let row = &*(*m.0.imag).cast::<[&[f64; 3]; 3]>();
            assert_eq!(
                row,
                &[&[11., 12., 13.], &[14., 15., 16.], &[17., 18., 19.]]
            );
        }
        destroy_complex_matrix_n(m);
    }

    #[test]
    fn get_environment_string_01() {
        let env = create_quest_env();
        let env_str = get_environment_string(&env);

        assert!(env_str.contains("CUDA="));
        assert!(env_str.contains("OpenMP="));
        assert!(env_str.contains("MPI="));
        assert!(env_str.contains("threads="));
        assert!(env_str.contains("ranks="));
    }
}
