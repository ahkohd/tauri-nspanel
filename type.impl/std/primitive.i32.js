(function() {
    var type_impls = Object.fromEntries([["cocoa",[]],["core_foundation",[]],["core_foundation_sys",[]],["core_graphics",[]],["core_graphics_types",[]],["serde",[]],["tokio",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[12,23,27,21,27,13,13]}