window.SIDEBAR_ITEMS = {"enum":[["PatchError","This type represents all possible errors that can occur when applying JSON patch"],["PatchOperation","JSON Patch single patch operation"]],"fn":[["diff","Diff two JSON documents and generate a JSON Patch (RFC 6902)."],["from_value","Create JSON Patch from JSON Value"],["merge","Patch provided JSON document (given as `serde_json::Value`) in place with JSON Merge Patch (RFC 7396)."],["patch","Patch provided JSON document (given as `serde_json::Value`) in-place. If any of the patch is failed, all previous operations are reverted. In case of internal error resulting in panic, document might be left in inconsistent state."],["patch_unsafe","Patch provided JSON document (given as `serde_json::Value`) in place. Operations are applied in unsafe manner. If any of the operations fails, all previous operations are not reverted."]],"struct":[["AddOperation","JSON Patch ‘add’ operation representation"],["CopyOperation","JSON Patch ‘copy’ operation representation"],["MoveOperation","JSON Patch ‘move’ operation representation"],["Patch","Representation of JSON Patch (list of patch operations)"],["RemoveOperation","JSON Patch ‘remove’ operation representation"],["ReplaceOperation","JSON Patch ‘replace’ operation representation"],["TestOperation","JSON Patch ‘test’ operation representation"]]};