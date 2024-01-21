(function() {var type_impls = {
"selectors":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CountingBloomFilter%3CS%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#75-123\">source</a><a href=\"#impl-CountingBloomFilter%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"struct\" href=\"selectors/bloom/struct.CountingBloomFilter.html\" title=\"struct selectors::bloom::CountingBloomFilter\">CountingBloomFilter</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"selectors/bloom/trait.BloomStorage.html\" title=\"trait selectors::bloom::BloomStorage\">BloomStorage</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#81-83\">source</a><h4 class=\"code-header\">pub fn <a href=\"selectors/bloom/struct.CountingBloomFilter.html#tymethod.new\" class=\"fn\">new</a>() -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Creates a new bloom filter.</p>\n</div></details><section id=\"method.clear\" class=\"method\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#86-88\">source</a><h4 class=\"code-header\">pub fn <a href=\"selectors/bloom/struct.CountingBloomFilter.html#tymethod.clear\" class=\"fn\">clear</a>(&amp;mut self)</h4></section><section id=\"method.is_zeroed\" class=\"method\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#93-95\">source</a><h4 class=\"code-header\">pub fn <a href=\"selectors/bloom/struct.CountingBloomFilter.html#tymethod.is_zeroed\" class=\"fn\">is_zeroed</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.bool.html\">bool</a></h4></section><details class=\"toggle method-toggle\" open><summary><section id=\"method.insert_hash\" class=\"method\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#104-107\">source</a><h4 class=\"code-header\">pub fn <a href=\"selectors/bloom/struct.CountingBloomFilter.html#tymethod.insert_hash\" class=\"fn\">insert_hash</a>(&amp;mut self, hash: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>)</h4></section></summary><div class=\"docblock\"><p>Inserts an item with a particular hash into the bloom filter.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.remove_hash\" class=\"method\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#111-114\">source</a><h4 class=\"code-header\">pub fn <a href=\"selectors/bloom/struct.CountingBloomFilter.html#tymethod.remove_hash\" class=\"fn\">remove_hash</a>(&amp;mut self, hash: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>)</h4></section></summary><div class=\"docblock\"><p>Removes an item with a particular hash from the bloom filter.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.might_contain_hash\" class=\"method\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#120-122\">source</a><h4 class=\"code-header\">pub fn <a href=\"selectors/bloom/struct.CountingBloomFilter.html#tymethod.might_contain_hash\" class=\"fn\">might_contain_hash</a>(&amp;self, hash: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u32.html\">u32</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Check whether the filter might contain an item with the given hash.\nThis can sometimes return true even if the item is not in the filter,\nbut will never return false for items that are actually in the filter.</p>\n</div></details></div></details>",0,"selectors::bloom::BloomFilter"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-CountingBloomFilter%3CS%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#125-138\">source</a><a href=\"#impl-Debug-for-CountingBloomFilter%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"selectors/bloom/struct.CountingBloomFilter.html\" title=\"struct selectors::bloom::CountingBloomFilter\">CountingBloomFilter</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"selectors/bloom/trait.BloomStorage.html\" title=\"trait selectors::bloom::BloomStorage\">BloomStorage</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#129-137\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","selectors::bloom::BloomFilter"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-CountingBloomFilter%3CS%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#67\">source</a><a href=\"#impl-Clone-for-CountingBloomFilter%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"selectors/bloom/struct.CountingBloomFilter.html\" title=\"struct selectors::bloom::CountingBloomFilter\">CountingBloomFilter</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"selectors/bloom/trait.BloomStorage.html\" title=\"trait selectors::bloom::BloomStorage\">BloomStorage</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#67\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"selectors/bloom/struct.CountingBloomFilter.html\" title=\"struct selectors::bloom::CountingBloomFilter\">CountingBloomFilter</a>&lt;S&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.75.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","selectors::bloom::BloomFilter"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-CountingBloomFilter%3CS%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#67\">source</a><a href=\"#impl-Default-for-CountingBloomFilter%3CS%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"selectors/bloom/struct.CountingBloomFilter.html\" title=\"struct selectors::bloom::CountingBloomFilter\">CountingBloomFilter</a>&lt;S&gt;<span class=\"where fmt-newline\">where\n    S: <a class=\"trait\" href=\"selectors/bloom/trait.BloomStorage.html\" title=\"trait selectors::bloom::BloomStorage\">BloomStorage</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</span></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/selectors/bloom.rs.html#67\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"selectors/bloom/struct.CountingBloomFilter.html\" title=\"struct selectors::bloom::CountingBloomFilter\">CountingBloomFilter</a>&lt;S&gt;</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.75.0/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","selectors::bloom::BloomFilter"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()