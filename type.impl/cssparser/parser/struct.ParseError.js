(function() {var type_impls = {
"selectors":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ParseError%3C'i,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#135\">source</a><a href=\"#impl-ParseError%3C'i,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'i, T&gt; <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.basic\" class=\"method\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#137\">source</a><h4 class=\"code-header\">pub fn <a href=\"cssparser/parser/struct.ParseError.html#tymethod.basic\" class=\"fn\">basic</a>(self) -&gt; <a class=\"struct\" href=\"cssparser/parser/struct.BasicParseError.html\" title=\"struct cssparser::parser::BasicParseError\">BasicParseError</a>&lt;'i&gt;</h4></section></summary><div class=\"docblock\"><p>Extract the fundamental parse error from an extensible error.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into\" class=\"method\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#148-150\">source</a><h4 class=\"code-header\">pub fn <a href=\"cssparser/parser/struct.ParseError.html#tymethod.into\" class=\"fn\">into</a>&lt;U&gt;(self) -&gt; <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, U&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;U&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Like <code>std::convert::Into::into</code></p>\n</div></details></div></details>",0,"selectors::parser::SelectorParseError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CBasicParseError%3C'i%3E%3E-for-ParseError%3C'i,+T%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#65\">source</a><a href=\"#impl-From%3CBasicParseError%3C'i%3E%3E-for-ParseError%3C'i,+T%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'i, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"cssparser/parser/struct.BasicParseError.html\" title=\"struct cssparser::parser::BasicParseError\">BasicParseError</a>&lt;'i&gt;&gt; for <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#67\">source</a><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(this: <a class=\"struct\" href=\"cssparser/parser/struct.BasicParseError.html\" title=\"struct cssparser::parser::BasicParseError\">BasicParseError</a>&lt;'i&gt;) -&gt; <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, T&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<BasicParseError<'i>>","selectors::parser::SelectorParseError"],["<section id=\"impl-StructuralPartialEq-for-ParseError%3C'i,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#127\">source</a><a href=\"#impl-StructuralPartialEq-for-ParseError%3C'i,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'i, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, E&gt;</h3></section>","StructuralPartialEq","selectors::parser::SelectorParseError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-ParseError%3C'i,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#127\">source</a><a href=\"#impl-Debug-for-ParseError%3C'i,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'i, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, E&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#127\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","selectors::parser::SelectorParseError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-ParseError%3C'i,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#127\">source</a><a href=\"#impl-PartialEq-for-ParseError%3C'i,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'i, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, E&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#127\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;<a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, E&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/cmp.rs.html#242\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","selectors::parser::SelectorParseError"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-ParseError%3C'i,+E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#127\">source</a><a href=\"#impl-Clone-for-ParseError%3C'i,+E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'i, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, E&gt;<div class=\"where\">where\n    E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cssparser/parser.rs.html#127\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"cssparser/parser/struct.ParseError.html\" title=\"struct cssparser::parser::ParseError\">ParseError</a>&lt;'i, E&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.77.2/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.77.2/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","selectors::parser::SelectorParseError"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()