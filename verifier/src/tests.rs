    use crate::engine::{PublicInputs, VerifierEngine};
    use zkcg_common::{
        state::ProtocolState,
        types::Commitment,
        errors::ProtocolError,
    };
    use crate::backend_stub::StubBackend;

    fn dummy_commitment() -> Commitment {
        Commitment([42u8; 32])
    }

    fn initial_state() -> ProtocolState {
        ProtocolState::genesis()
    }

    fn valid_inputs(state: &ProtocolState) -> PublicInputs {
        PublicInputs {
            threshold: 10,
            old_state_root: state.state_root,
            nonce: state.nonce + 1,
        }
    }

    #[test]
    fn valid_state_transition_succeeds() {
        let state = initial_state();
        let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(StubBackend::default()),
    );


        let inputs = valid_inputs(&state);
        let commitment = dummy_commitment();

        let result = engine.process_transition(
            b"valid-proof",
            inputs,
            commitment,
        );

        assert!(result.is_ok());

    }


    #[test]
    fn invalid_nonce_is_rejected() {
        let state = initial_state();
        let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(StubBackend::default()),
    );


        let mut inputs = valid_inputs(&state);
        inputs.nonce = state.nonce; // replay attempt

        let commitment = dummy_commitment();

        let err = engine.process_transition(
            b"valid-proof",
            inputs,
            commitment,
        )
        .unwrap_err();

        assert!(matches!(err, ProtocolError::InvalidNonce));
    }

    #[test]
    fn state_root_mismatch_is_rejected() {
        let state = initial_state();
        let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(StubBackend::default()),
    );


        let mut inputs = valid_inputs(&state);
        inputs.old_state_root = [1u8; 32]; // forged root

        let commitment = dummy_commitment();

        let err = engine.process_transition(
            b"valid-proof",
            inputs,
            commitment,
        )
        .unwrap_err();

        assert!(matches!(err, ProtocolError::StateMismatch));
    }

    #[cfg(not(feature = "zk-halo2"))]
    #[test]
    fn policy_violation_is_rejected() {
        let state = initial_state();
        let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(StubBackend::default()),
    );


        let mut inputs = valid_inputs(&state);
        inputs.threshold = 0; // violates policy

        let commitment = dummy_commitment();

        let err = engine.process_transition(
            b"valid-proof",
            inputs,
            commitment,
        )
        .unwrap_err();

        assert!(matches!(err, ProtocolError::PolicyViolation));
    }

    #[test]
    fn state_updates_after_valid_transition() {
        let state = initial_state();
        let mut engine = VerifierEngine::new(
        state.clone(),
        Box::new(StubBackend::default()),
    );


        let inputs = valid_inputs(&state);
        let commitment = dummy_commitment();

        let result = engine.process_transition(
            b"valid-proof",
            inputs,
            commitment.clone(),
        );
        result.unwrap();
        #[cfg(not(feature = "zk-halo2"))]
        {
            let updated = engine.state();

            assert_eq!(updated.nonce, 1);
            assert_eq!(updated.state_root, commitment.0);
        }

        let updated = engine.state();

        assert_eq!(updated.nonce, 1);
        assert_eq!(updated.state_root, commitment.0);
    }
    
