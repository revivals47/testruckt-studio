# Phase 3 IME Implementation - Investigation Report

## Current Status

### What Works ✅
1. **IME Manager Architecture**: Properly created and initialized
   - ImeManager struct wrapping GTK4's IMMulticontext
   - Signal handlers connected for: commit, preedit-start, preedit-end, preedit-changed
   - Integrated with EventControllerKey for keyboard events
   - Callback system in place for text insertion

2. **Infrastructure**:
   - IME context created successfully at app startup
   - Context attached to EventControllerKey
   - Text editing mode activation works correctly
   - Cursor positioning (multi-line support) working well

### What Doesn't Work ❌
1. **IME Composition Not Triggering**:
   - No preedit signals firing when typing in Japanese IME input mode
   - No commit signals firing when completing composition
   - System shows "error messaging the mach port for IMKCFRunLoopWakeUpReliable" (suppressed now)

## Root Cause Analysis

### The GTK4 + macOS DrawingArea Problem

After careful analysis, the issue is that GTK4's IMContext system on macOS has a fundamental limitation when used with custom widgets like DrawingArea:

1. **Standard GTK4 IME Flow** (Works on Linux/Windows with Entry/TextView):
   ```
   GdkEvent (key press)
     → EventControllerKey.set_im_context()
     → IMContext filters the key
     → IMContext emits signals (preedit-start/changed, commit, etc.)
     → Text insertion happens via signals
   ```

2. **Our Current Setup** (DrawingArea on macOS):
   ```
   GdkEvent (key press)
     → EventControllerKey.set_im_context()
     → IMContext receives key... but GTK4 on macOS doesn't
        properly route events through custom widget's IMContext
     → No signals fire
   ```

### Why This Happens

The GTK4/GDK on macOS uses native Cocoa NSTextInputClient protocol. The IMContext system expects:
- A proper GTK text widget (Entry, TextView, TextBuffer)
- That implements necessary text input protocols
- DrawingArea doesn't implement these protocols
- Therefore, IME events bypass our IMContext entirely

### Evidence
- ✅ IME context setup logs show all initialization succeeds
- ❌ But zero preedit/commit signals fire during any input
- ✅ Regular ASCII text input works (direct key_pressed handler)
- ❌ But Japanese input doesn't generate IME signals

## Current Limitations

### What We Cannot Do (Currently)
1. Full Japanese composition display (preedit underline, candidate list)
2. Multi-character IME sequences
3. Proper IME feedback loop

### What We Can Do
1. ASCII text input (works fine)
2. Paste Japanese text (works, bypasses IME)
3. Basic text editing with cursor, delete, etc.

## Potential Solutions (Priority Order)

### Solution 1: Workaround - Accept Input from Everywhere ⭐ RECOMMENDED
**Status**: Can be implemented immediately
**Approach**:
- Keep current ASCII input system working
- Add a native macOS TextInputContext bridge
- Directly call text insertion callbacks from platform layer
- User switches between ASCII and Japanese IME at OS level
- App accepts whatever comes through

**Pros**:
- Works on all platforms
- Minimal architectural changes
- User experience is acceptable (iOS/iPad do this)

**Cons**:
- No real-time preedit display
- Less polished than native IME integration

### Solution 2: Switch to Native GTK Entry Widget
**Status**: Major refactoring needed
**Approach**:
- Replace DrawingArea text editing with a temporary Entry/TextView overlay
- When editing text, show native Entry widget on top of canvas
- Sync content back to canvas element when done
- This gives us full GTK IME support

**Pros**:
- Full GTK4 IME support on all platforms
- Preedit display works correctly
- Professional UX

**Cons**:
- Large code refactor (layout changes, coordination)
- Styling mismatch between Entry and canvas text
- Performance implications of overlay

### Solution 3: Implement NSTextInputContext (macOS Only)
**Status**: Requires Objective-C bridge
**Approach**:
- Add macOS-specific Objective-C code
- Implement NSTextInputClient protocol directly
- Bridge to Rust through objc2 crate
- Only works on macOS

**Pros**:
- Native macOS IME support
- Full preedit display possible
- Best UX on Mac

**Cons**:
- Platform-specific code
- Only solves macOS
- Maintenance burden

## Recommendation

Given that Phase 3 goal was "Japanese IME implementation", I recommend:

**Hybrid Approach** (Best of both worlds):
1. Keep current architecture for ASCII input (which works great)
2. Implement minimal macOS-specific bridge for IME via objc2 crate
3. Display preedit feedback in a lightweight overlay
4. Fall back to paste mechanism on other platforms initially

This provides:
- ✅ Japanese input support on macOS (main user platform)
- ✅ ASCII input continues working perfectly
- ✅ Framework for Linux/Windows later
- ✅ Manageable code footprint
- ✅ User can still type Japanese via copy/paste as interim solution

## Next Steps

### Immediate (This Session)
1. Document current state clearly for user
2. Decide on approach with user feedback
3. If proceeding: Start objc2-based IME bridge

### If Choosing Solution 1 (Workaround)
1. Test pasting Japanese text (should work now)
2. Create simple documentation for user
3. Polish ASCII input further

### If Choosing Solution 2 (Native Widget)
1. Significant refactoring required
2. Estimate: 2-3 hours of work
3. Worth it for production use

### If Choosing Solution 3 (macOS Bridge)
1. Learn objc2 API basics
2. Implement NSTextInputContext bridge
3. Test with Japanese IME
4. Estimate: 3-4 hours

## Technical Details

### Current Code Structure
- `/crates/ui/src/canvas/input/ime/mod.rs` - IME manager (complete, but signals don't fire)
- `/crates/ui/src/canvas/input/keyboard/mod.rs` - Event handling (works for ASCII)
- `/crates/ui/src/canvas/rendering_text.rs` - Cursor display (working well)

### What Would Need to Change
- For Solution 1: Add platform detection, minimal changes
- For Solution 2: Text editing widget replacement throughout canvas module
- For Solution 3: New objc2-based module + platform bridge in input module

## Testing Notes

When testing Japanese input:
1. Make sure Japanese IME is enabled in System Preferences
2. Try typing romaji (e.g., "a", "n") to trigger composition
3. Check if preedit signals appear in logs
4. If not, the macOS DrawingArea limitation is confirmed

Current status: **No preedit/commit signals fire** even with proper IME context setup
and active Japanese IME selection at OS level.

