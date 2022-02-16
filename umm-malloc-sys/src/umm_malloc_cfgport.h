
#ifdef CARGO_FEATURE_FIRST_FIT
    #define UMM_FIRST_FIT
#else
    #define UMM_BEST_FIT
#endif

#ifdef CARGO_FEATURE_CORTEX_M_INTERRUPT_CRITICAL_SECTION
    #define UMM_CRITICAL_DECL(tag) uint8_t _saved_primask_##tag = 0;
    #define UMM_CRITICAL_ENTRY(tag) \
        do { \
            __asm__ volatile ( \
            "mrs %[output], PRIMASK\n" \
            "cpsid i" \
            : [output] "=r" ( _saved_primask_##tag )); \
        } while (0)
    #define UMM_CRITICAL_EXIT(tag) \
        do { \
            if ((_saved_primask_##tag & 1) == 0) { \
                __asm__ volatile ("cpsie i\n"); \
            } \
        } while (0)
#elif defined(CARGO_FEATURE_EXTERN_CRITICAL_SECTION)
    extern void _umm_critical_entry(uint32_t*);
    extern void _umm_critical_exit(uint32_t*);
    #define UMM_CRITICAL_DECL(tag) uint32_t _cs_tag_##tag = 0;
    #define UMM_CRITICAL_ENTRY(tag) _umm_critical_entry( & _cs_tag_##tag )
    #define UMM_CRITICAL_EXIT(tag) _umm_critical_exit( & _cs_tag_##tag )
#endif

#ifdef CARGO_FEATURE_INIT_IF_UNINITIALIZED
    #define UMM_CHECK_INITIALIZED() UMM_INIT_IF_UNINITIALIZED()
#endif

#ifdef CARGO_FEATURE_HANG_IF_UNINITIALIZED
    #define UMM_CHECK_INITIALIZED() UMM_HANG_IF_UNINITIALIZED()
#endif
