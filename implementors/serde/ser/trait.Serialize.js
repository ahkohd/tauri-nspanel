(function() {var implementors = {
"json_patch":[["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"json_patch/struct.Patch.html\" title=\"struct json_patch::Patch\">Patch</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"json_patch/struct.AddOperation.html\" title=\"struct json_patch::AddOperation\">AddOperation</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"json_patch/struct.RemoveOperation.html\" title=\"struct json_patch::RemoveOperation\">RemoveOperation</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"json_patch/struct.ReplaceOperation.html\" title=\"struct json_patch::ReplaceOperation\">ReplaceOperation</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"json_patch/struct.MoveOperation.html\" title=\"struct json_patch::MoveOperation\">MoveOperation</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"json_patch/struct.CopyOperation.html\" title=\"struct json_patch::CopyOperation\">CopyOperation</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"json_patch/struct.TestOperation.html\" title=\"struct json_patch::TestOperation\">TestOperation</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"json_patch/enum.PatchOperation.html\" title=\"enum json_patch::PatchOperation\">PatchOperation</a>"]],
"plist":[["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"plist/dictionary/struct.Dictionary.html\" title=\"struct plist::dictionary::Dictionary\">Dictionary</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"plist/struct.Data.html\" title=\"struct plist::Data\">Data</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"plist/struct.Date.html\" title=\"struct plist::Date\">Date</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"plist/struct.Integer.html\" title=\"struct plist::Integer\">Integer</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"plist/struct.Uid.html\" title=\"struct plist::Uid\">Uid</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"plist/enum.Value.html\" title=\"enum plist::Value\">Value</a>"]],
"semver":[["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"semver/struct.Version.html\" title=\"struct semver::Version\">Version</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"semver/struct.VersionReq.html\" title=\"struct semver::VersionReq\">VersionReq</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"semver/struct.Comparator.html\" title=\"struct semver::Comparator\">Comparator</a>"]],
"serde":[],
"serde_json":[["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/struct.Map.html\" title=\"struct serde_json::Map\">Map</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.68.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"serde_json/enum.Value.html\" title=\"enum serde_json::Value\">Value</a>&gt;"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"serde_json/enum.Value.html\" title=\"enum serde_json::Value\">Value</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/value/struct.Number.html\" title=\"struct serde_json::value::Number\">Number</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_json/value/struct.RawValue.html\" title=\"struct serde_json::value::RawValue\">RawValue</a>"]],
"serde_with":[["impl&lt;'a, T, U&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"serde_with/ser/struct.SerializeAsWrap.html\" title=\"struct serde_with::ser::SerializeAsWrap\">SerializeAsWrap</a>&lt;'a, T, U&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;U: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.68.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"serde_with/trait.SerializeAs.html\" title=\"trait serde_with::SerializeAs\">SerializeAs</a>&lt;T&gt;,</span>"]],
"string_cache":[["impl&lt;Static:&nbsp;<a class=\"trait\" href=\"string_cache/trait.StaticAtomSet.html\" title=\"trait string_cache::StaticAtomSet\">StaticAtomSet</a>&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"string_cache/struct.Atom.html\" title=\"struct string_cache::Atom\">Atom</a>&lt;Static&gt;"]],
"tao":[["impl&lt;P&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tao/dpi/struct.LogicalPosition.html\" title=\"struct tao::dpi::LogicalPosition\">LogicalPosition</a>&lt;P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;P&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tao/dpi/struct.PhysicalPosition.html\" title=\"struct tao::dpi::PhysicalPosition\">PhysicalPosition</a>&lt;P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;P&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tao/dpi/struct.LogicalSize.html\" title=\"struct tao::dpi::LogicalSize\">LogicalSize</a>&lt;P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;P&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tao/dpi/struct.PhysicalSize.html\" title=\"struct tao::dpi::PhysicalSize\">PhysicalSize</a>&lt;P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/dpi/enum.Size.html\" title=\"enum tao::dpi::Size\">Size</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/dpi/enum.Position.html\" title=\"enum tao::dpi::Position\">Position</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tao/event/struct.RawKeyEvent.html\" title=\"struct tao::event::RawKeyEvent\">RawKeyEvent</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/event/enum.TouchPhase.html\" title=\"enum tao::event::TouchPhase\">TouchPhase</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/event/enum.TrayEvent.html\" title=\"enum tao::event::TrayEvent\">TrayEvent</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/event/enum.ElementState.html\" title=\"enum tao::event::ElementState\">ElementState</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/event/enum.MouseButton.html\" title=\"enum tao::event::MouseButton\">MouseButton</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/event/enum.MouseScrollDelta.html\" title=\"enum tao::event::MouseScrollDelta\">MouseScrollDelta</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tao/keyboard/struct.ModifiersState.html\" title=\"struct tao::keyboard::ModifiersState\">ModifiersState</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/keyboard/enum.NativeKeyCode.html\" title=\"enum tao::keyboard::NativeKeyCode\">NativeKeyCode</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/keyboard/enum.KeyCode.html\" title=\"enum tao::keyboard::KeyCode\">KeyCode</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/keyboard/enum.Key.html\" title=\"enum tao::keyboard::Key\">Key</a>&lt;'a&gt;"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/keyboard/enum.KeyLocation.html\" title=\"enum tao::keyboard::KeyLocation\">KeyLocation</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tao/window/enum.CursorIcon.html\" title=\"enum tao::window::CursorIcon\">CursorIcon</a>"]],
"tauri":[["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri/api/dir/struct.DiskEntry.html\" title=\"struct tauri::api::dir::DiskEntry\">DiskEntry</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri/api/ipc/struct.CallbackFn.html\" title=\"struct tauri::api::ipc::CallbackFn\">CallbackFn</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri/api/path/enum.BaseDirectory.html\" title=\"enum tauri::api::path::BaseDirectory\">BaseDirectory</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri/window/struct.Monitor.html\" title=\"struct tauri::window::Monitor\">Monitor</a>"]],
"tauri_runtime":[["impl&lt;P&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_runtime/window/dpi/struct.LogicalPosition.html\" title=\"struct tauri_runtime::window::dpi::LogicalPosition\">LogicalPosition</a>&lt;P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;P&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_runtime/window/dpi/struct.PhysicalPosition.html\" title=\"struct tauri_runtime::window::dpi::PhysicalPosition\">PhysicalPosition</a>&lt;P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;P&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_runtime/window/dpi/struct.LogicalSize.html\" title=\"struct tauri_runtime::window::dpi::LogicalSize\">LogicalSize</a>&lt;P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl&lt;P&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_runtime/window/dpi/struct.PhysicalSize.html\" title=\"struct tauri_runtime::window::dpi::PhysicalSize\">PhysicalSize</a>&lt;P&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_runtime/window/dpi/enum.Size.html\" title=\"enum tauri_runtime::window::dpi::Size\">Size</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_runtime/window/dpi/enum.Position.html\" title=\"enum tauri_runtime::window::dpi::Position\">Position</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_runtime/window/struct.MenuEvent.html\" title=\"struct tauri_runtime::window::MenuEvent\">MenuEvent</a>"]],
"tauri_utils":[["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.WindowUrl.html\" title=\"enum tauri_utils::config::WindowUrl\">WindowUrl</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.BundleType.html\" title=\"enum tauri_utils::config::BundleType\">BundleType</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.BundleTarget.html\" title=\"enum tauri_utils::config::BundleTarget\">BundleTarget</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.AppImageConfig.html\" title=\"struct tauri_utils::config::AppImageConfig\">AppImageConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.DebConfig.html\" title=\"struct tauri_utils::config::DebConfig\">DebConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.MacConfig.html\" title=\"struct tauri_utils::config::MacConfig\">MacConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.WixLanguageConfig.html\" title=\"struct tauri_utils::config::WixLanguageConfig\">WixLanguageConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.WixLanguage.html\" title=\"enum tauri_utils::config::WixLanguage\">WixLanguage</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.WixConfig.html\" title=\"struct tauri_utils::config::WixConfig\">WixConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.WebviewInstallMode.html\" title=\"enum tauri_utils::config::WebviewInstallMode\">WebviewInstallMode</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.WindowsConfig.html\" title=\"struct tauri_utils::config::WindowsConfig\">WindowsConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.BundleConfig.html\" title=\"struct tauri_utils::config::BundleConfig\">BundleConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.CliArg.html\" title=\"struct tauri_utils::config::CliArg\">CliArg</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.CliConfig.html\" title=\"struct tauri_utils::config::CliConfig\">CliConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.WindowConfig.html\" title=\"struct tauri_utils::config::WindowConfig\">WindowConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.CspDirectiveSources.html\" title=\"enum tauri_utils::config::CspDirectiveSources\">CspDirectiveSources</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.Csp.html\" title=\"enum tauri_utils::config::Csp\">Csp</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.DisabledCspModificationKind.html\" title=\"enum tauri_utils::config::DisabledCspModificationKind\">DisabledCspModificationKind</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.SecurityConfig.html\" title=\"struct tauri_utils::config::SecurityConfig\">SecurityConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.FsAllowlistScope.html\" title=\"enum tauri_utils::config::FsAllowlistScope\">FsAllowlistScope</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.FsAllowlistConfig.html\" title=\"struct tauri_utils::config::FsAllowlistConfig\">FsAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.WindowAllowlistConfig.html\" title=\"struct tauri_utils::config::WindowAllowlistConfig\">WindowAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.ShellAllowedCommand.html\" title=\"struct tauri_utils::config::ShellAllowedCommand\">ShellAllowedCommand</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.ShellAllowedArgs.html\" title=\"enum tauri_utils::config::ShellAllowedArgs\">ShellAllowedArgs</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.ShellAllowedArg.html\" title=\"enum tauri_utils::config::ShellAllowedArg\">ShellAllowedArg</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.ShellAllowlistScope.html\" title=\"struct tauri_utils::config::ShellAllowlistScope\">ShellAllowlistScope</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.ShellAllowlistOpen.html\" title=\"enum tauri_utils::config::ShellAllowlistOpen\">ShellAllowlistOpen</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.ShellAllowlistConfig.html\" title=\"struct tauri_utils::config::ShellAllowlistConfig\">ShellAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.DialogAllowlistConfig.html\" title=\"struct tauri_utils::config::DialogAllowlistConfig\">DialogAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.HttpAllowlistScope.html\" title=\"struct tauri_utils::config::HttpAllowlistScope\">HttpAllowlistScope</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.HttpAllowlistConfig.html\" title=\"struct tauri_utils::config::HttpAllowlistConfig\">HttpAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.NotificationAllowlistConfig.html\" title=\"struct tauri_utils::config::NotificationAllowlistConfig\">NotificationAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.GlobalShortcutAllowlistConfig.html\" title=\"struct tauri_utils::config::GlobalShortcutAllowlistConfig\">GlobalShortcutAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.OsAllowlistConfig.html\" title=\"struct tauri_utils::config::OsAllowlistConfig\">OsAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.PathAllowlistConfig.html\" title=\"struct tauri_utils::config::PathAllowlistConfig\">PathAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.ProtocolAllowlistConfig.html\" title=\"struct tauri_utils::config::ProtocolAllowlistConfig\">ProtocolAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.ProcessAllowlistConfig.html\" title=\"struct tauri_utils::config::ProcessAllowlistConfig\">ProcessAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.ClipboardAllowlistConfig.html\" title=\"struct tauri_utils::config::ClipboardAllowlistConfig\">ClipboardAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.AppAllowlistConfig.html\" title=\"struct tauri_utils::config::AppAllowlistConfig\">AppAllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.AllowlistConfig.html\" title=\"struct tauri_utils::config::AllowlistConfig\">AllowlistConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.PatternKind.html\" title=\"enum tauri_utils::config::PatternKind\">PatternKind</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.TauriConfig.html\" title=\"struct tauri_utils::config::TauriConfig\">TauriConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.UpdaterEndpoint.html\" title=\"struct tauri_utils::config::UpdaterEndpoint\">UpdaterEndpoint</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.WindowsUpdateInstallMode.html\" title=\"enum tauri_utils::config::WindowsUpdateInstallMode\">WindowsUpdateInstallMode</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.UpdaterWindowsConfig.html\" title=\"struct tauri_utils::config::UpdaterWindowsConfig\">UpdaterWindowsConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.UpdaterConfig.html\" title=\"struct tauri_utils::config::UpdaterConfig\">UpdaterConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.SystemTrayConfig.html\" title=\"struct tauri_utils::config::SystemTrayConfig\">SystemTrayConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.AppUrl.html\" title=\"enum tauri_utils::config::AppUrl\">AppUrl</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.BeforeDevCommand.html\" title=\"enum tauri_utils::config::BeforeDevCommand\">BeforeDevCommand</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/config/enum.HookCommand.html\" title=\"enum tauri_utils::config::HookCommand\">HookCommand</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.BuildConfig.html\" title=\"struct tauri_utils::config::BuildConfig\">BuildConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.PackageConfig.html\" title=\"struct tauri_utils::config::PackageConfig\">PackageConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.Config.html\" title=\"struct tauri_utils::config::Config\">Config</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"tauri_utils/config/struct.PluginConfig.html\" title=\"struct tauri_utils::config::PluginConfig\">PluginConfig</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/html/enum.PatternObject.html\" title=\"enum tauri_utils::html::PatternObject\">PatternObject</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/html/enum.IsolationSide.html\" title=\"enum tauri_utils::html::IsolationSide\">IsolationSide</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/enum.TitleBarStyle.html\" title=\"enum tauri_utils::TitleBarStyle\">TitleBarStyle</a>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"tauri_utils/enum.Theme.html\" title=\"enum tauri_utils::Theme\">Theme</a>"]],
"url":[["impl&lt;S&gt; <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"url/enum.Host.html\" title=\"enum url::Host\">Host</a>&lt;S&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,</span>"],["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"struct\" href=\"url/struct.Url.html\" title=\"struct url::Url\">Url</a>"]],
"wry":[["impl <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> for <a class=\"enum\" href=\"wry/webview/enum.FileDropEvent.html\" title=\"enum wry::webview::FileDropEvent\">FileDropEvent</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()