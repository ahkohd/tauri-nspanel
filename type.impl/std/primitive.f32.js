(function() {
    var type_impls = Object.fromEntries([["brotli",[]],["core_graphics",[]],["serde",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[13,21,13]}