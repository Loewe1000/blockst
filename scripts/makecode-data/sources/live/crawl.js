// Run in the browser console of a MakeCode editor (makecode.microbit.org or
// makecode.calliope.cc, opened with ?lang=de#editor or ?lang=en#editor).
// Walks every toolbox category, reads each flyout block through Blockly's
// API and prints one JSON line per block: id, colour, shape, inline flag,
// the block text with %n placeholders, the mouths (statement inputs) with
// their labels, the slot kinds and the default shadows. The *.jsonl files
// beside this script are its output; extract.py turns a saved console dump
// into them. MakeCode loads its translations at run time and publishes no
// per-block string files, so the running editor is the only source for the
// German (and the exact English) block texts.
(async function () {
  const B = pxt.blocks.requireBlockly(); const ws = B.getMainWorkspace();
  const rows0 = Array.from(document.querySelectorAll('.blocklyTreeRow'));
  const adv = rows0.find(r => /Fortgeschritten|Advanced/.test(r.textContent.trim())); if (adv) { adv.click(); await new Promise(r => setTimeout(r, 700)); }
  const rows = Array.from(document.querySelectorAll('.blocklyTreeRow')).filter(r => !/Fortgeschritten|Advanced|Erweiterungen|Extensions/.test(r.textContent.trim()));
  function kindOf(f) { if (f instanceof B.FieldLabel || f instanceof B.FieldImage) return 'label'; if (f instanceof B.FieldVariable) return 'variable'; if (f instanceof B.FieldDropdown) return 'dropdown'; if (f instanceof B.FieldNumber) return 'number'; if (f instanceof B.FieldTextInput) return 'text'; if (f instanceof B.FieldCheckbox) return 'checkbox'; return 'field'; }
  function spec(b) {
    const slots = [], defaults = [], mouths = []; let n = 0, cur = [];
    b.inputList.forEach(i => {
      if (i.type === 3) { const lab = i.fieldRow.filter(f => kindOf(f) === 'label' && !(f instanceof B.FieldImage)).map(f => f.getText().trim()).filter(Boolean).join(' '); mouths.push({head: cur.join(' '), label: lab}); cur = []; return; }
      i.fieldRow.forEach(f => { const k = kindOf(f); if (f instanceof B.FieldImage) return; const t = (f.getText ? f.getText() : '').trim(); if (k === 'label') { if (t) cur.push(t); } else { n++; cur.push('%' + n); slots.push(k); defaults.push(t); } });
      if (i.type === 1) { n++; cur.push('%' + n); const tb = i.connection && i.connection.targetBlock(); const chk = (i.connection.getCheck() || []).join('|'); slots.push(chk === 'Boolean' ? 'boolean' : 'value'); defaults.push(tb ? tb.type + '=' + (tb.inputList[0] && tb.inputList[0].fieldRow[0] && tb.inputList[0].fieldRow[0].getText ? tb.inputList[0].fieldRow[0].getText() : '') : ''); }
    });
    const tail = cur.join(' ');
    const shape = b.outputConnection ? ((b.outputConnection.getCheck() || []).join('|') === 'Boolean' ? 'boolean' : 'reporter') : (!b.previousConnection ? (mouths.length ? 'event' : 'hat') : (!b.nextConnection ? 'cap' : (mouths.length ? 'c-block' : 'stack')));
    return {t: b.type, c: b.getColour(), s: shape, inline: b.getInputsInline(), spec: mouths.length ? mouths[0].head : tail, mouths, tail: mouths.length ? tail : '', slots, def: defaults};
  }
  window.__cat = {target: pxt.appTarget.id, versions: pxt.appTarget.versions, lines: []};
  let prev = 0; const seen = {};
  for (const row of rows) {
    const label = row.textContent.trim(); const ns = row.getAttribute('data-ns') || '';
    row.click(); await new Promise(r => setTimeout(r, 800));
    const fl = ws.getFlyout ? ws.getFlyout() : ws.getToolbox().getFlyout(); const fws = fl.getWorkspace();
    const all = fws.getTopBlocks(true); const own = all.slice(prev); prev = all.length;
    own.forEach(b => { const key = b.type + '#' + b.inputList.length; if (seen[key]) return; seen[key] = 1; const s = spec(b); s.cat = ns; s.catLabel = label; window.__cat.lines.push(JSON.stringify(s)); });
  }
  console.log(window.__cat.lines.join('\n'));
})();
