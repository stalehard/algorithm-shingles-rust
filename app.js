// mylib.js
var FFI = require('ffi');

var lib = FFI.Library('./target/debug/liblibshingles.so', {
    'count_same': [ 'int', [ 'string', 'string', 'int' ] ]
});

module.exports = lib.count_same;


//var res = lib.count_same("rust c++", "rust rust", 2);
//console.log(res);
