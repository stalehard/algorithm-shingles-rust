// mylib.js
var FFI = require('ffi');

var lib = FFI.Library('./target/debug/liblibshingles.so', {
    'count_same': [ 'int', [ 'string', 'string', 'int' ] ]
});

module.exports = lib.count_same;