var getSame = require('../app.js');

describe('Routing', function() {
    describe('checking work algorithm shingles', function() {
        it('two similar text', function(done) {
            var res = getSame("rust rust", "rust rust", 2);
            if(res != 100) {
                throw new Error('not correct working');
            }
            done();
        });
        it('two not similar text', function(done) {
            var res = getSame("rust c++", "rust rust", 2);
            if(res != 0) {
                throw new Error('not correct working');
            }
            done();
        });
    });

});
