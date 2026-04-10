// Test that preserve_order=true preserves declaration order,
// and preserve_order=false (default) sorts alphabetically.

local obj = { c: 1, b: 2, a: 3 };

// objectFields
std.assertEqual(std.objectFields(obj), ['a', 'b', 'c']) &&
std.assertEqual(std.objectFields(obj, preserve_order=true), ['c', 'b', 'a']) &&
std.assertEqual(std.objectFields(obj, preserve_order=false), ['a', 'b', 'c']) &&

// objectFieldsAll (same object, no hidden fields)
std.assertEqual(std.objectFieldsAll(obj, preserve_order=true), ['c', 'b', 'a']) &&

// objectValues
std.assertEqual(std.objectValues(obj), [3, 2, 1]) &&
std.assertEqual(std.objectValues(obj, preserve_order=true), [1, 2, 3]) &&

// objectKeysValues
std.assertEqual(
  std.objectKeysValues(obj, preserve_order=true),
  [{key: 'c', value: 1}, {key: 'b', value: 2}, {key: 'a', value: 3}]
) &&

// manifestJson
std.assertEqual(
  std.manifestJsonMinified(obj, preserve_order=true),
  '{"c":1,"b":2,"a":3}'
) &&
std.assertEqual(
  std.manifestJsonMinified(obj, preserve_order=false),
  '{"a":3,"b":2,"c":1}'
) &&

// Overridden fields: the override takes the position from the overriding object
local base = { a: 1, b: 2, c: 3 };
local derived = base { b: 20 };
std.assertEqual(std.objectFields(derived, preserve_order=true), ['a', 'c', 'b']) &&
std.assertEqual(std.objectValues(derived, preserve_order=true), [1, 3, 20]) &&

// Object comprehension preserves insertion order
local comp = { [std.toString(i)]: i for i in [3, 1, 2] };
std.assertEqual(std.objectFields(comp, preserve_order=true), ['3', '1', '2']) &&

true
