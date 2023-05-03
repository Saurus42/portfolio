import init from './wasm.js';
init().then( result => {
    result.run();
} ).catch( () => alert( 'Masz przestarzałą przeglądarkę. Proszę ją zaktualizować.' ) );