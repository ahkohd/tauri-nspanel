(function() {var type_impls = {
"tauri":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Wry%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#1980\">source</a><a href=\"#impl-Wry%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"struct\" href=\"tauri_runtime_wry/struct.Wry.html\" title=\"struct tauri_runtime_wry::Wry\">Wry</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"tauri_runtime/trait.UserEvent.html\" title=\"trait tauri_runtime::UserEvent\">UserEvent</a>,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.plugin\" class=\"method\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2040\">source</a><h4 class=\"code-header\">pub fn <a href=\"tauri_runtime_wry/struct.Wry.html#tymethod.plugin\" class=\"fn\">plugin</a>&lt;P&gt;(&amp;mut self, plugin: P)<div class=\"where\">where\n    P: <a class=\"trait\" href=\"tauri_runtime_wry/trait.PluginBuilder.html\" title=\"trait tauri_runtime_wry::PluginBuilder\">PluginBuilder</a>&lt;T&gt; + 'static,</div></h4></section></div></details>",0,"tauri::Wry"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Runtime%3CT%3E-for-Wry%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2047\">source</a><a href=\"#impl-Runtime%3CT%3E-for-Wry%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"tauri_runtime/trait.Runtime.html\" title=\"trait tauri_runtime::Runtime\">Runtime</a>&lt;T&gt; for <a class=\"struct\" href=\"tauri_runtime_wry/struct.Wry.html\" title=\"struct tauri_runtime_wry::Wry\">Wry</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"tauri_runtime/trait.UserEvent.html\" title=\"trait tauri_runtime::UserEvent\">UserEvent</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Dispatcher\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Dispatcher\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"tauri_runtime/trait.Runtime.html#associatedtype.Dispatcher\" class=\"associatedtype\">Dispatcher</a> = <a class=\"struct\" href=\"tauri_runtime_wry/struct.WryDispatcher.html\" title=\"struct tauri_runtime_wry::WryDispatcher\">WryDispatcher</a>&lt;T&gt;</h4></section></summary><div class='docblock'>The message dispatcher.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.Handle\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Handle\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"tauri_runtime/trait.Runtime.html#associatedtype.Handle\" class=\"associatedtype\">Handle</a> = <a class=\"struct\" href=\"tauri_runtime_wry/struct.WryHandle.html\" title=\"struct tauri_runtime_wry::WryHandle\">WryHandle</a>&lt;T&gt;</h4></section></summary><div class='docblock'>The runtime handle type.</div></details><details class=\"toggle\" open><summary><section id=\"associatedtype.EventLoopProxy\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.EventLoopProxy\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"tauri_runtime/trait.Runtime.html#associatedtype.EventLoopProxy\" class=\"associatedtype\">EventLoopProxy</a> = <a class=\"struct\" href=\"tauri_runtime_wry/struct.EventProxy.html\" title=\"struct tauri_runtime_wry::EventProxy\">EventProxy</a>&lt;T&gt;</h4></section></summary><div class='docblock'>The proxy type.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2062\">source</a><a href=\"#method.new\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.new\" class=\"fn\">new</a>() -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"tauri_runtime_wry/struct.Wry.html\" title=\"struct tauri_runtime_wry::Wry\">Wry</a>&lt;T&gt;, <a class=\"enum\" href=\"tauri_runtime/enum.Error.html\" title=\"enum tauri_runtime::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Creates a new webview runtime. Must be used on the main thread.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.create_proxy\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2077\">source</a><a href=\"#method.create_proxy\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.create_proxy\" class=\"fn\">create_proxy</a>(&amp;self) -&gt; <a class=\"struct\" href=\"tauri_runtime_wry/struct.EventProxy.html\" title=\"struct tauri_runtime_wry::EventProxy\">EventProxy</a>&lt;T&gt;</h4></section></summary><div class='docblock'>Creates an <code>EventLoopProxy</code> that can be used to dispatch user events to the main event loop.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.handle\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2081\">source</a><a href=\"#method.handle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.handle\" class=\"fn\">handle</a>(&amp;self) -&gt; &lt;<a class=\"struct\" href=\"tauri_runtime_wry/struct.Wry.html\" title=\"struct tauri_runtime_wry::Wry\">Wry</a>&lt;T&gt; as <a class=\"trait\" href=\"tauri_runtime/trait.Runtime.html\" title=\"trait tauri_runtime::Runtime\">Runtime</a>&lt;T&gt;&gt;::<a class=\"associatedtype\" href=\"tauri_runtime/trait.Runtime.html#associatedtype.Handle\" title=\"type tauri_runtime::Runtime::Handle\">Handle</a></h4></section></summary><div class='docblock'>Gets a runtime handle.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.create_window\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2097\">source</a><a href=\"#method.create_window\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.create_window\" class=\"fn\">create_window</a>(\n    &amp;self,\n    pending: <a class=\"struct\" href=\"tauri_runtime/window/struct.PendingWindow.html\" title=\"struct tauri_runtime::window::PendingWindow\">PendingWindow</a>&lt;T, <a class=\"struct\" href=\"tauri_runtime_wry/struct.Wry.html\" title=\"struct tauri_runtime_wry::Wry\">Wry</a>&lt;T&gt;&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"tauri_runtime/window/struct.DetachedWindow.html\" title=\"struct tauri_runtime::window::DetachedWindow\">DetachedWindow</a>&lt;T, <a class=\"struct\" href=\"tauri_runtime_wry/struct.Wry.html\" title=\"struct tauri_runtime_wry::Wry\">Wry</a>&lt;T&gt;&gt;, <a class=\"enum\" href=\"tauri_runtime/enum.Error.html\" title=\"enum tauri_runtime::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Create a new webview window.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_activation_policy\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2176\">source</a><a href=\"#method.set_activation_policy\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.set_activation_policy\" class=\"fn\">set_activation_policy</a>(&amp;mut self, activation_policy: <a class=\"enum\" href=\"tauri/enum.ActivationPolicy.html\" title=\"enum tauri::ActivationPolicy\">ActivationPolicy</a>)</h4></section></summary><div class='docblock'>Sets the activation policy for the application. It is set to <code>NSApplicationActivationPolicyRegular</code> by default.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.show\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2188\">source</a><a href=\"#method.show\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.show\" class=\"fn\">show</a>(&amp;self)</h4></section></summary><div class='docblock'>Shows the application, but does not automatically focus it.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.hide\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2193\">source</a><a href=\"#method.hide\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.hide\" class=\"fn\">hide</a>(&amp;self)</h4></section></summary><div class='docblock'>Hides the application.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_device_event_filter\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2197\">source</a><a href=\"#method.set_device_event_filter\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.set_device_event_filter\" class=\"fn\">set_device_event_filter</a>(&amp;mut self, filter: <a class=\"enum\" href=\"tauri/enum.DeviceEventFilter.html\" title=\"enum tauri::DeviceEventFilter\">DeviceEventFilter</a>)</h4></section></summary><div class='docblock'>Change the device event filter mode. <a href=\"tauri_runtime/trait.Runtime.html#tymethod.set_device_event_filter\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.run_iteration\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2204\">source</a><a href=\"#method.run_iteration\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.run_iteration\" class=\"fn\">run_iteration</a>&lt;F&gt;(&amp;mut self, callback: F) -&gt; <a class=\"struct\" href=\"tauri/struct.RunIteration.html\" title=\"struct tauri::RunIteration\">RunIteration</a><div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"enum\" href=\"tauri_runtime/enum.RunEvent.html\" title=\"enum tauri_runtime::RunEvent\">RunEvent</a>&lt;T&gt;) + 'static,</div></h4></section></summary><div class='docblock'>Runs the one step of the webview runtime event loop and returns control flow to the caller.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.run\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#2283\">source</a><a href=\"#method.run\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"tauri_runtime/trait.Runtime.html#tymethod.run\" class=\"fn\">run</a>&lt;F&gt;(self, callback: F)<div class=\"where\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"enum\" href=\"tauri_runtime/enum.RunEvent.html\" title=\"enum tauri_runtime::RunEvent\">RunEvent</a>&lt;T&gt;) + 'static,</div></h4></section></summary><div class='docblock'>Run the webview runtime.</div></details></div></details>","Runtime<T>","tauri::Wry"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Wry%3CT%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#1841\">source</a><a href=\"#impl-Debug-for-Wry%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"tauri_runtime_wry/struct.Wry.html\" title=\"struct tauri_runtime_wry::Wry\">Wry</a>&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"tauri_runtime/trait.UserEvent.html\" title=\"trait tauri_runtime::UserEvent\">UserEvent</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/tauri_runtime_wry/lib.rs.html#1842\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.77.2/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.77.2/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.77.2/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.77.2/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","tauri::Wry"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()