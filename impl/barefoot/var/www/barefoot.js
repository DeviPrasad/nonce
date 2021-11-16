"use strict";

(function() {
    let doc = document;
    doc.addEventListener('DOMContentLoaded', (event) => {
        console.log("DOMContentLoaded");
    });
    function barefootOrigin() {
        return window.location.origin
    }
    function getSessionSole () {
        let loiter_uid = null;
        const strUser = sessionStorage.getItem("barefoot-sole");
        if (strUser) {
            let sole = JSON.parse(strUser);
            if (sole) {
                console.log("getSessionSole", sole);
                sole_did = sole.did;
            }
        }
        return sole_did;
    }

    function asyncFetch(receiver, pathServlet)
    {
        const urlLwc = originLoiterWebClient();
        receiver.onResult = receiver.onResult.bind(receiver);
        receiver.onError = receiver.onError.bind(receiver);
        (async function(sink) {
            try {
                console.log("asyncFetch: ", pathServlet);
                const response = await fetch(urlLwc + pathServlet);
                console.log("await ok?: ", response.ok);
                if (response.ok) {
                    const json = await response.json();
                    sink.onResult(json)
                } else {
                    console.log("Request Timeout Error - Please try again.");
                    sink.onError();
                }
           } catch (err) {
                console.log("Request Timeout Exception - Please try again.");
                sink.onError(err);
           }
          })(receiver);
    }
    function makeAsyncHttpGetRequest(restResource, receiver)
    {
        const urlLwc = originLoiterWebClient();
        let xhr = new XMLHttpRequest();
        receiver.xhr = xhr;
        let genericError = function() {
            console.log(xhr);
            window.alert("Loiter - Unknwon Error." + xhr.responseURL);
        }
        let onload = () => {
            console.log("xhr response: ", xhr);
            if (!(xhr.readyState == 4 && xhr.status < 400)) {
                console.log("xhr response: ", xhr);
                //receiver.error();
                //xhr.abort();
                return;
            }
            if (xhr.status == 200) {
                if (!(xhr.responseType && xhr.responseType.toLowerCase() === "json") && xhr.responseURL) {
                    const goto = xhr.responseURL + "?session_state=ASD123";
                    window.alert("redirecting to " + goto);
                    this.window.location.replace(goto);
                } else if (xhr.responseType && xhr.responseType.toLowerCase() === "json") {
                    receiver.success.bind(receiver);
                    receiver.success();
                } else {
                    console.error("Bad Content Type. Response rejected.");
                }
            } else {
                console.warn("Bad Request or unhandled status code: ", xhr);
            }
        }
        xhr.onload = onload;
        receiver.error = !receiver.error ? genericError : receiver.error;
        xhr.onerror = receiver.error.bind(receiver);
        xhr.open("GET", urlLwc + "/" + restResource, true);
        xhr.send(null);
    }

    function makeAsyncHttpPostRequest(restResource, payload, receiver, success, error)
    {
        let id = getLoggedInUserId();
        payload.caller = (json != null && id) ? id : null;
        const urlLwc = originLoiterWebClient();
        var req = new XMLHttpRequest();
        req.open("POST", urlLwc + restRes, true);
        receiver.xhr = req;
        req.onload = success.bind(receiver);
        req.onerror = error.bind(req);
        req.setRequestHeader("Content-type", "application/json;charset=utf-8");
        req.send(JSON.stringify(payload));
    }
})()
