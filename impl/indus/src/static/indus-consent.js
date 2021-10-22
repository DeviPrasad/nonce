(function() {
    let doc = this.document;
    let consent = null;
    function Consent() {
        this.onCreate = () => {
            console.log("Consent object created.");
        }
        this.onPrepare = () => {
            console.log("Consent object is prepared.");
        }
        this.onUnload = () => {
            console.error("Consent object is unloaded.");
        }
        return this;
    }
    doc.addEventListener('DOMContentLoaded', (event) => {
        consent = new Consent();
        consent.onCreate();
    });
    window.addEventListener('load', (event) => {
        consent.onPrepare();
    });
    window.addEventListener('beforeunload',  (event) => {
        event.preventDefault();
        consent.onUnload();
    }, {capture: false});
})()