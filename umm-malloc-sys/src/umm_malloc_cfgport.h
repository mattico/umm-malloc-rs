
#ifdef CARGO_FEATURE_FIRST_FIT
    #define UMM_FIRST_FIT
#else
    #define UMM_BEST_FIT
#endif

#ifdef CARGO_FEATURE_EXTERN_CRITICAL_SECTION
    extern void _umm_critical_entry(void);
    extern void _umm_critical_exit(void);
    #define UMM_CRITICAL_ENTRY() _umm_critical_entry()
    #define UMM_CRITICAL_EXIT() _umm_critical_exit()
#endif

#ifdef CARGO_FEATURE_INIT_IF_UNINITIALIZED
    #define UMM_CHECK_INITIALIZED() UMM_INIT_IF_UNINITIALIZED()
#endif

#ifdef CARGO_FEATURE_HANG_IF_UNINITIALIZED
    #define UMM_CHECK_INITIALIZED() UMM_HANG_IF_UNINITIALIZED()
#endif
