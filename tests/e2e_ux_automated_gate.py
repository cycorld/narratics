#!/usr/bin/env python3
"""
Narratics E2E Interactive UX Verification Gate
Tests real browser interaction, keyboard events, DOM scroll geometry,
and reactive state synchronization to eliminate manual QA regressions.
"""

import subprocess
import json
import sys
import time

def run_browser_eval(script: str) -> dict:
    # Escape quotes for bash execution
    cmd = ["agent-browser", "eval", script]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode != 0:
        raise RuntimeError(f"agent-browser eval failed: {res.stderr}")
    out = res.stdout.strip()
    try:
        data = json.loads(out)
        if isinstance(data, dict):
            return data
        return {"value": data}
    except json.JSONDecodeError:
        return {"raw": out}

def run_tests():
    print("==================================================")
    print("Narratics E2E Interactive UX Verification Gate")
    print("==================================================")
    
    # 0. Open page in desktop viewport
    subprocess.run(["agent-browser", "set", "viewport", "1280", "800"], check=True)
    subprocess.run(["agent-browser", "open", "http://127.0.0.1:3901/app"], check=True)
    time.sleep(1.0)
    
    tests_passed = 0
    total_tests = 6

    # Test 1: @ Mention Autocomplete Keyboard Navigation
    print("\n[1/5] Testing @ Mention Autocomplete Keyboard Navigation...")
    script_1 = """(() => {
        const ta = document.getElementById('manuscript-text-editor');
        const popup = document.getElementById('mention-popup');
        
        ta.value = '집필 테스트 중 인물 호출 @';
        ta.selectionStart = ta.value.length;
        ta.selectionEnd = ta.value.length;
        ta.dispatchEvent(new Event('input'));
        
        const popupVisible = popup.style.display === 'block';
        
        // Down
        ta.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowDown', bubbles: true, cancelable: true }));
        const items = Array.from(popup.querySelectorAll('.mention-item'));
        const isItem1Selected = items[1] && items[1].classList.contains('selected');
        
        // Enter
        ta.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true, cancelable: true }));
        const popupHiddenAfterEnter = popup.style.display === 'none';
        const hasInserted = ta.value.includes('@에테르 결정');
        
        return {
            popupVisible,
            isItem1Selected,
            popupHiddenAfterEnter,
            hasInserted
        };
    })()"""
    res1 = run_browser_eval(script_1)
    assert res1.get("popupVisible") is True, f"Popup failed to appear: {res1}"
    assert res1.get("isItem1Selected") is True, f"ArrowDown failed to select item 1: {res1}"
    assert res1.get("popupHiddenAfterEnter") is True, f"Enter failed to close popup: {res1}"
    assert res1.get("hasInserted") is True, f"Mention text was not inserted: {res1}"
    print("  ✓ Autocomplete Trigger -> ArrowDown Navigation -> Enter Insertion verified.")
    tests_passed += 1

    # Test 2: Typewriter Mode Vertical Ergonomics (45% Rule)
    print("\n[2/5] Testing Typewriter Mode Vertical Ergonomics (45% Rule)...")
    script_2 = r"""(async () => {
        const ta = document.getElementById('manuscript-text-editor');
        const btn = document.getElementById('btn-typewriter-mode');
        
        if (!btn.classList.contains('btn-active')) {
            btn.click();
        }
        ta.focus();
        
        let text = '';
        for (let i = 1; i <= 30; i++) {
            text += `단락 ${i} — 서사의 전개와 갈등의 고조를 기록하는 문단입니다.\n`;
        }
        ta.value = text;
        ta.selectionStart = text.length;
        ta.selectionEnd = text.length;
        ta.dispatchEvent(new Event('input'));
        ta.dispatchEvent(new KeyboardEvent('keyup', { key: 'ArrowDown' }));
        
        await new Promise(r => setTimeout(r, 400));
        
        const clientH = ta.clientHeight;
        const scrollT = ta.scrollTop;
        const mirror = document.getElementById('typewriter-cursor-mirror');
        const markerTop = mirror && mirror.lastElementChild ? mirror.lastElementChild.offsetTop : 0;
        const cursorY = markerTop - scrollT;
        const ratio = clientH > 0 ? (cursorY / clientH) : 0;
        
        return {
            typewriterActive: btn.classList.contains('btn-active'),
            clientH,
            scrollT,
            cursorY,
            ratio: Math.round(ratio * 100) / 100
        };
    })()"""
    res2 = run_browser_eval(script_2)
    assert res2.get("typewriterActive") is True, f"Typewriter mode not active: {res2}"
    assert int(res2.get("scrollT") or 0) > 0, f"Textarea failed to auto-scroll in typewriter mode: {res2}"
    ratio_val = float(res2.get("ratio") or 0)
    assert 0.35 <= ratio_val <= 0.65, f"Cursor Y ratio out of 45% ergonomic bounds: {res2}"
    print(f"  ✓ Typewriter scroll locked near 45% (Measured ratio: {res2.get('ratio')}).")
    tests_passed += 1

    # Test 3: Chapter Reparenting (Move Scene)
    print("\n[3/5] Testing Scene Reparenting (Move Node across Chapters)...")
    script_3 = """(async () => {
        const select = document.getElementById('current-scene-parent-select');
        if (!select) return { error: 'select not found' };
        
        // Select chap_2
        select.value = 'chap_2';
        select.dispatchEvent(new Event('change'));
        await new Promise(r => setTimeout(r, 200));
        
        // Return active scene info
        const activeScene = state.binder.find(b => b.id === state.active_scene_id);
        return {
            activeSceneId: state.active_scene_id,
            parent: activeScene ? activeScene.parent : null
        };
    })()"""
    res3 = run_browser_eval(script_3)
    assert res3.get("parent") == "chap_2", f"Scene reparent to chap_2 failed: {res3}"
    
    # Revert back to chap_1
    script_3_revert = """(async () => {
        const select = document.getElementById('current-scene-parent-select');
        select.value = 'chap_1';
        select.dispatchEvent(new Event('change'));
        await new Promise(r => setTimeout(r, 200));
        const activeScene = state.binder.find(b => b.id === state.active_scene_id);
        return { parent: activeScene ? activeScene.parent : null };
    })()"""
    res3_rev = run_browser_eval(script_3_revert)
    assert res3_rev.get("parent") == "chap_1", f"Reverting scene to chap_1 failed: {res3_rev}"
    print("  ✓ Reparenting to Chapter 2 and reverting to Chapter 1 verified.")
    tests_passed += 1

    # Test 4: Corkboard Active Chapter Synchronization
    print("\n[4/5] Testing Corkboard Mode Chapter Synchronization...")
    script_4 = """(() => {
        // Switch to corkboard
        const btnCork = document.getElementById('btn-view-corkboard');
        btnCork.click();
        
        // Click chap_2 folder
        const chap2El = document.querySelector('[data-id="chap_2"]');
        if (chap2El) chap2El.click();
        
        const selectedChap2 = document.getElementById('corkboard-folder-select').value;
        
        // Click scene_1 (which is in chap_1)
        const scene1El = document.querySelector('[data-id="scene_1"]');
        if (scene1El) scene1El.click();
        
        const selectedChap1 = document.getElementById('corkboard-folder-select').value;
        const activeCard = document.querySelector('.index-card.active-scene-card');
        const activeCardId = activeCard ? activeCard.getAttribute('data-id') : null;
        const folderTitle = document.getElementById('corkboard-folder-title').textContent;
        
        // Switch back to editor
        document.getElementById('btn-view-editor').click();
        
        return {
            folderTitle,
            selectedChap2,
            selectedChap1,
            activeCardId
        };
    })()"""
    res4 = run_browser_eval(script_4)
    assert res4.get("selectedChap2") == "chap_2", f"Corkboard failed to sync to Chapter 2: {res4}"
    assert res4.get("selectedChap1") == "chap_1", f"Corkboard failed to sync to Chapter 1: {res4}"
    assert res4.get("activeCardId") == "scene_1", f"Corkboard active card highlight mismatch: {res4}"
    print("  ✓ Corkboard auto-syncs to clicked chapter and active scene card.")
    tests_passed += 1

    # Test 5: Reversible Undo Toast
    print("\n[5/5] Testing Reversible Undo Notification...")
    script_5 = """(() => {
        showUndoToast('서사 단락이 삭제되었습니다.', () => {
            window.__test_undo_triggered = true;
        });
        
        const toast = document.getElementById('undo-toast');
        const isVisible = toast && toast.style.display === 'flex';
        const actionBtn = document.getElementById('btn-undo-action');
        if (actionBtn) actionBtn.click();
        
        return {
            isVisible,
            undoTriggered: !!window.__test_undo_triggered
        };
    })()"""
    res5 = run_browser_eval(script_5)
    assert res5.get("isVisible") is True, f"Toast failed to appear: {res5}"
    assert res5.get("undoTriggered") is True, f"Toast undo action callback not fired: {res5}"
    print("  ✓ Toast notification and undo callback execution verified.")
    tests_passed += 1

    # Test 6: Mobile Ergonomics & Drawer Navigation (390x844)
    print("\n[6/6] Testing Mobile Ergonomics & Drawer Navigation (390x844)...")
    subprocess.run(["agent-browser", "set", "viewport", "390", "844"], check=True)
    time.sleep(0.5)

    script_6 = """(() => {
        const quickBar = document.getElementById('mobile-quick-bar');
        const binder = document.getElementById('binder-panel');
        const inspector = document.getElementById('inspector-panel');
        const backdrop = document.getElementById('mobile-backdrop');
        const mBtnBinder = document.getElementById('m-btn-binder');
        const mBtnInspector = document.getElementById('m-btn-inspector');
        
        const isQuickBarVisible = quickBar && window.getComputedStyle(quickBar).display !== 'none';
        
        // 1. Open Binder Drawer via quick button
        mBtnBinder.click();
        const binderOpened = binder.classList.contains('mobile-open') && backdrop.classList.contains('visible');
        
        // 2. Close via backdrop click
        backdrop.click();
        const binderClosed = !binder.classList.contains('mobile-open') && !backdrop.classList.contains('visible');
        
        // 3. Open Inspector Drawer via quick button
        mBtnInspector.click();
        const inspectorOpened = inspector.classList.contains('mobile-open') && backdrop.classList.contains('visible');
        
        // 4. Close via backdrop click
        backdrop.click();
        const inspectorClosed = !inspector.classList.contains('mobile-open') && !backdrop.classList.contains('visible');
        
        return {
            isQuickBarVisible,
            binderOpened,
            binderClosed,
            inspectorOpened,
            inspectorClosed
        };
    })()"""
    res6 = run_browser_eval(script_6)
    assert res6.get("isQuickBarVisible") is True, f"Mobile quick bar not visible on 390x844: {res6}"
    assert res6.get("binderOpened") is True, f"Binder drawer failed to open on mobile: {res6}"
    assert res6.get("binderClosed") is True, f"Backdrop failed to close binder drawer: {res6}"
    assert res6.get("inspectorOpened") is True, f"Inspector drawer failed to open on mobile: {res6}"
    assert res6.get("inspectorClosed") is True, f"Backdrop failed to close inspector drawer: {res6}"
    
    # Restore viewport to desktop
    subprocess.run(["agent-browser", "set", "viewport", "1280", "800"], check=True)
    print("  ✓ Mobile quick bar visible, drawer overlays & backdrop dismissal 100% verified.")
    tests_passed += 1

    print("\n==================================================")
    print(f"ALL {tests_passed}/{total_tests} E2E INTERACTIVE UX TESTS PASSED (0 FAIL)")
    print("==================================================")

if __name__ == "__main__":
    try:
        run_tests()
    except Exception as e:
        print(f"\n❌ E2E Gate Failure: {e}", file=sys.stderr)
        sys.exit(1)
