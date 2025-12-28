# Build Notes - Remaining Issues

The project is mostly complete but has some compilation errors that need to be fixed:

## Remaining Compilation Errors (~12-13 errors)

### 1. DBus Service - `incoming` method
**File**: `src/dbus_service.rs`
**Issue**: `dbus::blocking::Connection` doesn't have an `incoming()` method
**Solution**: The DBus service implementation needs to be restructured. Consider:
- Using `dbus-tokio` for async DBus
- Or using a simpler polling approach with `conn.process()` and checking for messages differently
- Or using the dbus crate's `MatchRule` and message filtering

### 2. GTK Widgets Not Send/Sync
**File**: `src/app.rs`
**Issue**: GTK widgets (TextView, etc.) cannot be moved into async blocks because they're not `Send`
**Current code** tries to move `definition_view` into async block
**Solution**: The code already uses `MainContext::invoke` correctly, but needs to avoid moving the widget. Options:
- Don't use `move` on the async block, clone only the needed data
- Or restructure to get a clone of the buffer/text before the async block

### 3. TextBuffer set_editable trait bounds
**File**: `src/ui.rs`
**Issue**: `set_editable` method has unsatisfied trait bounds
**Solution**: May need to use a different method or ensure proper trait imports

## Quick Fixes Applied

1. ✅ Fixed version conflict (gtk4 0.8 → 0.7)
2. ✅ Switched to rustls (no OpenSSL dependency)
3. ✅ Fixed build.rs (removed GResource requirement)
4. ✅ Fixed glib channel API (using std::sync::mpsc instead)
5. ✅ Fixed DBus method_call API usage
6. ✅ Fixed gtk4 imports
7. ✅ Fixed api.rs syntax error

## Next Steps

1. Fix DBus service implementation (use proper blocking API or switch to async)
2. Fix GTK widget Send/Sync issues in async contexts
3. Fix TextBuffer set_editable call
4. Test compilation

The core architecture is sound - these are API usage issues that can be resolved with proper GTK4/DBus patterns.


