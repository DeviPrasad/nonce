(function() {
    let doc = document;
    loiter_signin_with_indus_id = () => {
        console.log("loiter_signin_with_indus_id");
        let session = {
            success: function() {
                console.log("loiter_signin_with_indus_id : ", this.xhr);
                try {
                    let res = JSON.parse(this.xhr.response);
                    console.log("loiter_signin_with_indus_id - after redirect: ",res);
                } catch (ex) { console.error(ex); }
            }
        };
        makeAsyncHttpGetRequest("lobby", session);
    }
    doc.addEventListener('DOMContentLoaded', (event) => {
        console.log("DOMContentLoaded");
        idSigninUser = doc.getElementById("loiter_signin_with_indus_id");
        if (idSigninUser) {
            idSigninUser.addEventListener('click', loiter_signin_with_indus_id);
        }
    });
    function originLoiterWebClient() {
        return window.location.origin
    }
    function getSessionUserId () {
        let loiter_uid = null;
        const strUser = sessionStorage.getItem("loiter_user");
        if (strUser) {
            let user = JSON.parse(strUser);
            if (user) {
                console.log("getLoggedInUserId", user);
                loiter_uid = user.loiter_uid;
            }
        }
        return loiter_uid;
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
            window.alert("Loiter - Unknwon Error.");
            console.log(xhr);
        }
        let onload = () => {
            console.log("xhr response: ", xhr);
            if (!(xhr.readyState == 4 && xhr.status < 400)) {
                receiver.error();
                xhr.abort();
                return;
            }
            if (xhr.status == 200) {
                if (!(xhr.responseType && xhr.responseType.toLowerCase() === "json") && xhr.responseURL) {
                    const goto = xhr.responseURL + "?session_state=ASD123";
                    window.alert("redirecting to " + goto);
                    this.window.location.replace(goto);
                } else if (xhr.responseType && xhr.responseType.toLowerCase() === "json"){
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
